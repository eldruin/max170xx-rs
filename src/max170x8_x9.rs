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
        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            /// Set table values.
            ///
            /// This unlocks the table registers, writes the table values and locks the
            /// table registers again.
            pub fn set_table(&mut self, table: &[u16; 64]) -> Result<(), Error<E>> {
                // unlock table registers
                self.write_u8_register(0x3F, 0x57)?;
                self.write_u8_register(0x3E, 0x4A)?;

                let mut data = [0; 129];
                data[0] = 0x40;
                for (i, v) in table.iter().enumerate() {
                    data[i * 2 + 1] = ((v & 0xFF00) >> 8) as u8;
                    data[i * 2 + 2] = (v & 0xFF) as u8;
                }
                self.i2c.write(ADDR, &data).map_err(Error::I2C)?;

                // lock table again
                self.write_u8_register(0x3F, 0x00)?;
                self.write_u8_register(0x3E, 0x00)
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
