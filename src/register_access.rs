pub const ADDR: u8 = 0b011_0110;

pub struct Register;
impl Register {
    pub const VCELL: u8 = 0x02;
    pub const SOC: u8 = 0x04;
    pub const MODE: u8 = 0x06;
    pub const VERSION: u8 = 0x08;
    pub const CRATE: u8 = 0x16;
    pub const COMMAND: u8 = 0xFE;
}

pub struct Command;
impl Command {
    pub const POR_43_44: u16 = 0x0054;
    pub const POR_X8_X9: u16 = 0x5400;
    pub const QSTRT: u16 = 0x4000;
}

macro_rules! impl_register_access {
    ($name:ident) => {
        impl<I2C, E> $name<I2C>
        where
            I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
        {
            pub(crate) fn write_register(
                &mut self,
                register: u8,
                data: u16,
            ) -> Result<(), Error<E>> {
                let payload: [u8; 3] =
                    [register, ((data & 0xFF00) >> 8) as u8, (data & 0xFF) as u8];
                self.i2c.write(ADDR, &payload).map_err(Error::I2C)
            }

            pub(crate) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
                let mut data = [0; 2];
                self.i2c
                    .write_read(ADDR, &[register], &mut data)
                    .map_err(Error::I2C)
                    .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
            }
        }
    };
}
