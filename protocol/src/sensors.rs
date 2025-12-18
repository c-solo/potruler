use crate::Sensor;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

/// Request sensors channel.
pub static SENSOR_CMD_CH: Channel<CriticalSectionRawMutex, SensorCmd, 10> = Channel::new();

/// Response sensors channel with telemetry data.
pub static TELEMETRY_CH: Channel<CriticalSectionRawMutex, Telemetry, 10> = Channel::new();

/// Subscribe commands for various sensors.
/// Where u64 is the interval in milliseconds.
#[derive(defmt::Format)]
pub enum SensorCmd {
    SubscribeTo {
        sensor: Sensor,
        poll_interval_ms: u64,
    },
}

/// Telemetry data from various sensors.
pub enum Telemetry {
    DistanceFront { mm: u16 },
    DistanceBack { mm: u16 },
}
