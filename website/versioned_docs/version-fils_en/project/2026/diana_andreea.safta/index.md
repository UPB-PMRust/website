# Musical Glove
A wearable music device

:::info 

**Author**: Safta Diana-Andreea \
**Group**:1221ED \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-dianulle

:::

<!-- do not delete the \ after your name -->

## Description

The goal of this project is to create a glove that allows users to control music through different hand gestures, turning physical movement into sound.By using motion and pressure sensors, the glove can detect movements and translate them into sound.

## Motivation

This project is heavily inspired by the Mi.Mu gloves developed by Imogen Heap who is also a singer and she uses them for her live performances. I found this concept really cool and engaging. I also wanted my project to involve data processing and building a simplified version of these gloves felt like a great fit.

## Architecture 
```
+--------------------------+
|       INPUT LAYER        |
|    (Motion / Pressure)   |
+------------+-------------+
             |
             v

+--------------------------+
|    PROCESSING LAYER      |
|         (STM32)          |
+------------+-------------+
             |
             v

+--------------------------+
|   COMMUNICATION LAYER    |
|         (USB)            |
+------------+-------------+
             |
             v

+--------------------------+
|    APPLICATION LAYER     |
|          (PC)            |
+------------+-------------+
             |
             v

+--------------------------+
|   AUDIO OUTPUT LAYER     |
|     (Sound Engine)       |
+--------------------------+
```      

## Log

<!-- write your progress here every week -->

### Week 6
    Decided on what I wanted to create and did research on what I needed for hardware and software.

### Week 7-8 
Ordered everything and changed the initial plan. I decided that I will make my own pressure sensors; the initial idea was to use pressure sensors and flex sensors, but due to their high price, I started to look for alternatives and found Pressure-Sensitive Conductive Sheet with which I create the sensors.

### Week 9 

## Hardware
STM32U5 Board: The main microcontroller to process data and handle communication.


MPU-6050 IMU Module: Tracks the orientation and tilt of the hand.


Pressure Sensors: Detect gestures like making a fist or pressing two fingers together.

### Schematics


### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-64](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D) | The microcontroller | [126 RON] |
| [MPU6050](https://www.optimusdigital.ro/en/inertial-sensors/13611-mpu6050-accelerometer-and-gyroscope-module-soldered-pins.html?search_query=MPU6050&results=5) | Motion Tracker | [16 RON] |
| [Pressure sheet](https://robot-italy.com/en/products/1361-pressure-sensitive-conductive-sheet-velostat-linqstat?_pos=1&_psq=pressure&_ss=e&_v=1.0) | DIY pressure sensors | [70 RON] |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://docs.embassy.dev/embassy-stm32/0.6.0/stm32c011d6/index.html) | Provides a hardware abstraction layer for  STM32 | Used for configuration |
| [defmt](https://defmt.ferrous-systems.com/) | Logging framework | Used for debugging |
| [panic-probe](https://crates.io/crates/panic-probe) | Panic handler| Used to define the behavior of panic! |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [MiMU Gloves](https://mimugloves.com/)
...
