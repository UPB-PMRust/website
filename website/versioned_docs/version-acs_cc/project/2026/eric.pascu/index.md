# Gimbal Stabilizer

A 3-axis camera stabilizer controlled by an STM32 Nucleo, with brushless motors and FOC drivers, using an MPU-6050 sensor for real-time stabilization.

:::info 

**Author**: Pascu Eric \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-PascuEric

:::

<!-- do not delete the \ after your name -->

## Description

The core of this project is a high-performance 3-axis stabilization system designed to turn shaky, handheld movement into cinematic, professional-grade footage. Instead of struggling with blurred or vibrating shots, a user can simply mount their camera and move freely—the gimbal handles the rest. By constantly monitoring its position in space, the device makes invisible, micro-adjustments to the Pitch, Roll, and Yaw axes. This results in a "floating" camera effect that allows creators to capture smooth walking shots, sweeping pans, and dynamic action sequences that would otherwise be impossible to record steadily by hand.

## Motivation

Gimbals have become an essential "secret weapon" in modern visual storytelling, bridging the gap between amateur clips and Hollywood-level production. Beyond just stabilizing travel vlogs for YouTubers, this technology is now a cornerstone of multiple industries: it allows drones to capture perfectly level aerial surveys, enables cinematographers to execute complex "one-take" sequences, and even assists in robotic surgery and satellite positioning.

As a photography and videography enthusiast, I have always been fascinated by how these devices effortlessly defy gravity. Owning a commercial gimbal inspired me to look "under the hood." I took on the challenge of building one from scratch—handling everything from 3D design to the complex math of stabilization algorithms—to truly master the physics and engineering that make modern cinematic magic possible.

## Architecture 

![Diagram](./images/architecture.svg)

## Log

### Week 20 - 26 April

Initial upload with the description, motivation, BOM and architecture scheme.

## Hardware

The gimbal uses an STM32 Nucleo as the main microcontroller, running both the sensor fusion and PID control firmware as well as the FOC algorithms for the motors. Three brushless gimbal motors (one per axis) are driven by three individual FOC driver boards. An MPU-6050 module provides 6-DoF inertial measurements for sensor fusion. A joystick provides manual operator input for orientation control. Power is supplied by a 3S LiPo battery, with a buck converter regulating the logic-level voltages.

### Schematics

WIP

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| STM32 Nucleo-64 | Main microcontroller — sensor fusion, PID and FOC | [115 RON](https://ro.farnell.com/stmicroelectronics/nucleo-u545re-q/development-brd-32bit-arm-cortex/dp/4216396?CMP=e-email-sys-orderack-GLB) |
| Brushless gimbal motor (x3) | One motor per axis (roll, pitch, yaw) | [3 × 50 RON](https://www.aliexpress.com/item/1005002058458786.html?spm=a2g0o.order_list.order_list_main.15.21ef1802e31OeE) |
| FOC driver (x3) | Field-Oriented Control driver board, one per motor | [3 × 34 RON](https://www.aliexpress.com/item/1005007634026470.html?spm=a2g0o.order_list.order_list_main.25.21ef1802e31OeE) |
| MPU-6050 | 6-DoF IMU — accelerometer + gyroscope for angle estimation | [18 RON](https://www.aliexpress.com/item/1005008796700745.html?spm=a2g0o.order_list.order_list_main.30.21ef1802e31OeE) |
| Joystick module (x1) | Manual operator control of target angle | [15 RON](https://www.aliexpress.com/item/1005007846718456.html?spm=a2g0o.order_list.order_list_main.5.21ef1802e31OeE) |
| 3S LiPo battery (x1) | Main power source (~11.1 V) | [90 RON](https://www.aliexpress.com/item/1005007883555706.html?spm=a2g0o.order_list.order_list_main.20.21ef1802e31OeE) |
| Buck converter (x1) | Steps down battery voltage to 5 V / 3.3 V for logic | [14 RON](https://www.aliexpress.com/item/1005009823447391.html?spm=a2g0o.order_list.order_list_main.10.21ef1802e31OeE) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy/tree/main/embassy-stm32) | Embassy HAL for STM32 | GPIO, I2C, ADC, PWM timers on the Nucleo |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [3 Axis Mobile Gimbal](https://techarduinocode.blogspot.com/2024/04/3-axis-mobile-gimbal.html)
2. [DIY Arduino Gimbal](https://howtomechatronics.com/projects/diy-arduino-gimbal-self-stabilizing-platform/)
3. [Reddit Forums](https://www.reddit.com/r/ECE/comments/jj0lgg/3axis_diy_phone_gimbal_bldc_control/)