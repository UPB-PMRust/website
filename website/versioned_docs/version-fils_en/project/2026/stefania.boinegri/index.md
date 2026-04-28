# AeroGlove: Interactive Gamepad
A wearable, motion-controlled gaming interface.

:::info 

**Author**: Boinegri Ștefania-Denisa \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/fils-project-2026-dboinegri-hue](https://github.com/UPB-PMRust-Students/fils-project-2026-dboinegri-hue)

:::

## Description

AeroGlove is a wearable, motion-controlled gaming interface that bridges the gap between physical movement and digital gameplay. Built around the STM32 Nucleo-U545RE-Q microcontroller, this project transforms natural hand gestures into native PC inputs, effectively acting as both a plug-and-play USB gamepad and an air mouse.

## Motivation

AeroGlove was directly inspired by my time as an exchange student in Sweden, where a Neurotechnology class introduced me to P300-based Brain-Computer Interfaces (BCI). Working with that technology made me realize the impact students can have in developing assistive devices. I am designing AeroGlove for accesibility, with the belief that the joy of gaming and diving into the digital world should be available to everyone, including those with physical limitations.

## Architecture 

The main components are the STM32 Nucleo-U545RE-Q microcontroller, an MPU6050 accelerometer and gyroscope, a tactile push-button, an RGB LED, and a vibration motor module. The MPU6050 connects via I2C to capture spatial orientation. The microcontroller communicates bidirectionally with the PC via a USB Data Cable, sending inputs and receiving real-time feedback signals back from the computer to trigger the PWM-driven vibration motor.

![Block Diagram](./diagram.svg)

## Log

### Week 5 - 11 May

Hardware wiring and basic project setup. Configured the Rust Embassy environment and got the MPU6050 sensor reading data via I2C. Wired the tactile button and tested basic GPIO inputs.

### Week 12 - 18 May

Focused on USB communication. Set up the USB HID device using the usbd-human-interface-device crate so the PC recognizes the board as a mouse/gamepad. Mapped the sensor data to screen cursor movements.

### Week 19 - 25 May

Added haptic and visual feedback. Configured the PWM for the vibration motor and set up the RGB LED to indicate device status. Final code cleanup, debugging, and testing for the final presentation.

## Hardware

The project uses the STM32 Nucleo-U545RE-Q as the microcontroller to process inputs and handle USB communication. It uses an MPU6050 for motion tracking, a tactile push-button for digital input, an RGB LED for visual feedback, and a vibration motor for haptic feedback.

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| STM32 Nucleo-U545RE-Q | Microcontroller | 107 RON |
| MPU6050 Module | Motion Sensor | 12 RON|
| Tactile Push-Button | Digital Input | 0.36 RON |
| RGB LED Module | Visual Feedback | 2 RON |
| Vibration Motor Module | Haptic Feedback | 5 RON |
| USB Data Cable | Power & Communication | 30 RON |
| Jumper Wires | Physical Connectivity | 7 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| embassy-rs | The core asynchronous runtime | Allows the microcontroller to handle multiple tasks simultaneously without blocking |
| embassy-stm32 | Hardware Abstraction Layer for STM32 | Provides digital drivers to control physical pins, I2C, PWM, and USB peripherals |
| embassy-time | Manages non-blocking timers and delays | Crucial for stabilizing the input signal of the tactile button |
| embassy-usb | Handles the low-level USB device stack | Establishes and maintains physical data connection between STM32 and host computer |
| usbd-human-interface-device | USB HID formatter | Formats raw data into standard USB HID reports so the PC recognizes it instantly |
| mpu6050 | Dedicated community driver | Processes raw I2C signals into usable spatial angles for steering |
| defmt | Debugging framework | Prints real-time debugging messages to PC screen without slowing down the main loop |

## Links

1. [Makers Making Change](https://www.makersmakingchange.com/)
2. [Build Your Own Air Mouse](https://hackaday.com/2025/03/17/build-your-own-air-mouse-okay/)