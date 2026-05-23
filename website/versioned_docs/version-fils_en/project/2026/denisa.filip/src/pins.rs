/// ─────────────────────────────────────────────────────────────────────────────
/// Pin assignments – Raspberry Pi Pico W (RP2040)
/// Read from the KiCad schematic provided in the project.
///
///  I²C bus 0  (Sensor 1 / LCD 1602)
///    SDA  →  GPIO 4   (physical pin 6)
///    SCL  →  GPIO 5   (physical pin 7)
///
///  I²C bus 1  (Sensor 2)
///    SDA  →  GPIO 8   (physical pin 11)
///    SCL  →  GPIO 9   (physical pin 12)
///
///  RGB LED 1  (common-anode, active-LOW)
///    R    →  GPIO 10  (physical pin 14)
///    G    →  GPIO 11  (physical pin 15)
///    B    →  GPIO 12  (physical pin 16)
///
///  RGB LED 2  (common-anode, active-LOW)
///    R    →  GPIO 13  (physical pin 17)
///    G    →  GPIO 14  (physical pin 19)
///    B    →  GPIO 15  (physical pin 20)
///
///  Buzzers  (passive, driven via PWM)
///    BZ1  →  GPIO 16  (physical pin 21)
///    BZ2  →  GPIO 17  (physical pin 22)
///
///  Digital Infrared obstacle sensors (digital OUT, active-LOW)
///    SENSOR_1_OUT →  GPIO 20  (physical pin 26)
///    SENSOR_2_OUT →  GPIO 21  (physical pin 27)
/// ─────────────────────────────────────────────────────────────────────────────

// I²C 0 – sensor 1 + LCD
pub const I2C0_SDA: u8 = 4;
pub const I2C0_SCL: u8 = 5;

// I²C 1 – sensor 2
pub const I2C1_SDA: u8 = 8;
pub const I2C1_SCL: u8 = 9;

// RGB LED 1
pub const LED1_R: u8 = 10;
pub const LED1_G: u8 = 11;
pub const LED1_B: u8 = 12;

// RGB LED 2
pub const LED2_R: u8 = 13;
pub const LED2_G: u8 = 14;
pub const LED2_B: u8 = 15;

// Buzzers
pub const BUZZER1: u8 = 16;
pub const BUZZER2: u8 = 17;

// Infrared obstacle sensor digital outputs
pub const SENSOR1_OUT: u8 = 20;
pub const SENSOR2_OUT: u8 = 21;

// I²C addresses
pub const LCD_I2C_ADDR:     u8 = 0x27; // PCF8574-based 1602 backpack (common default)
pub const SENSOR1_I2C_ADDR: u8 = 0x29; // TMF8820 default address
pub const SENSOR2_I2C_ADDR: u8 = 0x29; // same chip, different bus
