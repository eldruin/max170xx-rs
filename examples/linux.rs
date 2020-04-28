use linux_embedded_hal::I2cdev;
use max170xx::Max17043;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Max17043::new(dev);
    loop {
        let soc = sensor.soc().unwrap();
        let voltage = sensor.voltage().unwrap();
        println!("Charge: {:.2}%", soc);
        println!("Voltage: {:.2}V", voltage);
    }
}
