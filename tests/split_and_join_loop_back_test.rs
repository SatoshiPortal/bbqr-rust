use bbqr::{
    encode::Encoding,
    file_type::FileType,
    join::Joined,
    qr::Version,
    split::{Split, SplitOptions},
};
use pretty_assertions::assert_eq;

#[test]
fn test_loopback() {
    let encodings = [Encoding::Hex, Encoding::Base32, Encoding::Zlib];
    let sizes = [10, 100, 2000, 10_000, 50_000];
    let max_versions = [Version::V11, Version::V29, Version::V40];
    let low_entropy_options = [true, false];

    // cartesian product of all the options
    for encoding in &encodings {
        for &size in &sizes {
            for &max_version in &max_versions {
                for &low_entropy in &low_entropy_options {
                    let data = if low_entropy {
                        vec![b'A'; size]
                    } else {
                        (0..size).map(|_| rand::random::<u8>()).collect()
                    };

                    let split = Split::try_from_data(
                        &data,
                        FileType::Psbt,
                        SplitOptions {
                            encoding: *encoding,
                            max_version,
                            ..Default::default()
                        },
                    );

                    assert!(split.is_ok());

                    let split = split.unwrap();
                    assert!(split.version <= max_version);

                    if low_entropy {
                        assert_eq!(split.encoding, *encoding);
                    }

                    let joined = Joined::try_from_parts(split.parts);

                    assert!(joined.is_ok());

                    let joined = joined.unwrap();
                    assert_eq!(joined.file_type, FileType::Psbt);

                    if low_entropy {
                        assert_eq!(joined.encoding, *encoding);
                    }

                    assert_eq!(joined.encoding, split.encoding);
                    assert_eq!(joined.data, data);
                }
            }
        }
    }
}

#[test]
fn test_min_split() {
    let size = 10_000;
    let data: Vec<u8> = (0..size).map(|_| rand::random::<u8>()).collect();

    for min_split in 2..10 {
        let split = Split::try_from_data(
            &data,
            FileType::Transaction,
            SplitOptions {
                encoding: Encoding::Base32,
                min_split_number: min_split,
                ..Default::default()
            },
        )
        .unwrap();

        assert!(split.parts.len() >= min_split);

        let joined = Joined::try_from_parts(split.parts).unwrap();
        assert_eq!(joined.file_type, FileType::Transaction);
        assert_eq!(joined.data, data);
    }
}

#[test]
fn test_edge27() {
    let encodings = [Encoding::Hex, Encoding::Base32, Encoding::Zlib];
    let sizes = 1060..1080;
    let low_entropies = [true, false];

    for encoding in &encodings {
        for size in sizes.clone() {
            for &low_ent in &low_entropies {
                let data = if low_ent {
                    vec![b'A'; size]
                } else {
                    (0..size).map(|_| rand::random::<u8>()).collect()
                };

                let split = Split::try_from_data(
                    &data,
                    FileType::Transaction,
                    SplitOptions {
                        encoding: *encoding,
                        min_split_number: 1,
                        max_split_number: 2,
                        min_version: Version::V27,
                        max_version: Version::V27,
                    },
                )
                .unwrap();

                let parts = &split.parts;
                let count = parts.len();

                match split.encoding {
                    Encoding::Hex => assert_eq!(count, if size <= 1062 { 1 } else { 2 }),
                    Encoding::Zlib => {
                        assert_eq!(count, 1);
                        if low_ent {
                            assert!(parts[0].len() < 100);
                        }
                    }
                    Encoding::Base32 => assert_eq!(count, 1),
                }

                let joined = Joined::try_from_parts(split.parts).unwrap();

                assert_eq!(joined.data, data);
            }
        }
    }
}

#[test]
fn test_maxsize() {
    let encodings = [Encoding::Hex, Encoding::Base32];

    for encoding in &encodings {
        // version 40 capacity in chars, less header
        let nc = 4296 - 8;

        let packet_size = match encoding {
            Encoding::Hex => nc / 2,
            Encoding::Base32 => nc * 5 / 8,
            _ => unreachable!(),
        };

        let nparts = usize::from_str_radix("ZZ", 36).unwrap(); // 1295
        let data: Vec<u8> = (0..packet_size * nparts).map(|_| rand::random()).collect();

        let split = Split::try_from_data(
            &data,
            FileType::Transaction,
            SplitOptions {
                encoding: *encoding,
                ..Default::default()
            },
        )
        .unwrap();

        assert_eq!(split.version, Version::V40);

        let count = split.parts.len();
        assert_eq!(count, nparts);

        let joined = Joined::try_from_parts(split.parts).unwrap();
        assert_eq!(joined.data, data);
    }
}
