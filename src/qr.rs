use std::cmp::Ordering;

use crate::consts::QR_DATA_CAPACITY;

/// Enum containing all possible `QRCode` versions, taken from fast_qr crate
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    /// Version n°01
    V01 = 0,
    /// Version n°02
    V02 = 1,
    /// Version n°03
    V03 = 2,
    /// Version n°04
    V04 = 3,
    /// Version n°05
    V05 = 4,
    /// Version n°06
    V06 = 5,
    /// Version n°07
    V07 = 6,
    /// Version n°08
    V08 = 7,
    /// Version n°09
    V09 = 8,
    /// Version n°10
    V10 = 9,
    /// Version n°11
    V11 = 10,
    /// Version n°12
    V12 = 11,
    /// Version n°13
    V13 = 12,
    /// Version n°14
    V14 = 13,
    /// Version n°15
    V15 = 14,
    /// Version n°16
    V16 = 15,
    /// Version n°17
    V17 = 16,
    /// Version n°18
    V18 = 17,
    /// Version n°19
    V19 = 18,
    /// Version n°20
    V20 = 19,
    /// Version n°21
    V21 = 20,
    /// Version n°22
    V22 = 21,
    /// Version n°23
    V23 = 22,
    /// Version n°24
    V24 = 23,
    /// Version n°25
    V25 = 24,
    /// Version n°26
    V26 = 25,
    /// Version n°27
    V27 = 26,
    /// Version n°28
    V28 = 27,
    /// Version n°29
    V29 = 28,
    /// Version n°30
    V30 = 29,
    /// Version n°31
    V31 = 30,
    /// Version n°32
    V32 = 31,
    /// Version n°33
    V33 = 32,
    /// Version n°34
    V34 = 33,
    /// Version n°35
    V35 = 34,
    /// Version n°36
    V36 = 35,
    /// Version n°37
    V37 = 36,
    /// Version n°38
    V38 = 37,
    /// Version n°39
    V39 = 38,
    /// Version n°40
    V40 = 39,
}

/// Error Correction Coding has 4 levels, taken from fast_qr crate
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum ErrorCorrectionLevel {
    /// Low, 7%
    Low,
    /// Medium, 15%
    Medium,
    /// Quartile, 25%
    Quartile,
    /// High, 30%
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum CapacityTableEncoding {
    TotalBits = 0,
    Numeric = 1,
    Alphanumeric = 2,
    Byte = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QrsNeeded {
    pub version: Version,
    pub count: usize,
    pub data_per_qr: usize,
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

impl Version {
    pub fn data_capacity(&self) -> usize {
        let error_correction_level = ErrorCorrectionLevel::Low;
        let encoding = CapacityTableEncoding::Alphanumeric;

        QR_DATA_CAPACITY[*self as usize][error_correction_level as usize][encoding as usize]
            as usize
    }

    pub(crate) fn from_index(version_index: usize) -> Self {
        Self::from_number(version_index + 1)
    }

    pub(crate) fn from_number(version_number: usize) -> Self {
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
}

/// Convert between our type and fast-qr version type
#[cfg(feature = "qr-codes")]
impl From<Version> for fast_qr::Version {
    fn from(version: Version) -> Self {
        use fast_qr::Version as FastQrVersion;

        match version {
            Version::V01 => FastQrVersion::V01,
            Version::V02 => FastQrVersion::V02,
            Version::V03 => FastQrVersion::V03,
            Version::V04 => FastQrVersion::V04,
            Version::V05 => FastQrVersion::V05,
            Version::V06 => FastQrVersion::V06,
            Version::V07 => FastQrVersion::V07,
            Version::V08 => FastQrVersion::V08,
            Version::V09 => FastQrVersion::V09,
            Version::V10 => FastQrVersion::V10,
            Version::V11 => FastQrVersion::V11,
            Version::V12 => FastQrVersion::V12,
            Version::V13 => FastQrVersion::V13,
            Version::V14 => FastQrVersion::V14,
            Version::V15 => FastQrVersion::V15,
            Version::V16 => FastQrVersion::V16,
            Version::V17 => FastQrVersion::V17,
            Version::V18 => FastQrVersion::V18,
            Version::V19 => FastQrVersion::V19,
            Version::V20 => FastQrVersion::V20,
            Version::V21 => FastQrVersion::V21,
            Version::V22 => FastQrVersion::V22,
            Version::V23 => FastQrVersion::V23,
            Version::V24 => FastQrVersion::V24,
            Version::V25 => FastQrVersion::V25,
            Version::V26 => FastQrVersion::V26,
            Version::V27 => FastQrVersion::V27,
            Version::V28 => FastQrVersion::V28,
            Version::V29 => FastQrVersion::V29,
            Version::V30 => FastQrVersion::V30,
            Version::V31 => FastQrVersion::V31,
            Version::V32 => FastQrVersion::V32,
            Version::V33 => FastQrVersion::V33,
            Version::V34 => FastQrVersion::V34,
            Version::V35 => FastQrVersion::V35,
            Version::V36 => FastQrVersion::V36,
            Version::V37 => FastQrVersion::V37,
            Version::V38 => FastQrVersion::V38,
            Version::V39 => FastQrVersion::V39,
            Version::V40 => FastQrVersion::V40,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_capacity() {
        let expected = 4296;
        let actual = Version::V40.data_capacity();
        assert_eq!(expected, actual);

        let expected = 25;
        let actual = Version::V01.data_capacity();
        assert_eq!(expected, actual);

        let expected = 1990;
        let actual = Version::V26.data_capacity();
        assert_eq!(expected, actual);
    }

    #[test]
    fn version_test() {
        let actual = Version::from_number(1);
        matches!(actual, Version::V01);

        let actual = Version::from_number(40);
        matches!(actual, Version::V40);

        assert_eq!(Version::V01 as usize, 0);
    }
}
