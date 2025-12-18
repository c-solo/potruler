use crate::hardware::sensors::distance::DistanceSensor;
use core::cell::RefCell;
use embassy_stm32::{
    i2c::{I2c, Master},
    mode::Blocking,
};
use embassy_time::{Duration, Timer};
use embedded_hal_bus::i2c::RefCellDevice;
use static_cell::StaticCell;

pub mod cliff;
pub mod distance;

pub type SharedI2c = RefCellDevice<'static, I2c<'static, Blocking, Master>>;

/// Static storage for the I2C bus using StaticCell + RefCell pattern
pub static I2C_BUS: StaticCell<RefCell<I2c<'static, Blocking, Master>>> = StaticCell::new();

/// Polls all sensors in one task.
#[embassy_executor::task]
pub async fn sensor_polling(_distance_sensors: [DistanceSensor; 2]) {
    loop {
        // TODO: implement distance measurement
        // let distance = sensor.sensor.read_range_mm();
        Timer::after(Duration::from_millis(100)).await;
    }
}
