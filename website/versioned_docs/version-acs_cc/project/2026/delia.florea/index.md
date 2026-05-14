# Spy Video Car
A remote-controlled smart car with obstacle detection and onboard video capture.

:::info
Author: Florea Delia Cristina
GitHub Project Link: https://github.com/UPB-PMRust-Students/acs-project-2026-Deliutz
:::

## Description

Spy Video Car is a smart remote-controlled car built using the STM32 Nucleo-U545RE-Q board and programmed entirely in Rust. The car can be controlled wirelessly from a laptop using a custom Rust desktop application and a Bluetooth HC-05 module.

The system integrates multiple hardware components. A motor driver (L298N) controls four DC motors, enabling movement in all directions. An HC-SR04 ultrasonic sensor detects obstacles in front of the car, and this information is transmitted back to the application.

A camera module (OV7670) is connected directly to the STM32 and captures image frames. These frames are processed by the microcontroller and stored on a microSD card using an SD card module. This allows the car to function as a basic surveillance or “spy” device, capable of recording its surroundings.

The firmware is written using Rust and embassy-rs, focusing on low-level hardware interaction and asynchronous execution.

## Motivation

I chose this project to explore low-level embedded programming in Rust and to understand how different hardware components interact in a real system. The goal was to build a complete embedded system that includes control, sensing, communication, and data storage.

Additionally, I wanted to challenge myself by working with multiple peripherals simultaneously (motors, Bluetooth, sensors, camera, and storage) and to create something practical and interactive.

## Architecture

The project is structured as a modular embedded system where the STM32 acts as the central controller.

Main Components:
* **The Controller**: The STM32 Nucleo-U545RE-Q board — the main unit that handles motor control, communication, sensor reading, and data processing.
* **The Communication System**: The HC-05 Bluetooth module used to send commands from the laptop and receive status updates.
* **The Actuation System**: The L298N motor driver controlling four DC motors for movement.
* **The Sensing System**: The HC-SR04 ultrasonic sensor used to detect obstacles in front of the car.
* **The Video Capture System**: The OV7670 camera module connected directly to the STM32 for capturing image frames.
* **The Storage System**: A microSD card module connected via SPI, used to store captured images.

![System Diagram](./images/diagrama.svg)

## Log

### Week 5 - 11 May
- Project idea definition  
- Hardware selection  
- Initial setup (Rust + toolchain)

### Week 12 - 18 May
- STM32 configuration  
- Motor driver integration (L298N)  
- Movement testing  
- Bluetooth (HC-05) communication setup  

### Week 19 - 25 May
- Desktop control application setup  
- Camera (OV7670) integration  
- Image capture testing  

## Hardware

The system uses an STM32 microcontroller along with motors, sensors, communication modules, and storage components.

### Schematics

![Hardware Schematics](./images/schematic_pm.svg)

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| STM32 Nucleo-U545RE-Q | Main microcontroller | ~125 RON |
| L298N | Motor control | ~25 RON |
| HC-05 | Bluetooth | ~25 RON |
| HC-SR04 | Obstacle detection | ~10 RON |
| Modul SD Card | Storage | ~15 RON |
| Camera OV7670 | Image capture | ~30 RON |
| Motoare DC x4 | Movement | ~40 RON |
| Baterii | Power supply | ~120 RON |
| Fire | Connections | ~100 | 

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://docs.embassy.dev/embassy-stm32) | STM32 HAL for embassy-rs | GPIO, UART, timers |
| [embassy-time](https://docs.embassy.dev/embassy-time) | Async timing | PWM software control |
| [embedded-hal](https://docs.rs/embedded-hal) | Hardware abstraction | GPIO operations |
| [serialport](https://docs.rs/serialport) | Serial communication | Bluetooth on PC |
| [egui / eframe](https://github.com/emilk/egui) | GUI framework | Desktop control interface |

## Links

1. https://embassy.dev  
2. https://www.st.com/resource/en/reference_manual/rm0456-stm32u5-series-advanced-armbased-32bit-mcus-stmicroelectronics.pdf  
3. https://docs.rs/embedded-hal  
