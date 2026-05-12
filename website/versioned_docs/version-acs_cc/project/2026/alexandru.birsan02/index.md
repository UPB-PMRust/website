# Polargraph
A vertical drawing machine that uses a polar coordinate system to create pen-on-paper art

:::info 

**Author**: Alexandru-Vlad Bîrsan \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-vldxndr

:::

## Description

Polargraph - Vertical Plotter

The Polargraph is a vertical DRP (Digital Reconstruction Plotter) that operates on a polar coordinate system. Unlike the traditional plotter or printer that works on a X Y axis the polargrah transforms polar coordinates into coordonates on the paper by using two motors fixed over the paper that hold wires that have variable lenghts so they can position the drawing tool where it needs to be.


## Motivation

The idea came to me because I have previously studied architecture and have always wanted to have something that could draw using real writing utensils, because while a printer can draw exactly what you want using pin point accuracy it lacks the soul of a hand drawing. This project aims to have the accuracy of a printer while portraying a hand rendered drawing.


## Architecture 

![Diagram](./images/diagram.svg)

## Log

### Week 27 - 30 April

Wrote initial documentation and made the first diagram while ordering the parts. Decided to use preexisting software for transforming drawings into lists of instructions.

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The Polargraph is powered by an external 12V DC source to ensure constant torque for the stepper motors. For development purposes, the STM32 Nucleo is tethered via USB for real-time G-Code streaming and debugging

### Schematics


### Bill of Materials

| Device | Usage | Price |
| :--- | :--- | :--- |
| [STM32 Nucleo-U545](https://www.st.com/en/microcontrollers-microprocessors/stm32u545.html) | Main Controller (Brain of the project) | [Owned] |
| [NEMA 17 Stepper Motor (1.7A)](https://www.optimusdigital.ro/ro/motoare-pas-cu-pas/106-motor-pas-cu-pas-nema-17-40mm-17a.html) | Axis movement (2 pieces required) | [130 RON](https://www.optimusdigital.ro) |
| [TMC2208 Stepper Driver](https://www.optimusdigital.ro/ro/drivere-motoare-pas-cu-pas/2753-driver-motor-pas-cu-pas-tmc2208.html) | Silent motor control (2 pieces required) | [90 RON](https://www.optimusdigital.ro) |
| [SG90 Micro Servo](https://www.optimusdigital.ro/ro/servomotoare/9-servomotor-sg90.html) | Pen lift mechanism | [15 RON](https://www.optimusdigital.ro) |
| [12V 5A Power Supply](https://www.optimusdigital.ro/ro/surse-de-alimentare/123-sursa-de-alimentare-12v-5a.html) | External power for stepper motors | [55 RON](https://www.optimusdigital.ro) |
| [GT2 Pulleys & Belt Kit](https://www.optimusdigital.ro/ro/curele-si-fulii/145-fulie-gt2-20-dinti-5mm.html) | Mechanical transmission system | [40 RON](https://www.optimusdigital.ro) |
| [Breadboard MB-102](https://www.optimusdigital.ro/ro/prototipare/10-breadboard-830-puncte.html) | Prototyping and circuit connections | [15 RON](https://www.optimusdigital.ro) |
| [Jumper Wires M-M / F-M](https://www.optimusdigital.ro/ro/fire-conectori-si-socluri/894-set-fire-tata-tata-65-buc.html) | Connecting components to Nucleo | [15 RON](https://www.optimusdigital.ro) |
| [DC Jack Adapter](https://www.optimusdigital.ro/ro/fire-conectori-si-socluri/124-mufa-dc-mama-cu-terminal-block.html) | Connecting the 12V supply to breadboard | [5 RON](https://www.optimusdigital.ro) |
| [Capacitor Kit (100uF)](https://www.optimusdigital.ro/ro/componente-pasive/220-condensator-electrolitic-100uf-35v.html) | Power spike protection for drivers | [5 RON](https://www.optimusdigital.ro) |


## Software

| Library | Description | Usage |
| :--- | :--- | :--- |
| [stm32u5xx-hal](https://github.com/stm32-rs/stm32u5xx-hal) | Hardware Abstraction Layer | Managing GPIO for motor control, UART for G-Code streaming, and Timers for pulse generation. |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Embedded Abstraction Traits | Provides a standard interface for peripheral drivers, ensuring modular and testable code. |
| [micromath](https://github.com/tarkentat/micromath) | Fast Math Library | Used for efficient fixed-point and floating-point square root calculations in the Inverse Kinematics engine. |
| [cortex-m-rt](https://github.com/rust-embedded/cortex-m-rt) | Startup and Runtime | Handles the entry point and reset handler for the ARM Cortex-M33 processor. |
| [panic-halt](https://github.com/rust-embedded/panic-halt) | Panic Handler | Provides a simple panic strategy that halts the processor in case of critical software errors. |


#### Host Software & Toolchain

To transform digital images into physical drawings, the project uses a multi-step toolchain:

1. **Vectorization/G-Code Generation**: I use DrawingBotV3 (or Inkscape with G-Code extensions). These tools allow me to convert standard images (JPG/PNG) into paths using algorithms like *Squiggle*, *TSP (Travelling Salesman Problem)*, or *Stippling*.
2. **Path Optimization**: The software generates a list of G-Code commands (`G0`, `G1`) which represent coordinates in a Cartesian system.

## Links

1. [DrawingBotV3](https://drawingbotv3.com/) - The primary software for G-Code generation.
2. [Polargraph Physics](https://github.com/euphy/polargraph/wiki/Polargraph-physics) - Detailed explanation of the inverse kinematics involved.
3. [TMC2208 Datasheet](https://www.trinamic.com/fileadmin/assets/Products/ICs_Documents/TMC220x_TMC2224_datasheet_Rev1.09.pdf) - Technical specifications for the motor drivers.
4. [Example](https://www.youtube.com/watch?v=aiw3hkDvp-M) - Working example