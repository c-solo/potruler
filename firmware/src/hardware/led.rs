use embassy_time::with_timeout;
use esp_hal::{
    gpio::DriveMode,
    ledc,
    ledc::{
        LowSpeed,
        channel::{Channel, ChannelIFace},
        timer,
        timer::{Timer, TimerIFace},
    },
    time::Rate,
};
use log::info;
use protocol::{channels::LED_SIGNAL, command::LedCmd};

#[allow(dead_code)]
pub struct Led {
    name: &'static str,
    channel: Channel<'static, LowSpeed>,
}

impl Led {
    /// Creates new LED drive.
    /// Use StaticCell for timer to create a static timer reference it's
    /// important for embassy async tasks.
    pub fn new(
        name: &'static str,
        timer: &'static mut Timer<'static, LowSpeed>,
        mut channel: Channel<'static, LowSpeed>,
    ) -> Result<Self, ledc::channel::Error> {
        timer
            .configure(timer::config::Config {
                duty: timer::config::Duty::Duty14Bit,
                clock_source: timer::LSClockSource::APBClk,
                frequency: Rate::from_hz(24),
            })
            .unwrap();

        channel.configure(ledc::channel::config::Config {
            timer,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })?;

        info!("Led '{}' initialized", name);

        Ok(Self { name, channel })
    }

    pub fn on(&mut self) {
        if let Err(err) = self.channel.set_duty(100) {
            send_error(err)
        };
    }

    pub fn off(&mut self) {
        if let Err(err) = self.channel.set_duty(0) {
            send_error(err)
        };
    }
}

fn send_error(_err: ledc::channel::Error) {
    // todo impl
    // some Defect from modules should be sent through some channel somewhere, for example fo rlogging
    // or showing on a display
}

/// Main operation task for the LED.
/// Listens for commands on the [`LED_SIGNAL`] channel.
#[embassy_executor::task]
pub async fn led_task(mut led: Led) {
    let mut current_state = LedCmd::Off;

    loop {
        match current_state {
            LedCmd::On => {
                led.on();
                // blocks here in on state until next signal
                current_state = LED_SIGNAL.wait().await;
            }
            LedCmd::Off => {
                led.off();
                // blocks here in off state until next signal
                current_state = LED_SIGNAL.wait().await;
            }
            LedCmd::Blink(delay_ms) => {
                let duration = embassy_time::Duration::from_millis(delay_ms);

                led.on();
                if let Ok(new_cmd) = with_timeout(duration, LED_SIGNAL.wait()).await {
                    current_state = new_cmd;
                    continue;
                };

                led.off();
                if let Ok(new_cmd) = with_timeout(duration, LED_SIGNAL.wait()).await {
                    current_state = new_cmd;
                    continue;
                };
            }
        }
    }
}
