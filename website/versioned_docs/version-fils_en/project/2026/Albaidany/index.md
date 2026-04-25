# Project Name
A one line project description

:::info 

**Author**: Abdulqader Albaidhani\
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-Albaidany

:::

<!-- do not delete the \ after your name -->

## Description

My Project Idea: A sonar radar using a Time-of-Flight (ToF) laser sensor to detect foreign objects in an area, when an object enters the sonar's radius, an alert sound queue will occur, the sonar measures accross an area with both manual (using buttons) and automatic movements (using a button to switch modes).

## Motivation

I chose to make this because I'm very passionate about Intel-gathering gadgets and I wanted to persue many more similar ideas, this is a concept I really found interesting and I wanted to replicate it.

## Architecture 

Control Layer: NUCLEO-U545RE-Q microcontroller running Rust with Embassy.
Handles servo control (PWM), reads distance data via I2C from the sensor, processes button inputs, manages automatic/manual scanning logic, generates buzzer signals/sound queues, and streams radar data (angle + distance) to the laptop over USB.

Sensing Layer: VL53L0X Time-of-Flight (ToF) Distance Sensor Module mounted on a custom rotating platform.
Measures distance in real time at the current angle using laser time-of-flight (ToF) technology, providing precise range data for radar visualization via laptop.

Actuation Layer: SG90 Micro Servo Motor driven by PWM signals from the MCU.
Rotates the sensor across a 0°–180° sweep, allowing spatial scanning of the environment. Movement supports both automatic scan and manual directional control via user input using buttons, when in manual mode and inactive for ~3 seconds, the system returns back to automatic sweeping mode.

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
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...
