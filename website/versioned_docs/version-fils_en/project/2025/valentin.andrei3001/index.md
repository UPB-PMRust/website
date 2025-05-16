# Project Name
A rotative solar panel that moves toward the sun.

:::info 

**Author**: Andrei Valentin-Alexandru \
**GitHub Project Link**: https://github.com/ValiRupe

:::

## Description

The project will consist of a small solar panel attached to a servo motor. A number of photoresistors will be placed around the solar panel and the panel will move depending in the direction of the highest sunlight input. The energy will be stored and used for heating water behind it.


## Motivation

I wanted for a long time to learn and to do a project based around solar energy. I am not as motivated for a solar future as I am about the solar cell tehnology and it's intricacies.

## Architecture 

Raspberry Pi Pico 2W is the machine which controls everything.

The solar panel is the main component which will be used for the application in gathering energy.

The photoresistors will be used as sensors and with the use of an ADC we will get the information needed.

The servomotor will rotate the panel based on the data from the photoresistors.



## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Detail in a few words the hardware used.

### Schematics 
[Scheme](scheme.png)

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
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [40 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |


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
