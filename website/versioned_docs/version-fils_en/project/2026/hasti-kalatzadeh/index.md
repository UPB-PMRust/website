# Smart Fire Detection and Suppression System

An embedded smart fire safety system built with STM32 and Rust using Embassy async framework.

:::info

**Author**: Hasti Kalatzadeh \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-hasti-kalatzadeh

:::

## Description

This project is a smart fire detection and suppression system based on an STM32 microcontroller programmed in Rust.  
The firmware uses the Embassy async framework to run multiple tasks concurrently.

The system continuously monitors ambient temperature using a temperature sensor. When the measured temperature exceeds a predefined safety threshold, the system enters danger mode.

In danger mode:

- A buzzer alarm is activated
- A water pump starts automatically
- The LCD displays the warning message **DANGER**

When the temperature returns to a safe value:

- The pump stops
- The buzzer turns off
- The LCD displays **SAFE**

A push button is included for manual emergency stop. A potentiometer allows manual control of the water pump speed / pressure.

## Motivation

Fire hazards can happen unexpectedly and cause serious damage. I wanted to build a practical system that can automatically detect dangerous conditions and immediately react.

This project combines sensors, displays, motors, alarms, user input, and async embedded programming in Rust — covering many of the concepts studied throughout the course.

## Architecture

The STM32 microcontroller acts as the central controller, coordinating all peripherals through Embassy async tasks:

- **Sensor Task** — reads temperature from the DHT11 sensor continuously
- **Display Task** — updates the LCD 16x2 with current status (**SAFE / DANGER**)
- **Alarm Task** — controls the buzzer based on system state
- **Pump Task** — activates or stops the water pump via the MOSFET module
- **Input Task** — reads the push button for manual emergency stop

Peripheral interfaces used:

- **GPIO** — button input and buzzer output
- **Digital Sensor Input** — DHT11 temperature sensor
- **PWM / GPIO Output** — water pump activation through MOSFET
- **I2C** — LCD 16x2 display
- **ADC** — potentiometer for pump speed adjustment

## Log

### Week 5 - 11 May

Project idea selected and requirements defined. Components researched and ordered. Rust embedded environment and Embassy framework configured.

### Week 12 - 18 May

Hardware assembled on breadboard. Implemented sensor reading task and threshold detection logic. Tested buzzer and pump switching via MOSFET.

### Week 19 - 25 May

Integrated LCD display over I2C. Added push button debounce and manual stop logic. Potentiometer ADC reading connected to pump PWM speed control. Full system integration and testing.

## Hardware

The project is based on an STM32 development board which provides GPIO pins, timers, ADC inputs, and I2C communication. The DHT11 sensor reads temperature digitally. The water pump is switched by a MOSFET driver module controlled via GPIO/PWM. An LCD 16x2 screen shows system status over I2C. A buzzer provides audio alerts. A push button handles manual emergency stop and a potentiometer adjusts pump speed.

### Schematics

See schematic.svg

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| STM32 Development Board | Main controller | 104 RON |
| DHT11 Temperature Sensor Module | Temperature monitoring | 15.55 RON |
| LCD 16x2 with I2C backpack | Status display | 23.22 RON |
| Mini Water Pump 3-6V | Water spraying | 13.31 RON |
| MOSFET IRF520 Module | Pump switching | 12.60 RON |
| Active Buzzer Module | Audio alarm | 7.42 RON |
| Push Button | Manual emergency stop | 6.36 RON |
| Potentiometer 10K | Pump speed adjustment | 8.31 RON |
| Breadboard 830 points | Prototyping | 10.00 RON |
| Jumper Wires Set | Connections | 10.00 RON |
| PVC Tube | Water output routing | 2.49 RON |
| Resistor Set | Pull-up / protection | 5.00 RON |
| Diode 1N4007 | Pump protection | 2.00 RON |
| **Total** | | **~220 RON** |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | Async HAL for STM32 | Peripheral drivers for GPIO, PWM, ADC, I2C |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async task executor | Runs concurrent tasks for sensor, LCD, buzzer, pump |
| [embassy-time](https://crates.io/crates/embassy-time) | Async timers and delays | Periodic temperature monitoring and scheduling |
| [embassy-sync](https://crates.io/crates/embassy-sync) | Channels and mutexes | Shares state between async tasks |
| [embedded-hal](https://crates.io/crates/embedded-hal) | Hardware abstraction traits | Generic peripheral interfaces |
| [hd44780-driver](https://crates.io/crates/hd44780-driver) | LCD driver for HD44780 | 16x2 display control over I2C |
| [cortex-m](https://crates.io/crates/cortex-m) | ARM Cortex-M support | Low level MCU support |
| [cortex-m-rt](https://crates.io/crates/cortex-m-rt) | Runtime support | Startup and interrupt vectors |
| [panic-halt](https://crates.io/crates/panic-halt) | Panic handler | Stops execution on panic |

## Links

1. [Embassy-rs async embedded framework](https://embassy.dev/)
2. [Rust Programming Language](https://www.rust-lang.org/)
3. [Embassy GitHub Repository](https://github.com/embassy-rs/embassy)
4. [STM32-rs HAL crates](https://github.com/stm32-rs)
