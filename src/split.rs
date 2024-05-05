use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    encode::{EncodeError, Encoded},
    file_type::FileType,
    header::Header,
    qr::{QrsNeeded, Version},
    Encoding,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SplitError {
    #[error("No data found")]
    Empty,

    #[error("Cannot make the data fit")]
    CannotFit,

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
    pub min_split_size: usize,
    pub max_split_size: usize,
    pub min_version: Version,
    pub max_version: Version,
}

impl Default for SplitOptions {
    fn default() -> Self {
        Self {
            encoding: Encoding::Zlib,
            min_split_size: 1,
            max_split_size: 1295,
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

    let encoded = Encoded::try_new_from_data(bytes, options.encoding)?;
    let encoded_data_str = encoded.data.as_str();

    let best_version: QrsNeeded = find_best_version(&encoded, &options)?;

    let mut parts = Vec::with_capacity(best_version.count);
    let header_string = Header::new(encoded.encoding, file_type, best_version.count).to_string();

    for i in 0..best_version.count {
        let part_index_base36 = format!("{:0>2}", radix_fmt::radix(i, 36)).to_uppercase();
        let data_part = &encoded_data_str[i..i + best_version.data_per_qr];

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
    // use a heap to keep track of the best versions
    let mut heap = BinaryHeap::new();

    for version_index in options.min_version as usize..=options.max_version as usize {
        let version = Version::from_index(version_index);
        let qrs_needed = encoded.number_of_qrs_needed(version);

        // min heap, reverse the order
        heap.push(Reverse(qrs_needed));
    }

    let best = heap.pop().ok_or(SplitError::CannotFit)?.0;

    // sanity check
    if best.data_per_qr * best.count < encoded.data.len() {
        return Err(SplitError::CannotFit);
    }

    Ok(best)
}
