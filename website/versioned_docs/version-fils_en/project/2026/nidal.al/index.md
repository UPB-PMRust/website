# PicoTherm
PicoTherm, powered by the Embassy framework, delivers real-time temperature and pressure monitoring, visual alert signaling, and live display output through a compact embedded system.

:::info

**Author**: Nidal Al-Ramahi \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-nidalalramahi

:::

## Description
The project centers on **PicoTherm**, an embedded environmental monitoring system that demonstrates key principles of sensor interfacing, asynchronous task execution, and real-time display control. The system is built around a **BMP280 sensor**, which measures both **temperature** and **atmospheric pressure** and sends the data through an **I²C interface**.

To provide immediate feedback to the user, the system includes an **SPI-connected LCD screen** that continuously displays the current sensor readings. This creates a compact real-time interface that allows temperature and pressure values to be monitored without the need for additional tools or external software.

The project also includes a simple but effective **alert system** using **LEDs** and an **active buzzer**. When the measured temperature exceeds a predefined threshold, the warning LED is activated and the buzzer produces an audible alert. Under normal conditions, a separate LED can indicate that the system is functioning correctly.

All subsystems operate asynchronously using the **Embassy framework**, enabling efficient and non-blocking management of concurrent tasks. This allows the system to handle sensor polling, display updates, and alert control at the same time while maintaining responsive and stable behavior.

The goal of the project is to combine multiple communication protocols and peripherals into one practical embedded Rust application that is simple, reliable, and easy to extend.

## Motivation
The idea for PicoTherm was inspired by my interest in building a project that is both **practical** and **technically meaningful**. I wanted to create something that allowed me to work with real sensors, external peripherals, and communication protocols while keeping the design focused and manageable.

This project gave me the opportunity to explore several important embedded systems concepts in one implementation, including **I²C sensor communication**, **SPI display interfacing**, and **GPIO-based control** for LEDs and the buzzer. It also allowed me to gain hands-on experience with **Embassy** and asynchronous programming in embedded Rust.

I chose this project because monitoring systems are useful, clear in purpose, and easy to demonstrate. At the same time, they offer enough complexity to make the implementation interesting, especially when multiple peripherals must work together in real time.

## Architecture

### System Layout and Design
A system layout image will be added here later.

### Components Overview

#### Sensor Module

- **Environmental Sensor**:
    - **BMP280**: Measures temperature and atmospheric pressure in real time.
    - Connected through **I²C** for sensor data acquisition.

#### Display and Output System

- **LCD Screen (SPI-based)**:
    - Displays temperature and pressure values in real time.
    - Serves as the main visual interface of the project.

- **LED Indicators**:
    - **Normal Status LED**: Indicates normal operating conditions.
    - **Warning LED**: Activates when the measured temperature exceeds the defined threshold.

- **Active Buzzer**:
    - Provides an audible warning when the system detects a high-temperature condition.

### Connectivity
- **BMP280**: Communicates with the Raspberry Pi Pico through **I²C**, sending temperature and pressure data.
- **LCD Screen**: Connected via **SPI**, continuously displays the measured values and system status.
- **LEDs**: Controlled through GPIO pins and used for visual alert signaling.
- **Active Buzzer**: Connected to a GPIO pin and activated when the warning condition is met.

:::note
PicoTherm focuses on combining sensor input, real-time display output, and alert signaling into a compact embedded Rust system using Embassy.
:::

## Weekly Log
### Week 21 - 27 April
- Chose the project topic and defined the main system functionality.
- Selected the BMP280 sensor, LCD display, LEDs, and active buzzer as the main hardware components.
- Started organizing the software structure and documentation.

### Week 28 - 4 May
- Studied the communication interfaces required for the BMP280 and LCD modules.
- Began setting up the Embassy development environment.
- Planned the integration of sensor reading, display output, and alert logic.

### Week 5 - 11 May
- Started testing communication with the BMP280 sensor over I²C.
- Began testing the LCD screen over SPI.
- Worked on formatting and displaying measured values clearly.

