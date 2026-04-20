# Rusty Drone
A compact freestyle drone powered by an STM32 and ESP32 that streams real-time video over WiFi and is controlled via Bluetooth using a PS4/PS5 controller.

:::info 

**Author**: DUTU Ana-Antonia \
**GitHub Project Link**: https://github.com/anto987678
:::

<!-- do not delete the \ after your name -->

## Description

This project aims to develop a compact freestyle drone using Rust for embedded systems, combining real-time flight control on an STM32 microcontroller with wireless communication handled by an ESP32 module. The drone will stream live video over WiFi while being controlled via Bluetooth using a PS4/PS5 controller, and will also provide telemetry data such as altitude, temperature, speed, and battery level.

## Motivation

I chose this project because it combines multiple areas that interest me, such as embedded systems, wireless communication, and real-time control. Building a drone allows me to work with both hardware and software, while also gaining practical experience with Rust in an embedded context. Additionally, the project is challenging and interactive, involving sensors, communication protocols, and control algorithms, making it a great opportunity to better understand how complex systems are designed and integrated.

## Architecture 

Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The project uses an STM32 Nucleo U545RE-Q microcontroller for flight control, along with an ESP32-CAM module for WiFi video streaming and Bluetooth communication. It includes brushless motors with a 4-in-1 ESC, an MPU6050 IMU for motion sensing, and BMP280 and MAX6675 sensors for environmental data. Power is provided by a LiPo battery with a step-down converter, all mounted on a 5-inch drone frame with standard supporting components.

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
| [STM32 Nucleo U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | [~120 RON](https://www.digikey.ro/en/products/detail/stmicroelectronics/NUCLEO-U545RE-Q/22106570) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor for embedded systems | Runs concurrent tasks like sensor reading and motor control |
| [embassy-time](https://github.com/embassy-rs/embassy) | Time management utilities | Handles delays, timers, and scheduling |
| [embassy-sync](https://github.com/embassy-rs/embassy) | Synchronization primitives | Enables safe communication between async tasks |
| [embassy-stm32](https://github.com/embassy-rs/embassy) | HAL for STM32 microcontrollers | Interfaces with peripherals like GPIO, UART, I2C |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction traits | Standard interface for embedded components |
| [embedded-hal-async](https://github.com/rust-embedded/embedded-hal) | Async version of embedded-hal | Enables non-blocking peripheral communication |
| [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) | HAL for ESP32 | Interfaces with ESP32 hardware |
| [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc) | High-level ESP32 services | Provides WiFi and Bluetooth functionality |
| [embassy-net](https://github.com/embassy-rs/embassy) | Async networking stack | Handles WiFi communication |
| [smoltcp](https://github.com/smoltcp-rs/smoltcp) | TCP/IP stack | Used internally for network protocols |
| [mpu6050](https://github.com/japaric/mpu6050) | IMU driver | Reads acceleration and gyroscope data |
| [nalgebra](https://github.com/dimforge/nalgebra) | Linear algebra library | Processes sensor data and calculations |
| [pid](https://github.com/braincore/pid-rs) | PID controller implementation | Stabilizes the drone during flight |
| [panic-probe](https://github.com/knurling-rs/probe-run) | Debugging and panic handler | Helps with runtime error debugging |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Embassy Book](https://embassy.dev/book/#_what_is_embassy)
2. [STM32 32-bit Arm Cortex MCUs - Documentation](https://www.st.com/en/microcontrollers-microprocessors/stm32-32-bit-arm-cortex-mcus/documentation.html)
3. [MPU6050 documentatiom] (https://docs.sunfounder.com/projects/ultimate-sensor-kit/en/latest/components_basic/05-component_mpu6050.html)
