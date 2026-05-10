# Theremin Sound Station

A touchless audio station that equalizes and controls music tracks using hand gestures and distance sensing.

:::info
**Author**: Mitrea Briana-Ștefania \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-briana1012
:::

## Description

The **Theremin Sound Station** is an embedded audio system that allows users modify the basic audio equiliser parameters without touching it. By combining Time-of-Flight (ToF) distance sensing and gesture recognition, the device acts as a digital equaliser and controller. The right hand selects which part of the frequency spectrum to modify via gestures (left, right etc.), while the left hand's height (distance) determines the intensity of the change. 

The system reads `.wav` files from a microSD card, decodes them via an I2S DAC, and provides visual feedback through an LCD display and a 16-LED Neopixel ring. It also transmites the signal changes through a usb to a computer in order to display the wave.

## Motivation

As a 2nd-year student passionate about music and sound engineering, I wanted to combine my interest in unique musical instruments (like the Theremin) with embedded systems. I also want to be a sound engineer or design sound systems once I finish university, because I'm extreamly passionate about signal processing and sounds.

## Architecture

The project is built on the STM32 Nucleo U545RE-Q using the Embassy asynchronous framework.

## Log

### Week 30 March - 5 April
* Initial project proposal and research into Rust embedded ecosystem.
* Acquired the STM32 Nucleo U545RE-Q board.
* Set up the Rust toolchain (probe-rs, thumbv8m.main-none-eabihf).
* Bought required hardware.

### Week 13 - 19 April 
* Soldered extension pins to the sensors.
* Reseached SAI pins.

## Hardware

The project uses a modular design with three "Sound Cubes" to separate sensors from the main control unit, preventing interference.

### Schematics

The project was designed in Altium Designer.

### Bill of Materials

| Device | Usage | Approx. Price |
| --- | --- | --- |
| **STM32 Nucleo U545RE-Q** | Main Microcontroller (Cortex-M33) | 120 RON |
| **UDA1334A DAC** | I2S Audio Decoder | 27 RON |
| **VL53L0X Sensor** | Time-of-Flight Distance Sensing | 45 RON |
| **APDS-9960 Sensor** | Gesture and Proximity Detection | 14 RON |
| **ST7735 tft** | 128x160 Track Information Display and storage for music tracks | 48 RON |
| **16 Neopixel Ring** | Visual feedback/Equalizer levels | 20 RON |
| **JBL Go** | Final audio output (AUX) | - |

## Software

The project is written entirely in Rust using the `no_std` environment for maximum performance and memory safety.

| Library | Description | Usage |
| --- | --- | --- |
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for STM32 | Main framework for task management |
| [vl53l0x](https://crates.io/crates/vl53l0x) | ToF Driver | Reading hand height for EQ |
| [apds9960](https://crates.io/crates/apds9960) | Gesture Driver | Track and menu control |
| [embedded-sdmmc](https://crates.io/crates/embedded-sdmmc) | FAT/SDMMC stack | Reading .wav files from SD |
| [smart-leds](https://crates.io/crates/smart-leds) | WS2812B Driver | Controlling the Neopixel ring |

## Links
1. [Embassy Documentation](https://embassy.dev/book/dev/index.html)
2. [STM32U5 Reference Manual](https://www.st.com/resource/en/reference_manual/rm0456-stm32u5-series-32bit-arm-based-mcus-stmicroelectronics.pdf)
3. [Audio equilizer reference](https://www.youtube.com/watch?v=4o-_gUht_Xc)
