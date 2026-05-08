# Smart Parking System

A smart parking system with real-time spot monitoring and web-based reservation.

:::info

**Author**: Dragomirescu Alexandru  \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-alexdragomirescu06

:::

## Description

The Smart Parking System monitors parking spots in real time using ultrasonic sensors. A servo motor controls a barrier that opens automatically when there are open spots in the parking lot, or when a valid PIN is entered via a keypad in the case of a reserved spot. A display shows the current status of the parking lot and prompts you to enter the reservation code via a keypad, and a web interface hosted on the Raspberry Pi Pico W allows users to reserve spots remotely over WiFi.

## Motivation

I chose this project because it combines multiple hardware components and communication protocols into a single practical system. It covers areas like sensor reading, motor control, display output, network communication, and user input — making it a comprehensive project that reflects everything learned throughout the semester.

## Architecture

![Architecture Diagram](architecture.svg)

The system is built around the Raspberry Pi Pico W as the central controller, running software written in Rust using the embassy-rp async framework.

Main components:

* **Raspberry Pi Pico 2 W** — main microcontroller, handles all logic and WiFi
* **Ultrasonic Sensors (HC-SR04 x6)** — detect whether each parking spot is occupied
* **Servo Motor (SG90)** — controls the entry/exit barrier
* **ST7735 TFT Display (1.8" 128x160)** — shows parking spot availability and system status
* **3x4 Keypad** — allows users to enter a PIN to open the barrier
* **Status LEDs (x6)** — one per spot, give instant visual feedback on occupancy
* **Web Interface** — hosted on the Pico 2 W, accessible over WiFi for remote reservation

All components communicate with the Pico 2 W directly: ultrasonic sensors via GPIO, servo via PWM, TFT display via SPI, keypad via GPIO matrix scanning, LEDs via GPIO, and the web interface via the onboard WiFi stack.

## Log

### Week 4-5

After going through multiple possible projects, finally landed on the basic idea of a smart parking system and started researching exactly what hardware components would be needed.

### Week 5-6

Expanded on the original idea by adding the web interface for online reservations.

### Week 7-8

Ordered all necessary components, currently waiting on their delivery.

## Hardware

The hardware is based on the Raspberry Pi Pico 2 W microcontroller. Six HC-SR04 ultrasonic sensors detect the presence of a vehicle in each parking spot. An SG90 servo motor acts as a barrier gate. An ST7735 1.8" TFT display shows real-time status over SPI. A 3x4 matrix keypad is used for PIN input, and six status LEDs provide instant visual feedback on spot occupancy.

### Schematics


### Bill of Materials

| Device | Usage | Price |
| ------ | ----- | ----- |
| [Raspberry Pi Pico 2 W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | 52.18 RON |
| [HC-SR04 Ultrasonic Sensor](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf) | Parking spot detection (x6) | 12.80 RON each |
| [SG90 Servo Motor](http://www.ee.ic.ac.uk/pcheung/teaching/DE1_EE/stores/sg90_datasheet.pdf) | Barrier control | 15.29 RON |
| [ST7735 TFT 1.8" 128x160 SPI Display](https://www.displayfuture.com/Display/datasheet/controller/ST7735.pdf) | Status display | 53.48 RON |
| [3x4 Matrix Keypad](https://www.optimusdigital.ro/en/keypads/470-4x4-matrix-keypad.html) | PIN input | 8.91 RON |
| Breadboard 830 points (x3) | Prototyping | 12.42 RON each |
| LED Kit 3mm 100pcs | Status indicators | 10.53 RON |
| Male-Male Jumper Wires 20cm (x40) | Connections | 17.93 RON |
| Jumper Wires | Connections | 16.15 RON |
| 330Ω Resistors Kit (200pcs) | Current limiting for LEDs | 32.41 RON |


## Software

| Library | Description | Usage |
| ------- | ----------- | ----- |
| [embassy-rp](https://github.com/embassy-rs/embassy) | Async embedded framework for RP2040 | Main async runtime and peripheral drivers |
| [embassy-net](https://github.com/embassy-rs/embassy) | Networking stack for embassy | WiFi and TCP/IP for the web interface |
| [st7735s](https://github.com/sajattack/st7735-lcd-rs) | TFT display driver | Driving the ST7735 display over SPI |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Drawing text and shapes to the display |
| [heapless](https://github.com/rust-embedded/heapless) | Static data structures | Used for fixed-size buffers without heap allocation |

## Links

1. [Embassy-rs documentation](https://embassy.dev)
2. [Raspberry Pi Pico W datasheet](https://datasheets.raspberrypi.com/picow/pico-w-datasheet.pdf)
3. [HC-SR04 datasheet](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf)
