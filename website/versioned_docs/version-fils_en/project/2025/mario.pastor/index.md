# Health Monitor
A wearable device that tracks heart rate, blood oxygen saturation, and step count in real-time.

:::info 

**Author**: PASTOR Mario-Alexandru \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-l00tus

:::

## Description

This wearable health monitor continuously measures and displays the user's vital signs in real time. It uses a MAX30102 pulse oximeter sensor to calculate heart rate (BPM) and blood oxygen saturation (SpO₂), alongside an ADXL345 accelerometer to detect and count steps. The collected data is displayed on a compact SSD1306 screen, providing a clear and minimal interface. Additionally, the device hosts a server, allowing the latest readings to be retrieved via Wi-Fi.

## Motivation

I chose this project because I wanted to develop something that integrates both hardware and software in a meaningful way. I was especially motivated by the idea of building something that could have real-life applicability, so I decided to build a wearable health monitor, which felt like a good balance between technical challenge and practical usefulness.

## Architecture

### Schematic Diagram

![architecture-diagram](assets/architecture_diagram.svg)

### Components Overview

- **2x Raspberry Pi Pico 2W**:
    - one used as a debugger
    - one used to control the sensors and display
- **ADXL345 Accelerometer**: used to measure step count through 3-axis acceleration data
- **MAX30102**: used to measure heart rate and blood oxygen level by detecting changes in light absorption from red and IR LEDs using a photoplethysmography sensor 
- **SSD1306**: used to display real-time sensor data like the heart rate, SpO₂, and step count on an OLED screen

## Log

### Week 21 - 27 Apr
After my project got approved, I looked for the necessary hardware and software parts. After deciding on the parts, I ordered them.

### Week 28 - 4 May
I started to assemble the parts on the breadboards in order to test each component. I managed to read the data on all three axis of the accelerometer, to determine the heart rate and SpO₂ from the pulse oximeter and print the information in the terminal. Additionally, I tested the screen.

![testing-components](assets/week10_update.webp)

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

My project is built around the Raspberry Pi Pico 2W microcontroller, which collects and processes data from two sensors. An ADXL345 accelerometer is used to detect motion and count steps by measuring the changes in acceleration, while the MAX30102 pulse oximeter sensor measures heart rate and blood oxygen levels by detecting changes in light absorption due to blood flow. The SSD1306 OLED display shows in real-time the health metrics of the user. As of now, this is how the hardware looks like, but in the following weeks I will design a 3D case in order to make the device wearable.

![hardware](assets/hardware1.webp)

### Schematics

![kicad-schematic](assets/kicad_schematic.webp)

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [2x Raspberry Pi Pico 2W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller and the debugger | [2 x 40 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html) |
| [ADXL345](https://www.analog.com/media/en/technical-documentation/data-sheets/adxl345.pdf) | Accelerometer for detecting and counting steps | [13 RON](https://www.optimusdigital.ro/en/inertial-sensors/97-adxl345-tripple-axis-accelerometer.html) |
| [MAX30102](https://www.analog.com/media/en/technical-documentation/data-sheets/max30102.pdf) | Pulse oximeter and heart rate sensor | [13 RON](https://www.robofun.ro/breakout-senzor-ritm-cardiac-gy-max30102-arduino.html) |
| [SSD1306](http://www.lcdwiki.com/res/MSP096X/0.96inch_OLED_SPI_Module_MSP096X_User_Manual_EN.pdf) | OLED screen for displaying data | [24 RON](https://www.optimusdigital.ro/en/lcds/194-yellow-and-blue-096-oled-module-128x64-px.html) |
| [3x Breadboards](https://www.optimusdigital.ro/en/breadboards/44-400p-hq-breadboard.html) & [Wires](https://www.optimusdigital.ro/en/158-wires) | Wiring | [3 x 5 RON](https://www.optimusdigital.ro/en/breadboards/44-400p-hq-breadboard.html) + [8 RON](https://www.optimusdigital.ro/en/158-wires) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-executor](https://crates.io/crates/embassy-executor) | Asynchronous task executor | Used for task scheduling |
| [embassy-rp](https://crates.io/crates/embassy-rp) | RP2040 HAL for Embassy | Access to RP2040 peripherals (GPIO, SPI, I2C) |
| [embassy-time](https://crates.io/crates/embassy-time) | Timing utilities | Used for delays |
| [embassy-sync](https://crates.io/crates/embassy-sync) | Async synchronization primitives | Used for Mutex |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Rendering text on the screen |
| [ssd1306](https://crates.io/crates/ssd1306) | OLED display driver | Used to control the display |
| [display-interface-spi](https://crates.io/crates/display-interface-spi) | SPI display abstraction | Facilitates SPI communication between SSD1306 and device |
| [defmt](https://crates.io/crates/defmt) | Lightweight logging crate | Used for logging data |
| [heapless](https://crates.io/crates/heapless) | Data structures | Used for creating strings |

## Links

1. [Measure Heart Rate and SpO2 with MAX30102](https://projecthub.arduino.cc/SurtrTech/measure-heart-rate-and-spo2-with-max30102-eb4f74)