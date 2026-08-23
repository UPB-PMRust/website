# Hexapod
A six-legged walking robot with Bluetooth control, ToF obstacle detection, and IMU-based stability monitoring.

:::info 

**Author**: Vladimir-Nicolae Olteanu \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-Vldd28

:::

## Description

A six-legged "spider" robot built around the STM32 Nucleo-U545RE-Q. It walks using a tripod gait. Three legs always in contact with the ground — driven by twelve MG90S servos (two per leg: one for hip swing, one for leg lift). The robot accepts movement commands over Bluetooth from a phone, continuously samples a forward-facing time-of-flight sensor for obstacles, and reads an IMU to detect dangerous tilt angles. When obstacles are detected it stops or autonomously reroutes; when tilt exceeds a safe threshold it parks itself.

## Motivation

I wanted a project that combined embedded systems, real-time control, and mechanical design without falling into the "blink an LED" or "Arduino tutorial" category. A hexapod is a sweet spot: the tripod gait makes the locomotion problem tractable on an STM32 (no dynamic balancing required), but coordinating twelve servos against sensor feedback, designing the leg geometry, and managing power delivery for high transient currents are all genuine engineering challenges. It also gives me hands-on experience with I2C, UART, sensor fusion, and PWM fundamentals that transfer to almost any embedded role.

## Architecture

![Diagram](./images/spider_robot.svg)

## Log

<!-- write your progress here every week -->

### Week 27 April - 1 May
Started modeling the 3D parts and ordered the necessary hardware parts.

## Hardware

The robot is built around an STM32 Nucleo-U545RE-Q (Cortex-M33 @ 160 MHz, 512 KB flash, 272 KB RAM, hardware FPU + DSP) with a 2S LiPo power system. Twelve MG90S metal-gear servos drive the six legs (two degrees of freedom per leg: coxa for swing, femur for lift). A PCA9685 16-channel I2C PWM driver handles all servo signals to keep the MCU's timers and CPU free for the gait state machine and sensor fusion.

For sensing, an MPU6050 6-axis IMU provides accelerometer + gyroscope data for tilt detection, and a VL53L0X time-of-flight laser distance sensor handles forward obstacle detection. Both share an I2C bus with the PCA9685. An HC-05 Bluetooth module on a UART link receives commands from a phone-based controller app.

Power flows from a 2S 7.4V 2200mAh LiPo, through a UBEC stepping down to 5V/5A for the servos, to the PCA9685's V+ rail. The Nucleo is powered via its VIN pin from the same 5V rail, with logic and servo grounds tied together. Servos and logic are on separate power paths to prevent servo current spikes from browning out the MCU.

The chassis and legs are 3D-printed in PLA (body) and PETG (tibias, for some compliance on impact). Body footprint is roughly 20 × 15 cm; standing height ~9 cm.

### Schematics


### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main microcontroller (Cortex-M33, 160 MHz, FPU+DSP, built-in ST-LINK/V3) | Provided by university |
| [PCA9685 16-channel PWM driver](https://www.bitmi.ro/modul-i2c-pca9685-cu-16-canale-pentru-controlul-servomotoarelor-10559.html) | I2C-controlled PWM generation for all 12 servos | [25 RON](https://www.bitmi.ro/modul-i2c-pca9685-cu-16-canale-pentru-controlul-servomotoarelor-10559.html) |
| [MG90S metal-gear servo × 14](https://sigmanortec.ro/Servomotor-MG90S-angrenaje-metal-p209610310) | Leg actuation (12 in use, 2 spares) | [253 RON](https://sigmanortec.ro/Servomotor-MG90S-angrenaje-metal-p209610310) |
| [GY-521 MPU6050 IMU](https://sigmanortec.ro/Modul-giroscopic-si-accelerometru-3-axe-GY-521-p126016326) | 6-axis accelerometer + gyroscope for tilt detection and stability monitoring | [25 RON](https://sigmanortec.ro/Modul-giroscopic-si-accelerometru-3-axe-GY-521-p126016326) |
| [VL53L0X ToF sensor](https://sigmanortec.ro/Modul-VL53L0X-timp-de-zbor-p126182383) | Forward-facing laser distance sensor for obstacle detection (range up to 2 m) | [17 RON](https://sigmanortec.ro/Modul-VL53L0X-timp-de-zbor-p126182383) |
| [HC-05 Bluetooth module](https://sigmanortec.ro/Modul-Bluetooth-HC-05-p141736971) | Wireless serial link for phone-based command input | [27 RON](https://sigmanortec.ro/Modul-Bluetooth-HC-05-p141736971) |
| [Gens Ace 2S 2200mAh 30C LiPo (XT60)](https://www.emag.ro/) | Main power source (7.4V) | [87 RON](https://www.emag.ro/) |
| [B3AC LiPo balance charger](https://www.emag.ro/) | Safe charging and cell balancing for the LiPo | [74 RON](https://www.emag.ro/) |
| UBEC 5V/5A buck converter | Steps down 7.4V LiPo to a stable 5V rail for servos and logic | [FILL IN — pending purchase] |
| Piezo buzzer + WS2812 LED | Audio + visual status feedback (connect, low battery, obstacle) | ~10 RON |
| Misc. (jumper wires, perfboard, M2/M3 screws, 22 AWG silicone wire, heat-shrink, XT60 connector, switch) | Wiring and mechanical assembly | ~60 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy/tree/main/embassy-stm32) | Hardware Interface | Base library for project. |

## Links
