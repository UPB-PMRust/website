---
sidebar_position: 1
---

# Smart Air Purifier - andrei.tanase1012

## Description
An automated air filtration system that monitors PM2.5 levels and adjusts fan speed using an STM32 Nucleo-U545RE-Q.

## Motivation
Indoor air quality is a significant health factor, especially in areas with varying pollution levels like Otopeni. This project aims to create a responsive, low-power solution using Rust.

## Architecture
### Hardware Design
* **Microcontroller:** STM32 Nucleo-U545RE-Q
* **Sensor (PM2.5):** Plantower PMS5003
* **Sensor (Env):** AHT20 + BMP280 (I2C)
* **Display:** 0.96" OLED (SSD1306)
* **Actuator:** 12V PC Fan via IRLZ44N MOSFET

### Software Design
* **Language:** Rust
* **Framework:** Embassy (Async)
* **Features:** I2C sensor reading, PWM fan control, and real-time OLED updates.

## Weekly Log
* **Week 5:** Defined project scope and verified hardware compatibility.
* **Week 6:** Initialized GitHub repository and established documentation page.

## Bill of Materials
| Component | Quantity | Price |
| :--- | :---: | :--- |
| STM32 Nucleo | 1 | - |
| PMS5003 | 1 | - |
| MOSFET/Resistors | 1 kit | - |
