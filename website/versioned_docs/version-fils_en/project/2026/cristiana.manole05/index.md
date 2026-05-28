# Smart Garage Climate and Presence Control System

An IoT-based smart garage system for vehicle detection, automatic lighting, climate monitoring, and smart ventilation control.

:::info 

**Author**: Manole Cristiana \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-CristianaManole2005

:::


## Description

This project aims to develop a smart garage system using the NUCLEO-U545RE-Q development board, fully programmed in Rust. The system combines vehicle detection, automatic lighting, environmental monitoring, ventilation control, real-time monitoring and Ethernet networking.

Ultrasonic sensors will detect the presence of a vehicle inside or near the garage. When a vehicle is detected, the garage lights will automatically turn on. If no vehicle is detected for a period of time, the lights will turn off automatically.

A temperature and humidity sensor will monitor the garage environment. If the measured values exceed predefined thresholds, a ventilation system simulated using a motor with fan blades will automatically start. The fan can also be controlled manually using push buttons.

Limit switches will be used to monitor the garage door open and closed positions. LEDs will provide visual feedback for vehicle presence, fan status, light status, and fault conditions.

The system will communicate over Ethernet using the W5500 module and send real-time data to a monitoring dashboard running on a laptop connected to the same local network. The dashboard will display sensor values, vehicle presence, fan status, light status, door state, and operating mode.

## Motivation

The smart garage idea is a useful real-world scenario that can improve comfort, safety, and energy efficiency. It presents a practical example of how intelligent monitoring and automatic control can be integrated into a modern garage solution. It also offers the opportunity to develop both hardware and software components using Rust on an embedded platform.

## Architecture

This system is built using the **STM32 NUCLEO-U545RE-Q** development board, which coordinates all system functions.

- **Vehicle Detection Module:** Detects the presence of a car inside or near the garage using ultrasonic sensors and sends the information to the control unit for automatic lighting and monitoring.

- **Environmental Monitoring Module:** Reads temperature and humidity values using the dedicated sensor and sends the data to the controller for ventilation decisions.

- **Control Logic Module:** Processes all sensor inputs and decides when to activate the garage lighting, start or stop the ventilation system, and manage the overall operating mode of the system.

- **Actuator Control Module:** Controls the physical outputs of the system, including the garage LEDs, the fan motor, and other status indicators.

- **Networking Module:** Uses the W5500 Ethernet interface to transmit real-time data such as sensor values, vehicle presence, fan status, light status, and garage door state to a monitoring dashboard running on a laptop.

- **User Interface Module:** Includes push buttons for manual fan control and LEDs that provide visual feedback for the current operating state and fault conditions.

![Schema arhitectura](schema.drawio.svg)

## Log

### Week 3-5
I researched several possible project ideas and analyzed different concepts before choosing the smart garage system. After deciding on the final direction, I started identifying the hardware components needed for implementation. I looked into sensor modules, networking options, actuator control, user input methods, and visual feedback components compatible with the NUCLEO-U545RE-Q board.

### Week 8
During this week, I received the hardware components and tested them individually.

### Week 11
I assembled all the components on the breadboard, connected them according to the planned circuit, and created the complete circuit schematic in KiCad.

## Hardware

The project is based on the **STM32 NUCLEO-U545RE-Q** development board, used as the main controller. Ultrasonic sensors are used for vehicle detection, while a temperature and humidity sensor monitors the garage environment. A small motor with fan blades is used as the ventilation system, and push buttons allow manual control. LEDs provide visual feedback for the system states. The W5500 Ethernet module enables real-time data transmission to a laptop dashboard. All components are connected on a breadboard using jumper wires and powered with an external 5V supply where needed.

### Schematics

![Kicad schematics](smart_garage.webp)
![hardware photo](hardware_smart_garage.webp)

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| STM32 NUCLEO-U545RE-Q | Main microcontroller board | [125 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D) |
| W5500 Ethernet Module | Network communication and dashboard data transfer | [50 RON](https://www.emag.ro/modul-retea-ethernet-w5500-rqiurpn-spi-la-lan-compatibil-arduino-comunicare-spi-3-3v-23x26mm-11-netmodel-w5500/pd/D4HL8L3BM/) |
| HC-SR04 Ultrasonic Sensor | Vehicle presence detection | Already owned |
| SG90 Servo motor | Garage door movement mechanism | [25 RON](https://www.emag.ro/set-servomotor-sg90-unghi-de-lucru-180-grade-23-mm-9-g-3874783591829/pd/DXVZ3MYBM/) | 
| DHT11 Sensor | Temperature and humidity monitoring | Already owned |
| DC Motor + Fan Blades | Ventilation system simulation | Already owned |
| Push Buttons | Manual fan control | Already owned |
| LED Kit | Visual system status indicators | [~19 RON](https://www.emag.ro/kit-200-buc-led-3mm-5mm-de-diferite-culori-ai707/pd/D4DJYTMBM/) |
| Breadboard MB102 | Prototyping and circuit assembly | 17 RON |
| Jumper Wire Kit | Electrical connections | [47 RON](https://www.emag.ro/set-cabluri-dupont-trexora-120-piese-20cm-multicolor-pentru-proiecte-electronice-dzme069/pd/DRCFXS3BM/) |
| Resistor Kit | LED current limiting and circuits | [38 RON](https://www.emag.ro/set-600-rezistori-sinbinta-set-rezistori-1-4w-10-1m-30-tipuri-toleranta-1-20-de-bucati-pentru-fiecare-valoare-ideal-pentru-proiecte-electronice-si-ingineri-metal-130-22-80mm-multicolor-ww-462/pd/DGZ93K3BM/) |
| 5V Power Supply | External power for sensors and motor | Already owned |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | STM32 HAL | GPIO, PWM, SPI |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async runtime | Run tasks |
| [embassy-time](https://github.com/embassy-rs/embassy) | Time tools | Delays, timers |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware traits | Driver support |
| [heapless](https://github.com/rust-embedded/heapless) | Static containers | Buffers |
| [static_cell](https://github.com/embassy-rs/static-cell) | Static memory | Shared resources |
| [w5500](https://github.com/newAM/w5500-rs) | Ethernet driver | Network data |
| [embedded-nal](https://github.com/rust-embedded-community/embedded-nal) | Network API | TCP/UDP support |
| [defmt](https://github.com/knurling-rs/defmt) | Logging | Debug output |
| [panic-probe](https://github.com/knurling-rs/probe-run/tree/main/panic-probe) | Panic handler | Error debug |
| [dht-sensor](https://crates.io/crates/dht-sensor) | DHT sensor driver | Temp / humidity |
| [nb](https://crates.io/crates/nb) | Non-blocking API | Async drivers |

## Links

1. [Embassy-rs](https://embassy.dev/)
2. [Rust Embedded Book](https://docs.rust-embedded.org/book/)
3. [STM32 NUCLEO-U545RE-Q Overview](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)
4. [Embassy GitHub Repository](https://github.com/embassy-rs/embassy)
5. [W5500 Ethernet Controller](https://wiznet.io/products/ethernet-chips/w5500)
6. [embedded-hal crate](https://github.com/rust-embedded/embedded-hal)
7. [heapless crate](https://github.com/rust-embedded/heapless)
8. [Rust Crates Registry](https://crates.io/)
