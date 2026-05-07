# Reflex Arena: Multi-Sensory Reaction Game

A smart reaction time testing system that uses multi-sensory feedback (visual via OLED and audio via Buzzer) to measure user reflexes in precise milliseconds.

:::info

**Author**: Dalbuz Heorhii \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-Heorhii78

:::

## Description

The Reflex Arena is built around an STM32 Nucleo microcontroller that orchestrates multiple peripherals to test human reaction speeds. Instead of relying on a basic LED, the system utilizes a 0.96" OLED screen to display the user interface, instructions, and the final exact time. A randomized timer dictates when the stimulus occurs to prevent user anticipation. Once the visual cue appears on the screen and the audio cue plays through the Piezo Buzzer, a hardware timer begins counting. The user must press the corresponding push button as fast as possible to stop the timer, and the result is displayed on the screen.

## Motivation

I chose this project to implement the core concepts learned during the labs in a fun and interactive way. This project serves as a practical exercise in asynchronous task management, hardware timers, and peripheral interfacing via I2C and GPIO. My goal was to move beyond simple LED manipulation and understand how to orchestrate a complete embedded system with an actual visual interface and real-time user input. 

## Architecture

The STM32 interfaces with the components using the following protocols:

- **I2C Bus**: Drives the 0.96" OLED Display for rendering text, countdowns, and reaction time metrics.
- **GPIO (Digital Output)**: Controls the Piezo Buzzer to provide immediate audio stimulus.
- **GPIO (Digital Input)**: Reads the state of the push buttons, utilizing debouncing techniques to ensure accurate input registration.
- **Hardware Timers**: Used for generating the randomized delay and accurately measuring the reaction time in milliseconds.

**Component Connections:**

| Component | Interface | STM32 Pins |
|-----------|-----------|------------|
| 0.96" OLED Display | I2C | SDA / SCL (TBD) |
| Piezo Buzzer | GPIO | Digital Output (TBD) |
| Push Buttons | GPIO | Digital Inputs (TBD) |

## Log

### Week 1-4

I started doing research on potential project ideas. The initial idea was just using LEDs, but I decided to upgrade the concept to include an OLED display and audio feedback to make it more complex and engaging.

### Week 5-8

Started gathering the hardware components. Set up the Rust development environment and began looking into the documentation for the `ssd1306` display driver. Designed the basic state machine logic for the game (Idle -> Randomized Wait -> Trigger -> Result).

### Week 9

Received the remaining components. Verified the pinout compatibility between the Nucleo-U545RE-Q and the OLED display. Started the initial hardware assembly on the breadboard and tested basic I2C communication with the screen.

## Hardware

The core of the project is the STM32 Nucleo-U545RE-Q board. The visual interface is handled by a 0.96" I2C OLED display (SSD1306). Audio feedback is provided by a standard active piezo buzzer. Manual inputs are handled by tactile push buttons connected with pull-down resistors. The circuit is prototyped on a 400-point breadboard using standard Dupont jumper wires.

### Bill of Materials

| Device | Usage | Estimated Price |
|--------|--------|-------|
| STM32 Nucleo-U545RE-Q | The main microcontroller | Already got |
| 0.96" SSD1306 OLED Display (I2C) | Display interface and timer output | 19.00 RON |
| Active Piezo Buzzer | Audio stimulus | 5.00 RON |
| Push Buttons (Tactile) | User input | 2.00 RON |
| 10kΩ Resistors | Pull-down for buttons | 1.00 RON |
| Breadboard (400 points) | Prototyping | 20.00 RON |
| Jumper Wires (M-M, M-F) | Connections | 15.00 RON |

## Software

| Library / Interface | Description | Usage |
|---------------------|-------------|-------|
| `embassy-stm32` / `stm32-hal` | Hardware Abstraction Layer | I2C (OLED), GPIO (Buzzer, Buttons), Timers |
| `ssd1306` | OLED display driver | Initialization and framebuffer management for the display |
| `embedded-graphics` | 2D graphics library | Drawing text and UI elements on the OLED |
| Hardware RNG | Random Number Generator | Generating random delay times before the stimulus |

## Links

1. [Embedded Rust 101 course labs](https://embedded-rust-101.wyliodrin.com/docs/fils_en/category/lab)
2. [SSD1306 Crate Documentation](https://docs.rs/ssd1306/latest/ssd1306/)
3. [Embedded Graphics Library](https://docs.rs/embedded-graphics/latest/embedded_graphics/)
