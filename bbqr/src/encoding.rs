#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Hex,
    Zlib,
}

impl Encoding {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'H' => Some(Self::Hex),
            b'Z' => Some(Self::Zlib),
            _ => None,
        }
    }

    pub fn is_known_encoding(byte: u8) -> bool {
        Self::from_byte(byte).is_some()
    }
}
