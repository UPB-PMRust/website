#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::i2c::{self, I2c};
use embassy_stm32::mode::Blocking;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_time::{Duration, Timer};
use embedded_hal::Pwm as _;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

static ALBUMS: &[(&str, &str)] = &[
    ("AC/DC",          "Back in Black"),
    ("Beatles",        "Abbey Road"),
    ("Dire Straits",   "Brothers in Arms"),
    ("Guns N Roses",   "Use Illusion I"),
    ("Metallica",      "Master of Puppets"),
    ("Nirvana",        "In Utero"),
];

fn angle_to_duty(angle: u32, max: u32) -> u32 {
    let pulse = 1000 + angle * 1000 / 180;
    max * pulse / 20000
}

fn cd_to_angle(idx: usize) -> u32 {
    let deg = (idx as u32) * 26;
    if deg > 130 { 130 } else { deg }
}

const LCD: u8 = 0x27;
const RS:  u8 = 0x01;
const EN:  u8 = 0x04;
const BL:  u8 = 0x08;

async fn nib(i2c: &mut I2c<'_, Blocking, i2c::Master>, b: u8) {
    let _ = i2c.blocking_write(LCD, &[b|BL]);
    Timer::after(Duration::from_micros(1)).await;
    let _ = i2c.blocking_write(LCD, &[b|EN|BL]);
    Timer::after(Duration::from_micros(1)).await;
    let _ = i2c.blocking_write(LCD, &[(b&!EN)|BL]);
    Timer::after(Duration::from_micros(50)).await;
}

async fn send(i2c: &mut I2c<'_, Blocking, i2c::Master>, d: u8, m: u8) {
    nib(i2c, (d&0xF0)|m).await;
    nib(i2c, ((d<<4)&0xF0)|m).await;
}

async fn cmd(i2c: &mut I2c<'_, Blocking, i2c::Master>, c: u8) {
    send(i2c, c, 0).await;
    Timer::after(Duration::from_micros(100)).await;
}

async fn lcd_init(i2c: &mut I2c<'_, Blocking, i2c::Master>) {
    Timer::after(Duration::from_millis(50)).await;
    nib(i2c, 0x30).await; Timer::after(Duration::from_millis(5)).await;
    nib(i2c, 0x30).await; Timer::after(Duration::from_micros(150)).await;
    nib(i2c, 0x30).await; Timer::after(Duration::from_micros(150)).await;
    nib(i2c, 0x20).await; Timer::after(Duration::from_micros(150)).await;
    cmd(i2c, 0x28).await;
    cmd(i2c, 0x08).await;
    cmd(i2c, 0x01).await;
    Timer::after(Duration::from_millis(2)).await;
    cmd(i2c, 0x06).await;
    cmd(i2c, 0x0C).await;
}

async fn show(i2c: &mut I2c<'_, Blocking, i2c::Master>, l1: &str, l2: &str) {
    cmd(i2c, 0x80).await;
    let mut n = 0u8;
    for b in l1.bytes() { if n>=16{break;} send(i2c,b,RS).await; n+=1; }
    while n<16 { send(i2c,b' ',RS).await; n+=1; }
    cmd(i2c, 0xC0).await;
    n = 0;
    for b in l2.bytes() { if n>=16{break;} send(i2c,b,RS).await; n+=1; }
    while n<16 { send(i2c,b' ',RS).await; n+=1; }
}

async fn draw(i2c: &mut I2c<'_, Blocking, i2c::Master>, idx: usize, sel: bool) {
    let (artist, album) = ALBUMS[idx];
    if sel {
        show(i2c, artist, album).await;
    } else {
        let mut l1: String<17> = String::new();
        let n = idx + 1;
        if n < 10 { let _ = l1.push((b'0' + n as u8) as char); }
        else { let _ = l1.push((b'0' + (n/10) as u8) as char); let _ = l1.push((b'0' + (n%10) as u8) as char); }
        let _ = l1.push('.'); let _ = l1.push_str(album);
        show(i2c, &l1, artist).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("start");
    let p = embassy_stm32::init(Default::default());

    let mut i2c = I2c::new_blocking(
        p.I2C1, p.PB6, p.PB7,
        i2c::Config::default(),
    );

    let ch: PwmPin<_, embassy_stm32::timer::Ch1> = PwmPin::new(p.PA8, embassy_stm32::gpio::OutputType::PushPull);
    let mut pwm = SimplePwm::new(
        p.TIM1,
        Some(ch), None, None, None,
        Hertz(50),
        Default::default(),
    );
    let max = pwm.get_max_duty();
    pwm.set_duty(Channel::Ch1, angle_to_duty(0, max));
    pwm.enable(Channel::Ch1);
    info!("PWM max duty: {}", max);

    lcd_init(&mut i2c).await;
    show(&mut i2c, "  CD Menu v1.0  ", "Jos=naviga Btn=ok").await;
    Timer::after(Duration::from_millis(1200)).await;

    let joy_down = Input::new(p.PA1, Pull::Up);
    let joy_btn  = Input::new(p.PA2, Pull::Up);

    let mut idx: usize = 0;
    let mut last_down = false;
    let mut last_btn  = false;
    let mut ignore_until: u64 = 0;
    let mut ticks: u64 = 0;

    draw(&mut i2c, idx, false).await;

    loop {
        Timer::after(Duration::from_millis(50)).await;
        ticks += 50;

        let down = joy_down.is_low();
        let bn   = joy_btn.is_low();

        let down_edge = down && !last_down;
        let btn_edge  = bn   && !last_btn;

        last_down = down;
        last_btn  = bn;

        if btn_edge {
            ignore_until = ticks + 500;
            let angle = cd_to_angle(idx);
            let duty = angle_to_duty(angle, max);
            info!("Servo: {}gr duty={}", angle, duty);
            pwm.set_duty(Channel::Ch1, duty);
            draw(&mut i2c, idx, true).await;
            Timer::after(Duration::from_millis(1500)).await;
            ticks += 1500;
            draw(&mut i2c, idx, false).await;
        } else if down_edge && ticks > ignore_until {
            if idx < ALBUMS.len() - 1 { idx += 1; } else { idx = 0; }
            draw(&mut i2c, idx, false).await;
        }
    }
}