# Minesweeper
Classic implementation of the minesweeper game where the goal is to reveal all the cells without hitting a mine.

:::info 

**Author:** Dobrin David-Andrei
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/proiect-ddavid03dda

:::

## Description

Classic implementation of the minesweeper game where the goal is to reveal all the cells without hitting a mine. Also several difficulty levels that increase the number of mines on the map. The player can see in a menu the battery status and select the difficulty before the game starts and then play until he loses or wins.

## Motivation

It was a favorite game I used to play as a kid when the internet was down. I wanted to build a physical version I could keep around the house—complete with sounds and vibration—to give it a fresh, tactile feel.  

## Architecture

![Block-diagram of the Minesweeper hardware architecture](architecture.webp)

- **Raspberry Pi Pico 2 W** as central control unit, running the game logic in Rust.  
- **Vibration motor** to signal when the player hits a mine (loss).  
- **Buzzer** for sound effects—menu navigation, tile selection, win/loss alerts.  
- **ILI9341 LCD** to display the menu, battery status, and game grid.  
- **Buttons** for cursor movement, selection, and menu control.

## Log ­#

### Week 5 – 11 May

I have made the first steps in the project by choosing the hardware components and the software libraries that I will use. The ILI9341 LCD will be used to display the game grid and messages. I have also chosen to use a vibration motor for haptic feedback and a buzzer for audio feedback.

### Week 12 – 18 May

I have soldered both Raspberry Pi Pico 2 W, the main controller, and the Raspberry Pi Pico 1 W used as the debugger. I connected the all the buttons, the lcd, the buzzer and the vibration motor to the microcontroller. I build the schematic of the project in kiCad. 

### Week 19 – 25 May

I have implemented the game logic in Rust, including the difficulty selection menu, grid generation, tile revealing, and win/loss conditions. I also added the buzzer and vibration motor functionality for feedback on game events. The LCD displays the game grid and messages.

## Hardware

1. **Raspberry Pi Pico 2 W**  
   * **Purpose**: Main controller.  
   * **Function**: Runs the Minesweeper game logic in Rust, reads button inputs, and drives the display, buzzer, and vibration motor.

2. **Vibration Motor**  
   * **Purpose**: Haptic feedback.  
   * **Function**: Vibrates briefly when the player uncovers a mine (loss).

3. **Buzzer**  
   * **Purpose**: Audio feedback.  
   * **Function**: Emits tones for menu navigation, tile reveals, and win/loss alerts.

4. **ILI9341 LCD**  
   * **Purpose**: Visual interface.  
   * **Function**: Displays the difficulty menu, battery status, game grid, and end-game messages.

5. **Push-buttons (×5)**  
   * **Purpose**: User controls.  
   * **Function**: Up, Down, Left, Right for cursor movement; Select for revealing tiles and confirming menu choices.

### Hardware Overview

- The **Raspberry Pi Pico 2 W** orchestrates all inputs and outputs, executing the Minesweeper logic.  
- **Push-buttons** let the player navigate menus and move the cursor across the minefield.  
- The **ILI9341 LCD** renders the game grid, menus, and battery level in real time.  
- The **Buzzer** provides immediate audio cues for safe tile reveals, mine hits, and victories.  
- The **Vibration Motor** delivers a tactile buzz when a mine is uncovered, reinforcing the loss event.

Hardware video that showes that everything connected workes as expected: https://imgur.com/WwEwjHi .

![Hardware Picture1](hard1.webp)

![Hardware Picture2](hard2.webp)

![Hardware Picture3](hard3.webp)

## Schematics

![Minesweeper KiCad schematic](kicad_project.svg)

## Bill of Materials

