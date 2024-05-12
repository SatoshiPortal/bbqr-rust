//! Publicly available error types.

use crate::{decode, encode, header, join, split};

pub type SplitError = split::SplitError;

pub type JoinError = join::JoinError;
pub type HeaderError = header::HeaderParseError;

pub type EncodingError = encode::EncodeError;
pub type DecodeError = decode::DecodeError;