### Week 12 - 18 May
- Integrated the LEDs and active buzzer into the project.
- Implemented threshold-based warning behavior for high temperature.
- Continued improving interaction between the sensor, display, and alert outputs.

### Week 19 - 25 May
- Focused on final integration and debugging of all components.
- Improved display behavior and system responsiveness.
- Continued documentation and preparation of diagrams and images.

## Hardware
- **BMP280 Sensor**: Measures temperature and atmospheric pressure in real time.
- **SPI LCD Screen**: Displays sensor readings and system status.
- **Active Buzzer**: Provides an audible alert when the warning condition is reached.
- **LEDs**: Used to indicate normal and warning states.
- **Breadboard**: Used for prototyping the circuit.
- **Jumper Wires**: Used for connecting all components to the board.

- **System images and wiring pictures will be added later**.

### Schematics
A schematic will be added here later.

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| Raspberry Pi Pico W | The microcontroller | [1 x 50.00 RON](#) |
| [BMP280](#) | Temperature and pressure sensor (I²C) | [1 x 8.50 RON](#) |
| [SPI LCD Screen](#) | Real-time data display | [1 x 28.99 RON](#) |
| Active Buzzer Module | Audio warning output | [1 x 1.50 RON](#) |
| LED 3mm | Status indication | [2 x 0.30 RON](#) |
| BreadBoard 400 Points | Connectivity | [1 x 4.50 RON](#) |
| Set of Resistors | Current modulation | [1 x 10.00 RON](#) |
| Male-Male Wires 10cm | Connectivity | [1 x 4.99 RON](#) |
| Female-Male Wires | Connectivity | [1 x 5.99 RON](#) |
| **TOTAL** | - | **110.58 RON** |

## Software
| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-executor](https://docs.embassy.dev/embassy-executor/git/std/index.html) | Asynchronous executor for embedded systems | Used to manage concurrent tasks in a non-blocking way |
| [embassy-time](https://docs.embassy.dev/embassy-time/git/default/index.html) | Time management library | Used for delays and periodic sensor updates |
| [embassy-rp](https://docs.embassy.dev/embassy-rp/git/rp2040/index.html) | Raspberry Pi Pico-specific HAL for Embassy | Used for GPIO, SPI, I²C, and peripheral setup |
| [embassy-sync](https://docs.embassy.dev/embassy-sync/git/default/index.html) | Lightweight async synchronization tools | Used for coordination between concurrent tasks |
| [embassy-embedded-hal](https://docs.embassy.dev/embassy-embedded-hal/latest/embassy_embedded_hal/) | Embassy adaptation of embedded-hal traits | Used for shared peripheral access |
| [embedded-hal-async](https://docs.rs/embedded-hal-async/latest/embedded_hal_async/) | Async I/O traits for embedded devices | Used for BMP280 and display communication |
| [static-cell](https://docs.rs/static_cell/latest/static_cell/) | Safe static allocation support | Used for initializing peripherals with static lifetime |
| [defmt](https://docs.rs/defmt/latest/defmt/) | Logging framework for embedded Rust | Used for debugging and runtime logging |
| [defmt-rtt](https://docs.rs/defmt-rtt/latest/defmt_rtt/) | Real-Time Transfer backend for defmt | Enables RTT-based debugging output |
| [panic-probe](https://docs.rs/panic-probe/latest/panic_probe/) | Minimal panic handler for embedded Rust | Provides debug information on panic |
| [mipidsi](https://docs.rs/mipidsi/latest/mipidsi/) | Display driver for MIPI-compatible SPI displays | Used to initialize and control the LCD screen |
| [display-interface-spi](https://docs.rs/display-interface-spi/latest/display_interface_spi/) | SPI wrapper for display communication | Used to connect the display driver to the SPI bus |
| [embedded-graphics](https://docs.rs/embedded-graphics/latest/embedded_graphics/) | Graphics library for embedded displays | Used to render text and layout on the LCD |
| [heapless](https://docs.rs/heapless/latest/heapless/) | Fixed-capacity data structures for no_std | Used for text buffers without dynamic allocation |

## Links
[Projects 2024](https://pmrust.pages.upb.ro/docs/fils_en/category/projects-2024)