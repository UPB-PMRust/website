# Quadruped Spider Robot

A 4-legged spider robot controlled via Bluetooth, capable of walking in all directions.

:::info

**Author**: Vlad-Ștefan DAMIAN \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-vladduu

:::

## Description

A quadruped spider robot with 4 legs and 3 degrees of freedom per leg (12 servo motors total). Each leg has an ankle, knee and hip joint driven by SG90 servo motors through a PCA9685 16-channel PWM driver. The robot is controlled wirelessly using an HC-05 Bluetooth module and can get up from a lying position, walk forward, walk backward, turn left, turn right and it is controlled from the Arduino Bluetooth Control Application. The firmware runs on an STM32 NUCLEO-F446RE microcontroller written in Rust using the embassy-rs async framework.

## Motivation

I have always been fascinated by robots, especially biomimetic robots, machines that move like living creatures. With that idea in mind, i wanted to build a  quadruped spider robot cause it was one of my childhood dreams. Also, i knew that the challenge of making 12 motors work together in a coordinated gait, all in real time on a microcontroller will push me to deeply understand both hardware and software design.

## Architecture

The system is composed of:

- **STM32 NUCLEO-F446RE** — the main microcontroller that runs the Rust/embassy-rs firmware. It communicates with the PCA9685 over I2C and receives Bluetooth commands over UART.
- **PCA9685 PWM Driver** — generates PWM signals for all 12 servo motors simultaneously. Powered by a dedicated 5V external power supply.
- **HC-05 Bluetooth Module** — receives single-character commands (F/B/L/R/S) from a mobile RC app and forwards them to the MCU over UART.
- **DC-DC Step Down module XL4005** - used for the current regulation of the PCA9685 PWM Driver.
- **DC-DC Step Down module LM2596** - used for the current regulation of the microcontroller.
- **18650 Li-Ion batteries** - to power the circuit.

![Architecture Diagram](image_arch.svg)

The gait algorithm uses diagonal leg pairs (FL+RB, FR+LB) stepping one leg at a time. Each step consists of: lift knee → swing hip forward → plant foot → push hip back → reset. While a leg steps, the opposite hips shift slightly for balance stabilization.

## Log

### Week 1 - 3 
    Bought all the components needed.
### Week 4 - 7
    Started building the main frame and installig the SG90 servo motors, batteries, regulators, to test the idea and also the weight. First i made it just as a prototype.
