//lcd.rs

use embassy_rp::i2c::I2c;
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c::I2c as AsyncI2c;

const BACKLIGHT: u8 = 0b0000_1000;
const ENABLE: u8    = 0b0000_0100;
const RS_DATA: u8   = 0b0000_0001;

pub struct Lcd1602<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C, E> Lcd1602<I2C>
where
    I2C: AsyncI2c<Error = E>,
    E: core::fmt::Debug,
{
    pub async fn new(i2c: I2C, addr: u8) -> Self {
        let mut lcd = Self { i2c, addr };
        
        Timer::after(Duration::from_millis(50)).await;

        lcd.write_nibble(0x03, false).await;
        Timer::after(Duration::from_millis(5)).await;
        lcd.write_nibble(0x03, false).await;
        Timer::after(Duration::from_millis(1)).await;
        lcd.write_nibble(0x03, false).await;
        Timer::after(Duration::from_micros(150)).await;
        lcd.write_nibble(0x02, false).await; 
        lcd.send_cmd(0x28).await;
        lcd.send_cmd(0x0C).await;
        lcd.send_cmd(0x06).await;
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

        
        let _ = self.i2c.write(self.addr, &[data | ENABLE]).await;
        Timer::after(Duration::from_micros(1)).await;
        let _ = self.i2c.write(self.addr, &[data & !ENABLE]).await;
        Timer::after(Duration::from_micros(50)).await;
    }
}
