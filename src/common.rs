use crate::Config;

macro_rules! impl_common {
    ($ic:ident) => {
        /// Device driver
        #[derive(Debug)]
        pub struct $ic<I2C> {
            i2c: I2C,
            config: Config,
        }

        impl<I2C, E> $ic<I2C>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            /// Create new instance of the device.
            pub fn new(i2c: I2C) -> Self {
                $ic {
                    i2c,
                    config: Config { bits: 0x971C },
                }
            }

            /// Destroy driver instance, return I2C bus.
            pub fn destroy(self) -> I2C {
                self.i2c
            }

            /// Quick start
            ///
            /// Restarts fuel-gauge calculations in the same manner as initial power-up
            /// of the IC. This is useful if an application's power-up sequence
            /// is exceedingly noisy
            pub fn quickstart(&mut self) -> Result<(), Error<E>> {
                self.write_register(Register::MODE, Command::QSTRT)
            }

            /// Get IC version
            pub fn version(&mut self) -> Result<u16, Error<E>> {
                self.read_register(Register::VERSION)
            }
        }
        impl_register_access!($ic);
    };
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
