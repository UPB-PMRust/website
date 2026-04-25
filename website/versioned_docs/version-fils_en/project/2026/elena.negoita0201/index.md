---
id: index
title: Color Parking Car
sidebar_label: Color Parking Car
---

# Color Parking Car

An autonomous four-wheel-drive robot using an STM32 microcontroller and Rust to identify specific parking bays based on color-coded floor markers.

:::info
Author: Elena-Daniela NEGOIȚĂ
Group: 1222EEB
GitHub: [Color Parking Car Repository](https://github.com/UPB-PMRust-Students/fils-project-2026-daniela-negoita)
:::

## Description
This project features an autonomous robot built on the STM32U545RE-Q. The robot traverses a path to find one of three parking spots identified by Red, Green, or Blue floor markers. Since the motors lack rotation sensors, the robot utilizes precise time-based control and specific motor power levels to execute turns and reversals.

## Motivation
The project explores building autonomous systems with Rust. It demonstrates how a 4WD vehicle can navigate using only color cues and timed logic, rather than expensive distance sensors.

## Architecture
The software is structured as a State Machine with four distinct modes:
* Mode 1 (Driving): Drives forward while scanning for colors.
* Mode 2 (Checking): Confirms the detected color matches the target while filtering out shadows.
* Mode 3 (Parking): Executes a script of timed movements, such as turning left for 1.2s and reversing for 0.8s.
* Mode 4 (Finished): Stops all motors and enters a waiting state.

## Log
* Week 5 - 11 May: to do.
* Week 12 - 18 May: to do.
* Week 19 - 25 May: to do.

## Hardware
The chassis uses four independent DC motors controlled via Pulse Width Modulation (PWM) for speed adjustment and calibration.

| Device | Usage |
| :--- | :--- |
| STM32 Nucleo-U545RE-Q | Main Microprocessor |
| DC Gear Motors | Propulsion (Yellow TT style) |
| TB6612FNG | Dual H-Bridge Motor Drivers |
| TCS34725 | RGB Color Sensor (I2C) |
| 18650 Li-ion Cells | 3.7V Power Source |
| LM2596 | DC-DC Buck Converter |

## Software
The system is built on the Embassy async framework.

| Library | Usage |
| :--- | :--- |
| embassy-stm32 | Hardware abstraction for STM32U5 |
| tcs34725 | RGB Color Sensor driver |
| embedded-hal | Hardware Abstraction Layer |
| defmt | Efficient logging for debugging |
| cortex-m-rt | Startup code and runtime |