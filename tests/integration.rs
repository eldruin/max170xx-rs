mod common;
use crate::common::{destroy, new_max17043_4, Command, Register, ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

#[test]
fn can_create_and_destroy_max17043_4() {
    let sensor = new_max17043_4(&[]);
    destroy(sensor);
}

#[test]
fn can_get_version() {
    let version = 0xABCD;
    let mut sensor = new_max17043_4(&[I2cTrans::write_read(
        ADDR,
        vec![Register::VERSION],
        vec![0xAB, 0xCD],
    )]);
    let v = sensor.version().unwrap();
    assert_eq!(v, version);
    destroy(sensor);
}

#[test]
fn can_reset() {
    let mut sensor = new_max17043_4(&[I2cTrans::write(
        ADDR,
        vec![
            Register::COMMAND,
            ((Command::POR & 0xFF00) >> 8) as u8,
            (Command::POR & 0xFF) as u8,
        ],
    )]);
    sensor.reset().unwrap();
    destroy(sensor);
}

#[test]
fn can_quickstart() {
    let mut sensor = new_max17043_4(&[I2cTrans::write(
        ADDR,
        vec![
            Register::MODE,
            ((Command::QSTRT & 0xFF00) >> 8) as u8,
            (Command::QSTRT & 0xFF) as u8,
        ],
    )]);
    sensor.quickstart().unwrap();
    destroy(sensor);
}
