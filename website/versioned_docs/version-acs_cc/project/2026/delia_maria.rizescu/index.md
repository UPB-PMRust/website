# Mini Vending Machine
A vending machine that releases products after validating user payments and selections.

:::info 

**Author**: Rizescu Delia Maria \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-Xdelia11

:::

## Description

The system receives user input through push buttons, each corresponding to a specific product selection. An STM32 microcontroller acts as the central processing unit, responsible for validating the inserted credit and checking product availability. If the selected item is out of stock or the transaction cannot be completed, the system rejects the inserted coin. When all conditions are met, the microcontroller controls a stepper motor that drives a spiral mechanism to dispense the chosen product. Additionally, the system features an LCD display that provides real-time messages to the user, including payment confirmation, product selection prompts, and error notifications.

## Motivation
I chose this project because it brings together multiple important concepts from both hardware and software in a practical context. I find it particularly interesting to work on a product that is already part of everyday life and explore ways to improve it. This project also gives me the opportunity to expand my understanding of Rust in a real-world scenario while designing a system that is both useful and engaging.

## Architecture 

The project is divided into a few main parts that work together.

Main Architectural Components:

1. Payment Processing Unit (Input)
Role: Handles credit validation
Components: RFID RC522 (digital payment) and Limit Switch (physical coin detection)

2. User Interface (I/O)
Role: Responsible for user selections and provides feedback.
Components: 4x Push Buttons, LCD, and Passive Buzzer.

3. Dispensing and Change System (Output)
Logic: Manages the mechanical release of products and the return of change.
Components: 4x Stepper Motors and SG90 Servo (coin release).


![Architecture Diagram](images/diagrama_arhitectura.svg)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Detail in a few words the hardware used.

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
| [STM32](https://www.st.com/resource/en/datasheet/stm32f722ic.pdf) | The microcontroller | 110 RON |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

1. [Hardware Ideas and Design](https://www.youtube.com/watch?v=DO3AciBz_-A&t=5s)
2. [Lab Support](https://embedded-rust-101.wyliodrin.com/docs/acs_cc/category/lab)
3. [DIY Coin Dispenser](https://www.youtube.com/watch?v=V7L139xnTR0)
4. [Vending Machine Mechanism](https://www.youtube.com/watch?v=-gdm71P1k9c&t=2s)
