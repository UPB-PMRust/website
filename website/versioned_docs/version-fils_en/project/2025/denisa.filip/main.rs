use rppal::gpio::{Gpio, InputPin, OutputPin};
use linux_embedded_hal::I2cdev;
use hd44780_driver::{Hd44780, bus::I2cBus};
use std::{thread, time::Duration};


const GREEN_PIN: u8 = 17;
const YELLOW_PIN: u8 = 27;
const RED_PIN: u8 = 22;
const BUZZER_PIN: u8 = 23;
const SENSOR_PIN: u8 = 24;


const LCD_I2C_ADDR: u16 = 0x27;

fn main() {
    
    let gpio = Gpio::new().expect("Failed to initialize GPIO");

    let mut green = gpio.get(GREEN_PIN).unwrap().into_output();
    let mut yellow = gpio.get(YELLOW_PIN).unwrap().into_output();
    let mut red = gpio.get(RED_PIN).unwrap().into_output();
    let mut buzzer = gpio.get(BUZZER_PIN).unwrap().into_output();

    let sensor = gpio.get(SENSOR_PIN).unwrap().into_input();

   
    let i2c = I2cdev::new("/dev/i2c-1").expect("Failed to open I2C bus");
    let mut lcd = Hd44780::new(I2cBus::new(i2c, LCD_I2C_ADDR));
    lcd.reset().unwrap();
    lcd.clear().unwrap();
    lcd.write_str("Parking Sensor").unwrap();

    loop {
        let object_detected = sensor.is_low(); 
        let distance = if object_detected { 5 } else { 15 };

        
        lcd.set_cursor_pos(0x40).unwrap(); 
        lcd.write_str(&format!("Dist: {:2} cm  ", distance)).unwrap();

        
        if distance > 10 {
            green.set_high();
            yellow.set_low();
            red.set_low();
            buzzer.set_low();
        } else if distance > 6 {
            green.set_low();
            yellow.set_high();
            red.set_low();
            buzzer.set_low();
        } else {
            green.set_low();
            yellow.set_low();
            red.set_high();
            buzzer.set_high();
        }

        thread::sleep(Duration::from_millis(300));
    }
}