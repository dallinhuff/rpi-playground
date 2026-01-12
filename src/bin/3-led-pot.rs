//! This example reads voltage from a potentiometer & lights
//! one of three LEDs based on its reading
//!
//! GREEN: GP13
//! YELLOW: GP14
//! RED: GP15
//!
//! POT-LEFT -> GND
//! POT-CENTER -> ADC2
//! POT-RIGHT -> 3V3(OUT)

#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    adc::{Adc, Channel, InterruptHandler},
    bind_interrupts,
    gpio::{Level, Output, Pull},
};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

const MAX_READING: u16 = 4095;
const GREEN_MAX: u16 = (MAX_READING as f64 * 0.65) as u16;
const YELLOW_MAX: u16 = (MAX_READING as f64 * 0.85) as u16;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let mut adc = Adc::new(peripherals.ADC, Irqs, Default::default());
    let mut pot = Channel::new_pin(peripherals.PIN_28, Pull::None);

    let mut green = Output::new(peripherals.PIN_13, Level::Low);
    let mut yellow = Output::new(peripherals.PIN_14, Level::Low);
    let mut red = Output::new(peripherals.PIN_15, Level::Low);

    while let Ok(val) = adc.read(&mut pot).await {
        if val > YELLOW_MAX {
            red.set_high();
            yellow.set_low();
            green.set_low();
        } else if val > GREEN_MAX {
            red.set_low();
            yellow.set_high();
            green.set_low();
        } else {
            red.set_low();
            yellow.set_low();
            green.set_high();
        }
    }

    red.set_low();
    yellow.set_low();
    green.set_low();
}
