//! Join and decode parts of a string using the given encoding.
use std::io::Read as _;

use data_encoding::BASE32_NOPAD;
use data_encoding::HEXUPPER;

use crate::encode::Encoding;

/// Errors that can occur when decoding data
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DecodeError {
    #[error("Unable to decode hex part: {0}, error: {1}")]
    UnableToDecodeHex(usize, data_encoding::DecodeError),

    #[error("Unable to decode base32 part: {0}, error: {1}")]
    UnableToDecodeBase32(usize, data_encoding::DecodeError),

    #[error("Unable decompress zlib data: {0}")]
    UnableToInflateZlib(String),

    #[error("Decompressed payload exceeds the {limit} byte limit")]
    DecompressedTooLarge { limit: usize },
}

/// Ceiling on the size of an inflated `Zlib` payload.
///
/// A BBQr stream can carry at most [`MAX_PARTS`] parts of roughly 2.6 KB of
/// base32-decoded bytes each, so under 4 MB of *compressed* data — but nothing
/// bounds what that inflates to, and a few hundred bytes of DEFLATE expand to
/// tens of megabytes.
///
/// 16 MiB sits far above anything BBQr is used to carry, since the PSBTs and
/// transactions it was designed for are well under a megabyte even for large
/// multisig, and far below what would exhaust a mobile process.
pub const MAX_DECOMPRESSED_SIZE: usize = 16 * 1024 * 1024;

pub(crate) fn decode_ordered_parts(
    parts: &[String],
    encoding: Encoding,
) -> Result<Vec<u8>, DecodeError> {
    let decoded: Vec<u8> = match encoding {
        // `collect` into a Result, not `flat_map`: a Result iterates as empty
        // for Err, so flat_map silently dropped the failing part's bytes and
        // concatenated the rest. The caller then parsed a short payload as a
        // PSBT with no indication anything had gone wrong.
        Encoding::Hex => parts
            .iter()
            .enumerate()
            .map(|(index, part)| {
                HEXUPPER
                    .decode(part.as_bytes())
                    .map_err(|error| DecodeError::UnableToDecodeHex(index, error))
            })
            .collect::<Result<Vec<Vec<u8>>, DecodeError>>()?
            .concat(),

        Encoding::Base32 => decode_and_join_base32_parts(parts)?,

        Encoding::Zlib => {
            let bytes = decode_and_join_base32_parts(parts)?;

            // inflate zlib encoded data
            let decompress = flate2::Decompress::new_with_window_bits(false, 10);
            let decoder = flate2::read::ZlibDecoder::new_with_decompress(&bytes[..], decompress);

            // Read one byte past the limit so hitting it is distinguishable
            // from a payload that merely ends there. Without a bound, a small
            // hostile frame inflates until the process is killed: a few
            // hundred bytes of DEFLATE expand to tens of megabytes.
            let mut decoded = Vec::new();
            let mut limited = decoder.take(MAX_DECOMPRESSED_SIZE as u64 + 1);
            limited
                .read_to_end(&mut decoded)
                .map_err(|e| DecodeError::UnableToInflateZlib(e.to_string()))?;

            if decoded.len() > MAX_DECOMPRESSED_SIZE {
                return Err(DecodeError::DecompressedTooLarge {
                    limit: MAX_DECOMPRESSED_SIZE,
                });
            }

            decoded
        }
    };

    Ok(decoded)
}

fn decode_and_join_base32_parts(parts: &[String]) -> Result<Vec<u8>, DecodeError> {
    // See the Hex arm: collecting into a Result keeps a failing part fatal
    // instead of silently removing it from the payload.
    let decoded: Vec<u8> = parts
        .iter()
        .enumerate()
        .map(|(index, part)| {
            BASE32_NOPAD
                .decode(part.as_bytes())
                .map_err(|error| DecodeError::UnableToDecodeBase32(index, error))
        })
        .collect::<Result<Vec<Vec<u8>>, DecodeError>>()?
        .concat();

    Ok(decoded)
}
