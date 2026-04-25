# Smart Security System 

A modular security system for a miniature house with multi-sensor detection, configurable alerts, and remote access.

:::info

**Author**: Cranganu Claudia \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/fils-project-2026-clufluturas](https://github.com/UPB-PMRust-Students/fils-project-2026-clufluturas)

:::

## Description

This project is a modular security system for a miniature house, built on an STM32 development board and controlled through a companion app for remote access. The system can detect multiple types of security events: door opening, movement near the entrance, indoor motion, water leaks, and smoke detection. For each module, the user can independently choose the type of alert: a local sound alarm, a remote notification, or both. The system can be armed or disarmed remotely, and each security module can be turned on or off individually.

## Motivation

 The inspiration for this project came from my own house, which already has security systems in place for things like doors and flood detection. I wanted to recreate something similar in a miniature form, combining the sensors I find most useful in real life into one system that I could actually build and control myself. It was a good opportunity to understand how these kinds of systems work from the inside.

## Architecture

The system is organized into four main components that work together to provide detection, decision-making, alerting, and remote control.

- **Sensor Layer:** This layer includes all the sensors used by the system: the magnetic door sensor, PIR motion sensor, ultrasonic sensor, water sensor, and smoke sensor. Each sensor is responsible for detecting a specific type of event and sending the corresponding signal to the microcontroller.
- **STM32 Controller:** The STM32 acts as the core of the system. It reads the sensor inputs, checks the current configuration of the system, and decides how to respond depending on whether the system is armed, which modules are active, and which alert type is selected.
- **Communication Module:** This module provides the connection between the STM32 and the companion application. It is used to send notifications and system status information, while also receiving commands related to arming, disarming, and module configuration.
- **Companion Application:** The application offers the user an interface for interacting with the system remotely. It allows arming or disarming the system, enabling or disabling individual modules, selecting the alert type for each module, and monitoring the current status of the system.

```text
┌─────────────────────────────────────────────────────────┐
│                    Sensor Layer                         │
│  [Door] [PIR] [Ultrasonic] [Water] [Smoke]              │
└────────────────────┬────────────────────────────────────┘
                     │ GPIO / ADC
┌────────────────────▼────────────────────────────────────┐
│                 STM32 Controller                        │
│   - Armed/disarmed state                                │
│   - Module activation logic                             │
│   - Alert selection logic                               │
│   - Buzzer + LED output control                         │
└────────────────────┬────────────────────────────────────┘
                     │ UART / SPI / I2C
┌────────────────────▼────────────────────────────────────┐
│                Communication Module                     │
│                 Wi-Fi data exchange                     │
└────────────────────┬────────────────────────────────────┘
                     │ Wireless
┌────────────────────▼────────────────────────────────────┐
│                Companion Application                    │
│   - Arm / Disarm                                        │
│   - Module control                                      │
│   - Alert configuration                                 │
│   - Status monitoring                                   │
└─────────────────────────────────────────────────────────┘
```

## Log

### Week 5

I finalized the main idea of the project and decided on the overall direction of the system. I spent time researching similar security solutions and thinking about which features would be the most useful to include in my own project. I also ordered the STM32 development board and completed the initial idea documentation.

### Week 6-7

I ordered the rest of the hardware components needed for the project. After receiving them, I began testing some of them individually to better understand how they work and to make sure they could be integrated into the final system.

### Week 8-9

I started working on the physical structure of the project by building the miniature house. I began cutting the main parts, shaping the doors, and preparing the layout so the sensors could later be integrated into the correct positions.

![Miniature house in progress](house.webp)

## Hardware

The system uses an STM32 development board as the main controller. It receives input from the sensors, processes the security logic, and controls both the local alert outputs and the wireless communication with the companion application.

The hardware setup includes multiple sensors, each covering a different security scenario: a magnetic sensor for door monitoring, a PIR sensor for indoor motion detection, an ultrasonic sensor for perimeter monitoring, a water sensor for flood detection, and a gas sensor for fire-related situations. For local feedback, the system will use an acoustic alert output and visual status indicators, while remote communication is handled through a Wi-Fi module.

## Schematics

Place your KiCad or similar schematics here in SVG format.

## Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| STM32 Development Board | Main microcontroller | 120 RON |
| Ultrasonic Sensor (HC-SR04P) x2 | Perimeter monitoring | 20 RON |
| Water Sensor x3 | Flood detection | 5 RON |
| Gas / Smoke Sensor (MQ-2) | Smoke detection | 10 RON |
| Breadboard (MB102) | Circuit prototyping | 17 RON |
| Piezo Buzzer | Local sound alert | 8 RON |
| Magnetic Door Sensor (MC38) x2 | Door monitoring | 12 RON |
| Wi-Fi Module (ESP8266 ESP-01) | Wireless communication | 21 RON |
| ESP-01 Adapter Module | ESP8266 support module | 10 RON |
| LEDs + Resistors + Jumpers | Visual indicators and circuit support | 15 RON |
| LED Bar Display (10-segment) | Status indication | 5 RON |
| PIR Sensor (SR602) x3 | Indoor motion detection | 25 RON |
| Alarm Siren | Acoustic alert output | 23 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| `embassy-stm32` | Embassy HAL for STM32 microcontrollers | GPIO, timers, UART, sensor integration |
| `embassy-executor` | Async task executor | Running concurrent tasks for sensors and alerts |
| `embassy-time` | Async timers and delays | Timing, debouncing, periodic checks |
| `embassy-sync` | Synchronization primitives for Embassy | Shared state between async tasks |
| `embassy-futures` | Utilities for async control flow | Task coordination and async logic |
| `embedded-hal` | Hardware abstraction traits for embedded systems | Generic sensor and peripheral interfaces |
| `cortex-m` | ARM Cortex-M support crate | Low-level MCU support |
| `cortex-m-rt` | Runtime crate for Cortex-M devices | Startup and interrupt vector support |
| `panic-halt` | Minimal panic handler | Safe stop on unrecoverable errors |
| `heapless` | Fixed-capacity data structures for `no_std` systems | Buffers, queues, and small data storage |
| UART / Serial communication layer | Communication with the ESP8266 module | Sending commands and receiving remote control data |
| Companion app framework (TBD) | Interface for remote control and notifications | User interaction and system monitoring |

## Links

1. [Embassy documentation](https://embassy.dev/)
2. [STM32 NUCLEO-U545RE-Q product page](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)
3. [ESP8266 overview](https://www.espressif.com/en/products/socs/esp8266)
4. [HC-SR04 ultrasonic sensor datasheet](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf)
5. [MQ-2 gas sensor datasheet](https://www.pololu.com/file/0J309/MQ2.pdf)
6. [SR602 PIR sensor page / datasheet](https://www.robotics.org.za/SR602)
