# RustRescue
An autonomous firetruck robot that detects fire and puts it out.

:::info 

**Author**: Ioniță Carmen Ana-Maria \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-carmen020305#

:::

<!-- do not delete the \ after your name -->

## Description

RustRescue is a small autonomous robot designed to detect and extinguish fires without human intervention. It is built around the STM32 NUCLEO-U545RE-Q microcontroller and programmed in Rust using the embassy-rs framework. The firetruck continuously monitors its surroundings using three flame sensors and  one smoke sensor. When a flame or smoke is detected, the robot determines the direction of the threat and steers toward it while activating a siren and flashing LEDs, autonomously avoiding any obstacles in its path using an ultrasonic distance sensor. A display shows the current state of the system in real time. Once close enough to the fire, it stops and activates a small water pump to put it out. When the flame is no longer detected, the pump, siren, and LEDs turn off automatically. Then the robot will continue to look for another fire.

## Motivation

The inspiration behind this project is my father, who is a firefighter. Knowing that he puts his life in danger every time he goes to work made me think about ways technology could help. While a small robot cannot replace a firefighter, it could serve as a first responder in situations involving small or early-stage fires, acting quickly before the situation escalates and reducing the need for human intervention in dangerous environments.

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
