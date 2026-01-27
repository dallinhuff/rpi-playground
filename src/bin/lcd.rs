// This example types a message on an LCD screen.

#![no_std]
#![no_main]

use ag_lcd::{Cursor, LcdDisplay};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::i2c::{Config, I2c};
use embassy_time::{Delay, Timer};
use panic_probe as _;
use port_expander::Pcf8574;

const LCD_FREQ: u32 = 200_000;
const RELIABLE_INIT: u32 = 10_000;
const MESSAGE: &str = "Hello, World!";

const _: () = assert!(MESSAGE.len() <= 16);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");

    let peripherals = embassy_rp::init(Default::default());

    let sda = peripherals.PIN_6;
    let scl = peripherals.PIN_7;

    let delay = Delay;

    let mut config = Config::default();
    config.frequency = LCD_FREQ;

    let i2c = I2c::new_blocking(peripherals.I2C1, scl, sda, config);
    let mut i2c_expander = Pcf8574::new(i2c, true, true, true);

    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new_pcf8574(&mut i2c_expander, delay)
        .with_cursor(Cursor::Off)
        .with_reliable_init(RELIABLE_INIT)
        .build();

    defmt::info!("Initialized.");

    loop {
        defmt::info!("Clearing...");
        lcd.clear();
        Timer::after_secs(1).await;

        defmt::info!("Typing...");
        for ch in MESSAGE.bytes() {
            lcd.write(ch);
            Timer::after_millis(80).await;
        }

        Timer::after_secs(3).await;

        defmt::info!("Erasing...");
        for i in (0..MESSAGE.len()).rev() {
            lcd.set_position(i as u8, 0);
            lcd.write(b' ');
            Timer::after_millis(80).await;
        }
    }
}
