use crate::{
    consts::MAX_PARTS,
    encode::{EncodeError, Encoded},
    file_type::FileType,
    header::{int_to_padded_base_36, Header},
    qr::{QrsNeeded, Version},
    Encoding,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SplitError {
    #[error("No data found")]
    Empty,

    #[error("Cannot make the data fit")]
    CannotFit,

    #[error("Max split size is too large, max is {MAX_PARTS}, got {0}")]
    MaxSplitSizeTooLarge(usize),

    #[error(transparent)]
    EncodeError(#[from] EncodeError),
}

#[derive(Debug, Clone)]
pub struct Split {
    pub version: Version,
    pub parts: Vec<String>,
    pub encoding: Encoding,
}

#[derive(Debug, Clone)]
pub struct SplitOptions {
    pub encoding: Encoding,
    /// min number of parts to split into
    pub min_split_number: usize,
    /// max number of parts to split into
    pub max_split_number: usize,
    pub min_version: Version,
    pub max_version: Version,
}

impl Default for SplitOptions {
    fn default() -> Self {
        Self {
            encoding: Encoding::Zlib,
            min_split_number: 1,
            max_split_number: 1295,
            min_version: Version::V01,
            max_version: Version::V40,
        }
    }
}

impl Split {
    pub fn try_from_data(
        bytes: &[u8],
        file_type: FileType,
        options: SplitOptions,
    ) -> Result<Self, SplitError> {
        split_qrs(bytes, file_type, options)
    }
}

fn split_qrs(
    bytes: &[u8],
    file_type: FileType,
    options: SplitOptions,
) -> Result<Split, SplitError> {
    if bytes.is_empty() {
        return Err(SplitError::Empty);
    }

    if options.max_split_number > MAX_PARTS {
        return Err(SplitError::MaxSplitSizeTooLarge(options.max_split_number));
    }

    let encoded = Encoded::try_new_from_data(bytes, options.encoding)?;
    let encoded_data_str = encoded.data.as_str();

    let best_version: QrsNeeded = find_best_version(&encoded, &options)?;

    let mut parts = Vec::with_capacity(best_version.count);
    let header_string = Header::new(encoded.encoding, file_type, best_version.count).to_string();

    for i in 0..best_version.count {
        let start_byte = i * best_version.data_per_qr;
        let end_byte = (start_byte + best_version.data_per_qr).min(encoded_data_str.len());

        let part_index_base36 = int_to_padded_base_36(i);

        let data_part = &encoded_data_str[start_byte..end_byte];
        let part = format!("{}{}{}", header_string, part_index_base36, data_part);

        parts.push(part);
    }

    let split = Split {
        version: best_version.version,
        parts,
        encoding: encoded.encoding,
    };

    Ok(split)
}

fn find_best_version(encoded: &Encoded, options: &SplitOptions) -> Result<QrsNeeded, SplitError> {
    // keep the best option
    let mut best_option = None;

    for version_index in options.min_version as usize..=options.max_version as usize {
        let version = Version::from_index(version_index);
        let qrs_needed = encoded.number_of_qrs_needed(version);

        let qrs_needed_count = qrs_needed.count;

        // if this option needs more than the max, skip it
        if qrs_needed_count > MAX_PARTS {
            continue;
        };

        if qrs_needed.count < options.min_split_number {
            continue;
        }

        if qrs_needed.count > options.max_split_number {
            continue;
        }

        match &best_option {
            Some(ref best) => {
                if &qrs_needed < best {
                    best_option = Some(qrs_needed);
                }
            }
            None => {
                best_option = Some(qrs_needed);
            }
        }
    }

    if best_option.is_none() {
        return Err(SplitError::CannotFit);
    }

    let best = best_option.expect("just checked");

    // sanity check
    if best.data_per_qr * best.count < encoded.data.len() {
        return Err(SplitError::CannotFit);
    }

    Ok(best)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_split() {
        let data = vec![b'A'; 4000];
        let split = Split::try_from_data(
            &data,
            FileType::Psbt,
            SplitOptions {
                encoding: super::Encoding::Hex,
                min_split_number: 1,
                max_split_number: 1295,
                min_version: super::Version::V01,
                max_version: super::Version::V40,
            },
        );

        assert!(split.is_ok());

        let split = split.unwrap();
        assert!(split.version <= Version::V40);
        assert!(split.encoding == Encoding::Hex);

        assert_eq!(split.version, Version::V39);
        assert_eq!(split.parts.len(), 2);

        let header = Header::try_from_str(&split.parts[0]);
        assert!(header.is_ok());

        let header = header.unwrap();
        assert_eq!(header.num_parts, 2);
        assert_eq!(header.encoding, Encoding::Hex);
        assert_eq!(header.file_type, FileType::Psbt);
    }

    #[test]
    fn test_split_empty() {
        let data = vec![];
        let split = Split::try_from_data(
            &data,
            FileType::Psbt,
            SplitOptions {
                encoding: super::Encoding::Hex,
                min_split_number: 1,
                max_split_number: 1295,
                min_version: super::Version::V01,
                max_version: super::Version::V40,
            },
        );

        assert!(split.is_err());
        assert_eq!(split.unwrap_err(), SplitError::Empty);
    }

    #[test]
    fn test_another_split() {
        let data = vec![b'A'; 2000];
        let split = Split::try_from_data(
            &data,
            FileType::Psbt,
            SplitOptions {
                encoding: super::Encoding::Hex,
                min_split_number: 1,
                max_split_number: 1295,
                min_version: super::Version::V11,
                max_version: super::Version::V40,
            },
        );

        assert!(split.is_ok());

        let split = split.unwrap();
        assert!(split.version <= Version::V40);
        assert!(split.encoding == Encoding::Hex);

        assert_eq!(split.version, Version::V39);
        assert_eq!(split.parts.len(), 1);

        let header = Header::try_from_str(&split.parts[0]);
        assert!(header.is_ok());

        let header = header.unwrap();
        assert_eq!(header.num_parts, 1);
        assert_eq!(header.encoding, Encoding::Hex);
        assert_eq!(header.file_type, FileType::Psbt);
    }
}
