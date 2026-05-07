# AgriCheck

An agriculture-oriented multi-sensor reader.

:::info

**Author**: Stancu Stefanita-Ionut \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-Stefan5419

:::

<!-- do not delete the \ after your name -->

## Description

This project implements an IoT-based agriculture monitoring system that collects environmental data from field sensors, transmits it over LoRaWAN, and makes it accessible through a web application for visualization.

## Motivation

I particularly choose this project because it offers a solution to my personal problems, and I figured out it may help other agriculture-oriented tech enthusiasts.

## Architecture

The system follows a layered IoT architecture composed of multiple interconnected components responsible for data collection, transmission, processing, and visualization.

**Main Components** :
Data Acquisition Layer (Sensor Node)

Responsible for collecting environmental data from field sensors and preparing it for transmission.

**Communication Layer (LoRaWAN Network)**

Handles long-range wireless transmission of data from the sensor node to the network infrastructure via LoRaWAN.
Network Server (TTN)

Manages device authentication, payload decoding, and routing of data to external services.

**Backend Layer**

Processes incoming data, stores it in a database, and exposes it through APIs for further use.

**Application Layer (Web Interface)**

Provides a user-friendly interface for real-time monitoring and visualization of environmental data.

## Log

<!-- write your progress here every week -->

### Week 5 - 6

Finalized component research and ordered all necessary hardware. Waiting for delivery.
Already had the microcontroller, some dummy thermistor and photoresistor and started testing software.

## Hardware

**Hardware Used**
STM32 microcontroller, environmental sensors (soil moisture, temperature, humidity, light), LoRa transceiver (SX1276/SX1262), and a LoRa gateway for wireless data transmission(the gateway is a public one which is hosted on TTN).

### Schematics

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device                  | Usage                 | Price    |
| ----------------------- | --------------------- | -------- |
| Nucleo STM32            | The microcontroller   | BORROWED |
| Kit Arduino Intermediar | I will use some parts | 360 RON  |

### Kit components that will be used

- 1x Breadboard 830 tie-points MB-102
- 1x Breadboard power shield
- 1x Set of male-to-male jumper wires
- 1x Set of male-to-female Dupont wires
- 10K resistors
- 4.7K resistors
- 1x DHT11 temperature and humidity sensor
- 2x LDR 5516 photoresistors
- 1x NTC 10K thermistor
- 1x Water level sensor
- 1x 3.3V / 5V breadboard power supply module
- 1x 9V battery connector
- 2x PN2222A NPN transistors
- 2x 1N4007 diodes

## Software

| Library                                                       | Description                            | Usage                                  |
| ------------------------------------------------------------- | -------------------------------------- | -------------------------------------- |
| [stm32u5xx-hal](https://github.com/stm32-rs/stm32u5xx-hal)    | Hardware abstraction layer for STM32U5 | Access GPIO, SPI, I2C and peripherals  |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Embedded hardware abstraction traits   | Interface used by drivers and HAL      |
| [cortex-m-rt](https://github.com/rust-embedded/cortex-m-rt)   | Runtime for Cortex-M                   | Startup code and interrupt handling    |
| [cortex-m](https://github.com/rust-embedded/cortex-m)         | Cortex-M support crate                 | Low-level CPU functionality            |
| [lorawan-device](https://crates.io/crates/lorawan-device)     | LoRaWAN protocol stack                 | Handles OTAA, encryption and MAC layer |
| [embassy](https://github.com/embassy-rs/embassy)              | Async embedded framework               | Low-power async task execution         |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->
