#![no_std]
#![no_main]

use core::fmt::Write as _;
use core::sync::atomic::{AtomicU8, Ordering};
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, adc4};
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::i2c::I2c;
use embassy_stm32::usart::{Config, Uart};
use embassy_time::Timer;
use panic_probe as _;

use ssd1306::{prelude::*, size::DisplaySize128x64, rotation::DisplayRotation, I2CDisplayInterface, Ssd1306};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

const NUME: [&str; 13] = ["C","C#","D","D#","E","F","F#","G","G#","A","A#","B","C"];

const LEVELS: u8 = 8;                 // trepte de intensitate (rezolutie PWM)
static DUTY_R: AtomicU8 = AtomicU8::new(0);
static DUTY_G: AtomicU8 = AtomicU8::new(0);
static DUTY_B: AtomicU8 = AtomicU8::new(0);

// Task de PWM software: controleaza intensitatea celor 3 culori.
#[embassy_executor::task]
async fn led_task(mut r: Output<'static>, mut g: Output<'static>, mut b: Output<'static>) {
    loop {
        let dr = DUTY_R.load(Ordering::Relaxed);
        let dg = DUTY_G.load(Ordering::Relaxed);
        let db = DUTY_B.load(Ordering::Relaxed);
        for step in 0..LEVELS {
            if step < dr { r.set_high(); } else { r.set_low(); }
            if step < dg { g.set_high(); } else { g.set_low(); }
            if step < db { b.set_high(); } else { b.set_low(); }
            Timer::after_micros(700).await;
        }
    }
}

