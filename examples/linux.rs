extern crate linux_embedded_hal as hal;
use max1704x::Max1704x;

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Max1704x::new_max17043(dev);
    loop {
        let soc = sensor.soc().unwrap();
        let voltage = sensor.voltage().unwrap();
        println!("Charge: {:.2}%", soc);
        println!("Voltage: {:.2}V", voltage);
    }
}
