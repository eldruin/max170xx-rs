# Rust MAX170xx 1-Cell/2-Cell Fuel Gauge for Lithium-ion (Li+) Batteries Driver

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/max170xx.svg)](https://crates.io/crates/max170xx)
[![Docs](https://docs.rs/max170xx/badge.svg)](https://docs.rs/max170xx)
-->
[![Build Status](https://travis-ci.com/eldruin/max170xx-rs.svg?branch=master)](https://travis-ci.com/eldruin/max170xx-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/max170xx-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/max170xx-rs?branch=master)

This is a platform agnostic Rust driver for the ultra-compact, low-cost,
host-side fuel-gauge systems for lithium-ion (Li+) batteries in handheld
and portable equipment using the [`embedded-hal`] traits.

It is compatible with MAX17043, MAX17044, MAX17048, MAX17049, MAX17058 and MAX17059.

<!-- TODO
This driver allows you to:
-->
<!-- TODO
[Introductory blog post]()
-->

## The devices
<!-- TODO 
Datasheets: 
-->

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/max170xx-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