fn detect_acord(pressed: &[u8]) -> Option<(&'static str, u8, u8, u8)> {
    let mut pc = [false; 12];
    for &k in pressed { pc[(k % 12) as usize] = true; }
    let mut cls = heapless::Vec::<u8, 12>::new();
    for i in 0..12 { if pc[i] { let _ = cls.push(i as u8); } }
    if cls.len() != 3 { return None; }
    let root = cls[0];
    let (i1, i2) = (cls[1] - root, cls[2] - root);
    let nume = match (i1, i2) {
        (4, 7) => "Major",
        (3, 7) => "Minor",
        (3, 6) => "Diminished",
        (4, 8) => "Augmented",
        _ => return None,
    };
    Some((nume, root, i1, i2))
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("===firmware v3 ===");

    let i2c = I2c::new_blocking(p.I2C1, p.PB6, p.PB7, Default::default());
    let mut display = Ssd1306::new(I2CDisplayInterface::new(i2c), DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    let style = MonoTextStyleBuilder::new().font(&FONT_6X10).text_color(BinaryColor::On).build();

    // RGB -> preluate de led_task (R=PB5, G=PB3, B=PB4)
    let led_r = Output::new(p.PB5, Level::Low, Speed::Low);
    let led_g = Output::new(p.PB3, Level::Low, Speed::Low);
    let led_b = Output::new(p.PB4, Level::Low, Speed::Low);
    spawner.spawn(led_task(led_r, led_g, led_b).unwrap());

    // ADC4: volum=PA4, octava=PA7, microfon=PA6
    let mut adc = Adc::new_adc4(p.ADC4);
    adc.set_resolution_adc4(adc4::Resolution::Bits12);
    let mut pin_vol = p.PA4;
    let mut pin_oct = p.PA7;
    let mut pin_mic = p.PA6;

    let mut ucfg = Config::default();
    ucfg.baudrate = 115200;
    let mut usart = Uart::new_blocking(p.USART1, p.PA10, p.PA9, ucfg).unwrap();

        let buttons = [
        Input::new(p.PC0,  Pull::Up),  // 0  C
        Input::new(p.PB1,  Pull::Up),  // 1  C#
        Input::new(p.PC6,  Pull::Up),  // 2  D
        Input::new(p.PC7,  Pull::Up),  // 3  D#
        Input::new(p.PC9,  Pull::Up),  // 4  E
        Input::new(p.PC11, Pull::Up),  // 5  F
        Input::new(p.PA8,  Pull::Up),  // 6  F#
        Input::new(p.PC1,  Pull::Up),  // 7  G
        Input::new(p.PC3,  Pull::Up),  // 8  G#
        Input::new(p.PC8,  Pull::Up),  // 9  A
        Input::new(p.PC10, Pull::Up),  // 10 A#
        Input::new(p.PC12, Pull::Up),  // 11 B
    ];
    

    let mut committed: u16 = 0xFFFF;
    let mut prev_read: u16 = 0;
    // culoarea curenta (implicit alb, ca microfonul sa pulseze mereu)
    let (mut cr, mut cg, mut cb) = (true, true, true);

    loop {
        let mut pressed = heapless::Vec::<u8, 13>::new();
        let mut mask: u16 = 0;
        for (i, b) in buttons.iter().enumerate() {
            if b.is_low() { let _ = pressed.push(i as u8); mask |= 1 << i; }
        }

        let raw_vol = adc.blocking_read(&mut pin_vol, adc4::SampleTime::Cycles15);
        let raw_oct = adc.blocking_read(&mut pin_oct, adc4::SampleTime::Cycles15);
        let volum = (raw_vol as u32 * 100 / 4095) as u8;
        let octava = 2 + (raw_oct as u32 * 5 / 4096).min(4) as u8;

        // --- MICROFON -> intensitate ---
        let mut mn = u16::MAX;
        let mut mx = 0u16;
        for _ in 0..64 {
            let v = adc.blocking_read(&mut pin_mic, adc4::SampleTime::Cycles15);
            if v < mn { mn = v; }
            if v > mx { mx = v; }
        }
        let amplitude = mx - mn;
        // mapare amplitudine -> 1..LEVELS (SCALE il ajustezi dupa microfonul tau)
        const SCALE: u32 = 50; //te poti juca cu valoarea asta ca sa ajustezi sensibilitatea microfonului
        let bright = ((amplitude as u32 / SCALE).min(LEVELS as u32) as u8).max(1);

        // aplica culoarea x intensitate
        DUTY_R.store(if cr { bright } else { 0 }, Ordering::Relaxed);
        DUTY_G.store(if cg { bright } else { 0 }, Ordering::Relaxed);
        DUTY_B.store(if cb { bright } else { 0 }, Ordering::Relaxed);

        // --- actioneaza doar la schimbarea tastelor (debounce) ---
        if mask == prev_read && mask != committed {
            committed = mask;

            let mut l1 = heapless::String::<24>::new();
            let mut l2 = heapless::String::<24>::new();
            let mut json = heapless::String::<96>::new();
            // implicit alb
            cr = true; cg = true; cb = true;

            if pressed.len() >= 3 {
                if let Some((nume, root, i1, i2)) = detect_acord(&pressed) {
                    let rn = NUME[(root % 12) as usize];
                    let n0 = 60 + root;
                    let (n1, n2) = (n0 + i1, n0 + i2);
                    let _ = core::write!(l1, "{} {}", rn, nume);
                    let _ = core::write!(l2, "Vol:{}", volum);
                    let _ = core::write!(json,
                        "{{\"mode\":\"chord\",\"chord\":\"{} {}\",\"notes\":[{},{},{}],\"volume\":{}}}\r\n",
                        rn, nume, n0, n1, n2, volum);
                    // culoare dupa acord
                    match nume {
                        "Major"      => { cr=false; cg=true;  cb=false; }
                        "Minor"      => { cr=false; cg=false; cb=true;  }
                        "Diminished" => { cr=true;  cg=false; cb=false; }
                        "Augmented"  => { cr=true;  cg=true;  cb=false; }
                        _ => {}
                    }
                } else {
                    let _ = core::write!(l1, "Acord ?");
                    let _ = core::write!(l2, "Vol:{}", volum);
                    let _ = core::write!(json, "{{\"mode\":\"off\"}}\r\n");
                }
            } else if pressed.len() == 1 {
                let idx = pressed[0];
                let note = 12 * (octava + 1) + idx;
                let _ = core::write!(l1, "Nota:{}{}", NUME[(idx % 12) as usize], octava);
                let _ = core::write!(l2, "Vol:{} Oct:{}", volum, octava);
                let _ = core::write!(json,
                    "{{\"mode\":\"note\",\"note\":{},\"octave\":{},\"volume\":{}}}\r\n",
                    note, octava, volum);
            } else {
                let _ = core::write!(l1, "-");
                let _ = core::write!(l2, "Vol:{} Oct:{}", volum, octava);
                let _ = core::write!(json, "{{\"mode\":\"off\"}}\r\n");
            }

            display.clear(BinaryColor::Off).unwrap();
            Text::with_baseline(&l1, Point::new(0, 0),  style, Baseline::Top).draw(&mut display).unwrap();
            Text::with_baseline(&l2, Point::new(0, 20), style, Baseline::Top).draw(&mut display).unwrap();
            display.flush().unwrap();

            let _ = usart.blocking_write(json.as_bytes());
            info!("{}", json.as_str());
        }

        prev_read = mask;
        Timer::after_millis(10).await;
    }
}