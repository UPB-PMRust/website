---
sidebar_label: 'Joystick-Controlled Mini Excavator'
sidebar_position: 1
---

# Joystick-Controlled Mini Excavator
Articulated robotic system with 4 degrees of freedom, based on converting a mini-excavator into an intelligent machine controlled via joystick.

:::info 

**Author**: Mihai Andrei-Claudiu \
**GitHub Project Link**: [acs-project-2026-andreimmihai](https://github.com/UPB-PMRust-Students/acs-project-2026-andreimmihai)

:::

## Description
This project achieves the mechatronic conversion of a conventional excavator into an active robotic system with 4 degrees of freedom, electronically controlled via a joystick interface. The system processes analog signals on the OX/OY axes and digital inputs to coordinate four servomotors dedicated to base rotation, main arm lifting, stick movement, and bucket actuation. The control logic integrates signal mapping algorithms and software travel limits, eliminating the risk of mechanical overstressing of the joints. The final result is a machine capable of smooth and precise movements, optimized for object manipulation in the workspace by faithfully replicating joystick commands.

## Motivation

I chose this project because it offers a really cool practical challenge: how to adapt electronics onto an object that wasn’t designed for it. I want to learn how to manage multiple motors simultaneously in Rust and how to accurately read signals from a joystick to control a physical mechanism.

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