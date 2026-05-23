/// LCD 1602 display driver (PCF8574 I²C backpack).
///
/// We drive the display by bit-banging the PCF8574 I/O expander that is
/// soldered onto most 1602 I²C modules.  The backpack maps:
///
///   PCF8574 bit 7 (P7) → D7
///   PCF8574 bit 6 (P6) → D6
///   PCF8574 bit 5 (P5) → D5
///   PCF8574 bit 4 (P4) → D4
///   PCF8574 bit 3 (P3) → Backlight (active HIGH)
///   PCF8574 bit 2 (P2) → E  (enable)
///   PCF8574 bit 1 (P1) → RW (write = 0)
///   PCF8574 bit 0 (P0) → RS (register select: 0 = cmd, 1 = data)
///
/// We use 4-bit mode (only D4–D7), which means each byte is sent as two
/// nibbles with an enable pulse between them.

use embassy_rp::i2c::I2c;
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c::I2c as AsyncI2c;

const BACKLIGHT: u8 = 0b0000_1000;
const ENABLE: u8    = 0b0000_0100;
const RS_DATA: u8   = 0b0000_0001;
// RW always 0 (write)

/// Blocking-style LCD wrapper that hides all the nibble shuffling.
pub struct Lcd1602<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C, E> Lcd1602<I2C>
where
    I2C: AsyncI2c<Error = E>,
    E: core::fmt::Debug,
{
    /// Initialise the LCD in 4-bit mode.
    pub async fn new(i2c: I2C, addr: u8) -> Self {
        let mut lcd = Self { i2c, addr };
        // Wait for power-on
        Timer::after(Duration::from_millis(50)).await;

        // Special init sequence to switch into 4-bit mode
        lcd.write_nibble(0x03, false).await;
        Timer::after(Duration::from_millis(5)).await;
        lcd.write_nibble(0x03, false).await;
        Timer::after(Duration::from_millis(1)).await;
        lcd.write_nibble(0x03, false).await;
        Timer::after(Duration::from_micros(150)).await;
        lcd.write_nibble(0x02, false).await; // switch to 4-bit

        // Function set: 4-bit, 2 lines, 5×8 font
        lcd.send_cmd(0x28).await;
        // Display on, cursor off, blink off
        lcd.send_cmd(0x0C).await;
        // Entry mode: increment, no shift
        lcd.send_cmd(0x06).await;
        // Clear display
        lcd.clear().await;
        lcd
    }

    pub async fn clear(&mut self) {
        self.send_cmd(0x01).await;
        Timer::after(Duration::from_millis(2)).await;
    }

    pub async fn set_cursor(&mut self, col: u8, row: u8) {
        let row_offset = [0x00u8, 0x40];
        let addr = 0x80 | (col + row_offset[row as usize % 2]);
        self.send_cmd(addr).await;
    }

    pub async fn print(&mut self, s: &str) {
        for byte in s.bytes() {
            self.send_data(byte).await;
        }
    }

    // ── internals ────────────────────────────────────────────────────────────

    async fn send_cmd(&mut self, cmd: u8) {
        self.send_byte(cmd, false).await;
    }

    async fn send_data(&mut self, data: u8) {
        self.send_byte(data, true).await;
    }

    async fn send_byte(&mut self, byte: u8, rs: bool) {
        let hi = (byte >> 4) & 0x0F;
        let lo = byte & 0x0F;
        self.write_nibble(hi, rs).await;
        self.write_nibble(lo, rs).await;
    }

    async fn write_nibble(&mut self, nibble: u8, rs: bool) {
        let rs_bit = if rs { RS_DATA } else { 0 };
        let data = (nibble << 4) | BACKLIGHT | rs_bit;

        // Pulse enable high then low
        let _ = self.i2c.write(self.addr, &[data | ENABLE]).await;
        Timer::after(Duration::from_micros(1)).await;
        let _ = self.i2c.write(self.addr, &[data & !ENABLE]).await;
        Timer::after(Duration::from_micros(50)).await;
    }
}
