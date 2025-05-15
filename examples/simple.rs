use bbqr::{
    encode::Encoding,
    file_type::FileType,
    join::Joined,
    qr::Version,
    split::{Split, SplitOptions},
};

fn main() {
    let data: &[u8] = b"Hello, World!, but much larger";

    // or split the data using zlib encoding, and custom options
    let split = Split::try_from_data(
        data,
        FileType::UnicodeText,
        SplitOptions {
            encoding: Encoding::Zlib,
            min_split_number: 1,
            max_split_number: 100,
            min_version: Version::V03,
            max_version: Version::V30,
        },
    )
    .expect("Failed to split data");

    // print out each of the parts
    println!("SPLIT: {:?}", split);

    // join the parts
    let joined = Joined::try_from_parts(split.parts).expect("Failed to join parts");

    // print out the joined data
    println!("JOINED: {:?}", joined);

    match &joined.file_type {
        FileType::UnicodeText => {
            println!("JOINED TEXT: {}", String::from_utf8(joined.data).unwrap())
        }
        other => {
            panic!(
                "Unexpected file type {}: got {:?}",
                FileType::UnicodeText,
                other
            );
        }
    }
}
