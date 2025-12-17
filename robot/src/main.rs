#![no_std]
#![no_main]

use defmt_rtt as _;
use firmware::hardware::led;
use panic_probe as _;
use protocol::{channels::LED_SIGNAL, command::LedCmd};

use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, dma, peripherals, usart};
use embassy_stm32::{gpio::{Level, Output, Speed}, i2c, Config};
use defmt as _;

bind_interrupts!(struct Irqs {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    // init I2C for sensors
    let i2c = i2c::I2c::new_blocking(p.I2C1, p.PB8, p.PB9, i2c::Config::default());

    // PC13 - Black Pill on-board LED
    let led_pin = Output::new(p.PC13, Level::Low, Speed::Low);
    let led = led::Led::new("status_led", led_pin);

    defmt::info!("Hello from STM32F401CCU6!");

    LED_SIGNAL.signal(LedCmd::Blink(100));

    spawner.spawn(led::led_task(led)).unwrap();
}
