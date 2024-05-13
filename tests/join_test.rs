use bbqr::{file_type::FileType, join::Joined};

#[test]
fn test_real_scan() {
    let lines: Vec<String> = include_str!("../test_data/real-scan.txt")
        .lines()
        .filter(|ln| !ln.is_empty())
        .map(|ln| ln.to_string())
        .collect();

    let joined = Joined::try_from_parts(lines);

    assert!(joined.is_ok());

    let joined = joined.unwrap();
    assert_eq!(joined.file_type, FileType::UnicodeText);

    let unicode_text = String::from_utf8(joined.data);
    assert!(unicode_text.is_ok());

    let data = unicode_text.unwrap();
    assert!(data.contains("Zlib compressed"));
    assert!(data.contains("PSBT"));
}
