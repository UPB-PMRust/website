# RoboMop-The Remote Controlled Cleaning Robot
A smart cleaning robot controlled via WiFi with obstacle detection and LCD feedback.

:::info
**Author:** Manole Eliza-Maria \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/fils-project-2026-elizamanole
:::

## Description
RoboMop is a multi-sensor autonomous cleaning robot built on a 4WD car platform and is controlled by a Nucleo STM32U545RE-Q microcontroller, but also has remote WiFi capabilities through the ESP32 controller. 
My system uses multiple ultrasonic sensors for environmental which facilitate obstacle avoidance and navigation. The machine has a dual servo-driven mechanism placed at the rear side that moves up and down and side to side creating a real cleaning motion with the help of a sponge.
Also, an LCD display is placed on the machine to monitor the cleaning status and the runtime.

## Motivation
The goal of this project is to create a simple, low-cost and very practical cleaning robot that uses embedded systems programming in Rust, Real-Time motor and servo control autonomous movement with the help of the 4 different wheel motors and multiple other components. Personally, I love cleaning and this robot helps me keep my room tidy easily.

## Architecture
The system is structured around two main subsystems: the real-time control unit and the communication unit. 
The STM32 communicates with all components through different interfaces:
- **GPIO:** ultrasonic sensors for obstacle detection that are placed on the front and sides of the car
- **PWM:** controls DC motors via L298N and the servo motors for the cleaning mechanism
- **I2C:** LCD Display that is used for the cleaning status and runtime
- **UART:** Communication with ESP32 for WiFi control and data exchange
- **Timers:** Used for precise timing and task scheduling
![Architecture Diagram](./images/architecture.drawio.svg)

## Log
### Week 2-4
Form my first ideas and search ideas for my project.
### Week 5-7
Researched the market for the adequate components for the RoboMop and placing all the orders.
### Week 8
I received the components and tested them to ensure proper functionality.
### Week 9
Starting to assemble the project.

## Hardware
The physical pary of my project consists of the STM32U545RE-Q microcontroller as the main source of processing power. The ESP32 module enables the user to remotely communicate with the STM32 in order to give out commands.The ultrasonic sensors detect any unwanted objects in the way of the car, each wheel is powered by a motor and the most important part of the machine is the dual servo powered cleaning system that can move up and down and side to side in order for it to adapt to different cleaning scenarious.

### Schematics

### Bill of Materials
|  Device  |  Usage  |  Price  |
|----------|---------|---------|
| [STM32 Nucleo U545RE-Q](https://www.digikey.ro/en/products/detail/stmicroelectronics/NUCLEO-U545RE-Q/22106570) | The main microcontroller used | 126 RON |
| [ESP32 Development Board](https://sigmanortec.ro/placa-dezvoltare-esp32-ch340c-30p-usb-c-wifi-si-bluetooth) | Wi-Fi communication module | 41.19 RON |
| [L298N Dual H-Bridge Motor Driver](https://sigmanortec.ro/Punte-H-Dubla-L298N-p125423236) | Controls DC motors | 11 RON |
| [Smart Car 4WD Chassis Kit](https://sigmanortec.ro/Kit-sasiu-Smart-Car-4WD-p136281803) |Robot structure and mobility | 83 RON |
| [SG90 Servo Motor(180 degrees)](https://sigmanortec.ro/Servomotor-SG90-limit-switch-p141662062) | Drives cleaning mechanism | 19 RON |
| [Servo Mount(2-axis PT for SG90)](https://sigmanortec.ro/montura-servomotor-suport-camera-2-axe-pt-antivibratii-ptz-pentru-sg90-mg90s) | Supports cleaning mechanism | 8.5 RON |
| [HC-SR04 Ultrasonic Sensor](https://sigmanortec.ro/Senzor-Ultrasunete-HC-SR-04P-3-5-5V-p148477760) | Obstacle detection system | 30 RON
| [HC-SR04 Sensor Mount](https://sigmanortec.ro/suport-senzor-ultrasunete-hc-sr04) | Mechanical support for sensors | 12 RON |
| [LCD 1602 16x2 Display 5V](https://sigmanortec.ro/LCD-1602-p125700685) | Displays runtime and cleaning status | 11 RON |
| [I2C Interface Module for LCD 1602](https://sigmanortec.ro/Modul-interfata-I2C-LCD-1602-2004-p125700577) | Enables I2C communication | 5 RON |
| [Battery Holder 6xAA with switch](https://sigmanortec.ro/Suport-6-baterii-AA-cu-capac-si-intrerupator-p209447210) | Power supply | 11.5 RON |
| [AMS1117 Voltage Regulator](https://sigmanortec.ro/modul-coborator-tensiune-ams1117-6-12v-la-5v) | Voltage stabilization | 5 RON |
| [Active Buzzer](https://sigmanortec.ro/Buzzer-piezoelectric-activ-3-24V-HND-2312-p136261756) | Sound feedback | 8.5 RON |
| [PCB Prototyping Board](https://sigmanortec.ro/Placa-PCB-prototipare-o-fata-9x15-p190990342) | Final circuit mounting | 10 RON |
| [Silicone Wires](https://www.optimusdigital.ro/ro/kituri/11950-set-de-fire-siliconice-24awg-6-culori-9m-fiecare-0721248989734.html) | Electrical connection between components | 10 RON |

## Software
|  Library  |  Description  |  Usage  |
|-----------|---------------|---------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for STM32 microcontrollers | Controls GPIO, PWM(motors, buzzer,servo),I2C(LCD) and UART(ESP32) |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor | Runs concurrent tasks such as sensor reading and motor control |
| [embassy-time](https://github.com/embassy-rs/embassy) | Timing library | Handles delays and runtime measurement |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer | Provides generic interfaces for sensors and motors/servos |
| [cortex-m](https://github.com/rust-embedded/cortex-m) | Low-level microcontroller support | Access to STM32 core features |
| [panic-probe](https://github.com/knurling-rs/probe-run) | Debugging tool | Used for error handling and debugging |
| [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) | ESP32 HAL | Controls ESP32 hardware |
| [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc) | WiFi services | Enables communication between ESP32 and STM32 |

## Links
1. [Embassy Book](https://embassy.dev/book/)
2. [STM32 Documentation](https://www.st.com/resource/en/user_manual/um3062-stm32u3u5-nucleo64-boards-mb1841-stmicroelectronics.pdf)
3. [ESP32 Documentation](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/index.html)