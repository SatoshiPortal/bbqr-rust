use bbqr::{Encoding, FileType, Joined, Split, SplitOptions, Version};
use pretty_assertions::assert_eq;

#[test]
fn test_real_from_readme() {
    let data = include_str!("../test_data/BBQr.md");

    // let encodings = [Encoding::Hex, Encoding::Base32, Encoding::Zlib];
    let encodings = [Encoding::Hex];
    for encoding in encodings {
        let split = Split::try_from_data(
            data.as_bytes(),
            FileType::UnicodeText,
            SplitOptions {
                encoding,
                min_split_size: 1,
                max_split_size: 1295,
                min_version: Version::V01,
                max_version: Version::V40,
            },
        );

        assert!(split.is_ok());
        let split = split.unwrap();

        let join_from_readme = Joined::try_from_parts(split.parts);
        assert!(join_from_readme.is_ok());

        let join_from_readme = join_from_readme.unwrap();
        assert_eq!(String::from_utf8(join_from_readme.data).unwrap(), data);
    }
}
