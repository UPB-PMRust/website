# STM32 Mini Piano

A playable mini piano with multiple modes built on the STM32 Nucleo-U545RE-Q.

:::info

**Author**: Oproiu Eduard  
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-OproiuEduard

:::

## Description

A fully functional mini piano built on the STM32 Nucleo-U545RE-Q microcontroller. The user can press one of 8 buttons, each mapped to a musical note (C4 to C5). The corresponding note is played through a passive buzzer via PWM, and a display shows the current note being played. The device supports multiple operating modes: free play, practice mode (press the notes shown on the display in time), learning mode (the device guides you through a song), octave shifting, and a record & replay feature. Performance data is sent to a PC via USB serial.

## Motivation

Music and embedded systems are two areas of personal interest. This project combines both by building a functional musical instrument from scratch using low-level hardware control in Rust. Adding interactive modes like practice and learning makes it more than just a toy — it becomes a tool for actually learning music.

## Architecture

The project is structured around the following main components:

- **Input Module** — 8 tactile buttons (one per note C4–C5) plus 2 extra buttons for octave shift and mode switching
- **Sound Module** — Passive buzzer driven by PWM; frequency is set according to the active note and octave
- **Display Module** — LCD display showing the current note, active mode, and prompts for practice/learning modes
- **USB Serial Module** — Sends note events and session performance data to a PC over USB
- **Mode Controller** — State machine managing transitions between Free Play, Practice, Learning, and Record/Replay modes
- **Record/Replay Module** — Stores a sequence of button presses with timestamps and replays them

```
[Buttons x8] ──► [Mode Controller] ──► [Sound Module (PWM Buzzer)]
[Mode Button]        │                ──► [Display Module (LCD)]
[Octave Button]      │                ──► [USB Serial (PC)]
                     └──► [Record/Replay Buffer]
```

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The project uses the STM32 Nucleo-U545RE-Q as the main microcontroller. Eight tactile 6x6x6 push buttons are wired to GPIO input pins with 220Ω pull-down resistors. A passive 3.3V buzzer is connected to a PWM-capable timer output pin. The lab LCD display is connected via I2C. Two additional buttons handle octave shifting and mode switching. The RGB LED on the Nucleo board provides visual feedback while a note is active.

### Schematics

<!-- Add KiCad schematic exported as SVG here -->

### Bill of Materials

| Device | Usage | Price |
| --- | --- | --- |
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main microcontroller | provided by lab |
| [Passive Buzzer 3.3V](https://www.optimusdigital.ro/en/buzzers/635-3-v-or-33v-passive-buzzer.html) | Play tones via PWM | [0.99 RON × 3 = 2.97 RON](https://www.optimusdigital.ro) |
| [Tactile Button 6x6x6](https://www.optimusdigital.ro/en/buttons-switches/97-6x6x6-push-button.html) | Note input (8) + mode/octave (2) | [0.36 RON × 16 = 5.76 RON](https://www.optimusdigital.ro) |
| LCD Display (lab) | Show note and mode info | provided by lab |
| Breadboard | Wiring components | provided by lab |
| Jumper wires | Connections | provided by lab |
| Resistors 220Ω | Current limiting for buttons | provided by lab |

## Software

| Library | Description | Usage |
| --- | --- | --- |
| [embassy-rs](https://github.com/embassy-rs/embassy) | Async embedded framework for Rust | Main framework for async tasks, GPIO, PWM, I2C, USB |
| [embassy-stm32](https://github.com/embassy-rs/embassy/tree/main/embassy-stm32) | STM32 HAL for Embassy | Hardware access for STM32 Nucleo-U545RE-Q |
| [embassy-usb](https://github.com/embassy-rs/embassy/tree/main/embassy-usb) | USB device stack for Embassy | USB serial (CDC ACM) for sending data to PC |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer traits | Standard traits for GPIO, PWM, I2C |
| [heapless](https://github.com/rust-embedded/heapless) | Static data structures (no heap) | Used for the record/replay note buffer |

## Links

1. [Embassy-rs documentation](https://embassy.dev)
2. [STM32 Nucleo-U545RE-Q datasheet](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)
3. [Microprocessor Architecture course website](https://embedded-rust-101.wyliodrin.com/docs/fils_en/project)
