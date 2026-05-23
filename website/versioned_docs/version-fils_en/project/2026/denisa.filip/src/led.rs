//led.rs
use embassy_rp::gpio::{Level, Output};
use crate::distance::Zone;

pub struct RgbLed<'d> {
    r: Output<'d>,
    g: Output<'d>,
    b: Output<'d>,
}

impl<'d> RgbLed<'d> {
    pub fn new(r: Output<'d>, g: Output<'d>, b: Output<'d>) -> Self {
        let mut led = Self { r, g, b };
        led.off();
        led
    }
    pub fn off(&mut self) {
        self.r.set_high();
        self.g.set_high();
        self.b.set_high();
    }

    pub fn green(&mut self) {
        self.r.set_high();
        self.g.set_low();
        self.b.set_high();
    }

    pub fn yellow(&mut self) {
        self.r.set_low();
        self.g.set_low();
        self.b.set_high();
    }

    pub fn red(&mut self) {
        self.r.set_low();
        self.g.set_high();
        self.b.set_high();
    }

    pub fn set_zone(&mut self, zone: Zone) {
        match zone {
            Zone::Clear   => self.green(),
            Zone::Warning => self.yellow(),
            Zone::Danger  => self.red(),
        }
    }
}

pub fn set_both_leds(led1: &mut RgbLed<'_>, led2: &mut RgbLed<'_>, zone: Zone) {
    led1.set_zone(zone);
    led2.set_zone(zone);
}
