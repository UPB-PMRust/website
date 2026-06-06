
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    i2c::{Config as I2cConfig, I2c, InterruptHandler},
    peripherals::{I2C0, I2C1},
    pwm::{Config as PwmConfig, Pwm},
};
use embassy_time::{Duration, Timer};

mod buzzer;
mod distance;
mod lcd;
mod led;
mod pins;

use distance::Zone;
use led::{set_both_leds, RgbLed};
use lcd::Lcd1602;



bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
    I2C1_IRQ => InterruptHandler<I2C1>;
});


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Parking Sensor starting…");

    let p = embassy_rp::init(Default::default());

    let i2c0 = I2c::new_async(
        p.I2C0,
        p.PIN_5,  
        p.PIN_4,  
        Irqs,
        {
            let mut cfg = I2cConfig::default();
            cfg.frequency = 100_000;
            cfg
        },
    );

    let _i2c1 = I2c::new_async(
        p.I2C1,
        p.PIN_9,  
        p.PIN_8,  
        Irqs,
        {
            let mut cfg = I2cConfig::default();
            cfg.frequency = 100_000;
            cfg
        },
    );
    let mut lcd = Lcd1602::new(i2c0, pins::LCD_I2C_ADDR).await;
    lcd.set_cursor(0, 0).await;
    lcd.print("Parking Sensor  ").await;
    lcd.set_cursor(0, 1).await;
    lcd.print("Initialising... ").await;
    Timer::after(Duration::from_millis(1000)).await;

    let led1_r = Output::new(p.PIN_10, Level::High); 
    let led1_g = Output::new(p.PIN_11, Level::High);
    let led1_b = Output::new(p.PIN_12, Level::High);
    let mut led1 = RgbLed::new(led1_r, led1_g, led1_b);

    let led2_r = Output::new(p.PIN_13, Level::High);
    let led2_g = Output::new(p.PIN_14, Level::High);
    let led2_b = Output::new(p.PIN_15, Level::High);
    let mut led2 = RgbLed::new(led2_r, led2_g, led2_b);

    let mut bz1 = Pwm::new_output_a(p.PWM_SLICE0, p.PIN_16, buzzer::make_silent_config());
    let mut bz2 = Pwm::new_output_a(p.PWM_SLICE1, p.PIN_17, buzzer::make_silent_config());

    let sensor1 = Input::new(p.PIN_20, Pull::Up);
    let sensor2 = Input::new(p.PIN_21, Pull::Up);

    info!("Hardware initialised.");

    let mut beep_on = false;
    let mut prev_zone = Zone::Clear;

    loop {
        let s1 = sensor1.is_low();
        let s2 = sensor2.is_low();
        let zone = Zone::from_sensors(s1, s2);

        if zone != prev_zone {
            info!("Zone changed: {}", zone);

            set_both_leds(&mut led1, &mut led2, zone);

            lcd.set_cursor(0, 0).await;
            lcd.print("Distance Status ").await;
            lcd.set_cursor(0, 1).await;
            lcd.print(zone.label()).await;

            prev_zone = zone;
            beep_on = false; 
        }

        beep_on = !beep_on;

        buzzer::set_buzzer_zone(&mut bz1, zone, beep_on);
        buzzer::set_buzzer_zone(&mut bz2, zone, beep_on);

        buzzer::half_period(zone).await;
    }
}
