# Rock, Paper, Scissors Robotic Arm

A robotic hand that plays rock, paper, scissors using AI gesture recognition


:::info

    **Author:** Emilia Petre  \
    **Github Project Link:** https://github.com/UPB-PMRust-Students/fils-project-2026-emilia1881

:::

## Description

The purpose of this project is to build an interactive robotic arm that plays Rock, Paper, Scissors using a STM32 Nucleo microcontroller and the Rust programming language. It utilizes a Python-based MediaPipe AI model for real-time gesture recognition via webcam and communicates the results to the hardware via UART. The robotic hand is 3D-printed and uses servo motors to physically mimic the 3 possibilities - rock, paper or scissors. An OLED display tracks the score. The system integrates asynchronous Rust (Embassy) for hardware control and an ESP module to enable remote scoreboard access via WiFi.

## Motivation

This project combines Computer Vision and Embedded Rust, transforming digital gesture recognition into physical robotic motion. By implementing the classic Rock, Paper, Scissors game, the project  creates an interactive experience that demonstrates the power of asynchronous firmware in handling complex, real-time user interactions. So, the motivation was to have a tangible, end-to-end system, that feels like a challenge for both myself in building it and for the people interacting with it afterwards, when they play the game. 


## Architecture

![System Architecture](images/architecture.svg)


## Log

# Weeks 5-6

 - Researched possible project ideas and chose the Rock, Paper, Scissors Game
 - Researched different development boards and selected the STM32 Nucleo-U545RE-Q as the main microcontroller
 - Ordered STM32

 # Weeks 7-8

 - Researched additional hardware components and implementation methods
 - Ordered the necessary materials and performed initial functionality tests on the purchased modules

 # Week 9
 - Searching for a suitable 3D-printable robotic hand that can be adapted to project requirements and looking to start the 3D printing process

## Hardware

The hardware platform of the project is centered around the STM32 Nucleo-U545RE-Q development board, which acts as the main controller of the robotic arm. The SG90 servo motors are used to actuate the fingers of the hand and reproduce the rock, paper, and scissors gestures. A 0.96-inch SSD1306 OLED display connected through I2C is used to show the score and game status. Wireless connectivity is provided by an ESP-01 WiFi module communicating with the STM32 through UART. Gesture recognition is performed on a laptop using a USB webcam or the integrated camera. For prototyping and interconnections, a breadboard and jumper wires are used. The robotic hand itself is manufactured using 3D printing, while the system is powered by a 5V external power supply.

### Schematics

Place your KiCAD or similar schematics here in SVG format

### Bill of materials


| Device | Usage | Price |
|---|---|---|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main microcontroller of the system | ~127 RON |
| [SG90 Servo Motor](https://sigmanortec.ro/Servomotor-SG90-limit-switch-p141662062) | Controls the fingers of the robotic hand | ~ 5 x 10 RON |
| [OLED Display 0.96" SSD1306](https://sigmanortec.ro/Display-OLED-0-96-I2C-IIC-Albastru-p135055705) | Displays score and game status | ~ 17 RON |
| [ESP-01 WiFi Module](https://sigmanortec.ro/Modul-Wifi-ESP8266-Transreceiver-p134711871) | Wireless communication and remote scoreboard access | ~ 21 RON |
| [ESP-01 Adaptor Module](https://sigmanortec.ro/Modul-adaptor-pentru-ESP-01-ESP8266-5V-3-3V-p182230951) | Wireless communication and remote scoreboard access | ~ 10 RON |
| [Breadboard (760 points)](https://sigmanortec.ro/Breadboard-760-puncte-p190992404) | Rapid prototyping and circuit connections | ~ 10 RON |
| Jumper Wires Set | Electrical interconnections between modules | owned |
| 5V Power Supply (old charger)| Powers servos and electronic modules | owned |
| 3D Printed Robotic Hand Parts | Mechanical structure of the robotic hand | - |

## Software

| Library | Description | Usage |
|---|---|---|
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | Hardware abstraction layer for STM32 microcontrollers | Used to control GPIO, PWM, UART, I2C, timers and other peripherals |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async task executor for embedded systems | Runs concurrent firmware tasks such as motor control and communication |
| [embassy-time](https://crates.io/crates/embassy-time) | Time management utilities | Handles delays, timers, and periodic scheduling |
| [embassy-sync](https://crates.io/crates/embassy-sync) | Synchronization primitives | Enables safe communication between async tasks |
| [cortex-m](https://crates.io/crates/cortex-m) | Low-level ARM Cortex-M support crate | Used for processor-specific features and interrupts |
| [cortex-m-rt](https://crates.io/crates/cortex-m-rt) | Runtime support for Cortex-M targets | Handles startup code and interrupt vectors |
| [embassy-futures](https://crates.io/crates/embassy-futures) | Async utility crate | Used for joining and selecting multiple async tasks |
| [ssd1306](https://crates.io/crates/ssd1306) | OLED display driver | Controls the SSD1306 scoreboard display over I2C |
| [embedded-graphics](https://crates.io/crates/embedded-graphics) | 2D graphics library for embedded displays | Draws text, icons, and score information on OLED |
| [panic-probe](https://crates.io/crates/panic-probe) | Panic handler for embedded debugging | Reports firmware crashes during development |
| [defmt](https://crates.io/crates/defmt) | Lightweight embedded logging framework | Used for debug messages and diagnostics |
| [defmt-rtt](https://crates.io/crates/defmt-rtt) | RTT transport for defmt logs | Sends debug output to the host PC |
| [serialport](https://crates.io/crates/serialport) | Serial communication library for desktop Rust | Sends gesture recognition results from laptop to STM32 |
| [serde](https://crates.io/crates/serde) | Serialization and deserialization framework | Converts structured data between formats |
| [serde_json](https://crates.io/crates/serde_json) | JSON parsing library | Processes MediaPipe gesture data messages |
| [mediapipe](https://ai.google.dev/edge/mediapipe/solutions/guide) | Real-time gesture recognition framework | Detects hand landmarks and classifies gestures |
| [opencv-python](https://pypi.org/project/opencv-python/) | Computer vision library | Captures webcam frames and preprocesses images |

## Links

1. [Embassy Framework](https://embassy.dev/book/#_what_is_embassy)
2. [STM32 Nucleo-U545RE-Q](https://www.st.com/en/microcontrollers-microprocessors/stm32-32-bit-arm-cortex-mcus/documentation.html)
3. [MediaPipe Gesture Recognition](https://developers.google.com/mediapipe)
5. [ESP-01 WiFi Module Information](https://www.espressif.com/)

