# Thermal Imaging Embedded System (STM32 + MLX90640)

:::info
**Author**: Ionescu Andrei \
**Group**: 1221ED \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-ionescuaandrei
:::

## Description

This project consists of a portable embedded thermal imaging system built on an STM32 microcontroller. The system uses an MLX90640 infrared thermal sensor to capture temperature data as a 32x24 pixel frame, processes the data locally on the microcontroller, and renders a real-time thermal image on a color TFT display.

The device is designed as a handheld unit, powered by a battery and enclosed in a 3D-printed case. It supports capturing and storing thermal frames on a microSD card, navigating a simple user interface using a joystick, and optionally transmitting captured images to a mobile application over WiFi.

## Motivation

The goal of this project is to build a real-time embedded system that performs acquisition, processing, and visualization of sensor data directly on the microcontroller.

Compared to simple IoT or game-based systems, this project focuses on:
- handling large sensor datasets (thermal frames)
- performing local data processing (temperature mapping, normalization)
- rendering graphical output in real time
- integrating multiple peripherals (display, storage, communication)

Additionally, the project aims to replicate the functionality of a handheld thermal camera, providing both engineering complexity and a strong visual demonstration.

## Architecture

```
        +----------------------+
        |   MLX90640 Sensor    |
        |   (32x24 IR Array)   |
        +----------+-----------+
                   |
                   | I2C (fast)
                   v
        +---------------------------+
        |   STM32 (Main Controller) |
        |  (Processing Pipeline)    |
        +-----------+---------------+
                    |
     +--------------+----------------------+
     |              |                      |
     v              v                      v
+----------+   +------------+      +---------------+
| TFT      |   | microSD    |      | WiFi Module   |
| Display  |   | Storage    |      | (ESP8266/ESP32)|
+----------+   +------------+      +-------+-------+
     |                                      |
     v                                      | WiFi (TCP/HTTP)
+-------------+                            v
| UI Control  |                   +------------------+
| (Joystick)  |                   |   Mobile App     |
+-------------+                   | (iOS / Android)  |
                                   +------------------+
```

## Hardware Connections

```
MLX90640 (I2C)
SDA -> I2C SDA
SCL -> I2C SCL

TFT Display (SPI)
MOSI -> SPI MOSI
SCK  -> SPI CLK
CS   -> GPIO
DC   -> GPIO
RST  -> GPIO

microSD (SPI)
MOSI -> SPI MOSI
MISO -> SPI MISO
SCK  -> SPI CLK
CS   -> GPIO

Joystick
X/Y -> ADC inputs
SW  -> GPIO

WiFi Module (optional)
TX/RX -> UART
```

## Mobile Application *(add on)*

A companion app built with **React Native + Expo** (iOS / Android) connects to the device over WiFi. A lightweight **Node.js + Express** server runs on the same network, receives raw frames from the ESP32 over HTTP, and serves them to the app.

**Features:** live thermal stream, captured image history, download to gallery, temperature overlay (min/max/center), device status.

**API endpoints:** `GET /frame`, `GET /history`, `GET /image/<name>`, `POST /settings`, `GET /status`.

## Log

### Week 4 (Idea Selection)
- Initial idea was a reflex-based system
- Pivoted to thermal imaging after feedback
- Selected MLX90640 as core sensor

### Week 5 (Research)
- Studied MLX90640 communication and data format
- Researched display options
- Planned processing pipeline

### Week 6 (Architecture Design)
- Defined system blocks and data flow
- Selected peripherals (SD, joystick, WiFi)

### Week 7 (Planned Implementation)
- Start hardware integration
- Test I2C communication with sensor

## Hardware

The system is built around an STM32 microcontroller (NUCLEO-U545RE-Q for development). The MLX90640 sensor provides a 32x24 array of temperature values via I2C. A TFT display is used for real-time visualization.

A microSD card module is used for storing captured frames. A joystick provides navigation, while a WiFi module enables optional communication with a mobile application.

## Bill of Materials

| Device | Usage | Price |
|--------|------|------|
| STM32 NUCLEO-U545RE-Q | Main controller | 125 RON |
| MLX90640 | Thermal sensor | ~150 RON |
| TFT Display | Visualization | ~40 RON (already owned) |
| microSD Module | Storage | ~15 RON |
| Joystick | UI | ~10 RON |
| WiFi Module | Communication | ~25 RON (already owned)  |
| Power | Battery/Power bank | ~50 RON (already owned) |
| Misc | Wires etc | ~50 RON  (already owned) |
| **Total** | | **~465 RON** |

## Software

### Firmware (STM32 — Rust / Embassy)

| Library | Description |
|--------|-------------|
| embassy-stm32 | Async HAL |
| embassy-executor | Task scheduler |
| embassy-time | Timers |
| embedded-hal | Hardware abstraction |
| mlx9064x | Thermal sensor driver |
| embedded-graphics | Rendering |
| st7735-lcd / ili9341 | Display driver |
| heapless | Memory-safe structures |
| embedded-sdmmc | SD card |

### Mobile Add-on (Node.js / Express + React Native / Expo)

| Package | Description |
|---------|-------------|
| `express` | REST API server |
| `multer` / `sharp` | Frame upload handling and JPEG encoding |
| `expo-image`, `expo-media-library` | Image display and gallery save |
| `axios` | API calls from the app |
| `zustand` | State management |

## Links

- https://roboticworx.io/blogs/projects/opentemp-thermal-imager-infrared-thermometer
- https://docs.rs/mlx9064x
- https://embassy.dev
