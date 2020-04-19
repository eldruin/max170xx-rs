use crate::{ic, Command, Config, Error, Max1704x, Register};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c;

impl<I2C> Max1704x<I2C, ic::Max17043> {
    /// Create new instance of a MAX17043 device.
    pub fn new_max17043(i2c: I2C) -> Self {
        Max1704x {
            i2c,
            config: Config { bits: 0x971C },
            _ic: PhantomData,
        }
    }
}
impl<I2C> Max1704x<I2C, ic::Max17044> {
    /// Create new instance of a MAX17044 device.
    pub fn new_max17044(i2c: I2C) -> Self {
        Max1704x {
            i2c,
            config: Config { bits: 0x971C },
            _ic: PhantomData,
        }
    }
}

impl<I2C, IC> Max1704x<I2C, IC> {
    /// Destroy driver instance, return I2C bus.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C, E, IC> Max1704x<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Quick start
    ///
    /// Restarts fuel-gauge calculations in the same manner as initial power-up
    /// of the IC. This is useful if an application's power-up sequence
    /// is exceedingly noisy
    pub fn quickstart(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::MODE, Command::QSTRT)
    }

    /// Get state of charge of the cell as calculated by the ModelGauge
    /// algorithm as a percentage.
    pub fn soc(&mut self) -> Result<f32, Error<E>> {
        let soc = self.read_register(Register::SOC)?;
        Ok(f32::from((soc & 0xFF00) >> 8) + f32::from(soc & 0xFF) / 256.0)
    }

    /// Get IC version
    pub fn version(&mut self) -> Result<u16, Error<E>> {
        self.read_register(Register::VERSION)
    }

    /// Software reset
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::COMMAND, Command::POR)
    }
}

impl<I2C, E> Max1704x<I2C, ic::Max17043>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Get battery voltage
    pub fn voltage(&mut self) -> Result<f32, Error<E>> {
        let vcell = self.read_register(Register::VCELL)?;
        Ok(f32::from(vcell >> 4) / 800.0)
    }
}

impl<I2C, E> Max1704x<I2C, ic::Max17044>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Get battery voltage
    pub fn voltage(&mut self) -> Result<f32, Error<E>> {
        let vcell = self.read_register(Register::VCELL)?;
        Ok(f32::from(vcell >> 4) / 400.0)
    }
}

impl Config {
    fn with_high(self, mask: u16) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u16) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}
