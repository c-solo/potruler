#![no_std]
#![no_main]

use firmware::hardware::{led, sensors};
use protocol::{channels::LED_SIGNAL, command::LedCmd};

use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    i2c::{self},
    Config,
};
use embassy_time::{Duration, Timer};
use embedded_hal_bus::i2c::RefCellDevice;
use firmware::hardware::sensors::distance::DistanceSensor;

use defmt as _;
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    // Small delay to allow RTT connection to establish
    Timer::after(Duration::from_millis(100)).await;

    defmt::info!("Initializing STM32F401CCU6...");

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
        0x30,
    );

    // Initialize LED
    let led_pin = Output::new(p.PC13, Level::Low, Speed::Low);
    let led = led::Led::new("status_led", led_pin);

    defmt::info!("All sensors initialized successfully!");

    LED_SIGNAL.signal(LedCmd::Blink(100));

    // Spawn tasks
    spawner.spawn(led::led_task(led)).unwrap();
    spawner
        .spawn(sensors::sensor_polling([
            front_dist_sensor,
            back_dist_sensor,
        ]))
        .unwrap();

    defmt::info!("All tasks spawned, system ready!");
}
