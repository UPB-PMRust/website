# MiniMath Pocket Monsters
An embedded game where players catch Pokémon by solving math equations.

:::info
**Author:** Cojan Alexia-Ramona \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/fils-project-2026-0monia0  
:::

## Description 

MiniMath Pocket Monsters is an embedded game in which the player explores a virtual enviroment and ecounters 
Pokémon that can only be caught by solving mathematical equations. The player can move within the game either by using
physical buttons or by using motion input from an accelerometer. When a Pokémon randomly appears, the player is prompted 
with a mathematical equation that needs to be solved in a limited time. If the correct answer is given, the creature
is successfully captured, otherwise, it escapes. The difficulty of the mathematical challenges varies depending on the rarity of the encountered Pokémon.
There is also a score that shows how many Pokémon you caught, if u get one answer wrong, the score goes back to 0. 
The game provides visual feedback from a TFT display, where the game environment and interactions are rendered, an additional display for auxilliary information such as score or status, as well as LEDs. 
Audio feedback is given through a passive buzzer which provides sound effects during key events like successfuly capturing a Pokémon or losing it, and encounters.

## Motivation 

The motivation behind this project is personal, as well as educational. I chose this topic because i am interested in the Pokémon franchise and i like video games.
I wanted to bring my interests to life by making my own version of the popular Pokémon games. At the same time, this project gives me the opportunity to learn
and explore how interactive applications can be implemented on microcontrollers.The inspiration for this project comes from classic Pokémon games, which provide an engaging experience through simple mechanics.
Another source of inspiration is represented by the modern game Pokémon Go, where real world actions influence the behvior in the game. 
This project adapts that idea in a simplified manner by using motion input from an accelerometer, allowing the player to interact with the game in a more dynamic way.The game can also be considered an educational tool, 
as it encourages quick thinking and problem solving through time-limited mathematical challenges. By combining gameplay with simple equations, the system provides a fun and engaging way to practice basic arithmetic skills.

 ## Architecture

![Architecture](architecture.webp)

## Log

### Week 1-6: 
I started brainstorming for my project idea and getting approval from my professors.

### Week 6-9: 
I purchased all the needed components, doing tests to make sure everything works right and working on the documentation milestone.

### Week 9-14: 

## Hardware

The system is built around an STM32 microcontroller, which controls all the components and runs the main game logic.
The TFT display is connected using SPI and is used to show the game itself, including the map, player, and interactions. The OLED display is connected through I2C and is used to display additional information such as score or messages.
The MPU9250 sensor is also connected via I2C and is used to detect movement. The data from the accelerometer will be processed in order to detect walking patterns, which are then used to move the player in the game.
User input is provided through push buttons connected to GPIO pins, allowing direct control of the game. A passive buzzer is also connected to a GPIO pin and is used to generate simple melodies during gameplay.
LEDs will also be connected to GPIO pins through resistors and be used for debugging or simple visual feedback. All components are connected to the microcontroller using standard interfaces such as SPI, I2C, and GPIO.

## Schematics

Place your KiCAD schematics here.

## Bill of Materials

| Device | Usage | Price |
|--------|------|-------|
| STM32 Nucleo-U545RE-Q | main microcontroller board used to control the system | borrowed from laboratory |
| [BreadBoard 400 tie points](https://www.emag.ro/breadboard-400-puncte-ai059-s69/pd/DRJ66JBBM/) | used to connect components and build circuits without soldering the components together | [~7 RON](https://www.emag.ro/breadboard-400-puncte-ai059-s69/pd/DRJ66JBBM/) | 
| [Dupont wires male-to-male, 20 cm](https://www.emag.ro/10-x-fire-dupont-tata-tata-20cm-ai308-s451/pd/DV8M9WBBM/) | used to connect components on the breadboard and to the microcontroller | [~2 RON](https://www.emag.ro/10-x-fire-dupont-tata-tata-20cm-ai308-s451/pd/DV8M9WBBM/) |
| [LED kit 3mm/5mm, assorted colors](https://www.emag.ro/kit-200-buc-led-3mm-5mm-de-diferite-culori-ai707/pd/D4DJYTMBM/) | used for visual indicators | [~18 RON](https://www.emag.ro/kit-200-buc-led-3mm-5mm-de-diferite-culori-ai707/pd/D4DJYTMBM/) |
| [Resistors 220Ω, 1/4W](https://www.emag.ro/set-10-rezistoare-se-1-4w-220ohm-multicolor-k291-r1-r3-r4/pd/DYZD1DYBM/) | used to limit current in circuits, especially for LEDs | [~13 RON](https://www.emag.ro/set-10-rezistoare-se-1-4w-220ohm-multicolor-k291-r1-r3-r4/pd/DYZD1DYBM/) |
| [Tactile push buttons 6x6 mm](https://www.emag.ro/set-180-micro-intrerupatoare-tact-switch-6-x-6-mm-multicolor-1-y-012/pd/DC7GR8MBM/) | used as user input controls for interacting with the game | [~37 RON](https://www.emag.ro/set-180-micro-intrerupatoare-tact-switch-6-x-6-mm-multicolor-1-y-012/pd/DC7GR8MBM/) |
| [OLED display 0.96", 128x64, SSD1306, I2C](https://www.emag.ro/display-oled-0-96-inch-128x64-lcd-ssd1306-interfata-i2c-compatibil-arduino-albastru-bmx156/pd/DQ4HZF3BM/) | used to display additional information such as score | [~30 RON](https://www.emag.ro/display-oled-0-96-inch-128x64-lcd-ssd1306-interfata-i2c-compatibil-arduino-albastru-bmx156/pd/DQ4HZF3BM/) |
| [Passive buzzer module KY-006](https://www.bitmi.ro/module-electronice/modul-buzzer-pasiv-ky-006-10678.html) | used to generate sound effects and simple melodies during gameplay | [~3 RON](https://www.bitmi.ro/module-electronice/modul-buzzer-pasiv-ky-006-10678.html) |
| [MPU9250 9-axis sensor](https://electronicmarket.ro/product/mpu9250-9-axe-giroscop-accelerometru-modul-de-camp-magnetic) | the accelerometer will be used to detect walking motion by analyzing variations in acceleration | [~90 RON](https://electronicmarket.ro/product/mpu9250-9-axe-giroscop-accelerometru-modul-de-camp-magnetic) |
| [TFT display (1.3", IPS, ST7789V, SPI, 240x240](https://sigmanortec.ro/display-tft-13-ips-spi-65k-culori-lcd-st7789v-240x240-7p) | used as the main display for rendering game graphics | [~30 RON](https://sigmanortec.ro/display-tft-13-ips-spi-65k-culori-lcd-st7789v-240x240-7p) |

## Software

| Library | Description | Usage |
|--------|------------|-------|
| [embassy](https://github.com/embassy-rs/embassy) | Async embedded framework for Rust | Used for handling tasks and timing |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer | Used for interfacing with peripherals |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for rendering text and graphics on displays |
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the TFT display |
| [ssd1306](https://github.com/ssd1306-rs/ssd1306) | OLED display driver | Used for the OLED display |

## Links

1. [Understanding how accelerometer works](https://youtu.be/KuekQ-m9xpw?si=_hHmB_l6pxTLukfG)
2. [Accelerometer datasheet](https://www.ultralibrarian.com/2025/12/16/mpu9250-datasheet-explained-ulc/)

