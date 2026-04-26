# Smart Parking System

A smart parking system with real-time spot monitoring and web-based reservation.

:::info

**Author**: Dragomirescu Alexandru  
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-alexdragomirescu06

:::

## Description

The Smart Parking System monitors parking spots in real time using ultrasonic sensors. A servo motor controls a barrier that opens automatically when a valid PIN is entered via a keypad. An OLED display shows the current status of the parking lot, and a web interface hosted on the Raspberry Pi Pico W allows users to reserve spots remotely over WiFi.

## Motivation

I chose this project because it combines multiple hardware components and communication protocols into a single practical system. It covers areas like sensor reading, motor control, display output, network communication, and user input — making it a comprehensive project that reflects everything learned throughout the semester.

## Architecture

The system is built around the Raspberry Pi Pico W as the central controller, running software written in Rust using the embassy-rp async framework.

Main components:
- **Raspberry Pi Pico W** — main microcontroller, handles all logic and WiFi
- **Ultrasonic Sensors (HC-SR04)** — detect whether each parking spot is occupied
- **Servo Motor** — controls the entry/exit barrier
- **OLED Display (SSD1306)** — shows parking spot availability and system status
- **4x4 Keypad** — allows users to enter a PIN to open the barrier
- **Web Interface** — hosted on the Pico W, accessible over WiFi for remote reservation

All components communicate with the Pico W directly: ultrasonic sensors via GPIO, servo via PWM, OLED via I2C, keypad via GPIO matrix scanning, and the web interface via the onboard WiFi stack.

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The hardware is based on the Raspberry Pi Pico W microcontroller. Ultrasonic sensors (HC-SR04) are used to detect the presence of a vehicle in each parking spot. A servo motor acts as a barrier gate. An SSD1306 OLED display shows real-time status. A 4x4 matrix keypad is used for PIN input.

### Schematics

![Schematic](schematic.svg)

### Bill of Materials

| Device | Usage | Price |
| ------ | ----- | ----- |
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [HC-SR04 Ultrasonic Sensor](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf) | Parking spot detection | [7 RON](https://www.optimusdigital.ro/en/ultrasonic-sensors/9-hc-sr04-ultrasonic-sensor.html) |
| [SG90 Servo Motor](http://www.ee.ic.ac.uk/pcheung/teaching/DE1_EE/stores/sg90_datasheet.pdf) | Barrier control | [12 RON](https://www.optimusdigital.ro/en/servomotors/26-sg90-micro-servo-motor.html) |
| [SSD1306 OLED Display](https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf) | Status display | [20 RON](https://www.optimusdigital.ro/en/lcds/2894-096-i2c-oled-display.html) |
| [4x4 Matrix Keypad](https://www.optimusdigital.ro/en/keypads/470-4x4-matrix-keypad.html) | PIN input | [7 RON](https://www.optimusdigital.ro/en/keypads/470-4x4-matrix-keypad.html) |

## Software

| Library | Description | Usage |
| ------- | ----------- | ----- |
| [embassy-rp](https://github.com/embassy-rs/embassy) | Async embedded framework for RP2040 | Main async runtime and peripheral drivers |
| [embassy-net](https://github.com/embassy-rs/embassy) | Networking stack for embassy | WiFi and TCP/IP for the web interface |
| [ssd1306](https://github.com/jamwaffles/ssd1306) | OLED display driver | Driving the SSD1306 display over I2C |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Drawing text and shapes to the display |
| [heapless](https://github.com/rust-embedded/heapless) | Static data structures | Used for fixed-size buffers without heap allocation |

## Links

1. [Embassy-rs documentation](https://embassy.dev)
2. [Raspberry Pi Pico W datasheet](https://datasheets.raspberrypi.com/picow/pico-w-datasheet.pdf)
3. [HC-SR04 datasheet](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf)
