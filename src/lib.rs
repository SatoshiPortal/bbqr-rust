//! # BBQr - Protocol for spliting and joining large data into multiple QR codes
//! [BBQr Protocol](https://github.com/coinkite/BBQr/blob/master/BBQr.md)
//!
//! ## Usage
//!
//! ### Split large data up
//!
//! ```rust
//! use bbqr::{Encoding, FileType, Split, SplitOptions, Version};
//!
//! let data: &[u8] = b"Hello, World!, but much larger";
//!
//! // split the data using zlib encoding
//! let split = Split::try_from_data(
//!     data,
//!     FileType::UnicodeText,
//!     SplitOptions {
//!         encoding: Encoding::Zlib,
//!         min_split_number: 1,
//!         min_version: Version::V01,
//!         max_version: Version::V40,
//!         ..Default::default()
//!     },
//! ).expect("Failed to split data");
//!
//! // print out each of the parts
//! println!("{:#?}", split.parts);
//!
//! // generate svgs
//! let qr_codes = split.generate_qr_codes();
//! ```
//!
//! ### Join split QR codes
//!
//! ```ignore
//! let parts: Vec<String> = // get the parts from somewhere
//!
//! // join the parts
//! let joined = Joined::try_from_parts(parts);
//!
//! /// joined.data has the raw bytes
//! match &joined.encoding {
//!   Encoding::Unicode => String::from_utf8(joined.data),
//!   other => {
//!     // do whatever
//!   }
//! }
//! ```

pub(crate) mod consts;
pub(crate) mod decode;
pub(crate) mod encode;
pub(crate) mod error;
pub(crate) mod file_type;
pub(crate) mod header;
pub(crate) mod join;
pub(crate) mod qr;
pub(crate) mod split;

pub type Joined = join::Joined;
pub type Split = split::Split;
pub type SplitOptions = split::SplitOptions;

pub type Encoding = encode::Encoding;
pub type FileType = file_type::FileType;

pub type Version = qr::Version;
