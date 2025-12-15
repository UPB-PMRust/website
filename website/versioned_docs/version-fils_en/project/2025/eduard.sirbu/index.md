# Piano Player & Learner
A digital piano designed to help users learn and play songs with on-screen note guidance.

:::info 

**Author**: Sirbu-Boeti Eduard-Cristian \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-dm-2025-Sirbu-Boeti-Eduard

:::

## Description

The project is a 7-note piano intended as a simple learning system to follow the notes of a song. The user can play individual notes or up to two notes simultaneously, allowing for basic chords and harmonies aimed towards beginners. For intermediate learners we will make use of a joystick to experiment with changing the pitch of a note.

* We will use an LCD display to:
  * Guide the user to follow the displayed notes of a chosen song
  * Indicate system states (e.g. playing, recording, name of the song to be played etc.)
  
* The interface is a 4x4 button matrix used for controlling the system:
  * 1st row: Play more difficult pre-recorded songs
  * 2nd row: Guided play-along mode for easier songs using the display
  * 3rd row: Record and save user-played songs
  * 4th row: Auxiliary actions (start recording, confirm, cancel, etc.)

## Motivation

The main motiviation behind this project was me wanting to make use of the 4x4 button matrix component. Initially I wanted to make the tabletop game [Battleship](https://en.wikipedia.org/wiki/Battleship_(game)), but I wasn't satisfied with there being only 16 possible square grids due to the size of the button matrix. 

After fiddling around with buttons on the breadboard I liked the feel of it and wanted to try doing something with buttons that will fit nicely in your hand. Due to the size of the breadboard I was thinking of maybe using it as a remote controller for something but I ended up going with a small 7-note piano.

This was due to the fact that, when I was younger, I used to have a Yamaha digital piano with a lot of similar features and also because I saw that in previous years there have been similar piano projects.

## Architecture 

<!--Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other-->

## Log

### Week 5: 27 October - 2 November

### Week 6: 3 November - 9 November

### Week 7: 10 November - 16 November

### Week 8: 17 November - 23 November

### Week 9: 24 November - 30 November

### Week 10: 1 December - 7 December

### Week 11: 8 December - 14 December

### Week 12: 15 December - 21 December

## Hardware

I have provided links for reference.

* [STM32 NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)

* [Passive buzzers (×2)](https://www.optimusdigital.ro/ro/audio-buzzere/12247-buzzer-pasiv-de-33v-sau-3v.html)

* [Push buttons (×7)](https://protosupplies.com/product/tactile-momentary-pushbutton-yellow-12mm/)

* [16×2 character LCD display](https://ie.farnell.com/dfrobot/dfr0063/lcd-display-module-i2c-16x2-arduino/dp/3769973)

* [4×4 button matrix](https://protosupplies.com/product/tactile-keypad-4x4-matrix/)

* [Analog 2-axis XY joystick](https://www.pishop.us/product/analog-2-axis-thumb-joystick-with-select-button/)

* Connecting components:
  * Breadboard
  * Male-to-Male, Male-to-Female, Female-to-Female Cables

### Schematics

<!--Place your KiCAD schematics here.-->

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
-->

## Software

<!--
| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |
-->

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

<!--
1. [link](https://example.com)
2. [link](https://example3.com)
...
-->