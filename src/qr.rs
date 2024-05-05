use std::cmp::Ordering;

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

pub(crate) struct QrsNeeded {
    pub version: Version,
    pub count: usize,
    pub data_per_qr: usize,
}

impl Eq for QrsNeeded {}
impl PartialEq for QrsNeeded {
    fn eq(&self, other: &Self) -> bool {
        self.version as usize == other.version as usize
            && self.count == other.count
            && self.data_per_qr == other.data_per_qr
    }
}

// we want to sort by count first, then by version
impl Ord for QrsNeeded {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.count.cmp(&other.count) {
            Ordering::Equal => {
                let v1 = self.version as usize;
                let v2 = other.version as usize;
                v1.cmp(&v2)
            }
            other => other,
        }
    }
}
impl PartialOrd for QrsNeeded {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn version_data_capacity(version: Version) -> u16 {
    let error_correction_level = fast_qr::ECL::L;
    let encoding = CapacityTableEncoding::Alphanumeric;

    QR_DATA_CAPACITY[version as usize][error_correction_level as usize][encoding as usize]
}

pub(crate) fn version_from_index(version_index: usize) -> Version {
    version_from_number(version_index + 1)
}

pub(crate) fn version_from_number(version_number: usize) -> Version {
    match version_number {
        1 => Version::V01,
        2 => Version::V02,
        3 => Version::V03,
        4 => Version::V04,
        5 => Version::V05,
        6 => Version::V06,
        7 => Version::V07,
        8 => Version::V08,
        9 => Version::V09,
        10 => Version::V10,
        11 => Version::V11,
        12 => Version::V12,
        13 => Version::V13,
        14 => Version::V14,
        15 => Version::V15,
        16 => Version::V16,
        17 => Version::V17,
        18 => Version::V18,
        19 => Version::V19,
        20 => Version::V20,
        21 => Version::V21,
        22 => Version::V22,
        23 => Version::V23,
        24 => Version::V24,
        25 => Version::V25,
        26 => Version::V26,
        27 => Version::V27,
        28 => Version::V28,
        29 => Version::V29,
        30 => Version::V30,
        31 => Version::V31,
        32 => Version::V32,
        33 => Version::V33,
        34 => Version::V34,
        35 => Version::V35,
        36 => Version::V36,
        37 => Version::V37,
        38 => Version::V38,
        39 => Version::V39,
        40 => Version::V40,
        // okay to panic, will only be converting back and forth using the enum
        _ => panic!("Invalid version number"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_capacity() {
        let expected = 4296;
        let actual = version_data_capacity(Version::V40);
        assert_eq!(expected, actual);

        let expected = 25;
        let actual = version_data_capacity(Version::V01);
        assert_eq!(expected, actual);

        let expected = 1990;
        let actual = version_data_capacity(Version::V26);
        assert_eq!(expected, actual);
    }

    #[test]
    fn version_test() {
        let actual = version_from_number(1);
        matches!(actual, Version::V01);

        let actual = version_from_number(40);
        matches!(actual, Version::V40);

        assert_eq!(Version::V01 as usize, 0);
    }
}
