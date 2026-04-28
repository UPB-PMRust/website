# The Focus Station

A desk-mounted environmental monitoring device that evaluates working conditions and computes a real-time Focus Score to help improve productivity.

:::info 

**Author**: Adelina Maria Alexe \
**GitHub Project Link**: [link_to_github](https://github.com/UPB-PMRust-Students/acs-project-2026-AdelinaAlexe)

:::

<!-- do not delete the \ after your name -->

## Description

The Focus Station is a device that monitors the environment at a user's desk and computes a Focus Score (0–100) representing how favorable the conditions are for concentrated work. It reads temperature, pressure, carbon monoxide levels, ultraviolet light intensity, ambient noise, and user presence. Based on these inputs, the device displays the score and individual sensor readings on a color LCD, controls a desk fan automatically, signals Pomodoro work/break intervals through a buzzer, and transmits all data over Bluetooth to a companion application on the user's computer for logging and visualization.

## Motivation

I chose this project because it addresses a real problem I experience daily. As someone who spends long hours at a desk, I often fail to notice when the room gets too warm, the air becomes stale, or the noise level rises, all factors that silently reduce concentration and productivity.

Commercial solutions either monitor only one parameter (like a simple thermometer) or are expensive smart-home systems designed for entire houses. I wanted a compact, single-device solution focused specifically on the desk environment.

Additionally, the Pomodoro timer integration makes the device a complete productivity companion rather than just a passive monitor.

## Architecture 

- **Sensor Subsystem**: Reads environmental data from multiple sensors (temperature/pressure via I2C, CO/UV via ADC, noise and presence via digital GPIO). Each sensor runs as an independent async task, sampling at 1–5 second intervals.
- **Focus Score Engine**: Receives data from all sensors and computes a weighted score every second. Penalties are applied for unfavorable conditions (high temperature, stale air, excessive noise, UV glare, low pressure, user absence).
- **Pomodoro Timer**: A state machine that cycles through IDLE → WORK (25 min) → BREAK (5 min) → WORK, controlled by a physical push button. Transitions trigger buzzer alerts.
- **Display System**: An ILI9341 LCD shows the Focus Score, sensor readings with color-coded status indicators, the Pomodoro countdown, and contextual recommendations (e.g., "Open a window").
- **Output Control**: A PWM-driven fan activates automatically when the temperature is high and the user is present. A passive buzzer generates distinct tones for Pomodoro transitions and low Focus Score alerts.
- **Communication Module**: An HC-05 Bluetooth module transmits sensor data and the Focus Score as JSON over UART to a PC application every 10 seconds.
- **PC Application**: A Python script receives data over Bluetooth serial, stores it in an SQLite database, and displays time-series graphs through a web dashboard.

### Architecture Diagram

<!-- TODO: Add diagrams.net diagram exported as SVG -->
![Architecture Diagram](images/architecture.svg)

## Log

<!-- write your progress here every week -->

### Week 4 - 10 May

### Week 11 - 17 May

### Week 18 - 24 May

## Hardware

The project uses an STM32 Nucleo-U545RE-Q as the main microcontroller board, featuring an ARM Cortex-M33 core with an integrated ST-LINK/V3E debugger. A 2.4" ILI9341 TFT display connected via SPI serves as the main user interface, showing the Focus Score, sensor readings, and the Pomodoro countdown. A BMP280 sensor connected via I2C measures ambient temperature and barometric pressure. An MQ-7 gas sensor on an ADC pin detects carbon monoxide levels through a voltage divider. A GUVA-S12SD UV sensor on an ADC pin measures ultraviolet light intensity near the desk. A KY-038 sound sensor on a digital GPIO pin detects excessive ambient noise. An HC-SR501 PIR sensor on a digital GPIO pin detects user presence at the desk. A passive buzzer driven by PWM emits Pomodoro transition tones and low Focus Score alerts. A mini 5V DC fan controlled via PWM through an IRLZ44N MOSFET activates automatically when the temperature is high and the user is present. An HC-05 Bluetooth module connected via UART transmits sensor data to a companion PC application. The on-board user button B1 on PC13 controls the Pomodoro timer, and the on-board LED LD2 on PA5 indicates the current Pomodoro state. All components are connected on a breadboard with jumper wires.

### Schematics

<!-- TODO: Add KiCad schematic exported as SVG -->
<!--![Schematic](schematic.svg)-->

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|-------|-------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main microcontroller board | Provided by university |
| [LCD ILI9341 2.4" TFT](https://cdn-shop.adafruit.com/datasheets/ILI9341.pdf) | Display for Focus Score, sensors, and Pomodoro timer | ~70 RON |
| [BMP280](https://cdn-shop.adafruit.com/datasheets/BST-BMP280-DS001-11.pdf) | Temperature and pressure sensor | ~8 RON |
| [MQ-7](https://www.pololu.com/file/0j313/mq7.pdf) | Carbon monoxide / air quality sensor | ~32 RON |
| [GUVA-S12SD](https://cdn-shop.adafruit.com/datasheets/1918guva.pdf) | UV radiation sensor | ~25 RON |
| [KY-038 Sound Sensor](https://www.openimpulse.com/blog/wp-content/uploads/wpsc/downloadables/Sound-Sensor-Schematic.pdf) | Noise detection | ~5 RON |
| [HC-05 Bluetooth Module](https://www.robofun.ro/wireless/modul-bluetooth-hc-05-master-slave-cu-buton.html) | Wireless communication with PC | ~25 RON |
| [Passive Buzzer Module](https://www.handsontec.com/dataspecs/module/passive%20buzzer.pdf) | Pomodoro alerts and Focus Score warnings | ~5 RON |
| Mini Fan 5V DC | Automatic desk cooling | ~15 RON |
| 1N4007 Diode | Flyback protection for fan motor | ~1 RON |
| Resistors (10kΩ, 20kΩ) | Voltage divider for MQ-7 analog output | ~2 RON |
| Breadboard + Jumper Wires | Prototyping connections | ~20 RON |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Embassy documentation](https://embassy.dev/book/)
2. [STM32U545 Datasheet](https://www.st.com/resource/en/datasheet/stm32u545re.pdf)
3. [Nucleo-U545RE-Q User Manual (UM3062)](https://www.st.com/resource/en/user_manual/um3062-stm32u3u5-nucleo64-boards-mb1841-stmicroelectronics.pdf)
4. [Rust on STM32 Workshop](https://rust.ipworkshop.ro/docs/embassy/)
