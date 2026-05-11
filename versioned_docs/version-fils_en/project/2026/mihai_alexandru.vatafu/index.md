# IoT Smartwatch with Health Monitoring

A smartwatch based on RP2040 developed in Rust using the Embassy framework.

:::info

**Author**: Mihai Alexandru Vătafu \
**GitHub Project Link**: [https://github.com/upb-pmrust/project-mihai_alexandru_vatafu](https://github.com/upb-pmrust/project-mihai_alexandru_vatafu)

:::

## Description
This project aims to create a functional IoT smartwatch using a Raspberry Pi Pico W and a circular IPS screen (GC9A01), focusing on power efficiency and real-time task management.

## Motivation
I chose this project to explore the capabilities of the Rust programming language in embedded systems, specifically using the Embassy framework for asynchronous multitasking.

## Architecture
The system architecture consists of several interconnected modules:
- **Display Driver Layer**: Manages SPI communication with the GC9A01 circular screen.
- **Power Management Module**: Monitors battery levels via ADC and handles low-power states.
- **Sensor Integration**: Reads heart rate and SpO2 data from the MAX30102 sensor via I2C.

### KiCad Schematic
*(Add your KiCad screenshot here after you upload it to the images folder)*

### 3D Model
The chassis was designed to fit the 250mAh Li-Po battery and the circular display.
[Download .step Model](./images/smartwatch_model.step)