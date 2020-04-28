use crate::{Command, Error, Register, ADDR};
use embedded_hal::blocking::i2c;

impl_common!(Max17043);
impl_common!(Max17044);

macro_rules! impl_common_4x {
    ($ic:ident) => {
        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            /// Get state of charge of the cell as calculated by the ModelGauge
            /// algorithm as a percentage.
            pub fn soc(&mut self) -> Result<f32, Error<E>> {
                let soc = self.read_register(Register::SOC)?;
                Ok(f32::from((soc & 0xFF00) >> 8) + f32::from(soc & 0xFF) / 256.0)
            }

            /// Software reset
            pub fn reset(&mut self) -> Result<(), Error<E>> {
                self.write_register(Register::COMMAND, Command::POR_43_44)
            }
        }
    };
}
impl_common_4x!(Max17043);
impl_common_4x!(Max17044);

impl<I2C, E> Max17043<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Get battery voltage
    pub fn voltage(&mut self) -> Result<f32, Error<E>> {
        let vcell = self.read_register(Register::VCELL)?;
        Ok(f32::from(vcell >> 4) / 800.0)
    }
}

impl<I2C, E> Max17044<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Get battery voltage
    pub fn voltage(&mut self) -> Result<f32, Error<E>> {
        let vcell = self.read_register(Register::VCELL)?;
        Ok(f32::from(vcell >> 4) / 400.0)
    }
}
