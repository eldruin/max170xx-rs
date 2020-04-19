mod common;
use crate::common::{destroy, new_max17043, Command, Register, ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

#[test]
fn can_create_and_destroy_max17043() {
    let sensor = new_max17043(&[]);
    destroy(sensor);
}

#[test]
fn can_get_version() {
    let version = 0xABCD;
    let mut sensor = new_max17043(&[I2cTrans::write_read(
        ADDR,
        vec![Register::VERSION],
        vec![0xAB, 0xCD],
    )]);
    let v = sensor.version().unwrap();
    assert_eq!(v, version);
    destroy(sensor);
}

macro_rules! get_float {
    ($name:ident, $create:ident, $method:ident, $reg:ident, $v0:expr, $v1:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut sensor = $create(&[I2cTrans::write_read(
                ADDR,
                vec![Register::$reg],
                vec![$v0, $v1],
            )]);
            let v = sensor.$method().unwrap();
            assert!((v - 0.1) < $expected);
            assert!((v + 0.1) > $expected);
            destroy(sensor);
        }
    };
}
get_float!(get_soc, new_max17043, soc, SOC, 56, 151, 56.59);
get_float!(get_voltage, new_max17043, voltage, VCELL, 0x87, 0x8F, 2.71);

macro_rules! cmd_test {
    ($name:ident, $create:ident, $method:ident, $reg:ident, $cmd:ident) => {
        #[test]
        fn $name() {
            let mut sensor = $create(&[I2cTrans::write(
                ADDR,
                vec![
                    Register::$reg,
                    ((Command::$cmd & 0xFF00) >> 8) as u8,
                    (Command::$cmd & 0xFF) as u8,
                ],
            )]);
            sensor.$method().unwrap();
            destroy(sensor);
        }
    };
}
cmd_test!(reset, new_max17043, reset, COMMAND, POR);
cmd_test!(quickstart, new_max17043, quickstart, MODE, QSTRT);
