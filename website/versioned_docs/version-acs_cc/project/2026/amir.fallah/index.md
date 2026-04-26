# Inverse Pendulum Robot
Self balancing two wheeled pendulum robot.

:::info 

**Author**: Fallah-Mirzaei Amir \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-amir-FM

:::

## Description

A robot that uses an accelerometer and a gyroscope to sense when it is falling and revert to a vertical position, which is parallel to the weight vector. The user can connect to it via Bluetooth and give it commands to move: front, back, turn right, and turn left. Gyroscope and accelerometer sensor data are fed into a feedback-based control loop stabilization algorithm. The output of this program is fed directly into the stepper motor controller.

From the research I have done, I learned that there are multiple control algorithms, with PID being the easiest to implement and understand. As a bonus, I will provide the use with the possibility to select what control algorithm the robot uses.
## Motivation

This is a project I had on my radar for a long time. Having no experience with Control Systems, I thought it was a great idea to start with this project. I also like the fact that this is a hardware-inclined task, because my experience in university is software-heavy.

## Architecture 

![Diagram](./images/block_diagram.svg)

## Log

### Week 20 - 24 April
Made initial documentation and ordered hardware parts.

## Hardware

I am running the robot in a theadered format, for the ease of development, to concentrate my attention on building it, not on changing the battery every 40 minutes.

### Schematics

TBD

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32](https://www.farnell.com/datasheets/3927507.pdf) | Microcontroller | [129 RON](https://ro.farnell.com/stmicroelectronics/nucleo-u545re-q/development-brd-32bit-arm-cortex/dp/4216396?CMP=e-email-sys-orderack-GLB) |
| [MPU6050](https://cdn.sparkfun.com/datasheets/Sensors/Accelerometers/RM-MPU-6000A.pdf) | IMU (Gyroscope + Accelerometer) | [16 RON](https://www.emag.ro/modul-accelerometru-si-giroscop-mpu6050-ai382-s321/pd/DB606JBBM/)
| [HM-10](https://components101.com/sites/default/files/component_datasheet/HM10%20Bluetooth%20Module%20Datasheet.pdf) | Bluetooth Module | [27 RON](https://www.emag.ro/modul-bluetooth-4-0-ble-at-09-hm-10-msalamon-conectivitate-rapida-mic-si-eficient-ideal-pentru-proiecte-iot-economiseste-energie-fiabil-usor-de-montat-functioneaza-in-diverse-conditii-calitate-inalta-/pd/DHKDDVYBM/) |
| [LM2596](https://www.ti.com/lit/ds/symlink/lm2596.pdf) | Buck Converter 12V - 5V | [10 RON](https://www.emag.ro/modul-dc-dc-step-down-lm2596-765464701237/pd/DWHHRGBBM/) |
| [TB6612FNG](https://cdn.sparkfun.com/datasheets/Robotics/TB6612FNG.pdf) | Dual Motor Driver | [26 RON](https://www.emag.ro/driver-motor-tb6612fng-punte-h-dubla-control-pwm-1-2a-bmx474/pd/DSMDW83BM/) |
| [JGA25-371](https://people.ece.ubc.ca/~eng-services/files/courses/elec391-data_sheets/MOT4-info.pdf) | Encoder Disk Gear Motor 12v | [2 x 31 RON](https://www.aliexpress.com/item/1005007889281285.html) | 


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy/tree/main/embassy-stm32) | Hardware Interface | Base library for project. |

## Links

1. https://www.youtube.com/watch?v=IYOxj6VyC8s&pp=ygUXaW52ZXJ0ZWQgcGVuZHVsdW0gcm9ib3Q%3D
