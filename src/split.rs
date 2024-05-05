use crate::{Encoding, file_type::FileType, qr::Version};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SplitError {
    #[error("No data found")]
    Empty,
}

#[derive(Debug, Clone)]
pub struct SplitResult {
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

pub fn split_qrs(bytes: &[u8], file_type: FileType, options: SplitOptions) -> SplitResult {
   todo!() 
}
