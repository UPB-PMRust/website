
# GPS Fitness Tracker
A fitness tracking device that monitors physical activity by counting steps and recording GPS-based workouts, providing real-time statistics and storing workout data.
:::info

**Author**: Maraloi Ioana \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-Ioana-Maraloi

:::

<!-- do not delete the \ after your name -->

## Description

This project implements a fitness tracking device using a microcontroller programmed in Rust. The system operates in two modes. In daily mode, it detects steps using accelerometer data, estimates distance and calories, and displays real-time information on a screen. In training mode, activated via a button, it uses GPS data to track position, compute distance and speed, and record the route. The recorded data is saved in GPX format for later analysis.

## Motivation

I chose this project because it combines several important aspects of embedded systems, including sensor data processing, GPS-based measurements, and real-time data display. It provides a good balance between hardware interaction and software implementation, while also covering topics such as data acquisition, processing, and storage in a practical context.

## Architecture

<!-- 
Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other -->

The system is centered around a processing module running on the STM32 microcontroller. It receives data from the sensor, GPS, and control modules, processes it, then sends the results to the display and storage modules.

Main architecture components:
- **Control Module** – handles button input and mode switching
- **Sensor Module** – provides motion data for step detection
- **GPS Module** – provides position, speed, and time data
- **Processing Module** – computes steps, distance, speed, calories, and workout state
- **Display Module** – shows real-time activity information
- **Storage Module** – saves workout data for later analysis

![alt text](schema_pm_ioana_maraloi.svg)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The system is built around an STM32 Nucleo development board, which interfaces with multiple peripherals for sensing, display, and storage.

An MPU6050 accelerometer module is used to detect motion and enable step counting. A NEO-6M GPS module provides location data for tracking distance, speed, and route information.

A 2.4" display is used to present real-time data to the user, including activity statistics and system status. User input is handled through push buttons, allowing mode switching and interaction.

Workout data is stored using a microSD card module, enabling later analysis. Additional components such as LEDs are used for status indication, while a breadboard, jumper wires, and resistors are used for prototyping and circuit connections.

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
| [STM32 Nucleo-U545RE-Q ](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | provided by university |
| [ILI9341 2.4 inch display ](https://www.emag.ro/modul-lcd-2-4-240x320-pixeli-spi-controler-ili9341-3-3v-nykhqe-lcd-2-4-spi/pd/DNPYPD2BM/) | display for user interface and real-time data visualization | 80 RON |
| [MPU6050 GY-521 accelerometer module](https://www.emag.ro/modul-giroscop-mpu-6050-gy-521-accelerometru-arduino-3-axe-2-1-cm-x-1-1-cm-x-0-3-cm-albastru-c7/pd/DL3G1QYBM/) | Measures motion and is used for step detection |  32 RON |
| [MicroSD Card Module ](https://www.emag.ro/modul-de-expansiune-micro-sd-card-de-memorie-tf-pentru-arduino-dh000036/pd/DP8QQL3BM/) | Reads and writes data to the microSD card for storing workout logs | 19 RON |
| [GPS Module NEO6MV2 ](https://sigmanortec.ro/Modul-GPS-6MV2-p125423363) | Provides location data for tracking distance, speed, and route | 30 RON |
| microSD Card 8GB | Stores workout data and GPS logs | 5 RON |
|Push buttons| User input for mode switching and interaction| 8 RON |
| LEDs | Status indication and debugging | 5 RON |
| Breadboard + Jumper Wires | prototyping and connecting components | 30 RON |
| Resistors | Used for LEDs and buttons | 10 RON |

## Software
| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rs](https://embassy.dev/) | Async embedded framework for Rust | Peripheral handling and task management |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Drawing text and graphics on the display |
## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. https://embedded-rust-101.wyliodrin.com/docs/acs_cc/category/lab
<!-- 2. [link](https://example3.com)
... -->