| Device                                                                                                                                   | Usage                                 | Price                                                                                                                                     |
|------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| [Raspberry Pi Pico 2 W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html)                             | Main microcontroller                  | [80 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html)                                              |
| [Vibration motor module](https://www.optimusdigital.ro/ro/motoare-motoare-cu-vibratii/8251-modul-cu-motor-cu-vibraii.html)                 | Loss feedback vibration               | [15 RON](https://www.optimusdigital.ro/ro/motoare-motoare-cu-vibratii/8251-modul-cu-motor-cu-vibraii.html)                                  |
| [Colored wires male–male (40 × 10 cm)](https://www.optimusdigital.ro/ro/fire-fire-mufate/881-set-fire-mama-mama-40p-15-cm.html)             | Wiring between Pico and peripherals   | [5 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/881-set-fire-mama-mama-40p-15-cm.html)                                            |
| [Colored wires male–female (40 × 15 cm)](https://www.optimusdigital.ro/ro/toate-produsele/877-set-fire-mama-tata-40p-15-cm.html)             | Wiring sensor/buttons                 | [8 RON](https://www.optimusdigital.ro/ro/toate-produsele/877-set-fire-mama-tata-40p-15-cm.html)                                             |
| [Colored wires female–female (40 × 15 cm)](https://www.optimusdigital.ro/ro/fire-fire-mufate/881-set-fire-mama-mama-40p-15-cm.html)           | Breadboard‐to‐module wiring           | [7 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/881-set-fire-mama-mama-40p-15-cm.html)                                            |
| [Passive buzzer (3.3 V)](https://www.optimusdigital.ro/ro/audio-buzzere/12247-buzzer-pasiv-de-33v-sau-3v.html)                              | Sound effects                         | [1 RON](https://www.optimusdigital.ro/ro/audio-buzzere/12247-buzzer-pasiv-de-33v-sau-3v.html)                                               |
| [NPN Transistor 2N2222 TO-92](https://www.optimusdigital.ro/ro/componente-electronice-tranzistoare/935-tranzistor-s9013-npn-50-pcs-set.html) | Driving buzzer/vibration motor        | [0.17 RON](https://www.optimusdigital.ro/ro/componente-electronice-tranzistoare/935-tranzistor-s9013-npn-50-pcs-set.html)                  |
| [Breadboard HQ (400 pts)](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/44-breadboard-400-points.html)                         | Prototyping                           | [4.50 RON](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/44-breadboard-400-points.html)                                       |
| [Resistor 0.25 W 2.2 kΩ](https://www.optimusdigital.ro/ro/fire-fire-mufate/884-set-fire-tata-tata-40p-10-cm.html)                             | Current limiting                      | [0.10 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/884-set-fire-tata-tata-40p-10-cm.html)                                           |
| [Diode 1N4007](https://www.optimusdigital.ro/ro/componente-electronice-diode/7457-dioda-1n4007.html)                                          | Flyback protection                    | [0.50 RON](https://www.optimusdigital.ro/ro/componente-electronice-diode/7457-dioda-1n4007.html)                                            |
| [40-pin male header, 2.54 mm (×5)](https://www.optimusdigital.ro/en/pin-headers/464-colored-40p-254-mm-pitch-male-pin-header-red.html)        | GPIO breakout                         | [5 RON](https://www.optimusdigital.ro/en/pin-headers/464-colored-40p-254-mm-pitch-male-pin-header-red.html)                                 |
| [ILI9341 LCD (2.4″, SD slot)](https://www.bitmi.ro/module-electronice/ecran-lcd-ili9341-cu-touch-si-slot-pentru-card-sd-2-4-10797-bitmi-ro.html) | Display                               | [67 RON](https://www.bitmi.ro/module-electronice/ecran-lcd-ili9341-cu-touch-si-slot-pentru-card-sd-2-4-10797-bitmi-ro.html)                |
| Push-buttons × 5                                                                                                                          | User input                            | 10 RON                                                                                                                                     |

## Software

| Library                                            | Description                                    | Usage                                      |
|----------------------------------------------------|------------------------------------------------|--------------------------------------------|
| [rp2040-hal](https://github.com/rp-rs/rp2040-hal)   | Hardware Abstraction Layer for Raspberry Pi Pico | GPIO, PWM, ADC, timers                     |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Common traits for embedded hardware access   | Digital I/O, SPI bus, delay timers         |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library for embedded displays   | Rendering grid, menus, battery indicator   |
| [ili9341](https://github.com/almindor/ili9341)      | ILI9341 TFT display driver                     | SPI communication with the LCD             |
| [panic-halt](https://crates.io/crates/panic-halt)   | Minimal panic handler                          | Halts MCU on unrecoverable errors          |
| [nb](https://crates.io/crates/nb)                   | Non-blocking I/O traits                        | Debouncing buttons, reading battery level  |

#### Software Diagram

![Software Diagram](software_diagram.svg)

### SoftWare Design
The software is structured around the main game loop, which handles input, updates the game state, and renders the display. Key components include:
- **Game State Management**: Tracks the current difficulty, grid state, cursor position, and game status (ongoing, won, lost).
- **Input Handling**: Reads button presses to move the cursor, select tiles, and navigate menus.
- **Display Rendering**: Uses the `embedded-graphics` library to draw the game grid, menu options, and status messages on the ILI9341 LCD.

### Software Functions
The software is organized into several key functions and modules, each responsible for a specific aspect of the game:

#### Display Functions

`draw_title`   Renders the "Minesweeper" banner at the top of the screen.  
`draw_menu` / `draw_menu_option`   Animated difficulty selection UI.  
`draw_grid`   Draws the minefield, cursor, numbers, and covered tiles.  
`draw_mine_icon`, `draw_flag_icon`   Vector graphics used for mines, flags, and menu art.  
`highlight_cell`, `redraw_cell`, `reveal_cell`   Fine‑grained cell updates to keep frame‑rate high on the RP2040.

#### Game Logic Functions

`has_mine`   Returns true if the current cell contains a mine for the active difficulty.  
`count_adjacent_mines`   Counts the eight neighbouring cells to compute the number overlay.  
`flood_fill_reveal`   Recursive flood‑fill algorithm that reveals connected empty cells.  
`check_win_condition`   Checks whether every safe cell is revealed (player victory).  
`reveal_all_mines`   Exposes mines (or flags) when the game ends.  
`play_victory_melody`   Plays an 8‑bit style jingle via PWM.

#### Feedback & Sound Functions

`play_victory_melody`   Plays a rising major arpeggio and sustained high C when the player wins.  

Inline PWM bursts give crisp "clicks" on menu selections and tile reveals.

### Photos and Videos

![Final_project1](final_project1.webp)
![Final_project2](final_project2.webp)
![Final_project3](final_project3.webp)
![Final_project4](final_project4.webp)
![Final_project5](final_project5.webp)