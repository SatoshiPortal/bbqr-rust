pub mod consts;
pub mod decode;
pub mod encode;
pub mod error;
pub mod file_type;
pub mod header;
pub mod join;
pub mod qr;
pub mod split;

pub type Joined = join::Joined;
pub type Split = split::SplitResult;

pub type Encoding = encode::Encoding;
pub type FileType = file_type::FileType;
