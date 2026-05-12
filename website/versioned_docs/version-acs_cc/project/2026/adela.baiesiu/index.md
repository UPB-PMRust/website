# Piano Trainer
A wireless piano learning assistant with falling notes, physical keys, and real-time scoring.

:::info 

**Author**: Adela-Mihaela Băieșiu \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-Adela717.git

:::

<!-- do not delete the \ after your name -->

## Description

Piano Trainer is an embedded piano learning system built around a wireless keyboard unit and a main display unit.

The project is inspired by piano tutorial videos where colored notes fall toward the piano keys. The user has a 13-key piano keyboard covering one chromatic octave, from C to the next C. The keyboard unit detects key presses and sends them wirelessly using Bluetooth.

The main unit, built around the STM32 Nucleo-U545RE-Q development board, receives the key events, displays the falling notes on a TFT screen, generates audio feedback using a passive buzzer, and computes gameplay statistics.

The device includes a menu system for choosing predefined songs, a gameplay screen with falling notes, a results screen, and a best scores screen.

An additional feature of the project is a recording mode. The user can record a short custom melody using the physical piano keys. The system stores the played notes together with their timing and duration. After recording, the melody can be replayed or used as a custom practice song, where the recorded notes are displayed as falling notes and the user can try to play them back accurately.

## Motivation

I chose this project because I learned piano using online videos where colored notes move toward the keys and show when and for how long each note should be played. I wanted to recreate this learning method as a physical embedded device.

## Architecture 

- **Piano input module**: reads the 13 physical piano keys corresponding to one complete octave.
- **Menu input module**: reads the control buttons used for navigating the interface: UP, DOWN, SELECT, and BACK/PAUSE.
- **Display module**: renders the start menu, song selection screen, gameplay screen, falling notes, results screen, settings screen, and best scores screen.
- **Song engine**: stores each song as a sequence of note events. Each event contains the note, the expected start time, and the note duration.
- **Audio module**: generates musical tones using a passive buzzer controlled through PWM.
- **Scoring module**: compares the expected notes with the user input and computes the final score, accuracy, number of correct notes, missed notes, and maximum combo.
- **Best scores module**: keeps track of the best scores achieved during the current runtime session.
- **Settings module**: allows the user to enable or disable sound feedback.
- **Application state machine**: controls the transitions between the main menu, song selection, gameplay, results, settings, and best scores screens.
- **Wireless keyboard module**: reads the physical piano keys and menu buttons on a separate keyboard unit.
- **Bluetooth communication module**: sends key press, key release, and menu button events from the keyboard unit to the main display unit.
- **Recording module**: records a short custom melody played by the user, storing each note together with its timing and duration.
- **Playback module**: replays the recorded melody or converts it into a custom practice song with falling notes.
![Architecture diagram](./architecture.svg)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

- Chose the initial project idea and discussed possible approaches.
- Defined the main functionality: physical piano keys, falling notes on a display, audio feedback, song selection, and scoring.
- Started researching suitable components for the project.

### Week 12 - 18 May

- Selected the main hardware components.

### Week 19 - 25 May

- Updated the architecture to include Bluetooth communication between the keyboard unit and the main display unit.
- Refined the bill of materials and software module structure.

## Hardware

The project is built as two separate hardware units: a wireless keyboard unit and a main display unit.

The wireless keyboard unit contains the physical piano keys and the menu control buttons. It is responsible for detecting user input and sending key events to the main unit through Bluetooth. The keyboard contains 13 physical keys, covering one octave from C to the next C.

The main display unit contains the STM32 Nucleo-U545RE-Q development board, the TFT display, and the passive buzzer. It receives input events from the wireless keyboard unit, updates the game state, displays the falling notes, generates sound, and computes the final statistics.

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
| STM32 Nucleo-U545RE-Q | Main microcontroller board used to run the Rust firmware and control all peripherals | Provided |
| Raspberry Pi Pico 2 | Secondary microcontroller used in the wireless keyboard unit to read the piano keys and menu buttons and forward events to the Bluetooth UART module | TODO |
| Bluetooth UART Module x2 | Used for wireless communication between the keyboard unit and the main display unit | TODO |
| TFT SPI Display ST7789 2.8 inch 320x240 | Used to display the main menu, song selection screen, falling notes, results, settings, and best scores | 58.99 RON |
| MCP23017 I/O Expander | Used by the wireless keyboard unit to read most of the piano keys and menu buttons through I2C, reducing the number of GPIO pins required | 7.77 RON |
| 12x12mm Tactile Push Buttons x 18 | Used as physical piano keys and menu control buttons | 1.52 RON |
| Passive Buzzer Module 3.3V-5V | Used to generate musical tones through PWM | 6.44 RON |
| Breadboard | Used as the prototyping area for connecting the modules and components | 9.89 RON |
| Male-Male Jumper Wires | Used for breadboard connections between components | 10.3 RON |
| Female-Male Jumper Wires | Used for connecting modules with pins to the breadboard or Nucleo board | 8.66 RON |
| Female-Female Jumper Wires | Used for direct connections between modules with male header pins | 6.54 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| embassy-rs | Embedded async framework for Rust | Used for running the firmware on the STM32 Nucleo-U545RE-Q board |
| embassy-stm32 | STM32 hardware abstraction layer for Embassy | Used for SPI, UART, timers, PWM, and GPIO on the main display unit |
| embedded-hal | Common embedded hardware abstraction traits | Used as a common interface for hardware drivers |
| embedded-graphics | 2D graphics library for embedded displays | Used for drawing menus, falling notes, score screens, and UI elements |
| st7789 | Display driver for ST7789 TFT displays | Used to control the SPI TFT display |
| heapless | Fixed-capacity data structures for embedded systems | Used for storing songs, notes, menu items, and scoring data without dynamic allocation |
| embassy-rp | Raspberry Pi RP2040/RP2350 hardware abstraction layer for Embassy | Used for GPIO and I2C on the Raspberry Pi Pico 2 keyboard unit |
| Bluetooth communication library / driver | Bluetooth support for the wireless keyboard unit | Used for sending key press and key release events from the keyboard unit to the main display unit |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

...
