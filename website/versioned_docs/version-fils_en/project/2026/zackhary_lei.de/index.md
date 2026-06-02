# Steady-Hand Maze Challenge
A challenge of skill: navigate a metal loop through a wire maze without touching the wire with high-score tracking and profile management.



**Author**: Zackhary Lei Juanes De Vega \
**GitHub Project Link**: [(https://github.com/ZackharyLei)]

## Description

The project is an interactive,"Steady-Hand Maze" game. A metal loop must travel along a bent copper wire track without touching it. If the loop touches the wire, an active buzzer sounds. The project features an OLED display for menu navigation and to show remaining lives and a timer, a rotary encoder for profile selection/game control, and an SD card module to save high scores and player profiles. The firmware is written in Rust, utilizing the Embassy framework for efficient multitasking and handling of game logic, timing, and collision detection.


## Motivation

I chose this project to gain hands-on experience with embedded systems development in Rust. It provides a perfect balance between hardware interaction (interrupts, GPIO, SPI/I2C protocols) and software logic (state machines, file system management on SD cards, and graphics rendering). It serves as a practical application of real-time sensor processing and user interface design.

## Architecture 

The system relies on the STM32 Nucleo as the central processor.
 - **Input System**: A Rotary Encoder captures user navigation (for menus) and a touch-sensor (the copper wire) captures game events via GPIO interrupts.
 - **Control Logic**: The main loop manages the game state (Menu, Playing, GameOver, ScoreBoard).
 - **Output System**: The OLED display provides visual feedback (UI/Graphics), and the Active Buzzer provides auditory feedback.
 - **Storage System**: The SD Card module stores user profiles and persistent high-score data via SPI.

## Log

### Week 5 - 11 May
- Project planning, component sourcing, and setting up the Rust toolchain for STM32.

### Week 12 - 18 May


### Week 19 - 25 May


## Hardware

The project uses an STM32 Nucleo board as the microcontroller. User interaction is handled by a KY-040 rotary encoder and a custom-bent copper wire track. Feedback is provided via a 0.96" OLED display and an active 5V buzzer. All data is saved on a microSD card via an SPI module.

### Schematics

*(To be uploaded upon completion of the circuit diagram)*

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo](https://www.st.com/en/evaluation-tools/stm32-nucleo-64.html) | The microcontroller | 0 RON (Borrowed) |
| [OLED Display 128x64 I2C](https://www.optimusdigital.ro/) | UI and game feedback | 19 RON |
| [MicroSD Card Module SPI](https://www.optimusdigital.ro/) | SPI interface board | 10 RON |
| [MicroSD Card](https://www.optimusdigital.ro/) | Data storage | 18 RON |
| [KY-040 Rotary Encoder](https://www.optimusdigital.ro/) | Menu navigation | 8 RON |
| [Active Buzzer 5V](https://www.optimusdigital.ro/) | Game fail alarm | 2 RON |
| [Breadboard](https://www.optimusdigital.ro/) | Prototyping circuit connections | 20 RON |
| [Jumper Wires](https://www.optimusdigital.ro/) | Connecting components to Nucleo | 5 RON |
| [220-ohm Resistors](https://www.optimusdigital.ro/) | Current limiting for buzzer | 2 RON |
| [Copper Wire (Unifilar)](https://www.dedeman.ro/) | The maze track | 15 RON |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal) | Hardware Abstraction Layer | Pin/Clock/Bus management |
| [ssd1306](https://github.com/jamwaffles/ssd1306) | OLED Display driver | Drawing the UI and game text |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Shapes and fonts for the screen |
| [embedded-sdmmc](https://github.com/rust-embedded-community/embedded-sdmmc-rs) | SD Card FAT32 driver | Saving and loading scores |

## Links

1. [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
2. [STM32 Nucleo Documentation](https://www.st.com/en/evaluation-tools/stm32-nucleo-64.html)
3. [SSD1306 Display Crate Docs](https://docs.rs/ssd1306/latest/ssd1306/)