# Theremin
An instrument that plays without you ever touching it.

:::info 

**Author**: Dogărescu Alex \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/fils-project-2026-alexdogarescu](https://github.com/UPB-PMRust-Students/fils-project-2026-alexdogarescu)

:::

<!-- do not delete the \ after your name -->

## Description

A theremin is an instrument that uses your hands as a capacitor between two metal rods. This project aims to recreate that using two ultrasonic sensors. It also has a few effects that can be applied to the sound so it can be your own.

## Motivation

My hobby is music production. I have been doing ambiental, metal and digicore or adjacent genres. Especially digicore plays with the form of the wave to make unique melodies. I got the idea for this project when I was making a song - and I thought a theremin would be an interesting concept to explore.

## Architecture 

Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other

## Log

<!-- write your progress here every week -->

### Week 13 - 19 April

The STM32 arrived. Played with it and everything worked fine. I also messed around with an HC-SR04 from a friend to see how it works before mine arrives.

### Week 20 - 26 April

I ordered my own sensors and an LED display. Plan to make a working skeleton next week.

### Week 4 - 10 May

### Week 11 - 17 May

### Week 18 - 24 May

## Hardware

- STM32U545
- 2 x HC-SR04+ - sensors to detect distance to hands
- "ST7735" LCD Display - used to show current effect and how much % it is applied to the wave. Also mine is most likely a knockoff.


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
| [STM32U545](https://www.st.com/en/microcontrollers-microprocessors/stm32u535-545.html) | Microcontroller | [106 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D) |
| [2 x HC-SR04+](https://web.eece.maine.edu/~zhu/book/lab/HC-SR04%20User%20Manual.pdf) | Ultrasonic Sensors | [2 x 15 RON](https://www.optimusdigital.ro/en/ultrasonic-sensors/2328-senzor-ultrasonic-de-distana-hc-sr04-compatibil-33-v-i-5-v.html?search_query=HC-SR04+&results=15) |
| ["ST7735 LCD"](https://www.displayfuture.com/Display/datasheet/controller/ST7735.pdf) | Displaying effect information | [35 RON](https://www.optimusdigital.ro/en/lcds/8589-144-lcd-for-stc-stm32-and-arduino-boards-5-v.html?search_query=LCD&results=197) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [ST7735](https://github.com/afiskon/stm32-st7735) | Display driver for ST7735S | Used for the display |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...
