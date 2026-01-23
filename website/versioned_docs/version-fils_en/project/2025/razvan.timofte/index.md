# ColorPicky — The Rust Real-Life Color Picker

A compact, real-time color picking tool built in **Rust** using **Embassy** on the **RP2350 (Raspberry Pi Pico 2W)**. It samples colors from real-world objects with a **TCS34725** sensor, converts them to **RGB/HEX**, assigns a human-readable color name, and stores a small history of saved colors.

## Features
- Live color sampling (RGBC → RGB/HEX)
- Nearest-match color naming
- Two-screen UI: **Main** (live preview) and **History** (last 10 saved colors)
- Button gestures:
  - **Hold**: live sampling
  - **Release / Tap**: save color
  - **Double tap**: switch Main/History
  - **Long press (History)**: clear history

## Motivation
This project was inspired by practical use-cases in graphic design, photo/video color grading, and UI branding, where it is useful to capture and reuse colors from physical objects. The second goal was to gain hands-on experience with embedded Rust on RP2350 using Embassy, integrating multiple peripherals (I2C + SPI + GPIO) in a responsive application.

## Hardware
- **MCU**: Raspberry Pi Pico 2W (RP2350)
- **Sensor (I2C)**: TCS34725
- **Display (SPI)**: SSD1283A 130×130
- **Input**: push button

## Software (high-level)
- `embassy-rp`, `embassy-executor` (async runtime)
- `embedded-graphics` (UI)
- `defmt` (logging)

Firmware is organized around three responsibilities: periodic sensor sampling, UI rendering, and button input/gesture handling, with a shared application state containing the current color and a ring-buffer history.

## Build & Flash
Prerequisites: Rust + `probe-rs`.

```bash
cargo run --release

## Future work
- Add calibration and ambient-light compensation (gain/integration time configuration)
- Improve naming accuracy using perceptual distance (e.g., CIE Lab / ΔE) and a larger palette
- Design a compact enclosure (3D printed casing)
- Add a settings menu (sampling rate, integration time, brightness)

Author: Razvan Andrei Timofte — Group 1241EC (IoT)
