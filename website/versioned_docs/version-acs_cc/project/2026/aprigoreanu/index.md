# Braille E-Reader
An e-reader for Braille alphabet
:::info 

**Author**: Alexandra Prigoreanu\
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-aprigoreanu20

:::

<!-- do not delete the \ after your name -->

## Description

The purpose of the project is to build a Braille display, which loads content from an SD card and outputs the coresponding Braille letters on the display. The goal is to allow visually impaired people to read file contents using a device that has similar functionalities to that of a classic e-reader.

## Motivation

Over the last 20 years, E-readers have become a practical solution to encourage reading. Their lightweight design and ease of use made them a convenient alternative to purchasing and storing a large collection of printed books. This is especially relevant for blind people, as they require specially produced books using the Braille alphabet. This process can be both time-consuming, as well as expensive. E-readers offer an efficient solution by providing access to a wide range of books for visually impaired people.

## Architecture

### Software achitecture:
Software will be written in Rust, using Embassy. The main logic components are:
- input: read button input (next / previous page logic), read data source (from SD card)
- Braille encoder: convert ASCII characters to Braille letter pattern
- Display: convert Braille pattern to GPIO signals

## Log

<!-- write your progress here every week -->
### Week 20 - 26 April
Researched mechanical solutions for Braille pins

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

<!-- Detail in a few words the hardware used. -->
### Hardware Components:
- microcontroller: STM32 Nucleo-U545RE-Q with ARM Cortex-M33 core
- input system: SD card, user input buttons (next / previous page)
- actuation system: 6 actuators per Braille letter
- power system: 5V external supply, 3.3V MCU supply

### Schematics
Work in progress
<!-- Place your KiCAD or similar schematics here in SVG format. -->

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->
<!-- 
| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) | -->


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy/tree/main/embassy-stm32) | Hardware Abstraction Layer | HAL for bridging interaction between STM32 MCU and Rust software |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async executor | Schedules and runs async tasks |
| [defmt](https://github.com/knurling-rs/defmt) | Console printing | Allows printing to console for debug purposes |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Automatic Braille Display](https://www.sciencebuddies.org/science-fair-projects/project-ideas/Elec_p109/electricity-electronics/refreshable-braille-display)
2. [Braille News Reader](https://medium.com/exploring-android/braillebox-building-a-braille-news-reader-with-android-things-f848fe6de596)
