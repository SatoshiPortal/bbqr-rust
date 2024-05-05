use bbqr::{Encoding, FileType, Joined, Split, SplitOptions, Version};
// use pretty_assertions::assert_eq;

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

        println!("min_split: {}, split {}", min_split, split.parts.len());

        assert!(split.parts.len() >= min_split);

        let joined = Joined::try_from_parts(split.parts).unwrap();
        assert_eq!(joined.file_type, FileType::Transaction);
        assert_eq!(joined.data, data);
    }
}

// #[test]
// fn test_edge27() {
//     let encodings = ['H', '2', 'Z'];
//     let sizes = 1060..1080;
//     let low_ents = [true, false];
//
//     for &encoding in &encodings {
//         for size in sizes.clone() {
//             for &low_ent in &low_ents {
//                 let data = if low_ent {
//                     vec![b'A'; size]
//                 } else {
//                     (0..size).map(|_| rand::random::<u8>()).collect()
//                 };
//
//                 let (vers, parts) = split_qrs(&data, "T", Some(encoding), 1, 27).unwrap();
//                 assert_eq!(vers, 27);
//                 let count = parts.len();
//
//                 match encoding {
//                     'H' => assert_eq!(count, if size <= 1062 { 1 } else { 2 }),
//                     'Z' => {
//                         assert_eq!(count, 1);
//                         if low_ent {
//                             assert!(parts[0].len() < 100);
//                         }
//                     }
//                     '2' => assert_eq!(count, 1),
//                     _ => unreachable!(),
//                 }
//
//                 let (_, readback) = join_qrs(&parts).unwrap();
//                 assert_eq!(readback, data);
//             }
//         }
//     }
// }

// #[test]
// fn test_maxsize() {
//     let encodings = ['H', '2'];
//
//     for &encoding in &encodings {
//         let nc = 4296 - 8;
//         let pkt_size = match encoding {
//             'H' => nc / 2,
//             '2' => nc * 5 / 8,
//             _ => unreachable!(),
// }
