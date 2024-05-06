# bbqr-rust

Implementaion of the bbqr spec in rust:

https://github.com/coinkite/BBQr/blob/master/BBQr.md

## Usage

### Split large data up

```rust
use bbqr::{Encoding, FileType, Split, SplitOptions, Version};

let data: &[u8] = b"Hello, World!, but much larger";

// split the data using zlib encoding
let split = Split::try_from_data(
    data,
    FileType::UnicodeText,
    SplitOptions {
        encoding: Encoding::Zlib,
        min_split_number: 1,
        min_version: Version::V01,
        max_version: Version::V40,
        ..Default::default()
    },
).expect("Failed to split data");

// print out each of the parts
println!("{:#?}", split.parts);

// generate the qr codes
let qr_codes = split.generate_qr_codes();
```

### Join split QR codes

```rust
// get the parts from somewhere
let parts: Vec<String> = ...
// join the parts
let joined = Joined::try_from_parts(parts);

/// joined.data has the raw bytes
match &joined.encoding {
  Encoding::Unicode => String::from_utf8(joined.data),
  other => {
    // do whatever
  }
}
```
