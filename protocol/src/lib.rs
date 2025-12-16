#![no_std]

use crate::command::MoveCommand;

pub mod channels;
pub mod command;

pub enum CoreEvent {
    Motion(MoveCommand),
    EmergencyStop,
    LowBattery,
}
