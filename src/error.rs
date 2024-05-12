//! Publicly available error types.

use crate::{decode, encode, header, join, split};

pub type SplitError = split::SplitError;

pub type JoinError = join::JoinError;
pub type HeaderParseError = header::HeaderParseError;

pub type EncodeError = encode::EncodeError;
pub type DecodeError = decode::DecodeError;
