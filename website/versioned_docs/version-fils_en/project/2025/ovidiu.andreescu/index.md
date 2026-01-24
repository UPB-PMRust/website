# STM32 Retro Console
A portable gaming console in Rust, featuring a custom BIOS, a virtual machine emulating CHIP-8 ISA, and a game library loaded from an SD card.

:::info

**Author**: Andreescu Ovidiu-Ștefan \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-dm-2025-ovidiu-andreescu

:::

## Description

The STM32 Retro Console is a handheld embedded system designed to run classic retro games on resource-constrained hardware. It is built around the STM32F767ZI microcontroller and implements a complete CHIP-8 virtual machine capable of executing game logic, handling input, graphics, and sound in real time.

The system boots into a custom BIOS shell that initializes all peripherals and provides a simple menu interface for selecting game ROMs stored on an SD card. The BIOS manages SD card access, on-screen navigation, and basic user feedback before transferring control to the virtual machine.

Game graphics are rendered on a 128×64 OLED display via I²C, user input is handled through a matrix keypad connected to GPIO pins, and sound effects are generated using a PWM-controlled buzzer.
## Motivation

I chose this project to validate the full embedded development stack in Rust, from hardware bring-up to application logic. Specifically, I wanted to:
* Use a virtual machine (CHIP-8) to understand low-level instruction parsing.
* Create a complete system that boots independently and manages a file system, rather than running a single hardcoded script.
* Demonstrate interactivity by integrating display drivers, audio feedback, and matrix keypad input into a single portable device.

## Architecture

The system architecture is centered around the **STM32F767ZI** microcontroller, which acts as the host for the CHIP-8 interpreter.

**Main Components & Connections:**
1.  **Core System (STM32):** Handles the fetch-decode-execute cycle of the VM and manages peripheral timing.
2.  **Storage Subsystem:** An **SD Card Module** connects via SPI to provide a dynamic file system. The BIOS reads this to list and load `.ch8` ROM files into RAM.
3.  **Human Interface (Input):** A **$4\times4$ Matrix Keypad** is scanned via GPIO to map physical button presses to the 16-key CHIP-8 layout.
4.  **Human Interface (Output):**
  * **OLED Display:** Connects via I2C/SPI to render the $64\times32$ monochrome graphics buffer.
  * **Passive Buzzer:** Connects via PWM/GPIO to generate simple square wave tones for game sound.

![Architecture Block Diagram](./architecture.webp)

## Log

### Week - 15 December - 21 December
Focused on hardware bring-up and initial validation. Connected and verified all peripheral 
components (STM32F767ZI, OLED display, keypad, SD card module, buzzer). Implemented and tested low-level initialization 
code for GPIO, I2C, SPI, timers, and PWM to ensure stable communication with all devices.

### Week - 29 December - 4 January
Designed and implemented the BIOS layer. Developed the display driver integration, keypad scanning logic, and basic audio
feedback. Implemented the SD card initialization and filesystem access logic, including directory scanning and file selection,
and added a fallback.

### Week - 5 January - 11 January
Integrated the BIOS with the main application logic. Connected the BIOS ROM loader to the CHIP-8 virtual machine, ensured correct 
memory loading and execution flow, and implemented input mapping between the physical keypad and the CHIP-8 key layout. Refined timing, 
rendering, and control handoff between BIOS and emulator.

### Week - 12 January - 18 January
Focused on testing, debugging, and performance improvements. Fixed issues related to SD card reliability, opcode handling,
input responsiveness, and display update speed. Tuned execution and rendering frequency, validated multiple CHIP-8 games,
and ensured stable gameplay across different ROMs.

## Hardware

The project utilizes an STM32F767ZI Nucleo-144 board as the central processing unit, interfacing with a 128x64 OLED display via 
SPI/I2C for video output. User interaction is handled through a 4x4 matrix keypad scanned via GPIO, while game assets are 
loaded dynamically from an SD card module and audio is output through a passive buzzer.

### Schematics

![KiCAD Schematic Placeholder](./hardware.svg)

### Bill of Materials

| Device               | Usage | Price    |
|----------------------|--------|----------|
| [STM32F767ZI Nucleo] | The main microcontroller board | ~130 RON |
| [1.3" OLED Display]  | 128x64 Screen for rendering game graphics | ~25 RON  |
| [4x4 Matrix Keypad]  | User input for game controls (16 keys) | ~10 RON  |
| [SD Card Module]     | Interfacing with the SD card to load ROMs | ~10 RON  |
| [Passive Buzzer]     | Audio feedback (beeps/tones) | ~12 RON  |

## Software

| Library                   | Description | Usage |
|---------------------------|-------------|-------|
| `stm32f7xx-hal + embassy` | Hardware Abstraction Layer | Configures GPIO, SPI, and Timers for the STM32F7 |
| `embedded-graphics`       | 2D graphics library | Drawing pixels and text to the display buffer |
| `ssd1306`                 | Display Driver | Initializes and sends data to the OLED screen |
| `embedded-sdmmc`          | File System Library | Reading FAT32 file systems on the SD card |
| `cortex-m-rt`             | Runtime Crate | Startup code and entry point for bare metal execution |

## Links

1. [CHIPnGo - Inspiration](https://github.com/kurtjd/CHIPnGo)
2. [chip8stm32 - Inspiration](https://github.com/AlfonsoJLuna/chip8stm32)
3. [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH.htlm)
