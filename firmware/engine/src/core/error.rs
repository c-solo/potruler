//! Error handling module for the firmware.

use crate::{
    core::reflex::emergency_stop,
    protocol::{bus, SystemError},
};
use defmt::error;
use protocol::Sensor;

/// Handles all inner system errors.
/// Some errors trigger an emergency stop, while others are logged and sent via network to main system.
#[embassy_executor::task]
pub async fn error_handler() {
    loop {
        match bus::ERROR_CH.receive().await {
            err @ SystemError::SensorError(Sensor::Cliff) => {
                emergency_stop(err);
            }
            err @ SystemError::SensorError(Sensor::Distance)
            | err @ SystemError::SensorError(Sensor::Imu) => {
                // no need to stop the system for distance or Imu errors
                error!("{}", err);
                // TODO: send notification to main system status
            }
        }
    }
}
