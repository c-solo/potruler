//! Inner protocol modules for the STM32 firmware.

use defmt::Format;
use protocol::Sensor;

pub mod bus;

#[derive(Format)]
pub enum SystemError {
    /// I2C bus error, or sensor not responding
    SensorError(Sensor),
}
