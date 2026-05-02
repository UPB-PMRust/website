# Project Name
Sonar using ToF Distance Sensors 

:::info 

**Author**: Abdulqader Albaidhani \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-Albaidany

:::

<!-- do not delete the \ after your name -->

## Description

My Project Idea: A sonar radar using a Time-of-Flight (ToF) distance sensor to detect foreign objects in an area, when an object enters the sonar's radius, an alert sound queue will occur, the sonar measures accross an area with both manual (using buttons) and automatic movements (using a button to switch modes).

## Motivation

I chose to make this because I'm very passionate about Intel-gathering gadgets and I wanted to persue many more similar ideas, this is a concept I really found interesting and I wanted to replicate it.

## Architecture 

Control Layer: NUCLEO-U545RE-Q microcontroller running Rust with Embassy.
Handles servo control (PWM), reads distance data via I2C from the sensor, processes button inputs, manages automatic/manual scanning logic, generates buzzer signals/sound queues, and streams radar data (angle + distance) to the laptop over USB.

Sensing Layer: VL53L0X Time-of-Flight (ToF) distance sensor mounted on a custom rotating platform.
it measures distance in real time at the current angle using time-of-flight (ToF) technology, providing precise range data for radar visualization via laptop.

Actuation Layer: SG90 Micro Servo Motor driven by PWM signals from the MCU.
Rotates the sensor across a 0°–180° sweep, allowing spatial scanning of the environment. movement supports both automatic scan and manual directional control via user input using buttons, when in manual mode and inactive for ~3 seconds, the system returns back to automatic sweeping mode.

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

NUCLEO-U545RE-Q 
USB cable 
VL53L0X Time-of-Flight Distance Sensor Module 
SG90 Micro Servo Motor 
Passive Buzzer  
Green LED
Red LED
Push buttons
Resistors
Breadboard(atm)
Jump wires


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
| STM32 Development Board | The microcontroller | 105 RON |
| VL53L0X Time-of-Flight Distance Sensor | Distance Sensor | 25 RON |
| USB Cable | Cable for data transfer | 5 RON |
| SG90 Servo Motor | Sensor movement | 15 RON |
| Passive buzzer x3 | Sound queues | 2 RON |
| LEDs x4 | Visual queues | 4 RON |
| Buttons x3 | Manual controls | 5 RON |
| Resistors + Jumpers + breadboards | Supporting the circuit | 30 RON |



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| embassy-stm32 | Embassy HAL for STM32 microcontrollers | GPIO, timers, UART, sensor integration |
| embassy-executor | Async task executor | Running concurrent tasks (sensor read, servo control, USB streaming) |
| embassy-time | Async timers and delays | Servo sweep timing, debounce, 3s timeout logic |
| embassy-sync | Synchronization primitives | Sharing data between tasks (angle, distance, mode) |
| embassy-futures | Async utilities | Task coordination and control flow |
| embassy-usb | USB device stack (CDC ACM) | Sending radar data to laptop via USB |
| embedded-hal | Hardware abstraction traits | Generic interfaces for I2C, PWM, GPIO |
| vl53l0x custom drivers | Driver for VL53L0X sensor | Reading distance measurements over I2C |
| cortex-m | ARM Cortex-M support crate | Low-level MCU access |
| cortex-m-rt | Runtime for Cortex-M devices | Startup, interrupt vectors |
| heapless | Fixed-capacity data structures | Buffers for USB data (angle + distance) |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Visual of a Sonar](https://www.teledynemarine.com/products/product-line/sonar)
2. [Different types of Sonars](https://www.deeptrekker.com/news/sonar-systems)
...
