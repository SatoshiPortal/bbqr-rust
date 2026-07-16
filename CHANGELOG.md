# Changelog

## [Unreleased]

## [0.5.0] - 2026-07-16

### Added

- Add Key Teleport Receiver, Sender, and PSBT file types

## [0.4.1] - 2026-05-25

### Internal

- Update dependencies

## [0.4.0] - 2025-05-15

### Added

- Add simple split and join example
- Add PSBT decoding example

### Fixed

- Fix README and crate documentation examples to use `FileType::UnicodeText`

### Internal

- Update dependencies
- Bump MSRV to 1.73
- Improve CI GitHub Action

## [0.3.5] - 2025-03-17

### Internal

- Use `zlib-rs` implementation for zlib encoding/decoding, remove needing to build c lib

## [0.3.4] - 2024-07-24

- Add `TryFrom` to convert from number to `Version`
- Add `VersionError` type

## [0.3.3] - 2024-07-22

- Expose `Header::try_from_str` publicly

## [0.3.2] - 2024-07-22

- Publish to crates.io, and update link to docs

## [0.3.1] - 2024-05-15

- Update `fast_qr` dep, and force Alphanumeric encoding when making QR code with `fast_qr`

## [0.3.0] - 2024-05-13

- Add ability to add QR codes one by one and get the final result

## [0.2.0] - 2024-05-13

- Change API structure, expose types from the modules, avoid using type aliases at the top level

## [0.1.1] - 2024-05-12

- Exposes error types publicly

## [0.1.0] - 2024-05-06

- Initial release, support splitting and joining
