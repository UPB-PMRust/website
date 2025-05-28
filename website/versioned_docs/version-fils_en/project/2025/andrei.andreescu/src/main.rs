#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Input, Pull};
use embassy_rp::gpio::AnyPin;
use embassy_rp::i2c;
use embassy_rp::peripherals::{I2C0, PIN_0, PIN_1, PIN_2, PIN_3, PIN_4, PIN_10, PIN_11, PIN_12, PIN_13};
use embassy_rp::init;
use embassy_time::{Duration, Timer, Instant};
use embassy_executor::raw::Executor;
use embassy_macros::task;
use static_cell::StaticCell;
use {unwrap};
use embassy_net::{Stack, Config};
use embassy_rp::wifi::{WifiDevice, WifiConfig, WifiController};

static mut MOVEMENT_SENSOR: Option<Input<'static, _>> = None;
static mut LASER_SENSOR: Option<Input<'static, _>> = None;
static mut LIGHT_SENSOR: Option<Input<'static, _>> = None;

#[cortex_m_rt::entry]
fn main() -> ! {
    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(main_task(spawner)));
    });
    loop {}
}

#[task]
async fn main_task(_spawner: Spawner) {
    let p = init(Default::default());

    use embassy_net::Config;
    use embassy_net::Stack;
    use embassy_net::tcp::TcpSocket;
    use embassy_rp::wifi::{WifiDevice, WifiController, WifiConfig};

    // Initialize Wi-Fi
    let wifi = WifiDevice::new(p.WLAN, p.PIN_0, p.PIN_1, p.PIN_2);
    let controller = WifiController::new(wifi);
    let config = WifiConfig {
        ssid: "".into(),
        password: "".into(),
    };

    info!("Connecting to Wi-Fi...");
    controller.connect(config).await.unwrap();
    info!("Connected to Wi-Fi");

    unsafe {
        MOVEMENT_SENSOR = Some(Input::new(p.PIN_2, Pull::None));
        LASER_SENSOR = Some(Input::new(p.PIN_3, Pull::None));
        LIGHT_SENSOR = Some(Input::new(p.PIN_4, Pull::None));
    }

    let mut led_r = Output::new(p.PIN_11, Level::Low);
    let mut led_g = Output::new(p.PIN_12, Level::High);
    let mut led_b = Output::new(p.PIN_13, Level::High);
    let mut buzzer = Output::new(p.PIN_10, Level::Low);

    let _i2c = i2c::I2c::new_blocking(
        p.I2C0,
        p.PIN_1,
        p.PIN_0,
        i2c::Config::default(),
    );

    let mut time_in_zone: u64 = 0;

    loop {
        let movement_detected = check_movement_sensor();
        let laser_interrupted = check_laser_sensor();
        let light_changed = check_light_sensor();

        if movement_detected && laser_interrupted && light_changed {
            time_in_zone = 3; // Simulated time in seconds

            info!("ALERT: All sensors triggered. Activating alarm...");
            let alarm_start = Instant::now();

            led_r.set_high();
            led_g.set_low();
            led_b.set_low();
            buzzer.set_high();

            Timer::after(Duration::from_secs(3)).await;

            led_r.set_low();
            led_g.set_high();
            led_b.set_high();
            buzzer.set_low();

            let elapsed = alarm_start.elapsed();
            info!("Alarm deactivated. Intruder spent {} seconds in zone.", elapsed.as_secs());

            // Simulated email sending via HTTP request (mock)
            info!("(Simulated) Sending email: Intruder detected. Time spent: {}s", elapsed.as_secs());
            // Implement real email sending logic over WiFi using embassy-net or a webhook
            send_webhook_alarm_email(time_in_zone).await;

            info!("Sending webhook email...");
            //  Replace with real webhook sending using embedded-nal-async or similar
        }

        Timer::after(Duration::from_secs(2)).await;
    }
}

async fn send_webhook_alarm_email(time_spent: u64) {
    info!("Sending webhook email with time spent: {} seconds", time_spent);

}

fn check_movement_sensor() -> bool {
    unsafe {
        if let Some(sensor) = &MOVEMENT_SENSOR {
            sensor.is_high()
        } else {
            false
        }
    }
}

fn check_laser_sensor() -> bool {
    unsafe {
        if let Some(sensor) = &LASER_SENSOR {
            sensor.is_low() // assuming LOW means interrupted
        } else {
            false
        }
    }
}

fn check_light_sensor() -> bool {
    unsafe {
        if let Some(sensor) = &LIGHT_SENSOR {
            sensor.is_high() // simulate light change
        } else {
            false
        }
    }
}

use core::panic::PanicInfo;

#[crate::panic_handler]
fn panic(info: &PanicInfo) -> ! {
    defmt::error!("Panic: {}", info);
    loop {}
}