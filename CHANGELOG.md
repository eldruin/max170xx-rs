# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Changed
- Updated `embedded-hal` to version 1.0.0.
- Updated `linux-embedded-hal` to version 0.4.
- Updated `embedded-hal-mock` to version 0.10.

## [0.1.1] - 2023-12-11

### Changed

- MSRV was raised to 1.62.0.

### Fixed

- MAX17048 & MAX17049 `charge_rate` correctly handles negative values (such as when discharging). See: [PR #2](https://github.com/eldruin/max170xx-rs/pull/2)

## [0.1.0] - 2020-07-19

Initial release of the driver to crates.io.


<!-- next-url -->
[Unreleased]: https://github.com/eldruin/max170xx-rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/eldruin/max170xx-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/eldruin/max170xx-rs/releases/tag/v0.1.0
