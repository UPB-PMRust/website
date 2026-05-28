#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pin, Pull};
use embassy_rp::spi::{Config as SpiConfig, Spi};
use embassy_time::Delay;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use embedded_graphics_core::pixelcolor::raw::RawU16;
use panic_halt as _;

struct Gc9a01<'d> {
    spi: Spi<'d, embassy_rp::peripherals::SPI1, embassy_rp::spi::Blocking>,
    dc: Output<'d, AnyPin>,
    cs: Output<'d, AnyPin>,
}

impl<'d> Gc9a01<'d> {
    fn cmd(&mut self, c: u8) {
        self.cs.set_low();
        self.dc.set_low();
        self.spi.blocking_write(&[c]).ok();
        self.cs.set_high();
    }

    fn dat(&mut self, d: &[u8]) {
        self.cs.set_low();
        self.dc.set_high();
        self.spi.blocking_write(d).ok();
        self.cs.set_high();
    }

    fn init(&mut self) {
        self.cmd(0xEF);
        self.cmd(0xEB); self.dat(&[0x14]);
        self.cmd(0xFE);
        self.cmd(0xEF);
        self.cmd(0xEB); self.dat(&[0x14]);
        self.cmd(0x84); self.dat(&[0x40]);
        self.cmd(0x85); self.dat(&[0xFF]);
        self.cmd(0x86); self.dat(&[0xFF]);
        self.cmd(0x87); self.dat(&[0xFF]);
        self.cmd(0x88); self.dat(&[0x0A]);
        self.cmd(0x89); self.dat(&[0x21]);
        self.cmd(0x8A); self.dat(&[0x00]);
        self.cmd(0x8B); self.dat(&[0x80]);
        self.cmd(0x8C); self.dat(&[0x01]);
        self.cmd(0x8D); self.dat(&[0x01]);
        self.cmd(0x8E); self.dat(&[0xFF]);
        self.cmd(0x8F); self.dat(&[0xFF]);
        self.cmd(0xB6); self.dat(&[0x00, 0x20]);
        self.cmd(0x36); self.dat(&[0x48]);
        self.cmd(0x3A); self.dat(&[0x05]);
        self.cmd(0x90); self.dat(&[0x08, 0x08, 0x08, 0x08]);
        self.cmd(0xBD); self.dat(&[0x06]);
        self.cmd(0xBC); self.dat(&[0x00]);
        self.cmd(0xFF); self.dat(&[0x60, 0x01, 0x04]);
        self.cmd(0xC3); self.dat(&[0x13]);
        self.cmd(0xC4); self.dat(&[0x13]);
        self.cmd(0xC9); self.dat(&[0x22]);
        self.cmd(0xBE); self.dat(&[0x11]);
        self.cmd(0xE1); self.dat(&[0x10, 0x0E]);
        self.cmd(0xDF); self.dat(&[0x21, 0x0c, 0x02]);
        self.cmd(0xF0); self.dat(&[0x45, 0x09, 0x08, 0x08, 0x26, 0x2A]);
        self.cmd(0xF1); self.dat(&[0x43, 0x70, 0x72, 0x36, 0x37, 0x6F]);
        self.cmd(0xF2); self.dat(&[0x45, 0x09, 0x08, 0x08, 0x26, 0x2A]);
        self.cmd(0xF3); self.dat(&[0x43, 0x70, 0x72, 0x36, 0x37, 0x6F]);
        self.cmd(0xED); self.dat(&[0x1B, 0x0B]);
        self.cmd(0xAE); self.dat(&[0x77]);
        self.cmd(0xCD); self.dat(&[0x63]);
        self.cmd(0x70); self.dat(&[0x07, 0x07, 0x04, 0x0E, 0x0F, 0x09, 0x07, 0x08, 0x03]);
        self.cmd(0xE8); self.dat(&[0x34]);
        self.cmd(0x62); self.dat(&[0x18, 0x0D, 0x71, 0xED, 0x70, 0x70, 0x18, 0x0F, 0x71, 0xEF, 0x70, 0x70]);
        self.cmd(0x63); self.dat(&[0x18, 0x11, 0x71, 0xF1, 0x70, 0x70, 0x18, 0x13, 0x71, 0xF3, 0x70, 0x70]);
        self.cmd(0x64); self.dat(&[0x28, 0x29, 0xF1, 0x01, 0xF1, 0x00, 0x07]);
        self.cmd(0x66); self.dat(&[0x3C, 0x00, 0xCD, 0x67, 0x45, 0x45, 0x10, 0x00, 0x00, 0x00]);
        self.cmd(0x67); self.dat(&[0x00, 0x3C, 0x00, 0x00, 0x00, 0x01, 0x54, 0x10, 0x32, 0x98]);
        self.cmd(0x74); self.dat(&[0x10, 0x85, 0x80, 0x00, 0x00, 0x4E, 0x00]);
        self.cmd(0x98); self.dat(&[0x3e, 0x07]);
        self.cmd(0x35);
        self.cmd(0x21);
        self.cmd(0x11);
        cortex_m::asm::delay(12_000_000);
        self.cmd(0x29);
        cortex_m::asm::delay(2_000_000);
    }

