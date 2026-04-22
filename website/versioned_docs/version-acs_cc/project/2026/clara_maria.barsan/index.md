# 3D Printer
An engineered-from-scratch 3D printer powered by a 32-bit STM32 board, featuring custom hand-wired electronics, silent motor operation, and robust thermal management.

:::info 

**Author**: Bârsan Clara Maria \
**GitHub Project Link**: link_to_github

:::

<!-- do not delete the \ after your name -->

## Description

This project is a custom-built Cartesian 3D printer based on the classic Prusa i3 architecture.
Its main purpose is to manufacture physical 3D objects by melting and depositing plastic filament layer by layer.
The end-user interacts with the device by providing a digital 3D model that has been sliced into a "G-code" file.
Once started, the machine autonomously coordinates its three axes (X, Y, and Z) and regulates high temperatures to print the object.
It solves the problem of needing an affordable, highly customizable, and easily repairable manufacturing tool for personal DIY projects and prototyping.

## Motivation

My primary motivation for building this 3D printer is to create a personal manufacturing hub at home.
I want the independence to design, prototype, and print highly customized components for my other DIY endeavors.
Having a tailor-made machine will give me the freedom to bring complex ideas to life, ensuring my future projects are never limited by off-the-shelf parts.

## Architecture 

Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other

## Log

<!-- write your progress here every week -->

### Weeks 23 March - 12 April
Chose the project idea, researched components, and ordered hardware.

### Week 13 - 19 April
Assembled the frame and the Y axis.

### Week 20 - 26 April
Finished the setup for X and Z axis.

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
