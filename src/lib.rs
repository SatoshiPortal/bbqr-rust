//! # BBQr - Protocol for spliting and joining large data into multiple QR codes
//! Rust Implementation of the [BBQr Protocol](https://github.com/coinkite/BBQr/blob/master/BBQr.md) spec
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
//! // split the data using zlib encoding, and default options split the data using default options
//! let split = Split::try_from_data(data, FileType::UnicodeText, Default::default())
//!    .expect("Failed to split data");
//!
//! // or split the data using zlib encoding, and custom options
//! let split = Split::try_from_data(
//!     data,
//!     FileType::UnicodeText,
//!     SplitOptions {
//!         encoding: Encoding::Zlib,
//!         min_split_number: 2,
//!         max_split_number: 100,
//!         min_version: Version::V03,
//!         max_version: Version::V30,
//!     },
//! ).expect("Failed to split data");
//!
//! // print out each of the parts
//! println!("{:#?}", split.parts);
//!
//! // generate the qr codes
//! let qr_codes = split.generate_qr_codes();
//! ```
//!
//! ### Join split QR codes
//!
//! ```ignore
//! // get the parts from somewhere
//! let parts: Vec<String> = ...
//! // join the parts
//! let joined = Joined::try_from_parts(parts).expect("Failed to join parts");
//!
//! /// joined.data has the raw bytes
//! match &joined.encoding {
//!   Encoding::Unicode => String::from_utf8(joined.data),
//!   other => {
//!     // do whatever
//!   }
//! }
//! ```

pub mod error;

pub(crate) mod consts;
pub(crate) mod decode;
pub(crate) mod encode;
pub(crate) mod file_type;
pub(crate) mod header;
pub(crate) mod join;
pub(crate) mod qr;
pub(crate) mod split;

/// Joined data structure, includes the encoding, file type, and raw data in bytes
pub type Joined = join::Joined;

/// The split Data structure, includes the version, parts, and encoding
pub type Split = split::Split;

/// Split options, has a default implementation but you can customize it.
///
/// Set the qr version, encoding, and min/max split number
pub type SplitOptions = split::SplitOptions;

/// The encoding to use for the data, HEX, Base32, or Zlib, best to default Zlib
pub type Encoding = encode::Encoding;

/// The file type, currently only supports UnicodeText, Transaction, PSBT, Binary, and CBOR
pub type FileType = file_type::FileType;

/// The version of the QR code, from V01 to V40
pub type Version = qr::Version;
