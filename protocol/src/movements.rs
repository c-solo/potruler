use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

/// Movement command signal channel. Latest command is always winning.
pub static MOVE_CMD_SIGNAL: Signal<CriticalSectionRawMutex, MoveCmd> = Signal::new();

/// Command to control the movement of the robot.
#[derive(defmt::Format)]
pub struct MoveCmd {
    /// Left side speed: -1.0 to 1.0 (Forward/Backward)
    pub left: f32,
    /// Right side speed: -1.0 to 1.0 (Forward/Backward)
    pub right: f32,
}

impl MoveCmd {
    pub fn stop() -> Self {
        MoveCmd {
            left: 0.0,
            right: 0.0,
        }
    }
}
