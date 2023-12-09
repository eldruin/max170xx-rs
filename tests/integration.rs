mod base;
use crate::base::{
    destroy_43, destroy_44, destroy_48, destroy_49, destroy_58, destroy_59, new_43, new_44, new_48,
    new_49, new_58, new_59, Command, Register, ADDR,
};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

macro_rules! get_float {
    ($name:ident, $create:ident, $destroy:ident, $method:ident, $reg:ident, $v0:expr, $v1:expr, $expected:expr) => {
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
            $destroy(sensor);
        }
    };
}

macro_rules! cmd_test {
    ($name:ident, $create:ident, $destroy:ident, $method:ident, $reg:ident, $cmd:ident) => {
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
            $destroy(sensor);
        }
    };
}

macro_rules! common_tests {
    ($create:ident, $destroy:ident, $ic:ident) => {
        mod $ic {
            use super::*;

            #[test]
            fn can_create_and_destroy() {
                let sensor = $create(&[]);
                $destroy(sensor);
            }

            #[test]
            fn can_get_version() {
                let version = 0xABCD;
                let mut sensor = $create(&[I2cTrans::write_read(
                    ADDR,
                    vec![Register::VERSION],
                    vec![0xAB, 0xCD],
                )]);
                let v = sensor.version().unwrap();
                assert_eq!(v, version);
                $destroy(sensor);
            }

            cmd_test!(quickstart, $create, $destroy, quickstart, MODE, QSTRT);
        }
    };
}

mod common {
    use super::*;
    common_tests!(new_43, destroy_43, max17043);
    common_tests!(new_44, destroy_44, max17044);
    common_tests!(new_48, destroy_48, max17048);
    common_tests!(new_49, destroy_49, max17049);
    common_tests!(new_58, destroy_58, max17058);
    common_tests!(new_59, destroy_59, max17059);
}

mod max17043 {
    use super::*;
    get_float!(get_soc, new_43, destroy_43, soc, SOC, 56, 151, 56.59);
    get_float!(voltage, new_43, destroy_43, voltage, VCELL, 0x87, 0x8F, 2.71);
    cmd_test!(reset, new_43, destroy_43, reset, COMMAND, POR_43_44);
}

mod max17044 {
    use super::*;
    get_float!(get_soc, new_44, destroy_44, soc, SOC, 56, 151, 56.59);
    get_float!(voltage, new_44, destroy_44, voltage, VCELL, 0x87, 0x8F, 5.42);
    cmd_test!(reset, new_44, destroy_44, reset, COMMAND, POR_43_44);
}

macro_rules! set_table_test {
    ($name:ident, $create:ident, $destroy:ident) => {
        #[test]
        fn $name() {
            let mut expected: Vec<u8> = [0; 129]
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    return i as u8;
                })
                .collect();
            expected[0] = 0x40;
            let mut data = [0; 64];
            for i in 0..data.len() {
                data[i] = (((i * 2 + 1) << 8) | i * 2 + 2) as u16;
            }
            let mut sensor = $create(&[
                I2cTrans::write(ADDR, vec![0x3F, 0x57]),
                I2cTrans::write(ADDR, vec![0x3E, 0x4A]),
                I2cTrans::write(ADDR, expected),
                I2cTrans::write(ADDR, vec![0x3F, 0x00]),
                I2cTrans::write(ADDR, vec![0x3E, 0x00]),
            ]);
            sensor.set_table(&data).unwrap();
            $destroy(sensor);
        }
    };
}

mod max17048 {
    use super::*;
    get_float!(get_soc, new_48, destroy_48, soc, SOC, 48, 126, 48.49);
    get_float!(voltage, new_48, destroy_48, voltage, VCELL, 0xA4, 0x9F, 3.29);
    get_float!(rate, new_48, destroy_48, charge_rate, CRATE, 1, 0x45, 67.6);
    get_float!(
        negative_rate,
        new_48,
        destroy_48,
        charge_rate,
        CRATE,
        0xFE,
        0xBB,
        -67.6
    );
    cmd_test!(reset, new_48, destroy_48, reset, COMMAND, POR_X8_X9);
    set_table_test!(set_table, new_48, destroy_48);
}

mod max17058 {
    use super::*;
    get_float!(get_soc, new_58, destroy_58, soc, SOC, 48, 126, 48.49);
    get_float!(voltage, new_58, destroy_58, voltage, VCELL, 0xA4, 0x9F, 3.29);
    cmd_test!(reset, new_58, destroy_58, reset, COMMAND, POR_X8_X9);
    set_table_test!(set_table, new_58, destroy_58);
}

mod max17049 {
    use super::*;
    get_float!(get_soc, new_49, destroy_49, soc, SOC, 48, 126, 48.49);
    get_float!(voltage, new_49, destroy_49, voltage, VCELL, 0xA4, 0x9F, 6.58);
    get_float!(rate, new_49, destroy_49, charge_rate, CRATE, 1, 0x45, 67.6);
    get_float!(
        negative_rate,
        new_49,
        destroy_49,
        charge_rate,
        CRATE,
        0xFE,
        0xBB,
        -67.6
    );
    cmd_test!(reset, new_49, destroy_49, reset, COMMAND, POR_X8_X9);
    set_table_test!(set_table, new_49, destroy_49);
}

mod max17059 {
    use super::*;
    get_float!(get_soc, new_59, destroy_59, soc, SOC, 48, 126, 48.49);
    get_float!(voltage, new_59, destroy_59, voltage, VCELL, 0xA4, 0x9F, 6.58);
    cmd_test!(reset, new_59, destroy_59, reset, COMMAND, POR_X8_X9);
    set_table_test!(set_table, new_59, destroy_59);
}
