//! Join multiple QR codes into one large piece of data

use crate::{
    consts::HEADER_LENGTH,
    decode,
    encode::Encoding,
    file_type::FileType,
    header::{Header, HeaderParseError},
};

/// Errors that can occur when joining data
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum JoinError {
    #[error("No data found")]
    Empty,

    #[error("Conflicting/variable file type/encodings/sizes")]
    ConflictingHeaders,

    #[error("Too many parts, expected {0}, got {1}")]
    TooManyParts(usize, usize),

    #[error("Duplicated part index {0} has wrong content")]
    DuplicatePartWrongContent(usize),

    #[error("Part with index {0} has no data")]
    PartWithNoData(usize),

    #[error("Missing part, with index {0}")]
    MissingPart(usize),

    #[error(transparent)]
    HeaderParseError(#[from] HeaderParseError),

    #[error(transparent)]
    DecodeError(#[from] decode::DecodeError),
}

/// Joined data structure, includes the encoding, file type, and raw data in bytes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Joined {
    /// Encoding that was used in the QR codes, all parts must have the same encoding
    /// The encoding is used to decode the data, and its stated in the header of each split part
    pub encoding: Encoding,

    /// File type that was used in the QR codes, all parts must have the same file type
    /// The file type is determined by the header
    pub file_type: FileType,

    /// The data that was encoded in the QR codes
    pub data: Vec<u8>,
}

impl Joined {
    pub fn try_from_parts(parts: Vec<String>) -> Result<Self, JoinError> {
        let (header, data) = join_qrs(parts)?;
        Ok(Self {
            encoding: header.encoding,
            file_type: header.file_type,
            data,
        })
    }
}

// Take scanned data, put into order, decode, return type code and raw data bytes
fn join_qrs(input_parts: Vec<String>) -> Result<(Header, Vec<u8>), JoinError> {
    let header = get_and_verify_headers(input_parts.as_slice())?;

    // pre-allocate the parts, so we can insert them in the correct order, faster than sorting
    let mut orderered_parts = vec![String::new(); header.num_parts];

    for part in input_parts {
        if part.is_empty() {
            continue;
        }

        // get the index of the the current part
        // already checked in get_and_verify_headers that the header is long enough
        let index = usize::from_str_radix(&part[6..8], 36).unwrap();

        // more parts than the header says, error
        if index >= header.num_parts {
            return Err(JoinError::TooManyParts(header.num_parts, index + 1));
        }

        let current_part_content = &orderered_parts[index];
        let part_data = &part[8..];

        if !current_part_content.is_empty() && current_part_content != part_data {
            return Err(JoinError::DuplicatePartWrongContent(index));
        }

        if part_data.is_empty() {
            return Err(JoinError::PartWithNoData(index));
        }

        // store the part data in the correct order
        orderered_parts[index] = part_data.to_string();
    }

    // check if any part is missing
    for (index, part) in orderered_parts.iter().enumerate() {
        if part.is_empty() {
            return Err(JoinError::MissingPart(index));
        }
    }

    let data = decode::decode_ordered_parts(&orderered_parts, header.encoding)?;

    Ok((header, data))
}

/// Verify that all the headers have the same variable filetype, encodings and sizes
fn get_and_verify_headers(parts: &[String]) -> Result<Header, JoinError> {
    if parts.is_empty() {
        return Err(JoinError::Empty);
    }

    // find first non-empty line
    let first_header = parts
        .iter()
        .find(|line| !line.is_empty())
        .ok_or(JoinError::Empty)?;

    let header = Header::try_from_str(first_header)?;

    // verify that all the headers are the same
    for part in parts.iter().skip(1) {
        if part.is_empty() {
            continue;
        }

        if part.len() < HEADER_LENGTH {
            return Err(JoinError::ConflictingHeaders);
        }

        if part[0..6] != first_header[0..6] {
            return Err(JoinError::ConflictingHeaders);
        }
    }

    Ok(header)
}

#[cfg(test)]
mod tests {
    use crate::{encode::Encoding, file_type::FileType};

    use super::*;

    #[test]
    fn test_verify_header() {
        let parts = vec!["", "B$ZU0801", "B$ZU0801", "B$ZU0801", ""]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        let header = get_and_verify_headers(&parts);

        assert!(header.is_ok());
        assert_eq!(
            header.unwrap(),
            Header {
                encoding: Encoding::Zlib,
                file_type: FileType::UnicodeText,
                num_parts: 8
            }
        );
    }

    #[test]
    fn test_catches_empty() {
        let parts = vec!["", "", "", "", ""]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        let header = get_and_verify_headers(&parts);

        assert!(header.is_err());
        assert_eq!(header.unwrap_err(), JoinError::Empty);
    }

    #[test]
    fn test_catches_conflicting_headers() {
        let parts = vec!["", "B$ZU0801", "B$ZU0902", "B$ZU0803", ""]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        let header = get_and_verify_headers(&parts);

        assert!(header.is_err());
        assert_eq!(header.unwrap_err(), JoinError::ConflictingHeaders);
    }
}
