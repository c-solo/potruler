use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

use crate::command::LedCmd;

pub static LED_SIGNAL: Signal<CriticalSectionRawMutex, LedCmd> = Signal::new();
