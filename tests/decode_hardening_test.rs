//! Integrity and robustness of the join path.
//!
//! Joined bytes end up being parsed as a PSBT or a transaction by the caller,
//! so a decode that silently returns the wrong bytes is worse than one that
//! fails: the caller has no way to tell the difference.

use bbqr::{
    decode::MAX_DECOMPRESSED_SIZE,
    encode::Encoding,
    file_type::FileType,
    header::Header,
    join::{JoinError, Joined},
    split::{Split, SplitError, SplitOptions},
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

/// Header parsing slices at byte offsets while the length guard counts bytes,
/// so a multi-byte character crossing one of those offsets panicked. QR
/// content is attacker-controlled, and a panic in the scanning path takes the
/// wallet process down.
#[test]
fn a_multibyte_header_is_rejected_not_a_panic() {
    // Each of these is at least HEADER_LENGTH bytes long and has a multi-byte
    // character straddling a slice boundary.
    for input in [
        "\u{20AC}\u{20AC}\u{20AC}\u{20AC}",
        "B\u{20AC}ZU0801",
        "\u{20AC}B$ZU08",
        "B$\u{20AC}U0801",
        "B$ZU\u{20AC}801",
    ] {
        assert!(
            Header::try_from_str(input).is_err(),
            "expected a parse error for {input:?}, not a panic"
        );
    }
}

/// A joined payload cannot exceed what a valid BBQr stream can carry, so an
/// inflated size far beyond that means the DEFLATE stream is hostile. Without
/// a bound, `read_to_end` allocates until the process is killed.
#[test]
fn a_zlib_bomb_is_rejected_rather_than_inflated() {
    use flate2::{write::DeflateEncoder, Compression};
    use std::io::Write as _;

    // Raw DEFLATE, which is what the decoder is configured for: no zlib
    // wrapper and therefore no adler32 either. 64 MiB of zeros compresses at
    // close to DEFLATE's 1032:1 ceiling, so a stream spread over several parts
    // reaches gigabytes once inflated.
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(&vec![0u8; 64 * 1024 * 1024])
        .expect("write");
    let compressed = encoder.finish().expect("finish");

    let body = data_encoding::BASE32_NOPAD.encode(&compressed);
    // header: B$ + Z(zlib) + P(psbt) + total 01 + index 00
    let frame = format!("B$ZP0100{body}");

    let joined = Joined::try_from_parts(vec![frame]);
    assert!(
        joined.is_err(),
        "an over-large inflation must be refused, got {} bytes",
        joined.map(|j| j.data.len()).unwrap_or(0),
    );
}

/// The splitter cannot create a Zlib stream whose decoded form exceeds the
/// decoder limit
#[test]
fn an_over_limit_zlib_input_is_rejected_before_split() {
    let payload = vec![0u8; MAX_DECOMPRESSED_SIZE + 1];

    let split = Split::try_from_data(&payload, FileType::Psbt, SplitOptions::default());

    assert_eq!(
        split.unwrap_err(),
        SplitError::ZlibInputTooLarge {
            size: payload.len(),
            limit: MAX_DECOMPRESSED_SIZE,
        }
    );
}

/// The decoder limit is inclusive, so a source at the limit must still make a
/// stream that joins to the original data
#[test]
fn a_zlib_input_at_the_limit_roundtrips() {
    let payload = vec![0u8; MAX_DECOMPRESSED_SIZE];
    let split = Split::try_from_data(&payload, FileType::Psbt, SplitOptions::default())
        .expect("split at decoder limit");

    let joined = Joined::try_from_parts(split.parts).expect("join at decoder limit");

    assert_eq!(joined.data, payload);
}

/// Only the first part goes through `Header::try_from_str`; later parts are
/// length-checked and then sliced by byte offset. A later part carrying
/// multi-byte text therefore still reached a `&str` slice.
#[test]
fn a_later_part_with_multibyte_text_is_rejected_not_a_panic() {
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
    assert!(parts.len() >= 2);
    // Long enough to pass the length guard, with a multi-byte character
    // occupying bytes 4..7 so that byte offset 6 lands inside it.
    parts[1] = "AAAA\u{20AC}AAAA".to_string();
    assert!(
        !parts[1].is_char_boundary(6),
        "byte 6 must split a character"
    );

    assert!(
        Joined::try_from_parts(parts).is_err(),
        "a non-ASCII part must be an error, not a panic"
    );
}

/// The index characters are only known to exist, never to be base36, and the
/// parse `unwrap`ed. A frame with punctuation there took the process down.
#[test]
fn a_part_with_a_non_base36_index_is_rejected_not_a_panic() {
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
    assert!(parts.len() >= 2);
    let mut bytes = parts[1].clone().into_bytes();
    // Bytes 6 and 7 are the part index.
    bytes[6] = b'!';
    bytes[7] = b'!';
    parts[1] = String::from_utf8(bytes).expect("still utf8");

    assert!(
        Joined::try_from_parts(parts).is_err(),
        "a malformed part index must be an error, not a panic"
    );
}

/// A header-only first frame must not claim its payload slot or change the
/// joiner state
#[test]
fn a_header_only_first_frame_does_not_complete_the_join() {
    use bbqr::continuous_join::{ContinuousJoinError, ContinuousJoinResult, ContinuousJoiner};

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
    let total = split.parts.len();
    assert!(total >= 2);

    // a header-only frame has a valid header and no payload
    let header_only = split.parts[0][..8].to_string();

    let mut joiner = ContinuousJoiner::new();
    assert_eq!(
        joiner.add_part(header_only),
        Err(ContinuousJoinError::JoinError(JoinError::PartWithNoData(0)))
    );
    assert_eq!(
        joiner.add_part(String::new()).expect("read joiner state"),
        ContinuousJoinResult::NotStarted
    );

    let mut result = ContinuousJoinResult::NotStarted;
    for part in split.parts.iter().skip(1) {
        result = joiner.add_part(part.clone()).expect("add valid part");

        assert!(!matches!(result, ContinuousJoinResult::Complete(_)));
    }

    assert_eq!(result, ContinuousJoinResult::InProgress { parts_left: 1 });

    let result = joiner
        .add_part(split.parts[0].clone())
        .expect("add missing part");
    let ContinuousJoinResult::Complete(joined) = result else {
        panic!("missing part must complete the join");
    };

    assert_eq!(joined.data, payload);
}

/// `generate_qr_codes` mapped each part into a QR and then dropped the
/// failures, so the caller could receive fewer codes than there are parts and
/// display an animation that can never be reassembled.
#[test]
fn generating_qr_codes_does_not_silently_drop_frames() {
    use bbqr::qr::Version;

    // A part far larger than the smallest QR version can hold.
    let split = Split {
        version: Version::V01,
        parts: vec!["B$HP0100".to_string() + &"A".repeat(4000)],
        encoding: Encoding::Hex,
    };

    // Surfacing the failure is also correct; silently returning fewer is not.
    if let Ok(qrs) = split.generate_qr_codes() {
        assert_eq!(
            qrs.len(),
            split.parts.len(),
            "returned {} codes for {} parts",
            qrs.len(),
            split.parts.len()
        );
    }
}
