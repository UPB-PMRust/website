# CargoBot
Autonomous cargo robot with Bluetooth telemetry

:::info

**Author**: Andrei Rusanescu \
**GitHub Project Link**: link_to_github

:::

<!-- do not delete the \ after your name -->

## Description

CargoBot is an autonomous 4 wheeled robot that carries cargo and navigates surfaces with imperfections (bumps, ramps, carpet). It uses an STM32 Nucleo-U545RE-Q microcontroller programmed in Rust with Embassy-rs and sends real-time telemetry data to a PC dashboard via Bluetooth, while simultaneously displaying information on an onboard OLED.

The central idea of the project is measuring and visualizing the impact of cargo load on motor performance: when the robot carries something heavy, the PID controller automatically increases the PWM duty cycle to maintain a constant speed. This compensation is observable live on the robot's OLED, on the PC dashboard, and through 3 LEDs (green/yellow/red) that visually indicate the effort level.

## Motivation
I am particularly interested in cars and networking. This project combines multiple peripherals studied in the lab (PWM, GPIO, I2C, UART, Bluetooth) into a functional system. It clearly demonstrates a measurable technical behavior, which is the difference in motor effort with and without cargo. The wireless telemetry component aligns with my personal interests in networking and systems. The software runs on Rust with Embassy-rs.

## Architecture 

Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Detail in a few words the hardware used.

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