    fn fill(&mut self, color: Rgb565) {
        let raw: u16 = RawU16::from(color).into_inner();
        let hi = (raw >> 8) as u8;
        let lo = raw as u8;
        self.cmd(0x2A); self.dat(&[0x00, 0x00, 0x00, 0xEF]);
        self.cmd(0x2B); self.dat(&[0x00, 0x00, 0x00, 0xEF]);
        self.cmd(0x2C);
        self.cs.set_low();
        self.dc.set_high();
        let buf = [hi, lo];
        for _ in 0..(240 * 240) {
            self.spi.blocking_write(&buf).ok();
        }
        self.cs.set_high();
    }
}

impl<'d> DrawTarget for Gc9a01<'d> {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            if coord.x >= 0 && coord.x < 240 && coord.y >= 0 && coord.y < 240 {
                let x = coord.x as u16;
                let y = coord.y as u16;
                self.cmd(0x2A);
                self.dat(&[(x >> 8) as u8, x as u8, (x >> 8) as u8, x as u8]);
                self.cmd(0x2B);
                self.dat(&[(y >> 8) as u8, y as u8, (y >> 8) as u8, y as u8]);
                self.cmd(0x2C);
                let raw: u16 = RawU16::from(color).into_inner();
                self.dat(&[(raw >> 8) as u8, raw as u8]);
            }
        }
        Ok(())
    }
}

impl<'d> OriginDimensions for Gc9a01<'d> {
    fn size(&self) -> Size {
        Size::new(240, 240)
    }
}

#[entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());

    let mut spi_cfg = SpiConfig::default();
    spi_cfg.frequency = 32_000_000;

    let spi = Spi::new_blocking_txonly(
        p.SPI1,
        p.PIN_10,
        p.PIN_11,
        spi_cfg,
    );

    let dc  = Output::new(p.PIN_8.degrade(), Level::Low);
    let cs  = Output::new(p.PIN_9.degrade(), Level::High);
    let mut rst = Output::new(p.PIN_12.degrade(), Level::High);

    rst.set_low();
    cortex_m::asm::delay(1_000_000);
    rst.set_high();
    cortex_m::asm::delay(1_000_000);

    let mut display = Gc9a01 { spi, dc, cs };
    display.init();
    display.fill(Rgb565::BLACK);

    let btn2 = Input::new(p.PIN_15.degrade(), Pull::Up);
    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    let mut bpm = 72u32;

    loop {
        display.fill(Rgb565::BLACK);

        Text::new("12:45", Point::new(70, 90), text_style)
            .draw(&mut display).ok();

        let mut buffer = heapless::String::<32>::new();
        use core::fmt::Write;
        write!(buffer, "PULS {} BPM", bpm).ok();
        Text::new(&buffer, Point::new(40, 150), text_style)
            .draw(&mut display).ok();

        bpm += 1;
        if bpm > 90 { bpm = 72; }

        if btn2.is_low() {
            display.fill(Rgb565::RED);
        }

        cortex_m::asm::delay(62_500_000);
    }
}