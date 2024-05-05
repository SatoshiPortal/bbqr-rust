use std::io::Read as _;

use crate::encoding::Encoding;
use data_encoding::BASE32;
use data_encoding::HEXUPPER;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DecodeError {
    #[error("Unable to decode hex part: {0}, error: {1}")]
    UnableToDecodeHex(usize, data_encoding::DecodeError),

    #[error("Unable to decode base32 part: {0}, error: {1}")]
    UnableToDecodeBase32(usize, data_encoding::DecodeError),

    #[error("Unable decompress zlib data: {0}")]
    UnableToInflateZlib(String),
}

pub fn decode_ordered_parts(parts: &[String], encoding: Encoding) -> Result<Vec<u8>, DecodeError> {
    let decoded: Vec<u8> = match encoding {
        Encoding::Hex => parts
            .iter()
            .enumerate()
            .flat_map(|(index, part)| {
                HEXUPPER
                    .decode(part.as_bytes())
                    .map_err(|error| DecodeError::UnableToDecodeHex(index, error))
            })
            .flatten()
            .collect(),

        Encoding::Base32 => decode_and_join_base32_parts(parts)?,

        Encoding::Zlib => {
            let bytes = decode_and_join_base32_parts(parts)?;

            // inflate zlib encoded data
            let decompress = flate2::Decompress::new_with_window_bits(false, 10);
            let mut decoder =
                flate2::read::ZlibDecoder::new_with_decompress(&bytes[..], decompress);

            let mut decoded = Vec::new();
            decoder
                .read_to_end(&mut decoded)
                .map_err(|e| DecodeError::UnableToInflateZlib(e.to_string()))?;

            decoded
        }
    };

    Ok(decoded)
}

fn decode_and_join_base32_parts(parts: &[String]) -> Result<Vec<u8>, DecodeError> {
    let decoded: Vec<u8> = parts
        .iter()
        .enumerate()
        .flat_map(|(index, part)| {
            BASE32
                .decode(part.as_bytes())
                .map_err(|error| DecodeError::UnableToDecodeBase32(index, error))
        })
        .flatten()
        .collect();

    Ok(decoded)
}
