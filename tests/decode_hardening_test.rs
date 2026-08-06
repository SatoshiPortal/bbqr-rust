//! Integrity and robustness of the join path.
//!
//! Joined bytes end up being parsed as a PSBT or a transaction by the caller,
//! so a decode that silently returns the wrong bytes is worse than one that
//! fails: the caller has no way to tell the difference.

use bbqr::header::Header;

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
