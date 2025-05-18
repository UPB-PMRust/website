#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Output, Level, Pull};
use embassy_time::{Duration, Timer};
use panic_probe as _;

mod irqs;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // PIR → GP16 (intrare, Pull-Down)
    let pir = Input::new(p.PIN_16, Pull::Down);

    // LED roșu → GP18 (ieșire)
    let mut led_red = Output::new(p.PIN_18, Level::Low);

    // LED verde → GP19 (ieșire)
    let mut led_green = Output::new(p.PIN_19, Level::Low);

    loop {
        if pir.is_high() {
            // Mișcare detectată → aprinde LED roșu
            led_red.set_high();
            led_green.set_low();
            info!("Mișcare detectată!");
        } else {
            // Nicio mișcare → aprinde LED verde
            led_red.set_low();
            led_green.set_high();
        }

        Timer::after(Duration::from_millis(100)).await;
    }
}
