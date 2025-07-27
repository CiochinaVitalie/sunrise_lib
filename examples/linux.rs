use linux_embedded_hal::{Delay, I2cdev, Pin};
use linux_embedded_hal::sysfs_gpio::Direction;
use sunrise_lib::senseair::Sunrise;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // open I2C bus
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let delay = Delay;

    // configure GPIO pins according to your wiring
    let mut en = Pin::new(27);
    en.export()?;
    en.set_direction(Direction::Out)?;

    let mut nrdy = Pin::new(17);
    nrdy.export()?;
    nrdy.set_direction(Direction::In)?;

    // create Sunrise driver
    let mut sunrise = Sunrise::new(i2c, delay, Some(en), nrdy);
    sunrise.init(None).unwrap();

    let meas = sunrise.co2_measurement_get(None).unwrap();
    println!("CO2: {}", meas.measured_filtered_press_comp);
    Ok(())
}

