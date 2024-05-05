use std::io::Write as _;

use data_encoding::{BASE32_NOPAD, HEXUPPER};
use flate2::{write::ZlibEncoder, Compress, Compression};

use crate::{
    consts::HEADER_LENGTH,
    qr::{QrsNeeded, Version},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Hex,
    Base32,
    Zlib,
}

impl Encoding {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'H' => Some(Self::Hex),
            b'2' => Some(Self::Base32),
            b'Z' => Some(Self::Zlib),
            _ => None,
        }
    }

    pub fn as_byte(&self) -> u8 {
        match self {
            Self::Hex => b'H',
            Self::Base32 => b'2',
            Self::Zlib => b'Z',
        }
    }

    pub fn is_known_encoding(byte: u8) -> bool {
        Self::from_byte(byte).is_some()
    }

    pub fn split_mod(&self) -> usize {
        match self {
            Self::Hex => 2,
            Self::Base32 => 8,
            Self::Zlib => 8,
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum EncodeError {
    #[error("No data to encode")]
    Empty,

    #[error("Unable to compress data")]
    CompressionError(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encoded {
    pub encoding: Encoding,
    pub data: String,
}

impl Encoded {
    pub fn try_new_from_data(data: &[u8], encoding: Encoding) -> Result<Self, EncodeError> {
        if data.is_empty() {
            return Err(EncodeError::Empty);
        }

        let encoded = match encoding {
            Encoding::Hex => Self {
                data: HEXUPPER.encode(data),
                encoding: Encoding::Hex,
            },
            Encoding::Base32 => Self {
                data: BASE32_NOPAD.encode(data),
                encoding: Encoding::Base32,
            },
            Encoding::Zlib => {
                let compress = Compress::new_with_window_bits(Compression::best(), false, 10);
                let mut encoder =
                    ZlibEncoder::new_with_compress(Vec::with_capacity(data.len()), compress);

                encoder
                    .write_all(data)
                    .map_err(|e| EncodeError::CompressionError(e.to_string()))?;

                let compressed = encoder
                    .finish()
                    .map_err(|e| EncodeError::CompressionError(e.to_string()))?;

                // only use the compressed version if it's smaller
                if compressed.len() < data.len() {
                    // otherwise, use the compressed data
                    Self {
                        data: BASE32_NOPAD.encode(&compressed),
                        encoding: Encoding::Zlib,
                    }
                } else {
                    // if compressed data is larger, use the original data
                    Self {
                        data: BASE32_NOPAD.encode(data),
                        encoding: Encoding::Base32,
                    }
                }
            }
        };

        Ok(encoded)
    }

    pub(crate) fn number_of_qrs_needed(&self, version: Version) -> QrsNeeded {
        let data_size = self.data.len();
        let encoding = &self.encoding;

        let base_capacity = version.data_capacity() - HEADER_LENGTH;

        // we need to adjust the capacity to be a multiple of the encoding split mod
        let adjusted_capacity = base_capacity - (base_capacity % encoding.split_mod());

        let estimated_count = usize::div_ceil(data_size, adjusted_capacity);

        // if we can fit all the data in one qr code
        if estimated_count == 1 {
            return QrsNeeded {
                version,
                count: 1,
                data_per_qr: data_size,
            };
        }

        // the total capacity of our estimated count
        // all but the last QR need to use adjusted capacity to ensure proper split
        let total_capacity_of_estimated_count =
            (estimated_count - 1) * adjusted_capacity + base_capacity;

        let count = if total_capacity_of_estimated_count >= data_size {
            estimated_count
        } else {
            estimated_count + 1
        };

        QrsNeeded {
            version,
            count,
            data_per_qr: adjusted_capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::decode;

    use super::*;

    #[test]
    fn test_encode_compression() {
        let file_names = [
            "test_data/1in1000out.psbt",
            "test_data/1in100out.psbt",
            "test_data/1in10out.psbt",
            "test_data/1in20out.psbt",
            "test_data/1in2out.psbt",
            "test_data/devils-txn.txn",
            "test_data/finalized-by-ckcc.txn",
            "test_data/last.txn",
            "test_data/nfc-result.txn",
        ];

        for file_name in &file_names {
            let raw = std::fs::read(file_name).expect("Failed to read file");

            let encoded = Encoded::try_new_from_data(&raw, Encoding::Zlib);

            assert!(encoded.is_ok());

            let encoded = encoded.unwrap();
            assert_eq!(encoded.encoding, Encoding::Zlib);

            let check = decode::decode_ordered_parts(&[encoded.data.clone()], Encoding::Zlib);

            assert!(check.is_ok());

            let check = check.unwrap();
            assert_eq!(check, raw);

            let decode_as_base32 =
                decode::decode_ordered_parts(&[encoded.data.clone()], Encoding::Base32);
            assert!(decode_as_base32.is_ok());

            let decode_as_base32 = decode_as_base32.unwrap();
            assert!(decode_as_base32.len() < raw.len());
        }
    }
}
