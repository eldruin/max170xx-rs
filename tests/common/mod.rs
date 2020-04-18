use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use max1704x::{ic, Max1704x};

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

#[allow(unused)]
pub fn new_max17043_4(transactions: &[I2cTrans]) -> Max1704x<I2cMock, ic::Max17043_4> {
    Max1704x::new_max17043_4(I2cMock::new(transactions))
}

#[allow(unused)]
pub fn destroy<IC>(sensor: Max1704x<I2cMock, IC>) {
    sensor.destroy().done();
}
