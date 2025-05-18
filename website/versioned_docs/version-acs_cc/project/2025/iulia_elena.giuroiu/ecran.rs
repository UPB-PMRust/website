// Afișează "Hello, world!" pe un LCD I2C conectat la Raspberry Pi Pico W.

#![no_std]
#![no_main]

use core::panic;
use defmt_rtt as _;
use defmt::{info, error};
use embassy_executor::Spawner;
use embassy_rp::i2c::{self, Config as I2cConfig};
use embassy_time::{Delay, Timer, Duration};
use hd44780_driver::HD44780;
use panic_probe as _;

mod irqs;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    info!("Inițializare I2C LCD...");

    // I2C1: SDA = GP10, SCL = GP11
    let i2c = i2c::I2c::new_blocking(
        p.I2C1,
        p.PIN_11, // SCL
        p.PIN_10, // SDA
        I2cConfig::default(),
    );

    let mut delay = Delay;

    // Inițializează LCD-ul la adresa 0x27
    info!("Inițializare LCD la adresa 0x27...");
    let mut lcd = match HD44780::new_i2c(i2c, 0x27, &mut delay) {
        Ok(lcd) => lcd,
        Err(_e) => {
            error!("Eroare la inițializarea LCD");
            panic!("LCD init failed");
        }
    };
    info!("LCD inițializat cu succes");

    // Resetează și curăță ecranul
    info!("Resetez LCD...");
    if let Err(_e) = lcd.reset(&mut delay) {
        error!("Eroare la reset");
    }
    info!("Curăț ecranul...");
    if let Err(_e) = lcd.clear(&mut delay) {
        error!("Eroare la clear");
    }

    // Afișează mesajul
    info!("Scriu mesajul...");
    if let Err(_e) = lcd.write_str("Hello, world!", &mut delay) {
        error!("Eroare la scriere");
    }
    info!("Mesaj afișat pe LCD");

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}