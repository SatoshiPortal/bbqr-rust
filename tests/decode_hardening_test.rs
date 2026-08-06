//! Integrity and robustness of the join path.
//!
//! Joined bytes end up being parsed as a PSBT or a transaction by the caller,
//! so a decode that silently returns the wrong bytes is worse than one that
//! fails: the caller has no way to tell the difference.

use bbqr::{
    encode::Encoding,
    file_type::FileType,
    join::Joined,
    split::{Split, SplitOptions},
};

/// A part whose payload contains a character outside the encoding's alphabet
/// must fail the join, not be dropped from it.
///
/// `flat_map` over a `Result` yields nothing for `Err`, so the per-part decode
/// error was discarded and the surviving parts were concatenated: a single
/// mutated character silently shortened the payload.
#[test]
fn a_part_that_fails_to_decode_fails_the_join() {
    let payload = vec![0xABu8; 6000];
    let split = Split::try_from_data(
        &payload,
        FileType::Psbt,
        SplitOptions {
            encoding: Encoding::Hex,
            ..Default::default()
        },
    )
    .expect("split");

    let mut parts = split.parts.clone();
    assert!(parts.len() >= 2, "need a multi-part split to splice");

    // 'Z' is not in the uppercase-hex alphabet. Byte 9 is inside the payload,
    // past the 8-character header.
    let mut bytes = parts[1].clone().into_bytes();
    bytes[9] = b'Z';
    parts[1] = String::from_utf8(bytes).expect("still utf8");

    let joined = Joined::try_from_parts(parts);
    assert!(
        joined.is_err(),
        "a corrupted part must surface an error, got {} bytes instead of {}",
        joined.map(|j| j.data.len()).unwrap_or(0),
        payload.len(),
    );
}

/// The same guarantee for base32, which BBQr uses for both `Base32` and the
/// pre-compression stage of `Zlib`.
#[test]
fn a_base32_part_that_fails_to_decode_fails_the_join() {
    let payload = vec![0x5Au8; 6000];
    let split = Split::try_from_data(
        &payload,
        FileType::Psbt,
        SplitOptions {
            encoding: Encoding::Base32,
            ..Default::default()
        },
    )
    .expect("split");

    let mut parts = split.parts.clone();
    assert!(parts.len() >= 2, "need a multi-part split to splice");

    // '0', '1' and '8' are excluded from RFC 4648 base32.
    let mut bytes = parts[1].clone().into_bytes();
    bytes[9] = b'0';
    parts[1] = String::from_utf8(bytes).expect("still utf8");

    assert!(
        Joined::try_from_parts(parts).is_err(),
        "a corrupted base32 part must surface an error"
    );
}
