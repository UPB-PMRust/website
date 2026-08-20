# Arcade Gaming Console

:::info 

**Author**: Dămoc Mara - Andreea \
**GitHub Project Link**: [link_to_github](https://github.com/UPB-PMRust-Students/acs-project-2026-DamocMara)

:::


## Description

This arcade game is built on STM32 using bare-metal Rust. It polls four buttons for real-time movement on a LCD module. The system tracks gameplay logic, including collision detection and scoring. Feedback is provided via three LEDs for lives and a PWM buzzer for sound effects. The project combines two arcade games (Snake and Dodge), real-time SPI display rendering, GPIO button input with debouncing, PWM music playback via passive buzzer with melodies stored on a microSD card, LED-based lives indicator, and flash memory persistence for high scores.


## Motivation

The motivation behind this project stems from a nostalgic passion for classic arcade games. By building a mini-console from scratch, I wanted to take a fun and engaging approach to learning embedded systems. This project merges my interest in gaming with technical skill-building, providing a hands-on way to understand how hardware and software interact to create a real-time interactive experience.


## Architecture 
The STM microcontroller serves as the central control unit, directing and managing all hardware components and executing the game logic developed in Rust.
The four directional buttons are tactile switches connected to STM32 GPIO pins with hardware pull-up resistors and software debouncing to control player movement (Up, Down, Left, Right). Two additional buttons handle game selection (Snake or Dodge) and reset.
The  LCD display  is connected via SPI for real-time rendering of the game world, player character, score, and status messages.
Three LEDs indicate the remaining player lives, turning off one by one as lives are lost.
A passive buzzer connected to a GPIO pin provides PWM-driven audio feedback, playing melodies loaded from a microSD card connected via a secondary SPI bus.
A microSD card module connected via SPI2 stores the game music, allowing melodies to be swapped without reflashing the firmware.
Flash memory on the STM32 is used for persistent high score storage across power cycles.
![Project Architecture](./architecture.webp)

## Log


### Week 5 - 11 May
Connected hardware components and written initial project documentation

### Week 12 - 18 May
Implemented the snake game logic

### Week 19 - 25 May
Finished dodge game logic and 3D printed the console

## Hardware

The system is powered by an STM32 microcontroller (ARM Cortex-M), utilizing its SPI, GPIO, and PWM peripherals to interface with external components.
![Project hardware1](./poza3.webp)
![Project hardware1](./poza1.webp)
![Project hardware2](./poza2.webp)

### Schematics

![Project Schematic](./schematica.webp)

### Bill of Materials



| Device | Usage | Price |
|--------|--------|-------|
| [Display LCD](https://www.display-lcd.com/product_details/155.html) | Game screen | [35 RON](https://www.emag.ro/display-tft-lcd-1-77-128x160-rgb-65k-culori-cog-st7735s-compatibil-arduino-bmx644/pd/D8G20R3BM/) |
| [Breadboard](https://atl.aim.gov.in/ATL-Equipment-Manual/jumper-cable/) | Connecting the components | [7 RON](https://www.emag.ro/breadboard-400-puncte-ai059-s69/pd/DRJ66JBBM/) |
| [NPN transistors](https://www.st.com/resource/en/datasheet/bd241c.pdf) | Electronic switch | [3 x 1.5 RON](https://www.emag.ro/tranzistor-npn-to-92-bc547-ai1495/pd/DVWV0WMBM/) |
| [Buttons](https://docs.arduino.cc/built-in-examples/digital/Button/)| User input | [6 x 2.5 RON](https://docs.arduino.cc/built-in-examples/digital/Button/) |
| [Buzzer](https://docs.arduino.cc/libraries/buzzer/)| Auditory feedback | [12,5 RON](https://www.emag.ro/buzzer-pasiv-pe-pcb-elektroweb-fara-generator-convertor-5-v-3-d-012/pd/DSLMTMYBM/)|



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7735-lcd](https://github.com/almindor/st7735-lcd) | Display driver for ST7735 | Used for the 1.77" TFT display |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing sprites and text |
| [cortex-m-rt](https://crates.io/crates/cortex-m-rt) | Startup code and runtime | Bare-metal execution for STM32 |
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | Hardware abstraction layer for STM32 microcontrollers | GPIO, SPI, Flash peripherals |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async task executor for embedded systems | Running game loop, music and button tasks concurrently |
| [embassy-time](https://crates.io/crates/embassy-time) | Async timers and delays | Game tick timing, PWM note duration, debouncing |
| [mipidsi](https://crates.io/crates/mipidsi) | Display driver for MIPI-compatible displays | ST7735S TFT LCD initialization and rendering |

## Links


1. [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
2. [Cortex-M Guide](https://ferrous-systems.com/blog/rust-on-stm32/)
3. [Super Mario melody code](https://github.com/robsoncouto/arduino-songs/blob/master/supermariobros/supermariobros.ino)

