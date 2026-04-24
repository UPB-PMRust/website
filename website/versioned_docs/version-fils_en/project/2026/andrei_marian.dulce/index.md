# Smart 2D LiDAR Radar & Advanced Security Hub
A comprehensive bare-metal 2D scanning system and multi-sensor security alarm built with Rust and Embassy on the STM32.

:::info 

**Author**: DULCE Andrei-Marian \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/fils-project-2026-andrrei509](https://github.com/UPB-PMRust-Students/fils-project-2026-andrrei509)

:::
## Description

This project is an advanced, multi-layered active security and environmental monitoring hub built on the STM32. The core feature is a 2D scanning system using a VL53L0X Time-of-Flight (ToF) sensor mounted on a 28BYJ-48 stepper motor, which streams Cartesian coordinate data to a laptop via USB Bulk transfers to render a real-time radar map. Additional sensors provide intrusion detection and environmental telemetry.

## Motivation

I chose this project because it combines precise motor control, real-time data acquisition from a vast array of sensors, and PC-device USB communication into a single robust system. It is a highly practical and technically challenging way to explore asynchronous embedded programming in Rust.

## Architecture

![System Architecture Diagram](./schematic.svg)

Main software and system components:
- **Scanning Module**: reads distance from VL53L0X via I2C and steps motor via GPIO.
- **Intrusion Detection Module**: reads Doppler radar, vibration, and sound sensors.
- **Access Control Module**: handles the 4x4 matrix keypad and IR receiver.
- **Environmental Module**: reads BMP180 (I2C) and photoresistors (ADC).
- **Local Feedback Module**: controls the 4-digit display, buzzer (PWM), and LEDs.
- **Communication Module**: sends serialized coordinate payloads via USB Bulk to the PC.

## Log

### Week 4
- Established the project idea and worked on the component list.

### Week 5-6
- Ordered all hardware components (STM32, VL53L0X, stepper motor, etc.).
- Researched the Embassy framework and USB Bulk transfer implementation in Rust.

### Week 7
- Components arrived, started testing and tinkering with them.

### Week 8-9 
- Hardware Integration. Began assembling the core scanning module by mounting the VL53L0X ToF sensor to the 28BYJ-48 stepper motor shaft.
- Circuit Prototyping. Mapped out the pin connections on the 830-point breadboard for the I2C bus (sensor), GPIO sequence (ULN2003 driver), and the security sensor array.
- Software Design. Drafted the Embassy task architecture, defining the synchronization channels between the scanning task and the USB communication task.


## Hardware

The system uses an STM32 Nucleo-U545RE-Q as the central hub. It orchestrates a VL53L0X ToF sensor mounted on a 5V stepper motor for spatial scanning, alongside a suite of security sensors (Doppler radar, vibration, sound, keypad) and environmental sensors (BMP180, photoresistors). Everything is wired on a universal 830-point breadboard.

### Schematics

*To be added.*

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| [STM32 Nucleo-U545RE-Q](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D&utm_id=6470900573&utm_source=google&utm_medium=cpc&utm_marketing_tactic=emeacorp&gad_source=1&gad_campaignid=6470900573&gbraid=0AAAAADn_wf3z_FgLPlEhiqyMV9mn0tbtQ&gclid=CjwKCAjwqazPBhALEiwAOuXqdOdLtJJ-VfXVsbLczCE73wrj-NLHNWA5hmh-ZEuzBuzdJ_4QBonQeBoCZrcQAvD_BwE) | The main microcontroller | 106.59 RON |
| [VL53L0X ToF Sensor](https://www.emag.ro/senzor-de-masurare-a-distantei-tof-vl53l0x-ai280-s366/pd/DS9D93MBM/) | Distance measurement (I2C) | 28.00 RON |
| [Universal Breadboard 830 points](https://www.emag.ro/breadboard-universal-830-puncte-5904162803408/pd/DLQMNLMBM/) | Prototyping base | 15.67 RON |
| [28BYJ-48 5V Stepper Motor + ULN2003 Driver](https://www.optimusdigital.ro/en/stepper-motors/2487-step-by-step-28byj048-5v-motor-and-uln2003-driver-green-.html?search_query=28BYJ-48&results=1) | Motor for sensor sweeping | 15.00 RON |
| [RCWL-0516 Doppler Radar Sensor](https://sigmanortec.ro/Senzor-Doppler-Radar-RCWL-0516-p125423439) | Microwave motion detection | 6.20 RON |
| [BMP180 Sensor Module](https://sigmanortec.ro/Senzor-presiune-atmosferica-temperatura-BMP180-p126182252) | Temperature and pressure sensing (I2C) | 4.50 RON |
| [4x4 Matrix Keypad](https://www.optimusdigital.ro/en/touch-sensors/2441-tastatura-matriceala-4x4-cu-butoane.html?search_query=0104110000023537&results=1) | PIN code input for security | 4.00 RON |
| [0.56" 4-Digit 7-Segment Display](https://sigmanortec.ro/Display-digital-LED-0-56-4-digiti-7-segmente-Rosu-p128704274) | Local numerical data display | 10.00 RON |
| [Passive Buzzer 3.3V](https://www.optimusdigital.ro/en/buzzers/12247-3-v-or-33v-passive-buzzer.html?search_query=0104210000081527&results=1) | Proximity audio alarm (PWM) | 3.00 RON |
| [Photoresistors](https://www.optimusdigital.ro/en/others/28-5528-photoresistor.html?search_query=0104210000002072&results=1) | Ambient light sensing (ADC) | 7.60 RON |
| [Vibration / Tilt Sensor](https://sigmanortec.ro/Senzor-vibratie-miscare-inclinare-p126008775) | Tamper detection | 6.00 RON |
| [Microphone / Sound Sensor](https://sigmanortec.ro/Modul-microfon-senzor-sunet-p126025149) | Acoustic trigger | 5.00 RON |
| 10x LED 5mm (Mixed colors) | Directional and status visual indicators | 5.00 RON |
| [Transistors (2N2222, BC547) & Diodes (1N4148)] | Signal switching and protection | 10.00 RON |
| [Assorted Resistors (220Ω, 330Ω, 1kΩ, 4.7kΩ, 10kΩ)] | Current limiting and pull-ups/downs | 5.00 RON |
| [Capacitors (100uF, 470uF, 100nF)] | Power filtering and decoupling | 15.00 RON |
| [Dupont Wires (M-M, M-F, F-F)] | Establishing connections | 25.00 RON |
| [Switches & PCB Buttons] | Power control and inputs | 15.00 RON |
| **Total** | | **286.56 RON** |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for STM32 | Hardware access for I2C, GPIO, ADC, and PWM |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor | Schedules concurrent firmware tasks |
| [embassy-time](https://github.com/embassy-rs/embassy) | Embedded timers | Delay management and periodic sensor polling |
| [embassy-usb](https://github.com/embassy-rs/embassy) | USB device stack | Implementing USB Bulk endpoints for PC communication |
| [embassy-sync](https://crates.io/crates/embassy-sync) | Async synchronization primitives | Providing Channels and Signals to safely move sensor data between tasks. |
| [embassy-embedded-hal](https://crates.io/crates/embassy-embedded-hal) |Adapter for Embedded HAL | Bridging Embassy drivers with the standard Rust embedded ecosystem. |
| [keypad-rs](https://crates.io/crates/keypad) | Matrix keypad driver | Scanning and debouncing the 4x4 input matrix |
| [postcard](https://crates.io/crates/postcard) | Compact binary serialization | Serializing coordinate and security data into tiny packets for USB transmission. |
| [serde](https://serde.rs/) | Data serialization framework | Core framework used by Postcard to define the structure of analytics sent to the PC. |
| [cortex-m](https://crates.io/crates/cortex-m) | ARM Cortex-M low-level access | Handling low-level CPU operations and critical sections. | 
| [cortex-m-rt](https://crates.io/crates/cortex-m-rt) | Startup and runtime for ARM | Managing the reset handler and memory initialization of the STM32. |
| [panic-probe](https://crates.io/crates/panic-probe) | Debug panic handler | Catching code crashes and printing the stack trace over the debug probe. |
| [defmt](https://defmt.ferrous-systems.com/) | Deferred formatting | Providing high-efficiency logging to debug sensor values in real-time. |
| [micromath](https://github.com/tarcieri/micromath) | `no_std` math library | Computing trigonometric functions for coordinate mapping |
| [vl53l0x](https://crates.io/crates/vl53l0x) | ToF Sensor Driver | Reading distance data from your laser sensor over I2C. |
| [ht16k33](https://crates.io/crates/ht16k33) | Segment Display Driver | Controls the 7-segment display. |


## Links

1. [Embassy Framework Documentation](https://embassy.dev)
2. [STM32U545RE Reference Manual](https://www.st.com/resource/en/reference_manual/rm0456-stm32u5-series-armbased-32bit-mcus-stmicroelectronics.pdf)
3. [Rust Embedded Book](https://docs.rust-embedded.org/book/)
4. [VL53L0X API and Documentation](https://www.st.com/en/imaging-and-photonics-solutions/vl53l0x.html)
