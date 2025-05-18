#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Output, Level, Pull};
use embassy_time::{Timer, Duration};
use panic_probe as _;

mod irqs;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Buton pe GP17, cu pull-up activat
    let button = Input::new(p.PIN_17, Pull::Up);

    // LED pe GP18
    let mut led = Output::new(p.PIN_18, Level::Low);

    loop {
        if button.is_low() {
            // Buton apăsat
            led.set_high();
            info!("Buton APĂSAT");
        } else {
            // Buton eliberat
            led.set_low();
        }

        Timer::after(Duration::from_millis(100)).await;
    }
}
