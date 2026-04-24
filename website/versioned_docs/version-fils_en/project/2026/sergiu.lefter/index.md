# Smart Lockbox
A Rust-based electronic lockbox with keypad PIN access, servo lock control, local feedback, and ESP32 networking for logs/admin reset.

:::info

**Author**: Sergiu Lefter \
**GitHub Project Link**: [fils-project-2026-sergiulefter](https://github.com/UPB-PMRust-Students/fils-project-2026-sergiulefter)

:::

<!-- do not delete the \ after your name -->

## Description

This project is a smart lockbox controlled by an STM32 Nucleo board.
The user enters a PIN on a 3x4 keypad. If the PIN is correct, an SG90 servo unlocks the box. If it is wrong, access is denied and a passive buzzer gives warning feedback.

I also use a mechanical endstop to detect when the lid is closed, so the lock can return automatically to the locked position.
For the extended version, I added an ESP32 for network logging and a possible admin reset flow for the PIN.

## Motivation

I chose this project because it is practical and also a good embedded Rust challenge.
It combines hardware and software in one system: keypad input, control logic, servo actuation, sensor feedback, and communication.
I also wanted to build something physical that I can test and improve step by step.

## Architecture

Main architecture components:
- Input: 3x4 keypad, admin button, mechanical endstop.
- Main control: STM32 Nucleo (rental) running the lock logic and state machine.
- Output: SG90 servo for lock movement, LCD1602 (I2C) for messages, passive buzzer for alerts.
- Network side: ESP32 DevKitC for logging and possible admin reset/dashboard.
- Wiring setup: STM32 connects through a small interface protoboard (female headers), then to the main protoboard where modules are connected.

How components connect:
- Keypad, button, and endstop send inputs to the STM32.
- STM32 controls the servo, buzzer, and LCD.
- ESP32 communicates with STM32 over serial/UART for logging/admin features.
- Components are assembled on protoboard/PCB prototyping boards using silicone stranded wires.

### Software State Machine

The software is organized as a state machine with the following states:
- `LOCKED`
- `ENTERING_PIN`
- `UNLOCKED`
- `WAIT_FOR_CLOSE`
- `ADMIN_MODE`
- `ALARM`

### Architecture Diagram

```text
Input Block (3x4 Keypad, Admin Button, Mechanical Endstop)
        |
        | GPIO
        v
STM32 Nucleo (Main Control: PIN + State Machine)
        |
        | via small interface protoboard (female headers)
        v
Main Protoboard (distribution for components)
   |                 |                    |
   | I2C             | PWM                | GPIO/PWM
   v                 v                    v
LCD1602 + I2C     SG90 Servo         Passive Buzzer

STM32 Nucleo
        |
        | UART / Serial
        v
ESP32 DevKitC (Logging + optional Admin PIN reset)
```

## Log

<!-- write your progress here every week -->

### Week 4-5
Researched smart lockbox implementations and decided on the final project direction. Chose the main architecture around STM32 + keypad + servo and started the documentation page.

### Week 6-7
Updated the hardware plan and replaced several components: breadboard with protoboard, 4x4 keypad with 3x4 keypad, reed switch with mechanical endstop, and active buzzer with passive buzzer. Ordered all required parts and checked compatibility.

### Week 8
Defined the interface-board approach to avoid soldering on the rental STM32 board (female header bridge to the main protoboard). Added ESP32 networking scope for event logging and possible admin PIN reset flow.

## Hardware

The lockbox prototype uses a mixed setup with reusable university hardware and purchased modules.
The STM32 board is provided by the university as free rental (must be returned after project completion).
The rest of the circuit is assembled using protoboard/PCB prototyping boards and silicone stranded wires.

### Schematics

TODO: add KiCAD (or similar) schematic SVG in this folder and reference it here.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| STM32 development board (university borrow) | Main controller board for firmware development and peripheral control | 0 RON (borrowed) |
| [Placa dezvoltare ESP32-DevKitC, ESP32-WROOM-32D, 38P](https://sigmanortec.ro/placa-dezvoltare-esp32-devkitc-esp32-wroom-32d-38p) | Networking/logging module and potential admin dashboard support | 42.56 RON |
| [Placa PCB prototipare o fata 5x7](https://sigmanortec.ro/Placa-PCB-prototipare-o-fata-5x7-p159914959) | Small interface protoboard used between STM32 and the main wiring board | 2.95 RON |
| [Placa PCB prototipare fata dubla 7x9cm](https://sigmanortec.ro/Placa-PCB-prototipare-fata-dubla-7x9cm-p125747328) | Main (big) protoboard used for the rest of the components | 5.76 RON |
| [Bara 40 pini 2.54 tata](https://sigmanortec.ro/Bara-40-pini-2-54-tata-p126025250) | Header pins for module interconnection | 2.06 RON |
| [Bara pini 40p mama 2.54mm, pin rotund, segmentabil](https://sigmanortec.ro/bara-pini-40p-mama-254mm-pin-rotund-segmentabil) x3 | Female headers for detachable wiring/interfaces | 7.08 RON (total) |
| [Tastatura numerica 4x3, 12 butoane](https://sigmanortec.ro/tastatura-numerica-4x3-12-butoane) | PIN entry keypad (3x4) | 18.76 RON |
| [Buton 12x12x7.3](https://sigmanortec.ro/Buton-12x12x7-3-p160373654) | Local control/input button | 1.33 RON |
| [Endstop mecanic SS-5GL2](https://sigmanortec.ro/Endstop-mecanic-SS-5GL2-p136284192) | Position sensing (replaces reed switch) | 5.23 RON |
| [Servomotor SG90, 180 grade, cu limitator](https://sigmanortec.ro/Servomotor-SG90-limit-switch-p141662062) | Mechanical lock actuation | 9.49 RON |
| [Buzzer pasiv 5V](https://sigmanortec.ro/Buzzer-pasiv-5v-p172425809) | Audio feedback for success/error events | 1.45 RON |
| [LCD 1602](https://sigmanortec.ro/LCD-1602-p125700685) | Status display for prompts and lock messages | 11.18 RON |
| [Modul interfata I2C LCD 1602/2004](https://sigmanortec.ro/Modul-interfata-I2C-LCD-1602-2004-p125700577) | I2C adapter for LCD communication | 5.11 RON |
| [Tranzistor NPN BC547 TO-92](https://sigmanortec.ro/Tranzistor-NPN-BC547-TO-92-p126458754) | Driver/transistor stage in auxiliary control circuits | 1.21 RON |

## Software

| Library/Framework | Description | Usage |
|---------|-------------|-------|
| [Embassy](https://embassy.dev/) | Async embedded Rust framework | Main embedded architecture and task scheduling |
| [embassy-stm32](https://docs.embassy.dev/embassy-stm32/) | STM32 peripheral support for Embassy | GPIO/I2C/PWM integration on STM32 |
| [embassy-executor](https://docs.embassy.dev/embassy-executor/) | Async executor | Runs concurrent embedded tasks |
| [embassy-time](https://docs.embassy.dev/embassy-time/) | Timing, delays, and timeouts | Debounce logic, lock timers, alarm timing |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction traits | Portable driver interfaces |
| [embedded-hal-async](https://github.com/rust-embedded/embedded-hal) | Async HAL traits | Async peripheral operations |
| [defmt](https://github.com/knurling-rs/defmt) | Lightweight logging for embedded Rust | Structured firmware logs during development |
| [defmt-rtt](https://github.com/knurling-rs/defmt) | RTT transport for defmt | Real-time debug output |
| [panic-probe](https://github.com/knurling-rs/probe-run/tree/main/panic-probe) | Panic reporting | Panic handling during debugging |
| [heapless](https://github.com/rust-embedded/heapless) | `no_std` data structures | Fixed-capacity buffers/queues for embedded-safe logic |
| [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc) | ESP32 service layer in Rust | Networking/logging and admin-side integration |

## Links

1. [Project page requirements (FILS EN)](https://embedded-rust-101.wyliodrin.com/docs/fils_en/project)
2. [Embassy documentation](https://embassy.dev/)
3. [Rust Embedded Book](https://docs.rust-embedded.org/book/)
4. [ESP-RS Book](https://docs.esp-rs.org/book/)
