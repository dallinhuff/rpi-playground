//! This example reads voltage from a potentiometer & applies a proportional voltage using pwm
//! out to an LED.
//!
//! LED: GP15
//!
//! POT-LEFT -> GND
//! POT-CENTER -> ADC2
//! POT-RIGHT -> 3V3(OUT)

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Pull;
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // --- ADC setup ---
    let mut adc = Adc::new(peripherals.ADC, Irqs, Default::default());
    let mut pot = Channel::new_pin(peripherals.PIN_28, Pull::None);

    // --- PWM setup ---
    let desired_freq_hz = 25_000;
    let clock_freq_hz = embassy_rp::clocks::clk_sys_freq();
    let divider = 16u8;
    let period = (clock_freq_hz / (desired_freq_hz * divider as u32)) as u16 - 1;

    let mut pwm_config = Config::default();
    pwm_config.top = period;
    pwm_config.divider = divider.into();

    let mut pwm = Pwm::new_output_b(peripherals.PWM_SLICE7, peripherals.PIN_15, pwm_config);

    loop {
        let adc_val = adc.read(&mut pot).await.unwrap();
        let duty_period = u32::from(adc_val) * (u32::from(period) + 1) / 4096;

        pwm.set_duty_cycle(duty_period as u16).unwrap();

        Timer::after_millis(10).await;
    }
}
