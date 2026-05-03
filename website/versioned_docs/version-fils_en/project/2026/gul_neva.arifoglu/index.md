# Secure RFID Zhome

:::info 
**Author**: ARIFOGLU Gul Neva  \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-nevoss-s

:::


## Description

This project presents a smart home door system based on the STM32 microcontroller using the Rust programming language. The system provides secure and automated access control for a house entrance.

When a person approaches the door, a PIR motion sensor detects movement and activates the system. The user must scan an RFID card to gain access. If the card is valid, the door opens using a servo motor. If invalid, access is denied.

The system also uses LEDs and an LCD display to provide feedback, and a photoresistor to automatically control lighting after entry.

---

## Motivation

This project was chosen to simulate a real-life smart home system that combines security, automation, and embedded systems design. It provides practical experience with Rust in embedded environments and interaction with hardware components.

---

## Architecture
System workflow:

1. Person approaches the door  
2. PIR sensor detects motion  
3. RFID system activates  
4. Card is scanned  
5. STM32 verifies the card  
6. If valid:
   - Door opens (servo motor)
   - Green LED turns on
   - LCD displays welcome message  
7. If invalid:
   - Door remains closed
   - Red LED turns on  
8. After entry:
   - Light is controlled automatically using photoresistor  

---


## Log


## WEEK 1
* Project idea selection.
## WEEK 2
* Component research.  
## WEEK 3
* Hardware planning. 
## WEEK 4
* Initial setup.
## WEEK 7
* Started to connect components.

---


## Hardware

Main components:

- STM32 microcontroller  
- RFID module (RC522)  
- PIR motion sensor  
- Servo motor (SG90)  
- LCD display (16x2 I2C)  
- LEDs  
- Photoresistor  
- Breadboard and jumper wires  

The STM32 microcontroller controls all peripherals. The RFID module is used for authentication, the PIR sensor detects motion, and the servo motor controls the door mechanism. The LCD provides feedback to the user, while LEDs indicate system status. The photoresistor enables automatic lighting control.

---




## Bill of Materials

| Device                                                                                                                       | Usage                         | Price    |
| :--------------------------------------------------------------------------------------------------------------------------- | :---------------------------- | :------- |
| [STM32 NUCLEO](https://www.st.com/en/evaluation-tools/nucleo-boards.html)                                                    | The main microcontroller      | ~125 RON |
| [RFID RC522 Module](https://components101.com/wireless/rc522-rfid-module)                                                    | Card authentication           | ~40 RON  |
| [PIR Motion Sensor](https://components101.com/sensors/pir-sensor)                                                            | Motion detection              | ~10 RON  |
| [Servo Motor SG90](https://components101.com/motors/servo-motor-basics-pinout-datasheet)                                     | Door lock/unlock mechanism    | ~10 RON  |
| [LCD 16x2 I2C](https://lastminuteengineers.com/i2c-lcd-arduino-tutorial/)                                                    | Display system messages       | ~20 RON  |
| [LED Kit](https://www.emag.ro/kit-200-buc-led-3mm-5mm-de-diferite-culori-ai707/pd/D4DJYTMBM/)                                | Status indication (green/red) | ~5 RON   |
| [Photoresistor (LDR)](https://components101.com/sensors/ldr-light-dependent-resistor)                                        | Ambient light detection       | ~2 RON   |
| [Breadboard](https://www.emag.ro/breadboard-h-hct-tronic-830-puncte-de-conectare-abs-200x630-puncte-034-066/pd/DBNQ7R3BM/)   | Circuit prototyping           | ~20 RON  |
| [Jumper Wires](https://www.emag.ro/set-40-cabluri-jumper-tata-tata-pentru-breadboard-multicolore-20cm-034-030/pd/D18P4G3BM/) | Component connections         | ~20 RON  |




## Software

| Library                                                       | Description                                           | Usage                                                              |
| ------------------------------------------------------------- | ----------------------------------------------------- | ------------------------------------------------------------------ |
| [embassy-stm32](https://crates.io/crates/embassy-stm32)       | Hardware abstraction layer for STM32 microcontrollers | Used to control GPIO, timers, I2C and other peripherals            |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async task executor for embedded systems              | Runs concurrent tasks such as motion detection and RFID processing |
| [embassy-time](https://crates.io/crates/embassy-time)         | Time management utilities                             | Handles delays and timing operations                               |
| [embedded-hal](https://crates.io/crates/embedded-hal)         | Standard embedded hardware interfaces                 | Provides common interfaces for sensors and peripherals             |
| [mfrc522](https://crates.io/crates/mfrc522)                   | RFID reader driver                                    | Reads and processes RFID card data                                 |
| [lcd-lcm1602-i2c](https://crates.io/crates/lcd-lcm1602-i2c)   | LCD display driver                                    | Controls the 16x2 LCD display over I2C                             |
| [embedded-hal-bus](https://crates.io/crates/embedded-hal-bus) | Bus sharing utilities                                 | Allows multiple devices to share I2C communication                 |
| [defmt](https://crates.io/crates/defmt)                       | Lightweight logging framework                         | Used for debug messages                                            |
| [defmt-rtt](https://crates.io/crates/defmt-rtt)               | RTT transport for logs                                | Sends debug output to host PC                                      |
| [panic-probe](https://crates.io/crates/panic-probe)           | Panic handler for embedded debugging                  | Reports system crashes during development                          |

## Links

1. [link](https://embassy.dev/)
...
