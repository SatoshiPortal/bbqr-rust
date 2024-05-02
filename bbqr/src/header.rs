use crate::{encoding::Encoding, file_type::FileType};

#[derive(Debug, thiserror::Error)]
pub enum HeaderParseError {
    #[error("No data found")]
    Empty,

    #[error("Invalid encoding {0}")]
    InvalidEncoding(char),

    #[error("Invalid FileType {0}")]
    InvalidFileType(char),

    #[error("Invalid fixed header")]
    InvalidFixedHeader,

    #[error("Invalid header size, not long enough")]
    InvalidHeaderSize,

    #[error("Invalid header parts {0}")]
    InvalidHeaderParts(String),
}

pub struct Header {
    pub encoding: Encoding,
    pub file_type: FileType,
    pub num_parts: usize,
}

impl Header {
    pub fn try_from_str(header_str: &str) -> Result<Self, HeaderParseError> {
        if header_str.is_empty() {
            return Err(HeaderParseError::Empty);
        }

        // note: only safe to do if we are sure that the string is ASCII
        let first_header_bytes = header_str.as_bytes();

        if header_str.len() < 6 {
            return Err(HeaderParseError::InvalidHeaderSize);
        }

        let fixed_header = &header_str[0..2];
        if fixed_header != "B$" {
            return Err(HeaderParseError::InvalidFixedHeader);
        }

        let encoding_byte = first_header_bytes[2];
        let encoding = Encoding::from_byte(encoding_byte)
            .ok_or(HeaderParseError::InvalidEncoding(encoding_byte as char))?;

        let file_type_byte = first_header_bytes[3];
        let file_type = FileType::from_byte(file_type_byte)
            .ok_or(HeaderParseError::InvalidFileType(file_type_byte as char))?;

        // in base36
        let num_parts_str = &header_str[4..6];
        let num_parts = usize::from_str_radix(num_parts_str, 36).map_err(|_| {
            HeaderParseError::InvalidHeaderParts(format!(
                "Invalid number of parts: {}",
                num_parts_str
            ))
        })?;

        let header = Header {
            encoding,
            file_type,
            num_parts,
        };

        Ok(header)
    }
}
