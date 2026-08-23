# Reaction Arcade 

:::info
**Author:** Matei Zamfir \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/fils-project-2026-matinator1510-lgtm
:::

## Description

The project is an advanced, arcade-style reaction timer game built on the Raspberry Pi Pico 2 (RP2350) microcontroller. The device uses highly dynamic visual stimuli (a 20-LED WS2812B strip) and a heavy mechanical arcade button to evaluate how quickly a user can respond. The entire system is controlled via a single button using time-based presses (short tap to navigate, long hold to select/confirm). 

The game features multiple modes, including a "Classic Mode" for pure speed, and a "Hardcore Mode" that tests impulse control through random visual fake-outs. All menus, session data, and an arcade-style name entry system are displayed on a 16x2 I2C Character LCD. Top scores are serialized and saved persistently to the Pico 2's onboard Flash memory so they survive power cycles.

## Motivation

I chose this project to explore modern embedded systems using **Async Rust** via the **Embassy** framework. It combines real-time hardware input processing, non-blocking state machines, and microsecond-level hardware timers. It serves as a rigorous technical exercise in handling asynchronous tasks concurrently—such as smoothly animating PIO-driven LEDs while simultaneously awaiting debounced button interrupts and updating an I2C screen—without writing traditional blocking code.

## Architecture

The core of the system is the Raspberry Pi Pico 2 (RP2350), which manages the asynchronous application state machine (Menu > Game > Name Entry > Leaderboard).
* **Visual Output (UI):** A 16x2 Character LCD with a PCF8574T I2C backpack handles all text-based UI, menus, and exact millisecond readouts.
* **Visual Output (Stimulus):** A 20-LED WS2812B NeoPixel strip, driven completely in the background by the Pico 2's Programmable I/O (PIO) and DMA, acts as the countdown and visual game stimulus. 
* **User Input:** A single illuminated arcade push button. The software distinguishes between Short Presses (<500ms) and Long Presses (>500ms) with heavy debouncing to accommodate large microswitches.
* **Storage:** The system uses `embedded-storage-async` to erase and write serialized leaderboard data (`postcard`/`serde`) to a dedicated sector at the end of the RP2350's non-volatile Flash memory.

## Log

**Step 1:** Hardware planning and wiring the Pico 2 to the I2C LCD, WS2812B LED strip, and arcade button.
**Step 2:** Configuring the Embassy async runtime and writing the single-button, time-sensitive input driver.
**Step 3:** Implementing the hardware drivers (I2C display initialization and PIO WS2812B LED animations).
**Step 4:** Building the non-blocking state machine (Classic Mode, Hardcore Mode, and Anti-cheat/False-start detection).
**Step 5:** Integrating `serde` and `postcard` to serialize high scores and saving them to the RP2350's Flash memory.

## Hardware

The system uses a Raspberry Pi Pico 2 (RP2350) development board as the main microcontroller. Textual feedback is provided by a 16x2 LCD Display with an I2C backpack. Game stimuli are provided by a 20-LED WS2812B strip and the internal LED of the arcade button. User interaction is handled completely through one large arcade-style momentary push button. The system is powered via the Pico's USB connection (VBUS 5V) to support the power draw of the LEDs.

## Project Photo

[Insert photo of your physical project build here]

## Schematics

[Insert your KiCad schematic in SVG/PNG format here]

## Bill of Materials

### Hardware

| Device | Usage | Price |
| :--- | :--- | :--- |
| Raspberry Pi Pico 2 / Pico 2 W | The microcontroller (RP2350) | ~30.00 RON |
| 16x2 Character LCD (with I2C Backpack) | Display - menus, times, and leaderboard | ~25.00 RON |
| WS2812B LED Strip (20 LEDs) | Visual stimulus, countdowns, and game feedback | ~15.00 RON |
| Arcade Push Button (with LED) | User input - menu control and reaction response | ~15.00 RON |
| Resistors (e.g., 220Ω, 10kΩ) | Button LED current limiting & pull-up stabilization | ~2.00 RON |
| Breadboard & Wires | Prototyping and connections | ~15.00 RON |

### Software

| Library | Description | Usage |
| :--- | :--- | :--- |
| `embassy-rp` / `embassy-executor` | Async Hardware Abstraction Layer & Runtime | Manages RP2350 peripherals (GPIO, PIO, I2C, Flash) and async tasks without blocking the CPU. |
| `i2c-character-display` | LCD Driver | Formats and sends characters to the PCF8574T I2C backpack. |
| `smart-leds` / `PIO` | LED control traits | Sends precise timing signals via PIO to control the WS2812B color strip. |
| `embedded-storage-async` | Flash Memory Traits | Allows the game to read/write to the Pico 2's onboard flash safely. |
| `postcard` & `serde` | Serialization | Converts the Leaderboard struct into bytes so it can be saved to flash. |
| `heapless` | No-std data structures | Provides fixed-size Strings and arrays (no heap allocation required) for the arcade name entry system. |