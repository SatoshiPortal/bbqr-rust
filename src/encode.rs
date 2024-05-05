use std::io::Write as _;

use data_encoding::{BASE32_NOPAD, HEXUPPER};
use flate2::{write::ZlibEncoder, Compress, Compression};

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

    pub fn is_known_encoding(byte: u8) -> bool {
        Self::from_byte(byte).is_some()
    }

    pub fn split_mod(&self) -> u8 {
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
    pub fn try_new_from_data(data: Vec<u8>, encoding: Encoding) -> Result<Self, EncodeError> {
        if data.is_empty() {
            return Err(EncodeError::Empty);
        }

        let encoded = match encoding {
            Encoding::Hex => Self {
                data: HEXUPPER.encode(&data),
                encoding: Encoding::Hex,
            },
            Encoding::Base32 => Self {
                data: BASE32_NOPAD.encode(&data),
                encoding: Encoding::Base32,
            },
            Encoding::Zlib => {
                let compress = Compress::new_with_window_bits(Compression::best(), false, 10);
                let mut encoder =
                    ZlibEncoder::new_with_compress(Vec::with_capacity(data.len()), compress);

                encoder
                    .write_all(&data)
                    .map_err(|e| EncodeError::CompressionError(e.to_string()))?;

                let compressed = encoder
                    .finish()
                    .map_err(|e| EncodeError::CompressionError(e.to_string()))?;

                // only use the compressed version if it's smaller
                if compressed < data {
                    // otherwise, use the compressed data
                    Self {
                        data: BASE32_NOPAD.encode(&compressed),
                        encoding: Encoding::Zlib,
                    }
                } else {
                    // if compressed data is larger, use the original data
                    Self {
                        data: BASE32_NOPAD.encode(&data),
                        encoding: Encoding::Base32,
                    }
                }
            }
        };

        Ok(encoded)
    }
}
