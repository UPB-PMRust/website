# Smart Access Control System

:::info

**Author:** Popescu Carla-Indira \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/acs-project-2026-indira1505

:::

## Description

A laser-based smart access control system built on the Nucleo-STM32U545RE-Q board, programmed in Rust using the Embassy async framework. The system uses a KY-008 laser module aimed at an LDR photoresistor to detect intrusions. When the beam is interrupted, a red LED and an active buzzer are triggered as an alarm. Access control is handled via an RC522 RFID module — scanning a valid MIFARE card grants a 5–10 second window during which the laser can be interrupted without triggering the alarm. Scanning an invalid card three consecutive times locks the system and activates a continuous alarm, which can only be silenced by an admin-level card. All status messages (cardholder name, "Access Granted", "Access Denied") are displayed on an SSD1306 OLED screen.

## Motivation

I was inspired by a YouTube video of an analog laser alarm circuit and wanted to recreate it in a more modern, complex, and functional way using embedded Rust. The project gave me the opportunity to explore real-time embedded programming with Rust and Embassy, while building something visually striking for a live presentation. Unlike a simple PIR motion sensor, the laser beam creates a clear, visible security barrier that makes the demo immediately understandable and impressive. The added RFID access control layer transforms a simple alarm into a complete, realistic security system.

## Architecture

![Arhitectura sistemului](images/diagrama_pm.drawio.svg)

The **Nucleo-STM32U545RE-Q** is the central control unit, coordinating all components through an async state machine implemented in Embassy:

- The **KY-008 laser module** emits a continuous red beam (650nm, 5V) aimed at the **LDR photoresistor**. The LDR value is read via ADC and compared against a baseline calibrated at startup. A sudden drop in value signals beam interruption.
- The **RC522 RFID module** communicates via SPI at 3.3V. It reads MIFARE Classic 1K card UIDs, which are matched against a hardcoded list of authorized users and admin cards.
- The **SSD1306 OLED display** (128x64px) communicates via I2C and shows real-time system status: idle, access granted/denied, alarm active, or system locked.
- The **red LED** signals an active alarm; the **green LED** signals a successful access grant or system idle state.
- The **active buzzer**, driven by a BC547 NPN transistor (controlled via GPIO at 3.3V, powered at 5V), sounds the alarm when triggered.

The system operates as a state machine with the following states:
- **IDLE** — laser active, monitoring
- **ACCESS_GRANTED** — valid card scanned, grace period active (5–10s)
- **ALARM** — beam interrupted without valid access
- **LOCKED** — 3 consecutive invalid card scans, continuous alarm until admin card scanned

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

| Component | Quantity | Usage |
|-----------|----------|-------|
| Nucleo-STM32U545RE-Q | 1 | Main microcontroller |
| KY-008 Laser Module | 1 | Emits the laser beam (650nm, 5V) |
| LDR Photoresistor (5528) | 1 | Detects laser beam interruption via ADC |
| RC522 RFID Module | 1 | Reads MIFARE cards via SPI |
| MIFARE Classic 1K Cards | 5 | 1 admin + 3 users + 1 invalid test card |
| SSD1306 OLED Display (0.96") | 1 | Displays system status via I2C |
| Active Buzzer (5V) | 1 | Sounds the alarm |
| BC547 NPN Transistor | 1 | Drives the buzzer from 3.3V GPIO |
| Red LED (5mm) | 1 | Alarm indicator |
| Green LED (5mm) | 1 | Access granted / system OK indicator |
| Tactile Push Buttons (6x6mm) | 3 | Manual reset during testing |
| Resistor 220Ω | 2 | LED current limiting |
| Resistor 1kΩ | 1 | BC547 base protection |
| Resistor 10kΩ | 2 | LDR voltage divider + button pull-down |
| Breadboard (830 points) | 1 | Prototyping |
| Jumper wires (M-M, M-F) | ~50 | Connections |

## Schematics

## Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| Nucleo-STM32U545RE-Q | Microcontroller | (owned) |
| KY-008 Laser Module | Laser emitter | 3.15 RON |
| LDR Photoresistor 5528 (set of 5) | Light sensor | 14.14 RON |
| RC522 RFID Module + card + tag | Access control | 20.75 RON |
| MIFARE Classic 1K Cards (x3) | Access cards | 15.00 RON |
| SSD1306 OLED 0.96" I2C | Status display | 31.99 RON |
| Active Buzzer 5V | Alarm sound | 14.96 RON |
| BC547 NPN Transistor (x3) | Buzzer driver | 11.10 RON |
| LED Kit 200 pcs (red + green) | Visual indicators | 30.25 RON |
| Tactile Push Buttons (set of 25) | Manual reset | 37.50 RON |
| 220Ω Resistors (set of 10) | LED protection | ~5.00 RON |
| 1kΩ Resistors (set of 10) | Transistor base | ~5.00 RON |
| 10kΩ Resistors (set of 20) | LDR voltage divider | ~5.00 RON |
| Breadboard 830 points | Prototyping | 10.00 RON |
| Jumper wires 65 pcs | Connections | 10.16 RON |
| Cable set 40x10cm | Connections | 24.90 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://docs.embassy.dev) | STM32 async HAL | GPIO, SPI, I2C, ADC peripheral access |
| [embassy-executor](https://docs.embassy.dev) | Async executor | Concurrent task management |
| [embassy-time](https://docs.embassy.dev) | Async timers | Grace period timing, debounce |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware Abstraction Layer | Portable hardware interface traits |
| [ssd1306](https://crates.io/crates/ssd1306) | OLED display driver | Rendering text and status messages |
| [mfrc522](https://crates.io/crates/mfrc522) | RFID driver | Reading MIFARE Classic 1K card UIDs |
| [embedded-graphics](https://crates.io/crates/embedded-graphics) | 2D graphics library | Text rendering on OLED display |

## Links

1. [Embassy Framework Documentation](https://embassy.dev)
2. [STM32U545RE Reference Manual](https://www.st.com/en/microcontrollers-microprocessors/stm32u545re.html)
3. [ssd1306 crate documentation](https://docs.rs/ssd1306)
4. [mfrc522 crate documentation](https://docs.rs/mfrc522)
5. [Inspiration: Analog Laser Security Alarm (YouTube)](https://www.youtube.com/watch?v=RnR_K6qvaa8)