---
layout: page
title: Laser Security System
---

# Laser Security System

:::info

**Author:** Popescu Carla-Indira \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/acs-project-2026-indira1505

:::

## Description

A laser-based security system built on the Nucleo-STM32U545RE-Q board, programmed in Rust using the Embassy framework. The system uses a KY-008 laser module and an LDR photoresistor to detect interruptions in the laser beam. When the beam is broken, an alarm is triggered via a buzzer and a red LED. The system can be armed and disarmed using an RC522 RFID module with MIFARE cards. System status is displayed on an SSD1306 OLED screen.

## Motivation

I was inspired by a YouTube video of an analog laser alarm circuit and wanted to recreate it in a more modern and complex way, using embedded Rust. This project allowed me to explore embedded programming with Rust and Embassy, while also building something visually impressive and functional. The laser beam makes the demo very striking for a live presentation.

## Architecture

The **Nucleo-STM32U545RE-Q** is the central control unit, managing all components:

- The **KY-008 laser module** emits a continuous beam aimed at the **LDR photoresistor**, which detects interruptions via ADC.
- The **RC522 RFID module** communicates via SPI and is used to arm/disarm the system using MIFARE Classic 1K cards.
- The **SSD1306 OLED display** communicates via I2C and shows the current system status (armed, disarmed, alarm triggered).
- The **red and green LEDs** provide visual status indication.
- The **active buzzer**, driven by a BC547 transistor, sounds the alarm when the beam is interrupted.
- **Tactile push buttons** allow manual reset during testing.

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

| Component | Usage |
|-----------|-------|
| Nucleo-STM32U545RE-Q | Main microcontroller |
| KY-008 Laser Module | Emits the laser beam |
| LDR Photoresistor | Detects laser beam interruption |
| RC522 RFID Module | Arms/disarms the system via MIFARE cards |
| SSD1306 OLED Display | Displays system status |
| Active Buzzer + BC547 | Sounds the alarm |
| Red LED | Alarm indicator |
| Green LED | System armed/safe indicator |
| Tactile Push Buttons | Manual reset |
| Resistors, Capacitors | Supporting components |

## Schematics

## Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| Nucleo-STM32U545RE-Q | Microcontroller | - |
| KY-008 Laser Module | Laser emitter | - |
| LDR Photoresistor | Light sensor | - |
| RC522 RFID Module | Access control | - |
| SSD1306 OLED Display | Status display | - |
| Active Buzzer | Alarm sound | - |
| BC547 Transistor | Buzzer driver | - |
| LEDs (red, green) | Visual indicators | - |
| Tactile Push Buttons | Manual reset | - |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| embassy-stm32 | STM32 Peripherals | Accessing board peripherals |
| embassy-executor | Async executor | Task management |
| embedded-hal | Hardware Abstraction Layer | Hardware access |
| ssd1306 | OLED Display driver | Controlling the display |
| mfrc522 | RFID driver | Reading MIFARE cards |

## Links
