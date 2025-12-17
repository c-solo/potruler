#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    interrupt::software::SoftwareInterruptControl,
    ledc::{channel::Number, timer, Ledc},
    timer::timg::TimerGroup,
    Config,
};
use firmware::hardware::led;
use log::info;
use protocol::{channels::LED_SIGNAL, command::LedCmd};
use static_cell::StaticCell;

esp_bootloader_esp_idf::esp_app_desc!();

static LEDC_TIMER: StaticCell<timer::Timer<esp_hal::ledc::LowSpeed>> = StaticCell::new();

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    esp_println::logger::init_logger(log::LevelFilter::Info);

    let peripherals = esp_hal::init(Config::default());

    // INIT LEDC
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(esp_hal::ledc::LSGlobalClkSource::APBClk);
    let timer = LEDC_TIMER.init(ledc.timer(timer::Number::Timer0));
    let led_channel = ledc.channel(Number::Channel0, peripherals.GPIO18);

    let led = led::Led::new("status_led", timer, led_channel).unwrap();

    let timer_group0 = TimerGroup::new(peripherals.TIMG0);
    let timer0 = timer_group0.timer0;

    // INIT EMBASSY
    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let int0 = sw_ints.software_interrupt0;
    esp_rtos::start(timer0, int0);

    info!("Hello from async main!");

    LED_SIGNAL.signal(LedCmd::Blink(1000));

    spawner.spawn(led::led_task(led)).unwrap();
}
