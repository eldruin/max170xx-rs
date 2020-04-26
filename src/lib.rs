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

mod types;
pub use types::Error;
#[macro_use]
mod common;
#[macro_use]
mod register_access;
use register_access::{Command, Register, ADDR};
mod max17043_44;
pub use max17043_44::{Max17043, Max17044};
mod max170x8_x9;
pub use max170x8_x9::{Max17048, Max17049, Max17058, Max17059};

#[derive(Debug, Default, Clone, Copy)]
struct Config {
    bits: u16,
}
