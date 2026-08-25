# Reaction Gamepad

A classic reaction-based gamepad with STM32U545RETx microcontroller, playable via USB connection to PC.

:::info

**Author**: Furkan Korkmaz  
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-FurkanKorkmaz1

:::

\---

## Description

Reaction Gamepad is a fully functional implementation of a reaction-based game system, running on an STM32 microcontroller. The player uses 8 physical buttons connected to the microcontroller, which communicates with a PC via USB as a gamepad device. The game displays on the PC monitor through a custom graphics engine. Features include multiple game modes, difficulty levels, and visual feedback systems.

\---

## Motivation

This project demonstrates embedded game development on microcontrollers. It combines real-time input handling, game state management, USB protocol implementation, and Rust embedded systems programming. The reaction gamepad serves as a practical platform for learning microcontroller programming while creating an engaging interactive experience.

\---

## Architecture

**Main Components:**

* **STM32U545RETx Microcontroller**: Central processor running game logic, manages GPIO input and USB output
* **8 Physical Buttons**: User input connected to GPIO pins with debouncing
* **USB Gamepad Emulation**: HID protocol implementation for PC communication
* **Game State Manager**: Handles game state, score, and level progression
* **Collision Detection Engine**: Detects input validation and timing

**How They Connect:**

* Buttons connect to GPIO pins → Firmware reads debounced input
* Game logic updates based on button input
* Game state is converted to HID gamepad commands
* PC receives gamepad data via USB and renders graphics/audio

!\[snake\_game\_architecture](./images/snake\_game\_architecture.svg)

!\[tiny\_knight\_architecture](./images/tiny\_knight\_architecture.svg)

\---

## Log

### Week 5 - 11 May

Project concept selected. Decided to build a reaction-based gamepad using STM32U545RETx microcontroller. First version will use buttons as input with PC display for visual feedback.

### Week 12 - 18 May

Hardware architecture planned. Buttons configured as GPIO inputs. Main software logic designed with game loop structure and timing measurement.

### Week 19 - 25 May

Documentation page created. Project description, motivation, architecture, hardware plan, and software plan documented. Game modes and difficulty levels finalized in design.

\---

## Hardware

STM32U545RETx (Nucleo-U545RE-Q) development board serves as the main microcontroller. Prototype built on breadboard using 8 push buttons for input, USB connection to PC for gamepad communication. Buttons connected via GPIO pins with pull-down resistors for debouncing. All power supplied via USB cable.

\---

## Schematics

!\[gamepad](./images/gamepad.svg)

\---

## Bill of Materials

|Device|Usage|Price|
|-|-|-|
|STM32 Nucleo-U545RE-Q|Main microcontroller|Borrowed from FILS|
|Push Buttons (x8)|Player input controls|20 RON|
|Breadboard|Circuit assembly|18 RON|
|Jumper Wires|Component connections|20 RON|
|Resistors (10kΩ)|Button debouncing|1.20 RON|
|USB Micro Cable|Power and communication|12 RON|
|**Total**||**71.20 RON**|

\---

## Software

|Library|Description|Usage|
|-|-|-|
|embassy-stm32|STM32 hardware abstraction layer|Controls GPIO pins and USB device|
|embassy-usb|USB device stack|Implements HID gamepad protocol|
|usbd-hid|USB HID device support|Sends gamepad input to PC|
|embedded-hal|Embedded hardware traits|Standard interface for GPIO operations|
|stm32u5|STM32U5 specific bindings|Direct microcontroller access|

\---

## Links

1. [STM32U5 Datasheet](https://www.st.com/resource/en/datasheet/stm32u545re.pdf)
2. [Nucleo-U545RE-Q User Manual](https://www.st.com/resource/en/user_manual/um2912-stm32u545-discovery-kit-stmicroelectronics.pdf)
3. [Rust Embedded Book](https://rust-embedded.github.io/book/)
4. [Embassy Documentation](https://embassy.dev/)

