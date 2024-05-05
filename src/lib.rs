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

pub type Encoding = encode::Encoding;
pub type FileType = file_type::FileType;
