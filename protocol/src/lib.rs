#![no_std]

use crate::command::MoveCommand;

pub mod command;
pub mod channels;

pub enum CoreEvent {
    Motion(MoveCommand),
    EmergencyStop,
    LowBattery,
}
