# Project Name
A one line project description

:::info 

**Author**: Student Name \
**GitHub Project Link**: link_to_github

:::

## Description

Describe in a few words your project idea.

## Motivation

Why did you choose this project?

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
![photo](./monica.matache/Hardware1.webp)
![photo](./monica.matache/Hardware2.webp)

Detail in a few words the hardware used.

### Schematics
![Schematic](./monica.matache/Schematic2.webp)
Place your KiCAD schematics here.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico WH](https://ardushop.ro/ro/raspberry-pi/1945-raspberry-pi-pico-wh-wirelessheaders-6427854029621.html) | The main microcontroller with wireless and pre-soldered headers | [53 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [HC-SR505 PIR Motion Sensor](https://ardushop.ro/ro/module/508-modul-mini-senzor-pir-hc-sr505-6427854005922.html) | Detects motion to trigger the display of time and environmental data | [7 RON](https://ardushop.ro/ro/module/508-modul-mini-senzor-pir-hc-sr505-6427854005922.html) |
| [SPI LCD 1.8" 128x160 Module](https://ardushop.ro/ro/electronica/2124-modul-lcd-spi-128x160-6427854032546.html) | Displays time, temperature, humidity and weather | [29 RON](https://ardushop.ro/ro/electronica/2124-modul-lcd-spi-128x160-6427854032546.html) |
| [DS3231 Real-Time Clock Module](https://www.optimusdigital.ro/en/others/1102-ds3231-real-time-clock-module.html?search_query=DS3231+Real-time+Clock+Module&results=3) | Keeps accurate time even when the main power is off | [16 RON](https://www.optimusdigital.ro/en/others/1102-ds3231-real-time-clock-module.html) |
| Breadboard + jumper wires | Used to prototype the circuit without soldering | ~15 RON |


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
