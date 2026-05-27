# Endless Motion
A robot arm that cleans liquid around itself.

:::info

**Author**: Robert-Mario Coman \
**Github Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-robertcoman1

:::

## Description
**Endless Motion** is a robotic arm system that receives input from sensors placed in specific locations to detect the presence of a liquid. It processes these signals to determine the target area that requires intervention. Based on this data, it calculates control commands, moves to the detected zone, and executes a wiping action. The system continuously updates its movement, repeating the process based on newly received signals.

## Motivation
The primary inspiration for this project comes from the famous contemporary artwork *"Can't Help Myself"* (by Sun Yuan & Peng Yu), which features an industrial robotic arm trapped in an endless, repetitive loop of trying to clean up a liquid that continuously spreads across the floor. Fascinated by this visual and conceptual piece, I decided to engineer a small-scale, functional replica of this behavior.

## Architecture

![System Architecture Diagram](./images/diagram_pm.svg)

Based on the system diagram, the architecture is divided into the following logical and hardware blocks:

* **Liquid Level Sensors (Input):** The physical endpoints that detect the presence of liquid and send raw analog or digital signals to the microcontroller.
* **GPIO/ADC Input Driver:** The low-level hardware abstraction layer on the STM32 that reads the electrical signals from the sensors and translates them into software trigger events.
* **Async Task Manager (Rust Embassy):** The core logic router. It uses asynchronous tasks to handle sensor triggers without blocking the CPU, safely managing the overall state of the robot (e.g., Idle, Moving, Wiping).
* **Kinematics & Motion Controller:** Receives target zone data from the Task Manager and calculates the specific angles required for the robotic arm to reach the active area and execute the cleaning sequence.
* **Hardware Timers & PWM Output:** Converts the mathematically calculated angles into highly precise PWM electrical signals.
* **Servomotors (Output):** The physical actuators that interpret the PWM control signals to execute the physical movement of the arm.
* **External Power Supply:** A dedicated power source strictly for the servomotors.

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

* **Microcontroller:** STM32U545RE-Q (ARM Cortex-M33) - The "brain" of the system, responsible for reading sensor data and generating precise PWM control signals.
* **Servomotors:** 4x MG90S Micro Servomotors (180°, Metal Gear) - These provide the necessary torque and durability to physically move the robotic arm and press the sponge against the surface.
* **Sensors:** 3x Raindrop Sensor Modules (with LM393 comparators) - Placed in three distinct radial zones around the robot to detect the presence of liquid and trigger the cleaning state.
* **Power Supply Domain:** A dedicated 5V / 6A AC-DC wall adapter connected via a DC jack screw terminal. This heavily isolated power line feeds directly into the motors, preventing voltage drops and protecting the microcontroller from high-current draw.

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

| Device | Usage | Price |
|---|---|---|
| STM32U545RE-Q Nucleo | The microcontroller | [129 RON](https://ro.farnell.com/stmicroelectronics/nucleo-u545re-q/development-brd-32bit-arm-cortex/dp/4216396?CMP=e-email-sys-orderack-GLB) |
| MG90S Micro Servomotor (180°, Metal Gear) | Actuators providing precise physical movement for the robotic arm joints | [22.86 RON](https://electronix.ro/produs/servomotor-mg90s-180-de-grade/) x 4 |
| Raindrop Sensor Module (LM393) | Placed in 3 zones to detect liquid presence and trigger the wiping sequence | [8.76 RON](https://electronix.ro/produs/modul-cu-senzor-picaturi-de-ploaie-raindrop-arduino/) x 3 |
| 5V 6A AC-DC Power Adapter | Dedicated high-current power supply for the servomotors to prevent board resets | [159,89 RON](https://electronix.ro/produs/alimentator-5vdc-6a-30w/) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Hardware Abstraction Layer (HAL) for STM32 | Used to configure the physical pins: PWM for the servomotors and ADC/GPIO for the raindrop sensors. |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Lightweight Async Executor | Manages the core state machine, scheduling and running the non-blocking tasks (e.g., `Idle`, `Wiping`). |
| [embassy-time](https://github.com/embassy-rs/embassy) | Timekeeping and Timers | Controls the speed of the servo movements (the wiping animation) and handles sensor debounce delays. |
| [defmt](https://github.com/knurling-rs/defmt) | Highly efficient logging framework | Used to send real-time debug data (e.g., "Zone 2 Triggered", "Servo angle: 90°") to the PC over the USB probe. |
| [panic-probe](https://github.com/knurling-rs/defmt) | Panic handler | Catches any software crashes and prints the error stack trace to the terminal for easy debugging. |

## Links

1. [Project Inspiration: "Can't Help Myself" by Sun Yuan & Peng Yu (Guggenheim Museum)](https://www.guggenheim.org/artwork/34812)
2. [Mechanical 3D Model: "Micro Robot arm" by bentommye (Thingiverse)](https://www.thingiverse.com/thing:34829)

