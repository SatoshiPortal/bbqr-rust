//! QR code joining

use crate::header::{Header, HeaderParseError};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum JoinError {
    #[error("No data found")]
    Empty,

    #[error("Conflicting/variable file type/encodings/sizes")]
    ConflictingHeaders,

    #[error(transparent)]
    HeaderParseError(#[from] HeaderParseError),
}

fn decode_data(parts: &[String], encoding: char) -> Vec<u8> {
    // Implement the decode_data function based on the encoding
    // This function should decode the data from the given parts based on the encoding
    // and return the decoded data as a vector of bytes
    // You'll need to replace this placeholder implementation with the actual decoding logic
    vec![]
}
// Take scanned data, put into order, decode, return type code and raw data bytes
pub fn join_qrs(parts: Vec<String>) -> Result<(char, Vec<u8>), JoinError> {
    let header = get_and_verify_headers(parts.as_slice())?;

    // for p in parts {
    //     let idx = usize::from_str_radix(&p[6..8], 36).unwrap();
    //     assert!(
    //         idx < num_parts,
    //         "got part {} but only expecting {}",
    //         idx,
    //         num_parts
    //     );
    //
    //     if !data[idx].is_empty() {
    //         assert_eq!(
    //             data[idx],
    //             p[8..].to_string(),
    //             "dup part 0x{:02x} has wrong content",
    //             idx
    //         );
    //     } else {
    //         data[idx] = p[8..].to_string();
    //     }
    // }
    //
    // let missing: Vec<usize> = (0..num_parts).filter(|&i| data[i].is_empty()).collect();
    // assert!(missing.is_empty(), "parts missing: {:?}", missing);
    //
    // let raw = decode_data(&data, encoding);

    // maybe: decode objects here... U=>text, C=>obj, J=>obj

    todo!()
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

        if part.len() < 6 {
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
    use super::*;
    use crate::encoding::Encoding;
    use crate::file_type::FileType;

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
