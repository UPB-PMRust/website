#  8-Puzzle Touch Console with Camera 
A mini-console that turns your photos into an 8-puzzle you can play using the touchscreen or buttons.

:::info 

**Author**: Alexandru Maria-Mihaela \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-maria-alexandru

:::

<!-- do not delete the \ after your name -->

## Description

This project is a laptop-like mini-console equipped with an attached camera. When a game begins, the camera takes a photo that is sliced into a 3x3 grid, shuffled and one tile is removed to create a classic sliding 8-puzzle. Players must reassemble their picture by sliding the tiles using either the touchscreen or physical buttons. The interface also features a built-in timer that tracks both the current elapsed time and the player's best score.

## Motivation

I chose this project because it is based on a console I had when I was little. I could take a picture and instantly play it as a sliding puzzle, and I wanted to recreate that fun experience.

## Architecture 

<!-- 
 Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other
--->

The software architecture of the console is modular, divided into distinct layers that separate the core game logic from direct hardware interaction. The main components are:

- **Game Engine**: This is the software "brain" of the application. It manages the internal state of the 8-puzzle game (a 3x3 matrix), tracks the position of the empty space and validates allowed moves. It also evaluates the win condition (checking if the tiles are in the correct order) and keeps track of the move count or elapsed time.

- **Input Management Subsystem**: It is responsible for capturing and processing user input (buttons pressed, movements made on screen).

- **Graphics & Display Subsystem**: It is responsible for rendering the visual representation of the game state to the screen. This includes drawing the 3x3 grid, updating the positions of the numbered tiles, and rendering user interface elements (such as the move counter or timer). 

- **Audio Subsystem**: It translates specific game events triggered by the Game Engine into corresponding sound effects, such as a mechanical click when a tile successfully slides, an error tone for invalid moves, or a victory jingle when the puzzle is solved.

## Log

<!-- write your progress here every week -->

### Week 13 - 19 April
Ordered the hardware components.

### Week 20 - 26 April

### Week 27 - 03 May

### Week 04 - 10 May

## Hardware

<!--  Detail in a few words the hardware used. -->

- **STM32U5 microcontroller (Nucleo-U545RE-Q)**
- **3.5" Touch Screen 480x320 SPI TFT ILI9488:** The display unit of the console
- **VGA Camera OV7670 640 x 480px**: Captures images input for the system
- **Microswitches TC-1212T (12x12x7.3mm):** Mechanical tactile switches used for the directional pad and user action buttons
- **Acoustic buzzer for microcontrollers:** Provides auditory feedback and sound effects for various game events
- **DC-DC Step-up Module 1-5V to 5V:** A voltage regulator that boosts the fluctuating battery power to provide a stable 5V output for the microcontroller and display

### Schematics

<!-- Place your KiCAD or similar schematics here in SVG format. -->

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| [3.5" Touch Screen 480x320 SPI TFT ILI9488 (1x)](http://www.lcdwiki.com/3.5inch_SPI_Module_ILI9488_SKU:MSP3520) | Main display and visual output | [80.43 RON](https://www.drot.ro/platforma-arduino/149154-ecran-tactil-3-5-480x320-spi-tft-ili9488.html) |
| [VGA Camera OV7670 640 x 480px (2x)](https://web.mit.edu/6.111/www/f2016/tools/OV7670_2006.pdf) | Capturing images/video input | [23.10 RON](https://www.drot.ro/platforma-arduino/900-camera-vga-ov7670-640-x-480px.html) |
| [Acoustic buzzer for microcontrollers (1x)](https://components101.com/misc/buzzer-pinout-working-datasheet) | Audio feedback and sound effects | [4.21 RON](https://www.drot.ro/platforma-arduino/849-buzzer-acustic-pentru-microcontrolere.html) |
| [Microswitch TC-1212T - 12x12x7.3mm SMD PCB (5x)](https://docs.rs-online.com/1b59/0900766b81126ccb.pdf) | Mechanical switches for user input/controls | [10.44 RON](https://www.drot.ro/platforma-arduino/1180-mikroswitch-tc-1212t-12-x-12-x-7-3-mm-smd-pcb.html) |
| [Button cap for microswitch - Red (1x)](https://www.drot.ro/platforma-arduino/1181-buton-pentru-mikroswitch-ro-u-12-x-12-x-7-3-mm.html) | Plastic cap for microswitch (Red) | [1.30 RON](https://www.drot.ro/platforma-arduino/1181-buton-pentru-mikroswitch-ro-u-12-x-12-x-7-3-mm.html) |
| [Button cap for microswitch - Blue (1x)](https://www.drot.ro/platforma-arduino/1182-buton-pentru-mikroswitch-albastru-12-x-12-x-7-3-mm.html) | Plastic cap for microswitch (Blue) | [1.30 RON](https://www.drot.ro/platforma-arduino/1182-buton-pentru-mikroswitch-albastru-12-x-12-x-7-3-mm.html) |
| [Button cap for microswitch - Green (1x)](https://www.drot.ro/platforma-arduino/1183-buton-pentru-mikroswitch-verde-12-x-12-x-7-3-mm.html) | Plastic cap for microswitch (Green) | [1.30 RON](https://www.drot.ro/platforma-arduino/1183-buton-pentru-mikroswitch-verde-12-x-12-x-7-3-mm.html) |
| [Button cap for microswitch - Yellow (1x)](https://www.drot.ro/platforma-arduino/1184-buton-pentru-mikroswitch-galben-12-x-12-x-7-3-mm.html) | Plastic cap for microswitch (Yellow) | [1.30 RON](https://www.drot.ro/platforma-arduino/1184-buton-pentru-mikroswitch-galben-12-x-12-x-7-3-mm.html) |
| [Button cap for microswitch - White (1x)](https://www.drot.ro/platforma-arduino/1185-buton-pentru-mikroswitch-alb-12-x-12-x-7-3-mm.html) | Plastic cap for microswitch (White) | [1.30 RON](https://www.drot.ro/platforma-arduino/1185-buton-pentru-mikroswitch-alb-12-x-12-x-7-3-mm.html) |
| [4x AA Battery Holder SBH-341-2A (1x)](https://www.comf-hk.com/storage/SBH-341-2A-Data-Sheet.pdf) | Power supply holder for 4x AA batteries | [8.72 RON](https://www.drot.ro/platforma-arduino/1436-suport-pentru-patru-baterii-aa-sbh-341-2a.html) |
| [DC-DC Step-up Module 1-5V to 5V 500mA (1x)](https://www.ti.com/product/TPS61241) | Voltage regulator to provide stable 5V | [7.19 RON](https://www.drot.ro/platforma-arduino/1699-modul-step-up-dc-dc-de-la-1-5v-la-5v-500ma.html) |
| [Plexiglass mount for OV7670 camera module (2x)](https://www.drot.ro/platforma-arduino/123946-suport-pentru-modulul-de-camera-ov7670-plexiglas.html) | Physical mount for the camera module | [7.42 RON](https://www.drot.ro/platforma-arduino/123946-suport-pentru-modulul-de-camera-ov7670-plexiglas.html) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

1. [ How To Make Mini Laptop at Home ](https://www.youtube.com/watch?v=pFho9bYt6Us)
