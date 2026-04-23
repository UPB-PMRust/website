# PicoReflex F1 Edition
A reflex training game built on a Raspberry Pi Pico 2W, housed in an F1-inspired 3D-printed enclosure.

:::info 

**Author**: Sahnoun Taher \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-taherdaringa-maker

:::

## Description

PicoReflex F1 Edition is a reflex training game inspired by the reaction time demands of Formula 1 racing. The device uses a Raspberry Pi Pico 2W running async Rust with Embassy-rs. The player must react to a randomly triggered LED signal by pressing the corresponding button as fast as possible. The system measures reaction time in milliseconds, displays the result on an OLED screen, and provides audio feedback via a buzzer. Results are tracked across sessions and accessible over WiFi.

## Motivation

I chose this project because it combines all the core concepts covered in the MA course — GPIO, PWM, ADC, SPI, I2C, async Embassy, and WiFi — into a single functional device. The F1 theme reflects my personal interest in motorsport and gave me a creative direction for the enclosure design. Building a reflex trainer also has real practical value for anyone looking to measure and improve their reaction time.

## Architecture

The Raspberry Pi Pico 2W acts as the central controller, coordinating all peripherals:
 - **GPIO** — buttons for player input and LEDs for stimulus signals
 - **PWM** — buzzer for audio feedback on correct/incorrect reactions
 - **ADC** — potentiometer for difficulty/sensitivity adjustment
 - **SPI** — OLED display for showing reaction times and scores
 - **I2C** — MPU6500 sensor integration
 - **Async Embassy** — concurrent task handling for timers, input, display, and WiFi
 - **WiFi** — score reporting and remote leaderboard access

## Log

### Week 5 - 11 May

Decided on the project topic. Researched components needed for the Raspberry Pi Pico 2W reflex trainer. Planned the architecture and identified required libraries for Embassy-rs.

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The project is built around the Raspberry Pi Pico 2W (RP2350) which provides GPIO pins, SPI/I2C peripherals, ADC inputs, and onboard WiFi. An OLED display shows reaction scores. LEDs serve as stimulus signals and buttons capture player input. A buzzer provides audio feedback. A potentiometer allows difficulty adjustment. A MicroSD card stores session scores. Everything is housed in a custom F1-inspired 3D-printed enclosure.

### Schematics

Schematics in SVG format.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | borrowed |
| [Raspberry Pi Pico 2](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Debugging probe | [48.40 RON](https://www.emag.ro) |
| [OLED Display 0.96" I2C](https://www.sigmanortec.ro) | Displays reaction time and score | 16.96 RON |
| [MPU6500 Accelerometer Gyroscope I2C](https://www.sigmanortec.ro) | Motion sensor | 11.99 RON |
| [MicroSD Module](https://www.sigmanortec.ro) | Score storage | 4.38 RON |
| [Passive Buzzer 5V](https://www.sigmanortec.ro) | Audio feedback | 1.45 RON |
| [LED 5mm Red](https://www.sigmanortec.ro) | Stimulus signal | 0.30 RON |
| [LED 5mm Yellow](https://www.sigmanortec.ro) | Stimulus signal | 0.30 RON |
| [LED 5mm Green](https://www.sigmanortec.ro) | Stimulus signal | 0.30 RON |
| [LED 5mm Blue](https://www.sigmanortec.ro) | Stimulus signal | 0.30 RON |
| [Push Buttons 6x6x5 x5](https://www.sigmanortec.ro) | Player input | 1.80 RON |
| [Potentiometer RK097N 10K](https://www.sigmanortec.ro) | Difficulty adjustment | 3.03 RON |
| [Resistors 1K x20](https://www.sigmanortec.ro) | Current limiting for LEDs | 3.19 RON |
| [40 Dupont Wires 10cm](https://www.sigmanortec.ro) | Connections | 7.73 RON |
| [Breadboard 830 points](https://www.optimusdigital.ro) | Prototyping | 10.00 RON |
| [Micro USB Cable](https://www.optimusdigital.ro) | Power | 4.36 RON |
| [HAMA 124151 MicroSDHC 32GB Class 10](https://www.flanco.ro) | Score and data storage | 58.99 RON |
| **Total** | | **~176 RON** |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://crates.io/crates/embassy-rp) | Async HAL for RP2350 | Peripheral drivers for GPIO, PWM, ADC, SPI, I2C |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async task executor | Runs concurrent tasks for input, display, buzzer, WiFi |
| [embassy-time](https://crates.io/crates/embassy-time) | Async timers and delays | Measures reaction time in milliseconds |
| [embassy-sync](https://crates.io/crates/embassy-sync) | Channels and mutexes | Shares state between async tasks |
| [cyw43](https://crates.io/crates/cyw43) | WiFi driver for Pico 2W | Enables network connectivity for score reporting |
| [ssd1306](https://crates.io/crates/ssd1306) | OLED display driver | Shows reaction times and scores over I2C |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Renders text and graphics on the OLED display |

## Links

1. [Embassy-rs documentation](https://embassy.dev)
2. [Raspberry Pi Pico 2W datasheet](https://datasheets.raspberrypi.com/pico/pico-2-w-datasheet.pdf)
3. [SSD1306 OLED driver](https://crates.io/crates/ssd1306)
