use crate::{Command, Error, Register, ADDR};
use embedded_hal::blocking::i2c;

impl_common!(Max17048);
impl_common!(Max17049);
impl_common!(Max17058);
impl_common!(Max17059);

macro_rules! impl_common_x8_x9 {
    ($ic:ident) => {
        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            /// Get state of charge of the cell as calculated by the ModelGauge
            /// algorithm as a percentage.
            pub fn soc(&mut self) -> Result<f32, Error<E>> {
                let soc = self.read_register(Register::SOC)?;
                Ok(f32::from(soc) / 256.0)
            }

            /// Software reset
            pub fn reset(&mut self) -> Result<(), Error<E>> {
                self.write_register(Register::COMMAND, Command::POR_X8_X9)
            }
        }
    };
}
impl_common_x8_x9!(Max17048);
impl_common_x8_x9!(Max17049);
impl_common_x8_x9!(Max17058);
impl_common_x8_x9!(Max17059);

macro_rules! impl_common_x8 {
    ($ic:ident) => {
        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            /// Get battery voltage in Volts
            pub fn voltage(&mut self) -> Result<f32, Error<E>> {
                let vcell = self.read_register(Register::VCELL)?;
                Ok(f32::from(vcell) * 5.0 / 64000.0)
            }
        }
    };
}
impl_common_x8!(Max17048);
impl_common_x8!(Max17058);

macro_rules! impl_common_x9 {
    ($ic:ident) => {
        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            /// Get battery voltage in Volts
            pub fn voltage(&mut self) -> Result<f32, Error<E>> {
                let vcell = self.read_register(Register::VCELL)?;
                Ok(f32::from(vcell) * 5.0 / 32000.0)
            }
        }
    };
}
impl_common_x9!(Max17049);
impl_common_x9!(Max17059);

macro_rules! impl_common_48_49 {
    ($ic:ident) => {
        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            /// Get the approximate charge or discharge rate of the battery
            /// in percentage / hour
            pub fn charge_rate(&mut self) -> Result<f32, Error<E>> {
                let rate = self.read_register(Register::CRATE)?;
                Ok(f32::from(rate) * 0.208)
            }
        }
    };
}
impl_common_48_49!(Max17048);
impl_common_48_49!(Max17049);
