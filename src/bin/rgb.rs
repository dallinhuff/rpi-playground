//! This example cycles through the rainbow on an RGB LED.
//!
//! RED: GP15
//! GREEN: GP14
//! BLUE: GP13

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

struct Hsv(u16, u8, u8);
struct Rgb(u8, u8, u8);

impl From<Hsv> for Rgb {
    fn from(Hsv(h, s, v): Hsv) -> Self {
        let region = (h / 60) % 6;
        let remainder = h % 60;
        let v = v as u16;

        let p = (v * (255 - s as u16)) / 255;
        let q = (v * (255 - (s as u16 * remainder) / 60)) / 255;
        let t = (v * (255 - (s as u16 * (60 - remainder)) / 60)) / 255;

        let scale = |x| (x * 100 / 255) as u8;

        match region {
            0 => Rgb(scale(v), scale(t), scale(p)),
            1 => Rgb(scale(q), scale(v), scale(p)),
            2 => Rgb(scale(p), scale(v), scale(t)),
            3 => Rgb(scale(p), scale(q), scale(v)),
            4 => Rgb(scale(t), scale(p), scale(v)),
            _ => Rgb(scale(v), scale(p), scale(q)),
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let desired_freq_hz = 1000;
    let clock_freq_hz = embassy_rp::clocks::clk_sys_freq();
    let divider = 16u8;
    let period = (clock_freq_hz / (desired_freq_hz * divider as u32)) as u16 - 1;

    let mut pwm_config = Config::default();
    pwm_config.top = period;
    pwm_config.divider = divider.into();

    let (Some(mut green), Some(mut red)) = Pwm::new_output_ab(
        peripherals.PWM_SLICE7,
        peripherals.PIN_14,
        peripherals.PIN_15,
        pwm_config.clone(),
    )
    .split() else {
        return;
    };

    let mut blue = Pwm::new_output_b(
        peripherals.PWM_SLICE6,
        peripherals.PIN_13,
        pwm_config.clone(),
    );

    let mut hue_fp: u32 = 0; // 16.16 fixed point
    loop {
        let hue = ((hue_fp >> 16) as u16) % 360;
        let Rgb(r, g, b) = Hsv(hue, 255, 255).into();

        red.set_duty_cycle_percent(r).unwrap();
        green.set_duty_cycle_percent(g).unwrap();
        blue.set_duty_cycle_percent(b).unwrap();

        hue_fp += (1 << 16) / 4;
        Timer::after_millis(10).await;
    }
}
