//! This example uses 3 push buttons to select the color of an RGB LED.
//!
//! Red Button: GP15
//! Green Button: GP14
//! Blue Button: GP13
//!
//! Red LED: GP18
//! Green LED: GP17
//! Blue LED: GP16

#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::select::{Either3, select3};
use embassy_rp::gpio::{Input, Level, Output, Pull};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let mut red_button = Input::new(peripherals.PIN_15, Pull::Up);
    let mut green_button = Input::new(peripherals.PIN_14, Pull::Up);
    let mut blue_button = Input::new(peripherals.PIN_13, Pull::Up);

    let mut red_led = Output::new(peripherals.PIN_18, Level::High);
    let mut green_led = Output::new(peripherals.PIN_17, Level::Low);
    let mut blue_led = Output::new(peripherals.PIN_16, Level::Low);

    loop {
        match select3(
            red_button.wait_for_falling_edge(),
            green_button.wait_for_falling_edge(),
            blue_button.wait_for_falling_edge(),
        )
        .await
        {
            Either3::First(_) => {
                red_led.set_high();
                green_led.set_low();
                blue_led.set_low();
                red_button.wait_for_rising_edge().await;
            }
            Either3::Second(_) => {
                red_led.set_low();
                green_led.set_high();
                blue_led.set_low();
                green_button.wait_for_rising_edge().await;
            }
            Either3::Third(_) => {
                red_led.set_low();
                green_led.set_low();
                blue_led.set_high();
                blue_button.wait_for_rising_edge().await;
            }
        };
    }
}
