# bbqr-rust

Implementaion of the bbqr spec in rust:

https://github.com/coinkite/BBQr/blob/master/BBQr.md

## Usage

### Split large data up

```rust
let data: &[&u8] = std::fs::read_to_end("test.pbst");

// split the data using zlib encoding
let split = Split::try_from_data(
    data.as_bytes(),
    FileType::UnicodeText,
    SplitOptions {
        encoding: Encoding::Zlib,
        min_split_number: 1,
        min_version: Version::V01,
        max_version: Version::V40,
        ..Default.default()
    },
)?;

// print out each of the parts
println!("{:#?}", split.parts);

// generate svgs
let qr_codes = split.generate_qr_codes_svgs()
```

### Join split QR codes

```rust
let parts: Vec<String> = ...

/// join the parts
let joined = Joined::try_from_parts(parts);

/// joined.data has the raw bytes
match &joined.encoding {
  Encoding::Unicode => String::from_utf8(joined.data),
  other => {
    // do whatever
  }
}
```
