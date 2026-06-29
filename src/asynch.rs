//! Async device drivers using `embedded-hal-async`.
//!
//! Enable the `async` feature and use the device types in this module when your
//! I²C implementation provides the `embedded-hal-async` traits.

use crate::{Command, Error, Register, ADDR};
use embedded_hal_async::i2c;

macro_rules! impl_register_access {
    ($ic:ident) => {
        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::I2c<Error = E>,
        {
            async fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
                let payload: [u8; 3] =
                    [register, ((data & 0xFF00) >> 8) as u8, (data & 0xFF) as u8];
                self.i2c.write(ADDR, &payload).await.map_err(Error::I2C)
            }

            #[allow(unused)]
            async fn write_u8_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
                let payload: [u8; 2] = [register, data];
                self.i2c.write(ADDR, &payload).await.map_err(Error::I2C)
            }

            async fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
                let mut data = [0; 2];
                self.i2c
                    .write_read(ADDR, &[register], &mut data)
                    .await
                    .map_err(Error::I2C)?;
                Ok((u16::from(data[0]) << 8) | u16::from(data[1]))
            }
        }
    };
}

macro_rules! impl_common {
    ($ic:ident) => {
        /// Device driver.
        #[derive(Debug)]
        pub struct $ic<I2C> {
            i2c: I2C,
        }

        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::I2c<Error = E>,
        {
            /// Create new instance of the device.
            pub fn new(i2c: I2C) -> Self {
                $ic { i2c }
            }

            /// Destroy driver instance, return I2C bus.
            pub fn destroy(self) -> I2C {
                self.i2c
            }

            /// Quick start.
            ///
            /// Restarts fuel-gauge calculations in the same manner as initial power-up
            /// of the IC. This is useful if an application's power-up sequence
            /// is exceedingly noisy.
            pub async fn quickstart(&mut self) -> Result<(), Error<E>> {
                self.write_register(Register::MODE, Command::QSTRT).await
            }

            /// Get IC version.
            pub async fn version(&mut self) -> Result<u16, Error<E>> {
                self.read_register(Register::VERSION).await
            }
        }

        impl_register_access!($ic);
    };
}

impl_common!(Max17043);
impl_common!(Max17044);
impl_common!(Max17048);
impl_common!(Max17049);
impl_common!(Max17058);
impl_common!(Max17059);

macro_rules! impl_common_4x {
    ($ic:ident) => {
        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::I2c<Error = E>,
        {
            /// Get state of charge of the cell as calculated by the ModelGauge
            /// algorithm as a percentage.
            pub async fn soc(&mut self) -> Result<f32, Error<E>> {
                let soc = self.read_register(Register::SOC).await?;
                Ok(f32::from((soc & 0xFF00) >> 8) + f32::from(soc & 0xFF) / 256.0)
            }

            /// Software reset.
            pub async fn reset(&mut self) -> Result<(), Error<E>> {
                self.write_register(Register::COMMAND, Command::POR_43_44)
                    .await
            }
        }
    };
}
impl_common_4x!(Max17043);
impl_common_4x!(Max17044);

impl<I2C, E> Max17043<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    /// Get battery voltage in Volts.
    pub async fn voltage(&mut self) -> Result<f32, Error<E>> {
        let vcell = self.read_register(Register::VCELL).await?;
        Ok(f32::from(vcell >> 4) / 800.0)
    }
}

impl<I2C, E> Max17044<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    /// Get battery voltage in Volts.
    pub async fn voltage(&mut self) -> Result<f32, Error<E>> {
        let vcell = self.read_register(Register::VCELL).await?;
        Ok(f32::from(vcell >> 4) / 400.0)
    }
}

macro_rules! impl_common_x8_x9 {
    ($ic:ident) => {
        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::I2c<Error = E>,
        {
            /// Get state of charge of the cell as calculated by the ModelGauge
            /// algorithm as a percentage.
            pub async fn soc(&mut self) -> Result<f32, Error<E>> {
                let soc = self.read_register(Register::SOC).await?;
                Ok(f32::from(soc) / 256.0)
            }

            /// Software reset.
            pub async fn reset(&mut self) -> Result<(), Error<E>> {
                self.write_register(Register::COMMAND, Command::POR_X8_X9)
                    .await
            }

            /// Set table values.
            ///
            /// This unlocks the table registers, writes the table values and locks the
            /// table registers again.
            pub async fn set_table(&mut self, table: &[u16; 64]) -> Result<(), Error<E>> {
                // unlock table registers
                self.write_u8_register(0x3F, 0x57).await?;
                self.write_u8_register(0x3E, 0x4A).await?;

                let mut data = [0; 129];
                data[0] = 0x40;
                for (i, v) in table.iter().enumerate() {
                    data[i * 2 + 1] = ((v & 0xFF00) >> 8) as u8;
                    data[i * 2 + 2] = (v & 0xFF) as u8;
                }
                self.i2c.write(ADDR, &data).await.map_err(Error::I2C)?;

                // lock table again
                self.write_u8_register(0x3F, 0x00).await?;
                self.write_u8_register(0x3E, 0x00).await
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
            I2C: i2c::I2c<Error = E>,
        {
            /// Get battery voltage in Volts.
            pub async fn voltage(&mut self) -> Result<f32, Error<E>> {
                let vcell = self.read_register(Register::VCELL).await?;
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
            I2C: i2c::I2c<Error = E>,
        {
            /// Get battery voltage in Volts.
            pub async fn voltage(&mut self) -> Result<f32, Error<E>> {
                let vcell = self.read_register(Register::VCELL).await?;
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
            I2C: i2c::I2c<Error = E>,
        {
            /// Get the approximate charge or discharge rate of the battery
            /// in percentage / hour.
            pub async fn charge_rate(&mut self) -> Result<f32, Error<E>> {
                let rate = self.read_register(Register::CRATE).await? as i16;
                Ok(f32::from(rate) * 0.208)
            }
        }
    };
}
impl_common_48_49!(Max17048);
impl_common_48_49!(Max17049);
