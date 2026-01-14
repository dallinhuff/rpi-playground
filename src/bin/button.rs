//! This example uses a push button to toggle an LED on/off
//!
//! Button: GP15
//! LED: GP16

#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Duration, Timer};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut button = Input::new(peripherals.PIN_15, Pull::Up);
    let mut led = Output::new(peripherals.PIN_16, Level::Low);

    loop {
        button.wait_for_falling_edge().await;
        Timer::after(Duration::from_millis(20)).await; // debounce
        if button.is_low() {
            led.toggle();
            button.wait_for_rising_edge().await;
        }
    }
}
