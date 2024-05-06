use std::fmt::{self, Display, Formatter};

use radix_fmt::radix;

use crate::{consts::HEADER_LENGTH, Encoding, FileType};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum HeaderParseError {
    #[error("No data found")]
    Empty,

    #[error("Invalid encoding {0}")]
    InvalidEncoding(char),

    #[error("Invalid FileType {0}")]
    InvalidFileType(char),

    #[error("Invalid fixed header")]
    InvalidFixedHeader,

    #[error("Invalid header size, not long enough, expected {HEADER_LENGTH} bytes, got {0}")]
    InvalidHeaderSize(usize),

    #[error("Invalid header parts {0}")]
    InvalidHeaderParts(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub encoding: Encoding,
    pub file_type: FileType,
    pub num_parts: usize,
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let base_36_count = int_to_padded_base_36(self.num_parts);

        write!(
            f,
            "B${}{}{}",
            self.encoding.as_byte() as char,
            self.file_type.as_byte() as char,
            base_36_count
        )
    }
}

impl Header {
    pub fn new(encoding: Encoding, file_type: FileType, num_parts: usize) -> Self {
        Self {
            encoding,
            file_type,
            num_parts,
        }
    }

    pub fn try_from_str(header_str: &str) -> Result<Self, HeaderParseError> {
        if header_str.is_empty() {
            return Err(HeaderParseError::Empty);
        }

        // note: okay to work directyl with bytes here, since only ASCII is used in the protocol
        let first_header_bytes = header_str.as_bytes();

        let header_len = header_str.len();
        if header_len < HEADER_LENGTH {
            return Err(HeaderParseError::InvalidHeaderSize(header_len));
        }

        let fixed_header = &header_str[0..2];
        if fixed_header != "B$" {
            return Err(HeaderParseError::InvalidFixedHeader);
        }

        let encoding_byte = first_header_bytes[2];
        let encoding = Encoding::from_byte(encoding_byte)
            .ok_or(HeaderParseError::InvalidEncoding(encoding_byte as char))?;

        let file_type_byte = first_header_bytes[3];
        let file_type = FileType::from_byte(file_type_byte)
            .ok_or(HeaderParseError::InvalidFileType(file_type_byte as char))?;

        let num_parts_str = &header_str[4..6];
        let num_parts = usize::from_str_radix(num_parts_str, 36).map_err(|_| {
            HeaderParseError::InvalidHeaderParts(format!(
                "Invalid number of parts: {num_parts_str}",
            ))
        })?;

        let header = Header {
            encoding,
            file_type,
            num_parts,
        };

        Ok(header)
    }
}

pub(crate) fn int_to_padded_base_36(num: usize) -> String {
    // if less than 36 need padding
    if num < 36 {
        // using in format directly does not adding padding for some reason
        let base_36 = radix(num, 36).to_string().to_uppercase();
        return format!("{:0>2}", base_36);
    }

    format!("{:#}", radix(num, 36))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Encoding;
    use crate::FileType;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_header_parse() {
        let header_str = "B$ZU0801EBB6HXMU5ARSYYW2WLJIDMYZLUYC4DIBYL5AYOWJGJCBJROGN6D5WFRHNB34OYIDHD25HKZ7KGWAFM43AAH6LKT43MZN53PLFXXTEKJ27DUYBDVM37BNUY3BK6MKSPZQPNUVHY733MDKXYEYRGMV3YF764TDAWROUFESL6CBW6OWLMIKCXAVR3JLOTCCDVQY6FA7SUFMZKVYWFGEXHZX33SW5MQ7YW3DEZVCORL474AVLWA2NRVLZLIBCBFWTZDNHZ3IZ5BTAJOBMS2KGAPTW22GB7UU6KEMKJ7TF56S7IGJWKYEEHJLBQVHBPOISVJSX7Y5MFUPDPLDKCU6HRVFQNNK2MQ4DFY3ZQGDPRWVU3JATYVXBUD45T4PNLHKKPBONPLGZPN3FUESULOIGFDXHT7IWK2MX3OIA6AIC6LONCCEX3VENJ2NAFN2HSXGV64YABDGQKREDMLULDPMRME3IRF6OCCGGL4U7L6PTM527KHP5YLPMMPPA5QASTKHGNC56DEVPXWS2GBGI3J4KZX754JNXLDPSWSWCUBR25GCUYL7JZUHG76KBLX7U5DRWY2BO7L7CC5273Q7VY6VZWCG47VY7H2G4QU4FI2VX7PD2PCUS3LOJWFLF36N5NHS6ECD2JELY36XTRE3Z72AMJ2FJM6Z77Q7LNRR3GPPZJQIS57VZ4IQMWABYVSHTCS3353R5DBIIFRUYB25RPO6UXODOH3Q43AET3BYZ2XKXFZI4XNPV2RO2KZWPEWZWNTZEWRODP5OW3STAPEQ6FLXNFEIBSLKIQF2O4VXJQOFWISXNAFJX66SYHHOU2T7KKLQZDGDQUGFMPXAE2KSDNJVVHYX2GUGUXFDFQZOTEQGFA7BHCS3PG2S64WADRUYXADVO2YGHEXOLRCQIB7WL5RA6QXB2UFHVM5U5NBVCDVRQXCAAIRBXNRVMHAFRUQJMIDVSKRDCRLSQF44DW3LWR3UVJ2OICNVVSOXBFPTGJOYYUT3SODNPYA4MIVJJ2FI55ZCRPTXOU6B5ZHEXJC7GBOGNAT5Y2SGT2WXDUDG3KLGXIYT3POQJO3K2JORO7AXZLIYTPFKNHOW6NSSQAVDBDXLNFHAMLV3NBVA4OQZ57KAA3O2Z2D2IXMSGYY5UWOXTRA23KPSEBEPP2E6R2CGFVK3BVR3PSEYKOOCV5K24PVNXRWULRNOVQKPGLTFWKD4NS7BIMAOSTCLIKAOAFKV4CZDHTRHXWHU";

        let header = Header::try_from_str(header_str).unwrap();
        assert_eq!(header.encoding, Encoding::Zlib);
        assert_eq!(header.file_type, FileType::UnicodeText);
        assert_eq!(header.num_parts, 8);
    }

    #[test]
    fn fails_on_bad_header() {
        let header_str = "B#888888";
        let header = Header::try_from_str(header_str);

        assert!(header.is_err());
        assert!(header.unwrap_err() == HeaderParseError::InvalidFixedHeader);
    }

    #[test]
    fn test_int_to_padded_base_36() {
        assert_eq!(int_to_padded_base_36(0), "00");
        assert_eq!(int_to_padded_base_36(2), "02");
        assert_eq!(int_to_padded_base_36(1), "01");
        assert_eq!(int_to_padded_base_36(35), "0Z");
        assert_eq!(int_to_padded_base_36(36), "10");
        assert_eq!(int_to_padded_base_36(37), "11");
        assert_eq!(int_to_padded_base_36(556), "FG");
        assert_eq!(int_to_padded_base_36(1295), "ZZ");
    }
}
