//! Communication between Core (STM32 MCU) and Main system (Rpi5).

#![no_std]

// TODO: Design

#[derive(defmt::Format)]
pub enum Sensor {
    /// Mesures distance, detects obstacle.
    Distance,
    /// Detects no ground under the robot (cliffs, stairs).
    Cliff,
    /// Inertial Measurement Unit, measures acceleration and rotation.
    Imu,
}

#[derive(defmt::Format)]
pub enum EngineEvent {
    Ready,
    EmergencyStop,
    LowBattery,
    Unavailable(Sensor),
}

/// Command to control the movement of the robot.
#[derive(defmt::Format)]
pub struct MoveCommand {
    /// -1.0 to 1.0 (Left/Right)
    pub x: f32,
    /// -1.0 to 1.0 (Forward/Backward)
    pub y: f32,
    /// -1.0 to 1.0 (Rotation)
    pub rot: f32,
    /// Speed Multiplier.
    pub speed: f32,
}
