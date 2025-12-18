#![no_std]
#![no_main]

use engine::{
    core::error,
    hardware::{led, sensors, sensors::distance::DistanceSensor},
};

use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    i2c::{self},
    Config,
};
use embassy_time::{Duration, Timer};
use embedded_hal_bus::i2c::RefCellDevice;

use defmt as _;
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    // Small delay to allow RTT connection to establish
    Timer::after(Duration::from_millis(10)).await;

    // Initialize LED
    let led_pin = Output::new(p.PC13, Level::Low, Speed::Low);
    let led = led::Led::new("status_led", led_pin);

    spawner.spawn(led::led_task(led)).unwrap();
    spawner.spawn(error::error_handler()).unwrap();

    let i2c = i2c::I2c::new_blocking(p.I2C1, p.PB8, p.PB9, i2c::Config::default());
    // Store I2C bus in StaticCell with RefCell for shared access
    let i2c_bus = sensors::I2C_BUS.init(RefCell::new(i2c));

    let front_dist_sensor = DistanceSensor::new(
        "front_dist",
        RefCellDevice::new(i2c_bus),
        Output::new(p.PB0, Level::Low, Speed::Low),
        0x30,
    );

    let back_dist_sensor = DistanceSensor::new(
        "back_dist",
        RefCellDevice::new(i2c_bus),
        Output::new(p.PB1, Level::Low, Speed::Low),
        0x31,
    );

    spawner
        .spawn(sensors::sensor_polling([
            front_dist_sensor,
            back_dist_sensor,
        ]))
        .unwrap();
}
