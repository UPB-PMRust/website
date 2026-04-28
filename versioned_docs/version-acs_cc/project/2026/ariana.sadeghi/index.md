---
id: ariana.sadeghi
title: Automated Optical Sorting System
sidebar_label: Automated Optical Sorting System
---

# Automated Optical Sorting System

## 1. General Description
The Automated Optical Sorting System is an embedded project designed to identify and sort objects based on their optical properties. Using an STM32 microcontroller, the system moves items along a conveyor belt and uses a light sensor to trigger a sorting arm.

## 2. Hardware Design
The system is currently in the **hardware assembly and mechanical prototyping phase**.

### Components:
* **STM32 Nucleo-U545RE-Q** (Processing Unit)
* **PMIMA V2.0 Shield** (Photoresistor, LCD, Buttons)
* **28BYJ-48 Stepper Motor** (Conveyor Drive)
* **SG90 Micro Servomotor** (Sorting Arm)

### Block Diagram
The following diagram represents the planned connections and communication protocols:

![Hardware Block Diagram](./schema_bloc.svg)

## 3. Software Logic (Preliminary)
The software is currently being developed. The high-level logic follows these main stages:
1. **Idle/Movement:** The stepper motor runs at a constant speed to move the conveyor.
2. **Object Detection:** The system continuously polls the ADC value from the photoresistor.
3. **Decision Making:** If the light intensity drops below a certain threshold, the system identifies an object and determines its category.
4. **Action:** The servomotor is activated via PWM to move the arm and sort the object.

## 4. Current Results
* **Hardware:** The mechanical structure and the conveyor belt are currently being assembled.
* **Software:** Initial tests for motor control and sensor reading have been performed successfully in isolation.
* **Integration:** System integration is pending completion of the mechanical assembly.

## 5. Future Work & Conclusions
The next steps involve:
* Finalizing the mechanical alignment between the sensor and the sorting arm.
* Tuning the threshold values for different types of objects.
* Implementing the final state machine in the control software.

## 6. Resources
* [Rust Embedded Book](https://docs.rust-embedded.org/book/)
* Microprocessor Architecture course labs (UPB)
* STM32U545RE Datasheet.