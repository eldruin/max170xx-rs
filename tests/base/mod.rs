use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use max170xx::{Max17043, Max17044, Max17048, Max17049, Max17058, Max17059};

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

macro_rules! create_destroy {
    ($create:ident, $destroy:ident, $ic:ident) => {
        #[allow(unused)]
        pub fn $create(transactions: &[I2cTrans]) -> $ic<I2cMock> {
            $ic::new(I2cMock::new(transactions))
        }

        #[allow(unused)]
        pub fn $destroy(sensor: $ic<I2cMock>) {
            sensor.destroy().done();
        }
    };
}

create_destroy!(new_43, destroy_43, Max17043);
create_destroy!(new_44, destroy_44, Max17044);
create_destroy!(new_48, destroy_48, Max17048);
create_destroy!(new_49, destroy_49, Max17049);
create_destroy!(new_58, destroy_58, Max17058);
create_destroy!(new_59, destroy_59, Max17059);
