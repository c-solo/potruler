#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{interrupt::software::SoftwareInterruptControl, timer::timg::TimerGroup, Config};
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    let peripherals = esp_hal::init(Config::default());
    esp_println::logger::init_logger(log::LevelFilter::Info);

    let timer_group0 = TimerGroup::new(peripherals.TIMG0);
    let timer0 = timer_group0.timer0;
    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let int0 = sw_ints.software_interrupt0;
    esp_rtos::start(timer0, int0);

    info!("Hello from async main!");

    spawner.spawn(task()).unwrap();
}

#[embassy_executor::task]
async fn task() {
    info!("Hello from async task!");
}
