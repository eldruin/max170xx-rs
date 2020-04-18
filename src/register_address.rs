use crate::{Error, Max1704x};
use embedded_hal::blocking::i2c;

pub const ADDR: u8 = 0b011_0110;

pub struct Register;
impl Register {
    pub const VCELL: u8 = 0x02;
    pub const SOC: u8 = 0x04;
    pub const MODE: u8 = 0x06;
    pub const VERSION: u8 = 0x08;
    pub const CONFIG: u8 = 0x0C;
    pub const COMMAND: u8 = 0xFE;
}

pub struct Command;
impl Command {
    pub const POR: u16 = 0x0054;
}

pub struct BitFlags;
impl BitFlags {}

impl<I2C, E, IC> Max1704x<I2C, IC>
where
    I2C: i2c::Write<Error = E>,
{
    pub(crate) fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        let payload: [u8; 3] = [register, ((data & 0xFF00) >> 8) as u8, (data & 0xFF) as u8];
        self.i2c.write(ADDR, &payload).map_err(Error::I2C)
    }
}

impl<I2C, E, IC> Max1704x<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E>,
{
    pub(crate) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(ADDR, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
    }
}
