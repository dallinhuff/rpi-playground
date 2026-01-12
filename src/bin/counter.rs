//! This example counts from 0-15 in a loop using 4 LEDs connected as follows:
//!
//! 1: GP12
//! 2: GP13
//! 4: GP14
//! 8: GP15

#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use panic_probe as _;

const DELAY_MS: u64 = 500;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut leds = [
        Output::new(peripherals.PIN_15, Level::Low),
        Output::new(peripherals.PIN_14, Level::Low),
        Output::new(peripherals.PIN_13, Level::Low),
        Output::new(peripherals.PIN_12, Level::Low),
    ];

    for num in (0..=15).cycle() {
        for (i, led) in leds.iter_mut().enumerate() {
            if (num >> i) & 1 == 1 {
                led.set_high();
            } else {
                led.set_low();
            }
        }

        Timer::after(Duration::from_millis(DELAY_MS)).await;
    }
}
