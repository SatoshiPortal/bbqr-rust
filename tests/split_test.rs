use bbqr::{Encoding, FileType, Joined, Split, SplitOptions, Version};
use pretty_assertions::assert_eq;

#[test]
fn test_real_scan_from_readme() {
    let real_scan = include_str!("../test_data/real-scan.txt");
    let data = include_str!("../test_data/BBQr.md");
    let split = Split::try_from_data(
        data.as_bytes(),
        FileType::UnicodeText,
        SplitOptions {
            encoding: Encoding::Zlib,
            min_split_size: 1,
            max_split_size: 1295,
            min_version: Version::V01,
            max_version: Version::V20,
        },
    );

    assert!(split.is_ok());

    let split = split.unwrap();

    assert!(split.version <= Version::V23);
    assert_eq!(split.parts.len(), 8);

    let real_scan_parts = real_scan
        .lines()
        .filter(|x| !x.is_empty())
        .map(String::from)
        .collect::<Vec<String>>();

    let join_from_readme = Joined::try_from_parts(real_scan_parts);
    assert!(join_from_readme.is_ok());

    let join_from_readme = join_from_readme.unwrap();

    assert_eq!(
        String::from_utf8(join_from_readme.data).unwrap(),
        String::from(data)
    );
}
