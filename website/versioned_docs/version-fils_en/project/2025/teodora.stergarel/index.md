# Noise-Triggered Visual Alert 

:::info 

**Author**: Stergarel Teodora \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-teodora-stergarel

:::

## Description

This project is an assistive device targeted towards deaf or hard-of-hearing users. It uses a microphone to detect specific keywords ("Hello”, "Attention”, "Help") and displays them on an OLED screen. Additionally, the system uses the OLED display to alert the user of loud noises detected in the vicinity and an LED strip to monitor the ambient noise level.

## Motivation

Many deaf or hard-of-hearing users are unable to detect environmental alerts that hearing people take for granted.  
This project aims to provide an accessible, real-time visual alert system that reacts to loud sounds and certain spoken keywords, increasing safety and awareness in everyday environments.

## Architecture 

- **Keyword Detection Module**: reads audio from the MEMS microphone via I2S, processes it to identify key phrases.
- **Noise Monitoring Task**: constantly samples ambient noise and detects peaks using RMS/thresholding.
- **Display Task**: uses `ssd1306` and `embedded-graphics` to show alerts and keyword matches on OLED.
- **LED Strip Controller**: visualizes ambient noise levels in real-time using a WS2812 LED strip.
- **Control Logic**: activated via a physical button, and manages task coordination.

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May
In the first week, I focused on thoroughly researching the project idea. I explored different options for implementing a visual noise alert system and evaluated the necessary hardware. I compared different microphones, displays, and microcontrollers to ensure compatibility with Rust and the RP2040 platform. After finalizing the hardware design, I placed the component orders and started drafting the documentation. I created the initial project page, describing the purpose, architecture, and goals of the system.
### Week 12 - 18 May
This week, the hardware components arrived, and I got the headers to the two Raspberry Pi Pico 2W boards soldered - one designated as the main board and the other as a debugger. I encountered challenges in setting up the debug probe correctly but resolved them by following the probe-rs documentation and ensuring the correct GND/VCC wiring between boards. I proceeded to wire all the components onto the breadboard, carefully verifying connections for the microphone, button, display, and logic level shifter. I also designed and completed a detailed schematic in KiCad, which reflects the real wiring. Finally, I tested each component individually - confirming that the button and OLED display functioned - and determined that the OLED required a level shifter for stable operation.
### Week 19 - 25 May
This week, I wrote the embedded Rust logic to read analog values from the DUFAFF microphone using the embassy_rp::adc driver on the main Pico board. I implemented a polling loop that samples the ADC and maps the amplitude to LED color states (green/yellow/red) based on tunable thresholds. For sudden peaks, I trigger an OLED message using the ssd1306 crate over I2C, displaying "loud noise detected". The logic is gated by a push button using an async task with embassy_time::Timer and embassy_sync::signal::Signal for coordination.
I am currently working on the implementation of keyword detection ("hello", "attention", "sorry").
## Hardware

![1:](./1.webp)
![2:](./2.webp)

The project is built around the **Raspberry Pi Pico W** microcontroller and integrates the following components:

- **INMP441 MEMS Microphone** (I2S): Used to detect ambient sounds and recognize keywords like “Hello”, “Attention”, and “Help”.
- **SSD1306 OLED Display** (0.96", I2C): Displays alerts and recognized keywords in real time.
- **WS2812 LED Strip**: Provides a visual representation of ambient noise intensity.
- **Push Button**: Used to activate or reset listening mode.
- **Logic Level Converter**: Ensures voltage compatibility between the Pico and the 5V LED strip.
- **Capacitor (1000μF, 25V)**: Used for LED power stabilization.
- **Wires and Breadboard**: For prototyping and connecting components.

### Schematics

Below is the schematics of the device:

![Project Schematics:](./architecture.webp)

KiCad Schematics:

![KiCad Schematics:](./rust.svg)


### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Modul microfon omnidirectional](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microphone | [20 RON](https://www.bitmi.ro/modul-microfon-omnidirectional-interfata-i2s-mems-inmp441-11003.html?gad_source=1&gclid=Cj0KCQjwqIm_BhDnARIsAKBYcmuBUV_YFqcN24x17xXztcQ3M9MRrACTFx02nfLMn8JOe7VPRAFTBYsaAuFZEALw_wcB) |
| [Ecran OLED](https://www.bitmi.ro/componente-electronice/ecran-oled-0-96-cu-interfata-iic-i2c-10488.html) | The display | [19 RON](https://www.bitmi.ro/componente-electronice/ecran-oled-0-96-cu-interfata-iic-i2c-10488.html) |
| [Modul LED](https://www.bitmi.ro/module-electronice/modul-led-semafor-compatibil-arduino-10405.html) | The LEDs | [3 RON](https://www.bitmi.ro/module-electronice/modul-led-semafor-compatibil-arduino-10405.html) |
| [Convertor de nivel logic](https://www.optimusdigital.ro/ro/interfata-convertoare-de-niveluri/12562-convertor-de-nivel-logic-cu-2-canale-33v-5v-ttl.html
) | The logic convertor for the microphone | [3 RON](https://www.bitmi.ro/componente-electronice/ecran-oled-0-96-cu-interfata-iic-i2c-10488.html) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [ssd1306](https://crates.io/crates/ssd1306) | Display driver for SSD1306 OLED displays | Used to control the 0.96" I2C OLED screen |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library for embedded systems | Used to render text, symbols and visuals on the display |
| [embassy](https://github.com/embassy-rs/embassy) | Async runtime for embedded Rust | Manages non-blocking tasks for I2S, display, button, etc. |
| [micromath](https://crates.io/crates/micromath) | Math functions | Used to calculate pitch/roll from accelerometer data |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer for microcontrollers | Base traits used by drivers like `ssd1306` and I2S mic |
| [rp-pico](https://crates.io/crates/rp-pico) | Support crate for Raspberry Pi Pico | Provides HAL and board pin mappings |
| [i2s](https://github.com/ryankurte/rust-i2s) | I2S audio support | Reads audio data from the MEMS INMP441 microphone |