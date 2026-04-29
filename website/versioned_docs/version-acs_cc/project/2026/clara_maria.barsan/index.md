# 3D Printer
An engineered-from-scratch 3D printer powered by a 32-bit STM32 board, featuring custom hand-wired electronics, silent motor operation, and robust thermal management.

:::info 

**Author**: Bârsan Clara Maria \
**GitHub Project Link**: [link_to_github](https://github.com/clarabarsan/pm-rust-3d-printer/tree/project/clara_maria.barsan)

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

![Sistem arhitecture](diagram.drawio.webp)

The architecture of this custom 3D printer controller is designed as a standalone, state-machine-driven embedded system. To ensure safety, signal integrity, and a strict separation of concerns, the system is logically divided into distinct architectural subsystems. This design isolates low-voltage processing logic from high-voltage mechanical and thermal actuation.

**Core Processing Unit (MCU):** The central "brain" of the system is the STM32 Nucleo microcontroller. It is responsible for parsing G-code, executing the internal state machine, and orchestrating asynchronous tasks (Motion, Thermal, and I/O management).

**Data Storage Subsystem:** A Micro SD Card Module acts as the local repository for G-code files, allowing the printer to operate fully standalone.

**Sensory & Feedback Subsystem:** The input layer continuously gathers physical data from the machine's environment (temperatures and physical axis limits) and feeds it back to the processing unit to close the control loop.

**Motion Subsystem:** The kinematic layer translates digital movement commands into physical steps, driving the printer's axes and the extruder mechanism.

**Thermal Subsystem:** The high-current heating layer manages the rapid heating of the print bed and the hotend nozzle based on PID calculations from the Core Processing Unit.

**Cooling Subsystem:** The thermal management layer controls the hotend heatsink ventilation to prevent heat creep and maintain hardware integrity.

## Log

<!-- write your progress here every week -->

### Weeks 23 March - 12 April
Chose the project idea, researched components, and ordered hardware.

### Week 13 - 26 April
Assembled the frame and the Y axis.

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
