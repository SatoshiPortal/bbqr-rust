//! File type enum for the different file types supported by the BBQr standard

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The file type supported by the BBQr standard
pub enum FileType {
    Psbt,
    Transaction,
    Json,
    Cbor,
    UnicodeText,
    KeyTeleportReceiver,
    KeyTeleportSender,
    KeyTeleportPsbt,
}

impl FileType {
    pub fn from_byte(byte: u8) -> Option<FileType> {
        match byte {
            b'P' => Some(FileType::Psbt),
            b'T' => Some(FileType::Transaction),
            b'J' => Some(FileType::Json),
            b'C' => Some(FileType::Cbor),
            b'U' => Some(FileType::UnicodeText),
            b'R' => Some(FileType::KeyTeleportReceiver),
            b'S' => Some(FileType::KeyTeleportSender),
            b'E' => Some(FileType::KeyTeleportPsbt),
            _ => None,
        }
    }

    pub fn as_byte(&self) -> u8 {
        match self {
            FileType::Psbt => b'P',
            FileType::Transaction => b'T',
            FileType::Json => b'J',
            FileType::Cbor => b'C',
            FileType::UnicodeText => b'U',
            FileType::KeyTeleportReceiver => b'R',
            FileType::KeyTeleportSender => b'S',
            FileType::KeyTeleportPsbt => b'E',
        }
    }

    pub fn is_known_filetype(byte: u8) -> bool {
        Self::from_byte(byte).is_some()
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FileType::Psbt => write!(f, "PSBT"),
            FileType::Transaction => write!(f, "Transaction"),
            FileType::Json => write!(f, "JSON"),
            FileType::Cbor => write!(f, "CBOR"),
            FileType::UnicodeText => write!(f, "Unicode Text"),
            FileType::KeyTeleportReceiver => write!(f, "Key Teleport Receiver"),
            FileType::KeyTeleportSender => write!(f, "Key Teleport Sender"),
            FileType::KeyTeleportPsbt => write!(f, "Key Teleport PSBT"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileType;

    #[test]
    fn key_teleport_file_types_roundtrip_bytes() {
        let cases = [
            (b'R', FileType::KeyTeleportReceiver),
            (b'S', FileType::KeyTeleportSender),
            (b'E', FileType::KeyTeleportPsbt),
        ];

        for (byte, file_type) in cases {
            assert_eq!(FileType::from_byte(byte), Some(file_type));
            assert_eq!(file_type.as_byte(), byte);
            assert!(FileType::is_known_filetype(byte));
        }
    }

    #[test]
    fn key_teleport_file_types_have_display_names() {
        assert_eq!(
            FileType::KeyTeleportReceiver.to_string(),
            "Key Teleport Receiver"
        );
        assert_eq!(
            FileType::KeyTeleportSender.to_string(),
            "Key Teleport Sender"
        );
        assert_eq!(FileType::KeyTeleportPsbt.to_string(), "Key Teleport PSBT");
    }
}
