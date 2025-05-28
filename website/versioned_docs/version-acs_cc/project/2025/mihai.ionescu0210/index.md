# PicoSudoku
A microcontroller-powered Sudoku game, designed for the Raspberry Pi Pico 2

:::info

**Author**: Ionescu Mihai-Cosmin \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-COSSS02

:::

## Description

This project aims to provide a simple and engaging way to play the classic logic game Sudoku,
featuring three difficulty levels. The game grid is displayed on an LCD screen connected to a
Raspberry Pi Pico 2 microcontroller. The user can interact with the game using intuitive
hardware controls: a joystick for navigating between cells and a keypad for number input.
The system includes input validation and displays a warning inside the grid for invalid moves.
At the end of the game, the time needed to beat the puzzle is displayed.

[Video Demonstration](https://www.youtube.com/shorts/IdEYMHnrd7Y)

![Sudoku](images/Sudoku.webp)

## Motivation

This project combines my passion for logic puzzles with the challenge of creating a clean,
self-contained implementation. Sudoku's structured nature makes it an ideal candidate for
an embedded environment project.

## Architecture

![Diagram](images/Diagram.webp)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May
- Environment setup for picotool flashing
- USB logging for log messages instead of using a debugger
- Keypad initialization
- Keypad polling for input reading
- Joystick initialization
- Joystick polling for determining the direction when it's moved
- Multiplexed Input between the Keypad and the Joystick

### Week 12 - 18 May
- SPI bus initialisation
- Display initialisation
- Display comatibility with embedded-grpahics library

### Week 19 - 26 May
- Sudoku board generation
- Sudoku control using input peripherals
- Sudoku grid drawing
- Startup screen with difficulty selection
- Warnings inside grid for invalid moves
- Game over ending screen
- Timer for how long it takes to beat the level

## Hardware

The project uses four main hardware components:
- Raspberry Pi Pico 2 as the microcontroller
- 2.8-inch ST7789 SPI TFT LCD to display the Sudoku grid
- 3x4 matrix keypad for number input
- dual-axis XY joystick for navigating between cells.

![Hardware](images/Hardware.webp)

### Schematic

<!-- Place your KiCAD schematics here. -->
![Schematic](images/Schematic.svg)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico 2](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html) | The microcontroller | [42 RON](https://www.aliexpress.com/item/1005007660023339.html) |
| [2.8 Inch ST7789 LCD](https://www.buydisplay.com/download/ic/ST7789.pdf) | The display | [38 RON](https://www.aliexpress.com/item/1005006175220737.html) |
| [3X4 Matrix Switch Keypad](https://mm.digikey.com/Volume0/opasdata/d220001/medias/docus/794/3845_Web.pdf) | The keypad | [12 RON](https://www.aliexpress.com/item/4000873237364.html) |
| [Dual-axis XY Joystick Module](https://naylampmechatronics.com/img/cms/Datasheets/000036%20-%20datasheet%20KY-023-Joy-IT.pdf) | The joystick | [8 RON](https://www.aliexpress.com/item/1005007403082994.html) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy/tree/main/embassy-rp) | Raspberry Pi Pico HAL | GPIO, SPI, ADC setup |
| [embassy-sync](https://github.com/embassy-rs/embassy/tree/main/embassy-sync) | Synchronization primitives | Mutex for SPI bus |
| [embassy-time](https://github.com/embassy-rs/embassy/tree/main/embassy-time) | Time handling mechanisms | Measure Game Runtime |
| [embassy_usb_logger](https://github.com/embassy-rs/embassy/tree/main/embassy-usb-logger) | USB implementation of the log crate | Debugging & logging player actions |
| [mipidsi](https://github.com/almindor/mipidsi) | Crate for generic display drivers | Display driver for ST7789 |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Drawing primitives to the display |
| [picorand](https://github.com/inspier/picorand) | Fast random number generator library | Generating the Sudoku game board |
| [micromath](https://github.com/tarcieri/micromath) | Embedded-friendly math library | Common arithmetic operations |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Creating a Raspberry Pi Pico ST7789 based coloured text display](https://www.youtube.com/watch?v=T6y46CEDx2A)
2. [Pico Sudoku Puzzle Generator](https://forums.raspberrypi.com/viewtopic.php?t=317042)
3. [Driver for reading from standard 3X4 keypads](https://github.com/JohnSL/keypad2/blob/main/src/lib.rs)
