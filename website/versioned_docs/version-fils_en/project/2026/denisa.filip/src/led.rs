use embassy_rp::gpio::{Level, Output};
use crate::distance::Zone;

/// A single common-anode RGB LED driven by three GPIO outputs.
/// Common-anode means the LED is ON when the pin is LOW (active-low).
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

    /// Turn all channels off (all pins HIGH for common-anode).
    pub fn off(&mut self) {
        self.r.set_high();
        self.g.set_high();
        self.b.set_high();
    }

    /// Set LED to green (safe distance).
    pub fn green(&mut self) {
        self.r.set_high();
        self.g.set_low();
        self.b.set_high();
    }

    /// Set LED to yellow (warning – red + green).
    pub fn yellow(&mut self) {
        self.r.set_low();
        self.g.set_low();
        self.b.set_high();
    }

    /// Set LED to red (danger).
    pub fn red(&mut self) {
        self.r.set_low();
        self.g.set_high();
        self.b.set_high();
    }

    /// Update LED colour to reflect the current distance zone.
    pub fn set_zone(&mut self, zone: Zone) {
        match zone {
            Zone::Clear   => self.green(),
            Zone::Warning => self.yellow(),
            Zone::Danger  => self.red(),
        }
    }
}

/// Helper: set both LEDs to the same zone colour.
pub fn set_both_leds(led1: &mut RgbLed<'_>, led2: &mut RgbLed<'_>, zone: Zone) {
    led1.set_zone(zone);
    led2.set_zone(zone);
}
