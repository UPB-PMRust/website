# AWARE-GUIN
An interactive companion robot developed on STM32 using Async Rust (Embassy).
:::info 
**Author**: DASCALESCU ANDREI \
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
```
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

##Schematics

### Bill of Materials
| Device | Usage | Price |
|--------|--------|-------|
| [STM32U545RE Nucleo]| The main microcontroller running the Rust logic | [borrowed](#) |
| [SPI TFT Display (ST7789/ILI9341)](https://www.emag.ro/display-tft-lcd-3-2-inch-320x240-spi-driver-ili9341-fara-touch-compatibil-arduino-bmx654/pd/DV5CTY2BM/?ref=history-shopping_484726532_221614_1) | The screen used to render the telemetric data | [~70 RON](#) |
| [7-LED WS2812 Ring]([https://www.optimusdigital.ro/](https://www.drot.ro/platforma-arduino/7684-modul-led-rgb-7-x-neopixel-ws2812.html)) | Luminous feedback / Mood lighting | [~15 RON](#) |
| [TTP223](https://www.optimusdigital.ro/ro/senzori-senzori-de-atingere/861-modul-cu-senzor-capacitiv-ttp223.html?gad_source=1&gad_campaignid=19615979487&gbraid=0AAAAADv-p3DU4JKHkK5bfyzGYBRLoTdYx&gclid=CjwKCAjwqubPBhBOEiwAzgZX2iCeXwLEUxkYExgVLjRFnRTwgJz31Hll3oXMP39Iu_cHb7k0dCuHOxoCt7gQAvD_BwE) | Touch sensor for interaction | [2 RON](#) |
| [BME280](https://www.emag.ro/modul-senzor-temperatura-umiditate-presiune-bme280-ai0002-s34/pd/DR7HCZBBM/?ref=history-shopping_484726532_50435_1) | Telemetric sensor for temperature, humidity and pression| [~45 RON](#) |
| [I2C SPI DISPLAY 0.96" SH1106](https://www.emag.ro/display-oled-128-x-64-px-0-96-interfata-i2c-spi-sh1106-3-3-v-multicolor-5904162801497/pd/DJL6KLMBM/?ref=history-shopping_484726532_116388_1) | Here we have two displays that will be used as eyes for the AWARE-GUIN and they will display the emotions of it or different eyes if you interact. | [2x 32RON](#) |
| [Servomotor 90](https://www.optimusdigital.ro/ro/motoare-servomotoare/26-micro-servomotor-sg90.html?search_query=servomotoare+sg90&results=2) | They are supposed to move the arms if the Penguin it's angry. | [2x 14 RON](#) |

| Dupont Wires & Breadboard | Physical connections between components | [65 RON](#) |

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
