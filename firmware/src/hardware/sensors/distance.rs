//! Distance sensor module (VL53L0X) for measuring distance to objects and obstacles.

use crate::hardware::sensors::SharedI2c;
use defmt::info;
use embassy_stm32::gpio::Output;
use embassy_time::{block_for, Duration};
use vl53l0x::VL53L0x;

pub struct DistanceSensor {
    sensor: VL53L0x<SharedI2c>,
    addr: u8,
}

impl DistanceSensor {
    /// Creates distance sensor (VL53L0X).
    /// - `name` sensor name for logging.
    /// - `i2c` interface to communicate with the sensor.
    /// - `shut_pin` pin for shutting down the sensor (for setting new I2C addr).
    /// - `new_addr` I2C address that will be set for the sensor.
    pub fn new(
        name: &'static str,
        i2c: SharedI2c,
        mut shut_pin: Output<'static>,
        new_addr: u8,
    ) -> Self {
        // enable sensor
        shut_pin.set_high();
        // wait for sensor to boot
        block_for(Duration::from_millis(10));

        let mut sensor = VL53L0x::new(i2c).expect("TODO handle error");
        sensor.set_address(new_addr).expect("TODO to set address");

        info!(
            "Distance sensor '{}' initialized at address 0x{}",
            name, new_addr
        );

        DistanceSensor {
            sensor,
            addr: new_addr,
        }
    }
}
