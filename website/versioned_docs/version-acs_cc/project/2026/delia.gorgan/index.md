# MazeLock: The Tilting Puzzle Box
A secured box that opens upon solving a tilt-controlled digital maze puzzle.

:::info 

**Author**: GORGAN Delia \
**GitHub Project Link**: [UPB-PMRust-Students/acs-project-2026-deliagorgan](https://github.com/UPB-PMRust-Students/acs-project-2026-deliagorgan)

:::

<!-- do not delete the \ after your name -->

## Description

MazeLock is a secure storage box featuring a digital maze-based locking mechanism. Unlike traditional safes, this box unlocks its physical latch only after the user successfully navigates a virtual ball through a maze displayed on a TFT screen. The movement is entirely tilt-controlled, the user must physically rotate and tilt the box, allowing an internal accelerometer to translate these motions into the ball's navigation. Once the finish point is reached, the system triggers a servo-actuated lock to open the lid.

## Motivation

The inspiration for MazeLock stems from a cherished childhood memory of a physical maze piggy bank. That original mechanical box required navigating a small ball through a complex 3D path spanning all four sides to unlock the contents. This challenge acted as a barrier that prevented impulsive spending. Because the maze took time and effort to solve, it gave me a few extra minutes to reflect on whether I truly wanted to spend my savings, effectively teaching me the value of delayed gratification.

Recently, I repurposed that old piggy bank as a unique gift wrap for a friend. The added suspense and the challenge required to reveal the gift were highly appreciated and made the experience memorable. This sparked my desire to modernize the concept. I decided to build a contemporary version from scratch, transitioning from a purely mechanical puzzle to a digital system. By using an STM32 microcontroller and Rust, I aim to recreate that sense of challenge and reward while mastering the complexities of embedded hardware and software integration.

## Architecture 

![Architecture Diagram](diagrama_arhitectura_PM.svg)

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
