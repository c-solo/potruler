/// Command to control the LED.
pub enum LedCmd {
    On,
    Off,
    /// Blink for given ms.
    Blink(u64),
}

/// Command to control the movement of the robot.
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
