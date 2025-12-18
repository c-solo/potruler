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
    time::Hertz,
    Config,
};
use embassy_time::{Duration, Timer};
use embedded_hal_bus::i2c::RefCellDevice;
use engine::hardware::chassis::SkidSteer;

use defmt as _;
use defmt_rtt as _;
use engine::hardware::chassis;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    // Small delay to allow RTT connection to establish
    Timer::after(Duration::from_millis(10)).await;

    // Initialize LED
    let led_pin = Output::new(p.PC13, Level::Low, Speed::Low);
    let led = led::Led::new("status_led", led_pin);

    spawner.spawn(led::led_handler(led)).unwrap();
    spawner.spawn(error::error_handler()).unwrap();

    // Initialize motors
    let skid_steer = SkidSteer::new(p.TIM3, p.PA6, p.PA7, p.TIM4, p.PB6, p.PB7, Hertz::khz(20));
    spawner
        .spawn(chassis::movement_handler(skid_steer))
        .expect("failed to spawn movement handler");

    // Initialize I2C sensors
    let mut i2c_cfg = i2c::Config::default();
    i2c_cfg.frequency = Hertz::khz(400);
    let i2c = i2c::I2c::new_blocking(p.I2C1, p.PB8, p.PB9, i2c_cfg);
    // Store I2C bus in StaticCell with RefCell for shared access
    let shared_i2c = sensors::SHARED_I2C.init(RefCell::new(i2c));

    let front_dist_sensor = DistanceSensor::new(
        "front_dist",
        RefCellDevice::new(shared_i2c),
        Output::new(p.PB0, Level::Low, Speed::Low),
        0x30,
    );

    let back_dist_sensor = DistanceSensor::new(
        "back_dist",
        RefCellDevice::new(shared_i2c),
        Output::new(p.PB1, Level::Low, Speed::Low),
        0x31,
    );

    spawner
        .spawn(sensors::sensor_polling(front_dist_sensor, back_dist_sensor))
        .expect("failed to spawn sensor polling task");
}