### Week 8 - 10
    Seeing that the prototype failed(it was too heavy because of the construction materials, and the SG90 servo motors broke down very quickly and didn't have enough power for that weight), i decided to rebuild it using 3d printed parts and upgrading to mg90s metal gear servo motors.

## Hardware

The robot frame is built using PLA and AMS 3D printed parts and metal brackets. 12 MG90s servo motors are mounted across 4 legs (3 per leg: ankle, knee, hip). All servos are connected to a PCA9685 PWM driver which is controlled by the STM32 NUCLEO-F446RE over I2C. An HC-05 Bluetooth module connects to the MCU over UART for wireless control. The servos are powered by a dedicated external power supply composed of two 18650(a total of 7.4V) rechargeable batteries with a discharge of 30A and regulated to 6V using the XL4005 module.

**Servo Channel Mapping:**

| Channel | Joint |
|---------|-------|
| 0 | Right Back Ankle |
| 1 | Right Back Knee |
| 2 | Right Back Hip |
| 3 | Left Back Ankle |
| 4 | Left Back Knee |
| 5 | Left Back Hip |
| 6 | Front Left Ankle |
| 7 | Front Left Knee |
| 8 | Front Left Hip |
| 9 | Front Right Ankle |
| 11 | Front Right Knee |
| 12 | Front Right Hip |
| 13 | Front Right Ankle (remapped - ch10 damaged) |

### Photos

![Initial spider position image](initial_position.webp)
![Standing position image](standing_position.webp)
![Upside-down spider image](upside_down.webp)

### Schematics

![KiCad Schematic](spider.svg)

### Bill of Materials

| Device | Usage | Price |
| --- | --- | --- |
| [STM32 NUCLEO-F446RE](https://www.skroutz.ro/s/58712886/Nucleo-64-Stm32f446re.html) | Main microcontroller | [131 RON](https://www.skroutz.ro/s/58712886/Nucleo-64-Stm32f446re.html) |
| [PCA9685 PWM Driver](https://www.skroutz.ro/s/10560469/PCA9685-Modul-pentru-Arduino-16-Channel-PWM-Servo-Motor-Driver-I2C.html) | 16-channel I2C PWM servo driver | [32 RON](https://www.skroutz.ro/s/10560469/PCA9685-Modul-pentru-Arduino-16-Channel-PWM-Servo-Motor-Driver-I2C.html) |
| [MG90s Servo Motor](https://www.optimusdigital.ro/ro/motoare-servomotoare/271-servomotor-mg90s.html) | Leg joints (x12) | [19,33 RON x12 = 232 RON](https://www.optimusdigital.ro/ro/motoare-servomotoare/271-servomotor-mg90s.html) |
| [HC-05 Bluetooth Module](https://www.emag.ro/modul-bluetooth-hc-05-cl263/pd/D0966JBBM/?ref=sponsored_products_search_f_b_1_1&recid=recads_1_32fbfcfad6badb03a5a5715a583651c1e300e6c6667564a20a6976c19acbfa71_1776782861&aid=623e9e46-3c92-11f1-801c-06eaf0d4245d&oid=44261365&scenario_ID=1) | Wireless RC control | [46 RON](https://www.emag.ro/modul-bluetooth-hc-05-cl263/pd/D0966JBBM/?ref=sponsored_products_search_f_b_1_1&recid=recads_1_32fbfcfad6badb03a5a5715a583651c1e300e6c6667564a20a6976c19acbfa71_1776782861&aid=623e9e46-3c92-11f1-801c-06eaf0d4245d&oid=44261365&scenario_ID=1) |
| [18650 Batteries](https://www.optimusdigital.ro/ro/acumulatori-li-ion/13662-us18650vtc5c-2600mah-30a.html?search_query=Murata+US18650VTC5C+2600mAh+-+30A&results=1) | External servo power | [44 RON](https://www.optimusdigital.ro/ro/acumulatori-li-ion/13662-us18650vtc5c-2600mah-30a.html?search_query=Murata+US18650VTC5C+2600mAh+-+30A&results=1) |
| Building Materials | Robot frame | 100 RON |
| Jumper wires | Connections | [15 RON](https://www.optimusdigital.ro) |
| [18650 Batteries Case](https://www.emag.ro/suport-acumulator-liion-format-18650-2-celule-in-serie-18650-2/pd/DN292RYBM/?ref=history-shopping_482223871_3410_1) | Used for conecting the batteries in series (7.4V) | [20 RON](https://www.emag.ro/suport-acumulator-liion-format-18650-2-celule-in-serie-18650-2/pd/DN292RYBM/?ref=history-shopping_482223871_3410_1) |
| [DC-DC Step Down module LM2596](https://www.emag.ro/modul-dc-dc-step-down-lm2596-display-pentru-v-lm2596s-v-lcd/pd/DFFDSBMBM/) | Regulating the current to 5V for the microcontroller | [15 RON](https://www.emag.ro/modul-dc-dc-step-down-lm2596-display-pentru-v-lm2596s-v-lcd/pd/DFFDSBMBM/) |
| [DC-DC Step Down module XL4005](https://www.emag.ro/modul-dc-dc-step-down-5a-xl4005/pd/DCFDSBMBM/) | Regulating the current to 6V for the servo driver | [15 RON](https://www.emag.ro/modul-dc-dc-step-down-5a-xl4005/pd/DCFDSBMBM/) |
| Total | | **650 RON** |

## Software

| Library | Description | Usage |
| --- | --- | --- |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async task executor for Cortex-M | Runs concurrent tasks for the gait sequence and Bluetooth handling |
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | STM32 Hardware Abstraction Layer | Hardware drivers for I2C (servo), UART (Bluetooth), and GPIO |
| [embassy-time](https://crates.io/crates/embassy-time) | Time management utilities | Handles asynchronous time delays between servo movements |
| [embassy-sync](https://crates.io/crates/embassy-sync) | Synchronization primitives | Sharing state and passing Bluetooth commands between async tasks |
| [pwm-pca9685](https://crates.io/crates/pwm-pca9685) | PCA9685 PWM driver | Independent control of all 12 servo motors over I2C |
| [cortex-m](https://crates.io/crates/cortex-m) | Low-level Cortex-M processor access | Core processor operations and register access |
| [cortex-m-rt](https://crates.io/crates/cortex-m-rt) | Cortex-M minimal runtime | Device boot sequence and interrupt vector routing |
| [defmt](https://crates.io/crates/defmt) | Deferred formatting logging framework | Highly efficient console logging and formatting |
| [defmt-rtt](https://crates.io/crates/defmt-rtt) | RTT transport for defmt | Transmitting log messages to the host PC via Real-Time Transfer |
| [panic-probe](https://crates.io/crates/panic-probe) | Panic handler | Catches system errors/panics and prints them via defmt |

## Links

1. [embassy-rs documentation](https://embassy.dev/)
2. [STM32 NUCLEO-F446RE datasheet](https://www.st.com/en/evaluation-tools/nucleo-f446re.html)
3. [PCA9685 datasheet](https://www.nxp.com/docs/en/data-sheet/PCA9685.pdf)
4. [HC-05 Bluetooth module guide](https://components101.com/wireless/hc-05-bluetooth-module)
5. [Bluetooth RC Controller app](https://play.google.com/store/apps/details?id=braulio.calle.bluetoothRCcontroller)
