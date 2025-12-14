# RustBox
RustBox - The Rust Gaming Box

:::info 

**Author**: Frimu Aurel-Viorel \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-dm-2025-AurasV

:::

## Description

Using the power of two screens, two joysticks and a few buttons, I will (hopefully) implement a couple games that can be played in 1 versus 1 mode or singleplayer. Depending on how difficult it turns out to be the games included can be: Tic Tac Toe, Snake, Pong, Brick Breaker and more&trade;!

## Motivation

I have always wanted to create some portable gaming devices since I've first seen them on YouTube and other websites. Wanting to one up my roommate that made something similar - [Rogue Rust](https://embedded-rust-101.wyliodrin.com/docs/fils_en/project/2025/denis_iulian.pavel)

## Architecture 

![Architecture](diagram.svg)

The STM32 Board is the whole gaming station and it connects to the rest of the components like so:
 - To the 1602 LCD Display using I2C;
 - To the IPS TFT Display using SPI;
 - To the Two Axis HW-504 Joysticks using ADC for the X and Y movements and using GPIO for the push button they each have;
 - To the Push Button using GPIO;

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The hardware used is: 

NUCLEO-U545RE-Q - STM32 Board - 1x \
TFT IPS Display - GMT130-V1.0 with Driver IC - ST7789 using SPI - 1x \
LCD 1602 Text Display - using I2C - 1x \
HW-504 Joystick - 2x 

### Schematics

KiCAD schematic coming soon&trade;

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [NUCLEO-U545RE-Q - STM32 Board](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The Microcontroller | 0 RON - Loaned from Class |
| [Ecran LCD - 1602 IIC/I2C](https://www.handsontec.com/dataspecs/module/I2C_1602_LCD.pdf) | To Display Simple Text | [21.04 RON](https://www.emag.ro/ecran-lcd-1602-iic-i2c-albastru-ai848-s815/pd/D0WQLTMBM/) |
| [TFT Color Display - GMT130-V1.0 ST7789](https://goldenmorninglcd.com/tft-display-module/1.3-inch-240x240-st7789-gmt130-v1.0/) | Showing the Actual Game | [40.99 RON](https://www.emag.ro/afisaj-grafic-tft-ips-de-240x240px-spi-1-3-inch-tft-1-3-st7789-black/pd/DXKNW6MBM/) |
| [Two Axis Joystick - HW-504](https://www.handsontec.com/dataspecs/accessory/PS2-Joystick.pdf) | Used for Playing the Games | [5.35 RON](https://www.optimusdigital.ro/ro/senzori-senzori-de-atingere/742-modul-joystick-ps2-biaxial-negru-cu-5-pini.html) x 2 |
| [Push Button - for Breadboard](https://www.arduino.cc/documents/datasheets/Button.pdf) | Reset Button | [1 RON](https://www.emag.ro/push-button-pentru-breadboard-ai138-s254/pd/DGBZQGMBM/) |
| [Breadboard HQ - 830 Points](http://www.pgccphy.net/1020/datasheets/ELEGOO%20830%20430%20tie-points%20Breadboard.pdf) | Connecting Everything | Got as Part of a [Kit](https://www.emag.ro/kit-plusivo-microcontroller-starter-programabil-in-arduino-ide-x001fpqyl1/pd/DKJN9VMBM/), Standalone Price - [9.98 RON](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/8-breadboard-830-points.html) |
| [Female to Male Wires - 20 cm](https://www.optimusdigital.ro/en/wires-with-connectors/92-female-male-wire40p-20-cm.html) | Wires | 7.99 RON
| [Male to Male Wires - Various Sizes](https://www.optimusdigital.ro/en/wires-with-connectors/92-female-male-wire40p-20-cm.html) | Wires | Got as part of a [Kit](https://www.emag.ro/kit-plusivo-microcontroller-starter-programabil-in-arduino-ide-x001fpqyl1/pd/DKJN9VMBM/), standalone price - 7.99 RON |
| Total | If Hardware Obtained From Kits Are Accounted | 99.69 RON |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [mipidsi](https://github.com/almindor/mipidsi) | MIPI Display Serial Interface unified driver | Used to control the display |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |
| [defmt](https://github.com/knurling-rs/defmt) | Efficient, deferred formatting for logging on embedded systems | Used for logging |

## Links

1. [Rogue Rust](https://embedded-rust-101.wyliodrin.com/docs/fils_en/project/2025/denis_iulian.pavel)
2. [I Made an Original Xbox Portable](https://www.youtube.com/watch?v=W3OK9A_RbSI)
3. [I Built My Own Steam Deck!](https://www.youtube.com/watch?v=7uqis3KFYeo) 