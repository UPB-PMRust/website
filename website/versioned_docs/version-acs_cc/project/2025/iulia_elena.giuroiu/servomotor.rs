#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::pwm::{Config as PwmConfig, Pwm};
use embassy_time::{Timer, Duration};
use fixed::types::U16F16;
use fixed::traits::ToFixed;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // PWM config: 50 Hz
    let mut servo_config = PwmConfig::default();
    servo_config.top = 0x8B1F;
    servo_config.divider = 64.into();

    const PERIOD_US: usize = 20_000;
    const MIN_PULSE_US: usize = 500;
    const MAX_PULSE_US: usize = 2500;

    let min_pulse = (MIN_PULSE_US * servo_config.top as usize) / PERIOD_US;
    let max_pulse = (MAX_PULSE_US * servo_config.top as usize) / PERIOD_US;

    // GPIO15 → PWM Slice 7, Channel B
    let mut pwm_servo = Pwm::new_output_b(
        p.PWM_SLICE7,
        p.PIN_15,
        servo_config.clone(),
    );

    let mut direction = true;

    loop {
        if direction {
            servo_config.compare_b = max_pulse as u16;
        } else {
            servo_config.compare_b = min_pulse as u16;
        }

        pwm_servo.set_config(&servo_config);
        info!("Servomotor la {}°", if direction { 180 } else { 0 });

        direction = !direction;

        Timer::after(Duration::from_secs(2)).await;
    }
}
