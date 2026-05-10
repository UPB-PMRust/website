# IronBeats
A high-frequency rhythm game built on bare-metal Rust, featuring a 32-row LED "runway" and deterministic millisecond scoring.

:::info 

**Author**: Rares Zara \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-RaresZara

:::

<!-- do not delete the \ after your name -->

## Description

IronBeats is a 3-lane rhythm game (inspired by Piano Tiles and Guitar Hero) developed for the STM32F767ZI microcontroller. The game utilizes a 4-in-1 MAX7219 LED Matrix (8x32) as the primary game board where "notes" fall vertically. Players must interact with three physical buttons to catch notes within a strict timing window. A secondary 1.3" OLED displays the menu system, real-time score, and persistent leaderboard saved in the internal Flash memory. The project emphasizes deterministic latency and real-time concurrency using the Rust Embassy framework.

## Motivation

The choice of this project stems from a desire to explore Real-Time Systems and Hardware-Software Co-design. Developing a rhythm game requires solving critical engineering problems:

2. Deterministic Latency: Ensuring that button interrupts are processed with sub-millisecond precision.

1. Resource Management: Efficiently sharing a single SPI bus between two distinct display technologies (OLED and LED Matrix).

3. Rust in Embedded: Leveraging Rust's ownership model and the Embassy async executor to manage multiple concurrent tasks (audio, visuals, and logic) without a traditional Operating System.

## Architecture 

The system is centralized around the STM32F767ZI (Cortex-M7) which handles the high-speed game loop, user input debouncing, and peripheral data streaming.
[POWER & DEBUGGING SECTION]
      +-------------------+           +-------------------+
      |      Host PC      |           |      Host PC      |
      |   (Power Bank)    |           | (Serial Debugging)|
      +---------+---------+           +---------^---------+
                |                               |
                | (5V USB)                      | [UART / VCP]
                v                               v
      +---------------------------------------------------+
      |                                                   |
      |             NUCLEO-F767ZI (Cortex-M7)             |
      |              (Main Microcontroller)               |
      |                                                   |
      +--------+------------+------------+-----------+----+
               |            |            |           |
             [SPI]        [SPI]        [PWM]       [GPIO]
               |            |            |           |
               v            v            v           v
      +------------+  +------------+  +---------+  +------------+
      |  MAX7219   |  | 1.3" SH1106|  | 3x Pass.|  | 3x Buttons |
      |  8x32 LED  |  | OLED Disp. |  | Buzzers |  | 1x Joystick|
      |  Matrix    |  |            |  |         |  |            |
      +------------+  +------------+  +---------+  +------------+
       (Game Board)    (UI/Score)      (Audio)      (Controls)

## Log

<!-- write your progress here every week -->

### Week 1 - 7
Brainstormed the idea and consulted my professors regarding the project idea.
Hardware acquisition finalized (STM32, MAX7219, SH1106 OLED).

### Weeks 7-9
Developed the project logic and power scheme as well as implemented basic GPIO blinky and serial logging for debuggin every component involved in the project.

### Weeks 8-10

### Weeks 10-12

### Weeks 12-14

## Hardware

IronBeats is a bare-metal Rust rhythm game powered by an STM32F767ZI microcontroller, ensuring precise gameplay timing. An 8x32 LED matrix serves as the vertical runway for falling notes, while a 1.3" OLED handles menus and leaderboards. Three tactile buttons let players catch notes in specific lanes, with three passive buzzers providing real-time audio feedback. A joystick controls UI navigation, and a 5V power bank safely drives the entire hardware system

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32F767ZI Nucleo-144](https://www.optimusdigital.ro/ro/placi-stm/2304-nucleo-f767zi.html) | Main Controller (Cortex-M7) | 165 RON |
| [4-in-1 MAX7219 8x32 Matrix](https://www.optimusdigital.ro/ro/optoelectronica/20949-modul-cu-matrice-led-max7219.html) | Main 32-row game board | 45 RON |
| [Waveshare 1.3" OLED (SH1106)](https://www.drot.ro/display-oled-1-3-128x64-albastru-3-3-5v) | UI, Score, and Menu Display | 68 RON |
| [Passive Buzzer Module (3x)](https://www.optimusdigital.ro/ro/audio/18-buzzer-pasiv.html) | Tri-lane audio feedback | 15 RON |
| [Joystick Module](https://www.optimusdigital.ro/ro/input/10-modul-joystick.html) | Menu navigation | 12 RON |
| [Tactile Buttons (3x)](https://www.optimusdigital.ro/ro/input/11-buton-tactil.html) | Gameplay lane inputs | 3 RON |
| [1000uF Capacitor](https://www.optimusdigital.ro/ro/pasive/123-condensator-electrolitic-1000uf-25v.html) | Power smoothing for LED Matrix | 2 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for STM32 | Main framework for task management and peripheral control. |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D Graphics Library | Used for rendering the menu and score on the OLED. |
| [sh1106](https://crates.io/crates/sh1106) | SH1106 OLED Driver | Low-level driver for the 1.3" SPI display. |
| [max7219](https://crates.io/crates/max7219) | MAX7219 LED Driver | Handling the daisy-chained 8x32 LED matrices. |
| [embedded-storage](https://crates.io/crates/embedded-storage) | Flash storage abstraction | Used for persistent leaderboard management. |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [STM32F767ZI Reference Manual](https://www.st.com/resource/en/reference_manual/dm00224583-stm32f76xxx-and-stm32f77xxx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf)
2. [Rust Embedded Book](https://docs.rust-embedded.org/book)
3. [Embassy Async Framework Documentation](https://embassy.dev/book/dev/index.html)
4. [MAX7219 Daisy Chaining Logic Explained](https://datasheets.maximintegrAated.com/en/ds/MAX7219-MAX7221.pdf)


