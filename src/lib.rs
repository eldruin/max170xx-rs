//! This is a platform agnostic Rust driver for the ultra-compact, low-cost,
//! host-side fuel-gauge systems for lithium-ion (Li+) batteries in handheld
//! and portable equipment using the [`embedded-hal`] traits.
//!
//! It is compatible with MAX17043, MAX17044, MAX17048 and MAX17049.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! <!-- TODO
//! This driver allows you to:
//! -->
//! <!-- TODO
//! [Introductory blog post]()
//! -->
//!
//! <!-- TODO
//! ## The devices
//! Datasheets:
//! -->
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
#![deny(unsafe_code, missing_docs)]
#![no_std]

use core::marker::PhantomData;
mod device_impl;
mod types;
pub use crate::types::Error;
mod register_address;
use crate::register_address::{BitFlags, Command, Register};

/// MAX1704x device driver
#[derive(Debug)]
pub struct Max1704x<I2C, IC> {
    i2c: I2C,
    config: Config,
    _ic: PhantomData<IC>,
}

#[derive(Debug, Default, Clone, Copy)]
struct Config {
    bits: u16,
}

/// IC markers
pub mod ic {
    /// MAX17043 IC marker
    pub struct Max17043(());
    /// MAX17044 IC marker
    pub struct Max17044(());
}

mod private {
    use super::ic;
    pub trait Sealed {}

    impl Sealed for ic::Max17043 {}
    impl Sealed for ic::Max17044 {}
}
