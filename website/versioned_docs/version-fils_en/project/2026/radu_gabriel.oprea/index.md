# ESP32 Ukulele Tuner (Rust)
A real-time ukulele tuner using ESP32 and Rust with visual feedback on a TFT display.

:::info

**Author**: Oprea Radu - Gabriel \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-Radur12

:::

## Description

A simple real-time ukulele tuner that captures audio using a digital microphone, detects pitch, and displays tuning accuracy (flat, sharp, or correct) on a TFT screen.

## Motivation

This project was chosen to explore embedded systems programming with Rust, real-time audio processing, and hardware interfacing. It combines signal processing with a practical and interactive use case (musical instrument tuning), making it both educational and useful.

## Architecture

### Main components:

Audio Input Module (INMP441 microphone)
Signal Processing Module (pitch detection using FFT)
Control Unit (ESP32 microcontroller)
Output Interface (TFT display)

### Connections:

The microphone captures sound via I2S and sends it to the ESP32.
The ESP32 processes the signal to detect frequency (pitch detection).
The detected pitch is compared with standard ukulele tuning values (G4, C4, E4, A4).
Results are sent to the TFT display, showing whether the note is flat, sharp, or in tune.

### System Data Flow

![Diagram](./diagram.webp)

## Log

### Week 5 - 11 May

Initial project setup, component selection, and environment configuration for Rust on ESP32.

### Week 12 - 18 May

Implemented audio capture via I2S and basic signal processing (FFT).

### Week 19 - 25 May

Integrated display output and completed pitch detection logic with visual feedback.

## Hardware
ESP32 DevKit1 (ESP-WROOM-32)
INMP441 I2S Digital Microphone
1.3” TFT Display (ST7789, 240x240)

### Schematics

KiCAD schematics will be added here as soon as it's done.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [ESP32 DevKit V1](https://sigmanortec.ro/placa-dezvoltare-esp32-cu-wifi-si-bluetooth?SubmitCurrency=1&id_currency=2&srsltid=AfmBOoqsWrsP--au7FCZ8bSrXUsl5neEnXNTZ6X_o2jJOZFMJI9ANC0hGgE) | Main microcontroller | [42 RON](https://sigmanortec.ro/placa-dezvoltare-esp32-cu-wifi-si-bluetooth?SubmitCurrency=1&id_currency=2&srsltid=AfmBOoqsWrsP--au7FCZ8bSrXUsl5neEnXNTZ6X_o2jJOZFMJI9ANC0hGgE) |
| [INMP441 I2S Microphone](https://www.optimusdigital.ro/en/others/12548-inmp441-mems-high-precision-omnidirectional-microphone-module-i2s.html?srsltid=AfmBOoorwAe9erzXWIhca8h9l2gHLA-k6hmzWy75FjUOStzRLvaQ1qG6) | Audio input for pitch detection | [20 RON](https://www.optimusdigital.ro/en/others/12548-inmp441-mems-high-precision-omnidirectional-microphone-module-i2s.html?srsltid=AfmBOoorwAe9erzXWIhca8h9l2gHLA-k6hmzWy75FjUOStzRLvaQ1qG6) |
| [1.3” TFT Display (ST7789)](https://sigmanortec.ro/display-tft-13-ips-spi-65k-culori-lcd-st7789v-240x240-7p) | Displays tuning feedback | [31 RON](https://sigmanortec.ro/display-tft-13-ips-spi-65k-culori-lcd-st7789v-240x240-7p) |

---

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer | Used for interfacing with ESP32 peripherals |
| [rustfft](https://github.com/ejmahler/RustFFT) | FFT libraries | Used for pitch detection |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |
| [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) | I2S module | Used for audio input processing |
| [espup](https://github.com/esp-rs/espup) | ESP Rust tooling | Used for setting up and building the project |

---

## Links

1. [ESP32 INMP441 - audio](https://github.com/ZioTester/ESP32-MMB-MAX98357A-INMP441-audio)
2. [embedded-graphics examples](https://github.com/embedded-graphics/embedded-graphics/tree/master)
3. [rustfft documentation](https://github.com/ejmahler/RustFFT)