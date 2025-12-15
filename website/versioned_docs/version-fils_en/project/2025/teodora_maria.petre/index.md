# Smart Home Devices

This project aims to automate basic home devices based on the state of the room.

:::info

**Author:** Petre Teodora-Maria \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/project-dm-2025-730dora

:::

## Description

This project implements a Secure Access Smart Door system using the STM32 microcontroller and the Rust programming language. The main goal is to provide autonomous entry control: the door system activates when motion is detected and grants access only after a person provides the correct PIN code or RFID scan. A servo motor manages the lock, and colored LEDs, along with a screen, provide feedback, displaying a personalized welcome message upon successful authentication.


## Motivation

Since I was little I used to make dollhouses out of cardboard, but I’ve always felt like something was missing. I wanted to make them as real as possible – functional lights, doors, almost everything that a real house has – but I didn’t know how to.

##Components Overview

* Servomotor SG90 – mechanically closes and opens the door
* PIR Motion Detector (HC-SR501) – detects the person approaching the door
* RFID Reader Module – collects the security credentials for authentication
* LEDs – provides immediate visual feedback for granted/denied access
* LCD Screen – displays personalized **“Welcome, [user’s name]”** message upon successful entry


## Architecture 

![the architecture](architecture2.svg)



week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Detail in a few words the hardware used.

### Schematics

Place your KiCAD schematics here.

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
