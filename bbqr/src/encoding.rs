#[derive(Debug, Clone, Copy)]
pub enum Encoding {
    Hex,
    Zlib,
}

impl Encoding {
    pub fn from_byte(byte: u8) -> Option<Encoding> {
        match byte {
            b'H' => Some(Encoding::Hex),
            b'Z' => Some(Encoding::Zlib),
            _ => None,
        }
    }

    pub fn is_known_encoding(byte: u8) -> bool {
        Self::from_byte(byte).is_some()
    }
}
