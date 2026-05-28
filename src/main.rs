#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use embedded_graphics_core::pixelcolor::raw::RawU16;
use panic_halt as _;
use rp235x_hal as hal;
use hal::pac;
use hal::spi::Spi;
use hal::gpio::{FunctionSpi, FunctionSioOutput, PullNone, PullDown};
use hal::Clock;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;
use rp235x_hal::fugit::RateExtU32;

type SpiDisplay = Spi<hal::spi::Enabled, pac::SPI1, (
    hal::gpio::Pin<hal::gpio::bank0::Gpio11, FunctionSpi, PullDown>,
    hal::gpio::Pin<hal::gpio::bank0::Gpio10, FunctionSpi, PullDown>,
), 8>;

struct Display {
    spi: SpiDisplay,
    dc:  hal::gpio::Pin<hal::gpio::bank0::Gpio8,  FunctionSioOutput, PullDown>,
    cs:  hal::gpio::Pin<hal::gpio::bank0::Gpio9,  FunctionSioOutput, PullDown>,
}

impl Display {
    fn cmd(&mut self, c: u8) {
        self.cs.set_low();
        self.dc.set_low();
        let _ = self.spi.write(&[c]);
        self.cs.set_high();
    }
    fn dat(&mut self, d: &[u8]) {
        self.cs.set_low();
        self.dc.set_high();
        let _ = self.spi.write(d);
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
        self.cmd(0x70); self.dat(&[0x07,0x07,0x04,0x0E,0x0F,0x09,0x07,0x08,0x03]);
        self.cmd(0xE8); self.dat(&[0x34]);
        self.cmd(0x62); self.dat(&[0x18,0x0D,0x71,0xED,0x70,0x70,0x18,0x0F,0x71,0xEF,0x70,0x70]);
        self.cmd(0x63); self.dat(&[0x18,0x11,0x71,0xF1,0x70,0x70,0x18,0x13,0x71,0xF3,0x70,0x70]);
        self.cmd(0x64); self.dat(&[0x28,0x29,0xF1,0x01,0xF1,0x00,0x07]);
        self.cmd(0x66); self.dat(&[0x3C,0x00,0xCD,0x67,0x45,0x45,0x10,0x00,0x00,0x00]);
        self.cmd(0x67); self.dat(&[0x00,0x3C,0x00,0x00,0x00,0x01,0x54,0x10,0x32,0x98]);
        self.cmd(0x74); self.dat(&[0x10,0x85,0x80,0x00,0x00,0x4E,0x00]);
        self.cmd(0x98); self.dat(&[0x3e,0x07]);
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
        self.cmd(0x2A); self.dat(&[0x00,0x00,0x00,0xEF]);
        self.cmd(0x2B); self.dat(&[0x00,0x00,0x00,0xEF]);
        self.cmd(0x2C);
        self.cs.set_low();
        self.dc.set_high();
        for _ in 0..(240*240) {
            let _ = self.spi.write(&[hi, lo]);
        }
        self.cs.set_high();
    }
}

impl DrawTarget for Display {
    type Color = Rgb565;
    type Error = core::convert::Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where I: IntoIterator<Item = Pixel<Self::Color>> {
        for Pixel(coord, color) in pixels {
            if coord.x >= 0 && coord.x < 240 && coord.y >= 0 && coord.y < 240 {
                let x = coord.x as u16;
                let y = coord.y as u16;
                self.cmd(0x2A);
                self.dat(&[(x>>8) as u8, x as u8, (x>>8) as u8, x as u8]);
                self.cmd(0x2B);
                self.dat(&[(y>>8) as u8, y as u8, (y>>8) as u8, y as u8]);
                self.cmd(0x2C);
                let raw: u16 = RawU16::from(color).into_inner();
                self.dat(&[(raw>>8) as u8, raw as u8]);
            }
        }
        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size { Size::new(240, 240) }
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000u32,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0, pac.PADS_BANK0,
        sio.gpio_bank0, &mut pac.RESETS,
    );

    let mut rst = pins.gpio12.into_push_pull_output();
    rst.set_low();
    cortex_m::asm::delay(1_000_000);
    rst.set_high();
    cortex_m::asm::delay(1_000_000);

    let spi_mosi = pins.gpio11.into_function::<FunctionSpi>();
    let spi_sck  = pins.gpio10.into_function::<FunctionSpi>();
    let spi = Spi::<_, _, _, 8>::new(pac.SPI1, (spi_mosi, spi_sck))
        .init(&mut pac.RESETS, clocks.peripheral_clock.freq(),
              32_000_000u32.Hz(), embedded_hal::spi::MODE_0);

    let dc = pins.gpio8.into_push_pull_output();
    let cs = pins.gpio9.into_push_pull_output();

    let mut display = Display { spi, dc, cs };
    display.init();
    display.fill(Rgb565::BLACK);

    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    let red_style  = MonoTextStyle::new(&FONT_10X20, Rgb565::RED);
    let mut bpm = 72u32;

    loop {
        display.fill(Rgb565::BLACK);
        Text::new("12:45", Point::new(70, 100), text_style)
            .draw(&mut display).ok();
        let mut buf = heapless::String::<32>::new();
        use core::fmt::Write;
        write!(buf, "# {} BPM", bpm).ok();
        Text::new(&buf, Point::new(55, 140), red_style)
            .draw(&mut display).ok();
        bpm += 1;
        if bpm > 90 { bpm = 72; }
        cortex_m::asm::delay(62_500_000);
    }
}