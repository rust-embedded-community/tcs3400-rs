use linux_embedded_hal::I2cdev;
use tcs3400::Tcs3400;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Tcs3400::new(dev);
    sensor.enable().unwrap();
    sensor.enable_rgbc().unwrap();
    while !sensor.is_rgbc_status_valid().unwrap() {
        // wait for measurement to be available
    }
    let m = sensor.read_all_channels().unwrap();
    println!(
        "Measurements: clear = {}, red = {}, green = {}, blue = {}",
        m.clear, m.red, m.green, m.blue
    );
}
