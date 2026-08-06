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

pub(crate) struct ParsedPart<'a> {
    pub header: Header,
    pub index: usize,
    pub payload: &'a str,
}

impl<'a> ParsedPart<'a> {
    pub fn try_from_str(part: &'a str) -> Result<Self, JoinError> {
        let header = Header::try_from_str(part)?;
        let index = get_index_from_part(part, &header)?;
        let payload = part
            .get(HEADER_LENGTH..)
            .ok_or(JoinError::ConflictingHeaders)?;

        if payload.is_empty() {
            return Err(JoinError::PartWithNoData(index));
        }

        Ok(Self {
            header,
            index,
            payload,
        })
    }
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
    let mut parsed_parts = input_parts
        .iter()
        .filter(|part| !part.trim().is_empty())
        .map(|part| ParsedPart::try_from_str(part));

    let first_part = parsed_parts.next().transpose()?.ok_or(JoinError::Empty)?;
    let header = first_part.header;

    // keep missing parts distinct from received payloads
    let mut ordered_parts = vec![None; header.num_parts];

    for part in std::iter::once(Ok(first_part)).chain(parsed_parts) {
        let part = part?;

        if part.header != header {
            return Err(JoinError::ConflictingHeaders);
        }

        match &ordered_parts[part.index] {
            Some(current_payload) if current_payload != part.payload => {
                return Err(JoinError::DuplicatePartWrongContent(part.index));
            }
            Some(_) => continue,
            None => {}
        }

        ordered_parts[part.index] = Some(part.payload.to_string());
    }

    let ordered_payloads = ordered_parts
        .iter()
        .enumerate()
        .map(|(index, part)| part.as_deref().ok_or(JoinError::MissingPart(index)))
        .collect::<Result<Vec<_>, _>>()?;

    let data = decode::decode_ordered_parts(&ordered_payloads, header.encoding)?;

    Ok((header, data))
}

fn get_index_from_part(part: &str, header: &Header) -> Result<usize, JoinError> {
    // get the index of the the current part
    //
    // get_and_verify_headers only established that the part is long enough and
    // ASCII, never that these two characters are valid base36, so this cannot
    // unwrap: a frame with punctuation here would take the process down, and
    // frames come from a scanned QR.
    let index_str = part.get(6..8).ok_or(JoinError::ConflictingHeaders)?;

    let index = usize::from_str_radix(index_str, 36).map_err(|_| JoinError::ConflictingHeaders)?;

    // more parts than the header says, error
    if index >= header.num_parts {
        return Err(JoinError::TooManyParts(header.num_parts, index + 1));
    };

    Ok(index)
}

#[cfg(test)]
mod tests {
    use crate::{encode::Encoding, file_type::FileType};

    use super::*;

    #[test]
    fn test_parse_part() {
        let part = ParsedPart::try_from_str("B$ZU0801A").unwrap();

        assert_eq!(
            part.header,
            Header {
                encoding: Encoding::Zlib,
                file_type: FileType::UnicodeText,
                num_parts: 8
            }
        );
        assert_eq!(part.index, 1);
        assert_eq!(part.payload, "A");
    }

    #[test]
    fn test_catches_empty() {
        let parts = vec!["", "", "", "", ""]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        let joined = join_qrs(parts);

        assert!(joined.is_err());
        assert_eq!(joined.unwrap_err(), JoinError::Empty);
    }

    #[test]
    fn test_catches_conflicting_headers() {
        let parts = vec!["", "B$ZU0801A", "B$ZU0902B", "B$ZU0803C", ""]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        let joined = join_qrs(parts);

        assert!(joined.is_err());
        assert_eq!(joined.unwrap_err(), JoinError::ConflictingHeaders);
    }
}
