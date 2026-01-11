//! This example blinks an external LED connected to GP15 every 1s

#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use panic_probe as _;

const INTERVAL_MS: u64 = 1000;
const BLINK_MS: u64 = 50;
const _: () = assert!(BLINK_MS < INTERVAL_MS);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");

    let peripherals = embassy_rp::init(Default::default());
    let mut led = Output::new(peripherals.PIN_15, Level::Low);

    defmt::info!("Initialized.");

    loop {
        defmt::info!("LED on!");
        led.set_high();
        Timer::after(Duration::from_millis(BLINK_MS)).await;

        defmt::info!("LED off!");
        led.set_low();
        Timer::after(Duration::from_millis(INTERVAL_MS - BLINK_MS)).await;
    }
}
