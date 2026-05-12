# AWARE-GUIN
An interactive companion robot developed on STM32 using Async Rust (Embassy).
:::info 
**Author**: [DASCALESCU ANDREI] \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/fils-project-2026-yvcc28s62s-cmd]
:::
## Description
AWARE-GUIN is an intelligent desktop companion designed to react to its environment through visual expressions and light feedback. The "brain" of the project is a state-of-the-art STM32U5 microcontroller, programmed exclusively in Rust. The system runs on the `Embassy` asynchronous framework, concurrently managing an SPI display for the face/UI, a smart LED ring (NeoPixel) for expressing moods, and I2C sensors for gathering environmental data.

## Motivation
The primary motivation for this project was to transition from classic, sequential embedded programming (C/C++) to the modern, memory-safe, and asynchronous ecosystem provided by Rust. AWARE-GUIN served as the perfect sandbox to explore advanced concepts such as cooperative multitasking on bare-metal hardware, direct hardware abstraction layer (HAL) manipulation, and real-world debugging of communication protocols (high-speed SPI and I2C).

## Architecture 
```text
               [ DATA & TIME ]                          [ VISUAL DISPLAY ]
                .-------------.                         .---------------.
                |  RTC Module |                         | Displays (SPI)|
                |  Real-Time  |                         | Multiple TFT/ |
                '-------------'                         | OLED          |
                      ^                                 '---------------'
                      | (I2C)                                   ^
                      v                                         | (SPI 1 & 2)
.-------------.                 .-------------------.           v     .-------------.
| HC-05 (UART)|                 |                   |                 | WS2812 Ring |
|  Bluetooth  |<---(RX/TX)----->|  AWARE-GUIN Core  |<----(SPI @3MHz)>|  RGB LEDs   |
| Comm Module |                 | (STM32 Nucleo-U5) |                 | Expressions |
'-------------'                 |                   |                 '-------------'
                                '-------------------'
                      ^                                         ^
                      | (I2C / ADC)                             | (PWM Signal)
                      v                                         v
                .-------------.                         .---------------.
                |  Multiple   |                         | Servo Motors  |
                |   Sensors   |                         |  Mechanical   |
                |  (Env/IR)   |                         |  Actuation    |
                '-------------'                         '---------------'
                [ PERCEPTION ]                             [ ACTION ]

## Log
### Week 5 - 11 May
* Researched STM32U5 documentation and set up the Rust embedded toolchain (`probe-rs`, `defmt`).
* Configured the Embassy async framework and successfully ran an async Blinky task.
* Conducted initial SPI communication tests and hardware initialization for the TFT display.

### Week 12 - 18 May
* Performed advanced hardware debugging on the SPI bus (resolving "white screen" issues and adjusting clock polarity - MODE_0 / MODE_3).
* Successfully integrated the `mipidsi` and `embedded-graphics` crates.
* Controlled the WS2812 LED ring via SPI using the `smart-leds` crate and generated non-blocking animations.

### Week 19 - 25 May
* Wired and tested the I2C environmental sensors.
* Unified all components (Display, LEDs, Sensors) into a single asynchronous `main` execution loop.
* Optimized the codebase for the `release` profile and finalized the hardware schematics.

## Hardware
The system relies on an STM32 Nucleo board as the central processing unit, expanded with peripheral modules connected directly to the exposed header pins.

### Schematics
![KiCAD schematic](./Circuit.svg)

### Bill of Materials
| Device | Usage | Price |
|--------|--------|-------|
| [STM32U545RE Nucleo](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The main microcontroller running the Rust logic | [~110 RON](#) |
| [SPI TFT Display (ST7789/ILI9341)](https://www.optimusdigital.ro/) | The screen used to render the robot's face | [~40 RON](#) |
| [7-LED WS2812 Ring](https://www.optimusdigital.ro/) | Luminous feedback / Mood lighting | [~15 RON](#) |
| [I2C Sensor Module](https://www.optimusdigital.ro/) | Gathering environmental data | [~20 RON](#) |
| Dupont Wires & Breadboard | Physical connections between components | [~25 RON](#) |

## Software
| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for the STM32 family | Configuring system clocks, I2C, SPI, and GPIO pins. |
| [mipidsi](https://github.com/almindor/mipidsi) | Universal display driver | Low-level communication and hardware initialization for the screen controller. |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Hardware-agnostic rendering of text, geometries, and expressions on the screen. |
| [ws2812-spi](https://github.com/smart-leds-rs/ws2812-spi) | WS2812 driver | Driving the smart LED ring directly through SPI pins. |
| [defmt](https://defmt.ferrous-systems.com/) | Ultra-fast logging framework | Printing system logs (Info, Debug, Error) via the debugging probe interface. |

## Links
1. [Embassy Book - Official Documentation for Rust Async Embedded](https://embassy.dev/book/dev/index.html)
2. [Embedded Graphics - Visual guide and examples](https://docs.rs/embedded-graphics/latest/embedded_graphics/)
3. [Rust on STM32 - The Embedded Rust Book](https://docs.rust-embedded.org/book/)