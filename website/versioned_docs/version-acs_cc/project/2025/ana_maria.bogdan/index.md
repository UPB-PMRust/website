# Light Pattern Memory Game
A light pattern recognition memory game powered by a Raspberry Pi Pico and Rust.

:::info

**Author**: Bogdan Ana-Maria-Iulia \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-AnaBogdan

:::

## Description
A fully interactive memory game built on a **Raspberry Pi Pico - RP2040**, using the **Rust** programming language and Embassy async framework, based on recognizing and reproducing light patterns. The player must memorize a sequence of LEDs flashing in a 4x4 grid and reproduce it using capacitive touch sensors. The project features multiple game modes, a scoring system, timeout-based timer, high score storage in flash memory, and audio feedback. In multiplayer mode, two players compete to replicate sequences set by each other. Sequences can be random or predefined, and feedback is also given through varied sounds. At the end of each round, players' scores are compared and displayed.

### Features:
+ **4x4 WS2812 RGB LED matrix** for displaying light sequences.
+ **Capacitive touch sensor (TTP229)** for player input.
+ **240x240 ST7735 OLED Display** for visual game status, scores, and instructions.
+ Audio feedback for correct/incorrect actions and UI sounds.
+ Multiplayer mode with random or user-defined sequences.
+ High score saving to flash memory.
+ Timeout-based turn system to increase challenge.

## Motivation
This project was inspired by a memory game I played during a Neurolympics online assessment. I wanted to recreate the experience in hardware using **Rust** and the **RP2040**, turning a digital challenge into a hands-on embedded system that combines pattern recognition, real-time input, and visual/audio feedback.

## Architecture
The **Raspberry Pi Pico - RP2040** acts as the central unit, managing peripherals and orchestrating the game flow.

- **Capacitive Touch Module (GPIO/I2C)**: detects player input.
- **WS2812 LED Matrix (SPI)**: displays animated light patterns.
- **OLED Display ST7735 (SPI)**: shows current mode, instructions, score, and end results.
- **Audio Buzzer (PWM/GPIO)**: emits feedback tones based on actions.
- **Internal Flash**: stores the highest score between sessions.

### Game Flow Summary:
1. **Startup**: OLED shows welcome message and instructions.
2. **Pattern Display**: A sequence of LEDs light up in the 4x4 grid.
3. **Player Turn**: User reproduces the sequence via touch sensors.
4. **Feedback**: Correct/incorrect tones and visual confirmation.
5. **Scoring**: Points are awarded per round; failure resets current round.
6. **Multiplayer (optional)**: Each player sets a sequence for the other to repeat.
7. **Game End**: Scores are displayed; high score is updated if beaten.


### Block Scheme
![diagram](block_scheme.webp)

## Log

### Week 5 – 11 May

Completed hardware setup, including pin configuration and purchasing missing cables. Initialized the software repository.

### Week 12 – 18 May

Completed final hardware steps: connected all peripherals to the Raspberry Pi Pico and verified connectivity. Created the KiCad schematics for the hardware setup. On the software side, set up all modules and peripherals, wrote initial test code for each to verify functionality, and pushed the code to GitHub.

#### Photos
![diagram](projectPMhardware1.webp)
![diagram](projectPMhardware2.webp)
![diagram](projectPMhardware3.webp)

### Week 19 – 25 May

This week marked the completion of the software development phase for the Light Pattern Memory Game. The focus was on finalizing core functionality, integrating all modules, and conducting extensive debugging and testing to ensure the game runs reliably.

## Hardware

1. **Raspberry Pi Pico - RP2040** 
- Main microcontroller. Manages all peripheral communication, handles game logic, input processing, and feedback signals.

2. **4x4 WS2812 RGB LED Matrix**
- Displays animated light patterns. Controlled via SPI or bit-banged protocol depending on implementation.

3. **TTP229 Capacitive Touch Sensor Module**
- Used for detecting user input. Offers up to 16 touch-sensitive buttons mapped to the LED matrix grid.

4. **ST7735 OLED Display (240x240)**
- Shows game status, round instructions, score updates, and high score.

5. **Audio Buzzer (Piezo)**
- Provides auditory feedback (correct/incorrect sounds, game tones, and timer alerts). Controlled via PWM.

6. **Breadboard and Jumper Wires**
- For prototyping, connecting all components in a modular and easily adjustable layout.

7. **Power Supply (USB / 5V adapter)**
- Powers the Raspberry Pi Pico and peripherals safely and consistently.

### Schematics
![diagram](kicad_proj.svg)

## Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico](https://www.emag.ro/microcontroller-raspberry-pi-rp2040-pico/pd/DKQQWNMBM) | Microcontroller | [30 RON](https://www.emag.ro/microcontroller-raspberry-pi-rp2040-pico/pd/DKQQWNMBM) x 2 |
| [4x4 LED Matrix WS2812](https://www.emag.ro/matrice-led-4x4-16bit-ws2812-rgb-ai1153/pd/DMDBNJMBM) | Game Outputs | [20 RON](https://www.emag.ro/matrice-led-4x4-16bit-ws2812-rgb-ai1153/pd/DMDBNJMBM) |
| [ST7735 OLED 240x240 Display](https://www.emag.ro/display-lcd-ips-1-3-inch-240x240-65k-hd-interfata-spi-controler-st7735-arduino-rx169/pd/DC1HY6YBM) | Visual display | [33 RON](https://www.emag.ro/display-lcd-ips-1-3-inch-240x240-65k-hd-interfata-spi-controler-st7735-arduino-rx169/pd/DC1HY6YBM)|
| [TP229 Capacitive Touch Sensor Module](https://www.optimusdigital.ro/ro/senzori-senzori-de-atingere/1112-modul-senzor-de-atingere-capacitiv-ttp229.html?search_query=tastatura&results=51) | Player Input | [9 RON](https://www.optimusdigital.ro/ro/senzori-senzori-de-atingere/1112-modul-senzor-de-atingere-capacitiv-ttp229.html?search_query=tastatura&results=51) |
| [Breadboard](https://www.optimusdigital.ro/en/breadboards/8-breadboard-hq-830-points.html?search_query=breadboard&results=363) | Connectivity | [10 RON](https://www.optimusdigital.ro/en/breadboards/8-breadboard-hq-830-points.html?search_query=breadboard&results=363) x2 |
| [Breadboard, Jumpers, Power supply etc](https://www.emag.ro/set-componente-electronice-breadboard-830-puncte-led-uri-compatibil-arduino-si-raspberry-pi-zz00044/pd/DRXG4XYBM/?path=set-componente-electronice-breadboard-830-puncte-led-uri-compatibil-arduino-si-raspberry-pi-zz00044/pd/DRXG4XYBM) | Connectivity | [60 RON](https://www.emag.ro/set-componente-electronice-breadboard-830-puncte-led-uri-compatibil-arduino-si-raspberry-pi-zz00044/pd/DRXG4XYBM/?path=set-componente-electronice-breadboard-830-puncte-led-uri-compatibil-arduino-si-raspberry-pi-zz00044/pd/DRXG4XYBM) |

TOTAL: ~200 RON

## Software

This project implements an enhanced memory game on an RP2040 microcontroller using the Embassy async runtime. The game combines a WS2812 LED matrix, an ST7789 color display, and a TTP229 capacitive keypad to create an interactive experience where players memorize and reproduce sequences of illuminated squares. The code handles all hardware initialization, including SPI setup for the LEDs and display, and runs asynchronous tasks for keypad input detection, buzzer sound feedback, and game state management. Players progress through increasing difficulty levels, with lives and scoring tracked and shown on the screen, alongside visual and audio feedback for success and errors.

The game logic is designed as a state machine cycling through menu, sequence display, player input, and game-over or victory states. Each stage leverages carefully timed LED animations and screen drawings to guide the player. Debounced keypad input triggers buzzer sounds and updates the game flow asynchronously, providing a smooth user interface. The project showcases advanced embedded Rust techniques with asynchronous concurrency, precise peripheral control, and a modular structure suitable for educational and hobbyist embedded gaming applications.

Some of the functions used are:
- show_menu: Clears the display and draws the game title, instructions, and high score text.
- draw_game_grid: Draws the 4x4 grid on the screen, coloring specific squares based on the game sequence.
- display_score: Shows the player’s current score at the top-left corner of the screen.
- display_lives: Displays remaining lives as heart icons at the top-right corner.
- display_message: Clears the center area and shows a custom message like “Game Over” or “Success.”
- clear_sequence_highlight: Removes highlights from squares previously lit to reset the grid display.
- highlight_square: Lights up a single square in the grid with a specified color to indicate the sequence step.

### Schematics
![diagram](diagrama_software.webp)

| Library | Description | Usage |
|--------|-------------|--------|
| [embassy-rp](https://github.com/embassy-rs/embassy) | RP2040 HAL | Async device and timer management |
| [embedded-hal-async](https://github.com/rust-embedded/embedded-hal) | HAL Traits | I2C abstraction |
| [ttp229](https://docs.rs/ttp229/latest/ttp229/) | Capacitive Sensor Driver | Used for the game input |
| [st7735](https://docs.rs/st7735/0.1.0/st7735/) | OLED Driver | Display game mode and rules |
| [ws2812_spi](https://docs.rs/ws2812-spi/latest/ws2812_spi/) | LED Matrix Driver | Display game mode and rules |
| [heapless](https://github.com/rust-embedded/heapless) | Fixed capacity collections | Buffer text messages for OLEDs |
| [defmt](https://github.com/knurling-rs/defmt) + [defmt-rtt](https://github.com/knurling-rs/defmt) | Logging Framework | Used for real-time debug output over RTT, ideal for embedded logging |
| [panic-probe](https://github.com/knurling-rs/panic-probe) | Panic Handler | Provides panic messages compatible with defmt |

## Links
1. [Game inspiration - Neurolympics](https://neurolympics.nl/campaign/index-default.html?c=43)
2. [Using TTP229 Capacitive Keypad with Raspberry Pi](https://www.youtube.com/watch?v=AMnMUnouQ9I&ab_channel=TerrySturtevant)


