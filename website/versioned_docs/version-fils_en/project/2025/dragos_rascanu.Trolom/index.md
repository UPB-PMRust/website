# Remote-controlled car with LiDAR
Embedded Rust and Embassy that features keyboard-based WiFi control and real-time 2D environment mapping using LiDAR.

:::info 

**Author**: Rascanu Dragos \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-dm-2025-Trolom

:::

## Description

A remote-controlled car with real-time 2D LiDAR mapping capabilities, built with Rust for embedded systems. Control the car via WiFi using your keyboard and visualize its surroundings through live mapping on the display of the pc.

## Motivation

 - LiDAR Logic: I wanted to gain hands-on experience with 360° LiDAR sensors, specifically learning how to decode raw laser distance data into a 2D coordinate system.

 - Wireless Data Streaming: I was motivated to implement a high-bandwidth wireless pipeline. I wanted to stream live sensor data directly over WiFi to a PC, simulating how professional autonomous robots and vacuum cleaners operate in the real world.

 - Rust in Motion: I wanted to use Embedded Rust to ensure memory safety and high performance when handling the concurrent tasks of driving motors and streaming telemetry data.

## Architecture 

Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other

## Log

### Week 5

Started buying the components, first were the driver module, two wheel motors and a LCD. Starting looking for an ESP32 as well as a LiDAR sensor

### Week 7

LiDAR module arrived, checked if the motors and module worked and controlled them through uart.

### Week 9

ESP32-S3 finally arrived and i could start working on the project.

### Week 11

Tested the ESP32 board and started writing the code for sending the LiDAR readings through WiFi to my pc.

## Hardware

| Component | Purpose | Cost |
|-----------|---------|------|
| STM32 Nucleo-U545RE-Q | Main microcontroller | 128 lei |
| ESP32-S3 | WiFi communication module | 43 lei |

### Motion Control
| Component | Purpose | Cost |
|-----------|---------|------|
| 2× DC Gearmotors with wheels | Drive motors | 30 lei |
| DRV8833 Dual Motor Driver | Motor control | 25 lei |
| MG90S Servo Motor | Front-wheel steering | 19 lei |

### Sensing & Display
| Component | Purpose | Cost |
|-----------|---------|------|
| M1C1 360° LiDAR Module | Environment scanning | 137 lei |
| ST7735 1.8" LCD Display | Status display | 29 lei |


### Schematics

Place your KiCAD schematics here.

### Bill of Materials

| Component | Purpose | Cost |
|-----------|---------|------|
| STM32 Nucleo-U545RE-Q | Main microcontroller | borrowed |
| ESP32-S3 | WiFi communication module | 43 lei |
| 2× DC Gearmotors with wheels | Drive motors | 30 lei |
| DRV8833 Dual Motor Driver | Motor control | 25 lei |
| MG90S Servo Motor | Front-wheel steering | 19 lei |
| M1C1 360° LiDAR Module | Environment scanning | 137 lei |
| ST7735 1.8" LCD Display | Status display | 29 lei |
| | Total: | ~283 lei |

## Software

Library | Description | Usage |
|---------|-----------|----------------------------|
| embassy-executor | Async/await runtime | Manages concurrent tasks like motor PWM and UART parsing without an RTOS. |
| embassy-stm32 | HAL for STM32 | Provides low-level access to the U545RE peripherals (UART, PWM, GPIO). |
| esp-hal | HAL for ESP32-S3 | Drives the WiFi hardware and communication peripherals on the S3 Mini.
| embassy-net | Async network stack | Handles the TCP/UDP stack for sending LiDAR data over WiFi.
| smoltcp | TCP/IP stack | The underlying networking engine used by the ESP32 for WiFi communication. |
| defmt | Deferred logging | High-efficiency logging for debugging the STM32 via RTT. |

## Links

### DriveSight (https://embedded-rust-101.wyliodrin.com/docs/fils_en/project/2025/andrei.neagu1910)
Manually controlled robotic platform with real-time video streaming and camera feed monitoring.

### Smart Ride (https://embedded-rust-101.wyliodrin.com/docs/fils_en/project/2025/andra.popovici)
Web app-controlled car with ultrasonic sensors for distance measurement and virtual mapping.

### RC Car with Physical Controller(https://embedded-rust-101.wyliodrin.com/docs/fils_en/project/2025/alin_ioan.gheorghe)
Raspberry Pi Pico-based car with WiFi joystick controller for wireless operation.

