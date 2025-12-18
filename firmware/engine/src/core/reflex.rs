//! Only emergency stop related control logic (aka reflexes).
//! Planning and higher level control should be handled by rpi via UPD.

use crate::{
    hardware::led::LedCmd,
    protocol::{bus::LED_SIGNAL, SystemError},
};
use defmt::error;

/// Trigger an emergency stop due to the given cause.
pub fn emergency_stop(cause: SystemError) {
    error!("Emergency stop triggered ({})!", cause);
    LED_SIGNAL.signal(LedCmd::Blink(10));
    // TODO: send emergency stop command to motor controller and report to rpi
}
