# Atari Breakout
A clone of the classic video game

:::info 

**Author**: Dumitru Silvia-Alexandra \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/proiect-silviaalex](https://github.com/UPB-PMRust-Students/proiect-silviaalex)

:::

## Description

The project implements the arcade video game Breakout developed by Atari and released in 1976. This game consists of a sideways-moving paddle used to control the direction of a bouncing ball in order to eliminate all the bricks in the level and avoid missing to hit the ball when it comes back to the paddle. In this project, the video game is displayed on an LCD, played with a joystick and having responsive sound coming from a buzzer.

## Motivation

I have a deep passion for retro video games and wanted to build one for the project.

## Architecture

The main components are:
- the input, for controlling the paddle and other types of movement outside of gameplay
- the video output, for showing the state of the game, main menu, and maybe a leaderboard
- the audio output, for the SFX and, potentially, the music

These components are connected to the microcontroller, which is processing the inputs and updating the outputs.

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Hardware used:
- Raspberry Pi Pico 2 W, the microcontroller
- TFT LCD with ST7735R driver, the display
- Biaxial Joystick, the joystick with 2 analog outputs for movement and 1 digital output for pressing it down
- Buzzer, for sound
- Breadboard, for placing all the components on
- Pin headers, soldered to the microcontroller
- Jumper wires, for interconnecting components
- Resistors, if need be

### Schematics

Work in progress at the moment.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| Raspberry Pi Pico 2 W | The microcontroller | [39.66 lei](https://www.optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html) |
| TFT LCD with ST7735R driver | The display | [28,99 lei](https://www.optimusdigital.ro/en/lcds/1311-modul-lcd-spi-de-18-128x160.html) |
| Biaxial Joystick | The input | [5,35 lei](https://www.optimusdigital.ro/en/touch-sensors/742-ps2-joystick-breakout.html) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy) | RP2350 HAL | Used for embedded programming |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | Graphics crate | Used for drawing |
| [mipidsi](https://github.com/almindor/mipidsi) | Generic display driver to connect to TFT displays | Used for the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

Also work in progress.
1. [link](https://example.com)
2. [link](https://example3.com)
...
