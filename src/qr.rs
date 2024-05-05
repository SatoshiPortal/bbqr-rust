use crate::consts::QR_DATA_CAPACITY;

pub type Version = fast_qr::Version;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum CapacityTableEncoding {
    TotalBits = 0,
    Numeric = 1,
    Alphanumeric = 2,
    Byte = 3,
}

pub fn version_to_chars(version: Version) -> u16 {
    let error_correction_level = fast_qr::ECL::L;
    let encoding = CapacityTableEncoding::Alphanumeric;

    QR_DATA_CAPACITY[version as usize][error_correction_level as usize][encoding as usize]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_capacity() {
        let expected = 4296;
        let actual = version_to_chars(Version::V40);
        assert_eq!(expected, actual);

        let expected = 25;
        let actual = version_to_chars(Version::V01);
        assert_eq!(expected, actual);

        let expected = 1990;
        let actual = version_to_chars(Version::V26);
        assert_eq!(expected, actual);
    }
}
