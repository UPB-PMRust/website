# Electronic Chess Board with Autonomous Play and Legal Move Display

An electronic chess board that autonomously moves pieces using an XY electromagnetic system, detects piece positions with Hall sensors, displays legal moves with LEDs, and shows a timer on LCD.



:::info

**Author:** Onea Amalia-Mihaela  \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/acs-project-2026-AmaliaOnea

:::

## Description

This project is an autonomous chess board powered by an STM32 Nucleo-U545RE-Q microcontroller. The board uses a system of two stepper motors to move an electromagnet along the X and Y axes beneath the board, allowing it to physically move chess pieces (which contain small magnets) without human intervention. Hall effect sensors placed under each of the 64 squares detect the presence and position of pieces. An 8x8 LED matrix highlights the possible moves for each piece, and an LCD display shows the chess timer for each player.

The board implements a chess engine in Rust that computes valid moves and executes them automatically, moving pieces physically on the board.

## Motivation

I chose this project because it combines multiple engineering disciplines: mechanical design, electronics, and software. A self-playing chess board is a fascinating challenge that requires precise motor control, real-time sensor reading, and complex game logic — all implemented in Rust on an embedded system. It is a project that is both technically challenging and visually impressive, making it a perfect showcase for the knowledge gained throughout the semester.

## Architecture

### System Architecture

The system is composed of the following main modules:

- **STM32 Nucleo-U545RE-Q**: The main microcontroller that runs the chess engine, controls the motors, reads the Hall sensors, and drives the LEDs and LCD.
- **XY Motion System**: Two stepper motors (one for each axis) move a carriage beneath the board. An electromagnet mounted on the carriage attracts and moves the magnetic chess pieces.
- **Hall Sensor Matrix**: 64 Hall effect sensors (one per square) detect the presence of a magnetic chess piece on each square.
- **LED Matrix**: An 8x8 LED matrix (one LED per square) displays valid moves for the selected piece.
- **LCD Display**: A 16x2 I2C LCD shows the chess timer for both players.
- **Power Supply**: A 12V external power supply powers the stepper motors and electromagnet. A voltage regulator steps down to 5V for the logic components.

### Block Diagram

```
+---------------------------+
|  STM32 Nucleo-U545RE-Q    |
|                           |
|  +--------+  +---------+  |
|  | Chess  |  | Motor   |  |
|  | Engine |  | Control |  |
|  +--------+  +---------+  |
|       |            |      |
+-------|------------|------+
        |            |
        v            v
+---------------+  +------------------+
| Hall Sensor   |  | Stepper Motor X  |
| Matrix (8x8)  |  | Stepper Motor Y  |
+---------------+  | Electromagnet    |
        |          +------------------+
        v
+---------------+  +---------------+
| LED Matrix    |  | LCD I2C 1602  |
| (8x8)         |  | (Timer)       |
+---------------+  +---------------+
```


## Hardware

### 1. STM32 Nucleo-U545RE-Q
- **Role**: Main microcontroller
- **Specs**: ARM Cortex-M33, 160MHz, 256KB SRAM, 2MB Flash
- **Used for**: Running the chess engine, controlling all peripherals via GPIO, I2C, and PWM

### 2. Stepper Motors (x2) — NEMA 17
- **Role**: Move the electromagnet carriage along X and Y axes
- **Specs**: 1.8° step angle, 12V
- **Connection**: Controlled via A4988 driver modules

### 3. A4988 Stepper Motor Drivers (x2)
- **Role**: Drive the stepper motors from STM32 GPIO signals
- **Interface**: STEP and DIR pins connected to STM32 GPIO

### 4. Electromagnet 12V (x1)
- **Role**: Attract and move magnetic chess pieces beneath the board
- **Control**: Switched on/off via MOSFET IRF540N

### 5. MOSFET IRF540N (x1)
- **Role**: Control the electromagnet (the STM32 cannot supply enough current directly)
- **Connection**: Gate connected to STM32 GPIO, drain to electromagnet, source to GND

### 6. Diode 1N4007 (x1)
- **Role**: Flyback protection for the electromagnet
- **Connection**: In parallel with the electromagnet, reverse biased

### 7. Hall Effect Sensors SS49E (x64)
- **Role**: Detect the presence of a magnetic chess piece on each square
- **Interface**: Analog output read via ADC on STM32
- **Arrangement**: One sensor per square, arranged in an 8x8 matrix

### 8. LED Matrix 8x8 (x1)
- **Role**: Display valid moves for the currently selected piece
- **Interface**: Controlled via GPIO (row/column multiplexing)

### 9. Resistors 220Ω (x64)
- **Role**: Current-limiting resistors for LEDs

### 10. LCD I2C 1602 (x1)
- **Role**: Display the chess timer for both players
- **Interface**: I2C (SDA, SCL)

### 11. Power Supply 12V (x1)
- **Role**: Power the stepper motors and electromagnet

### 12. Voltage Regulator LM7805 (x1)
- **Role**: Step down 12V to 5V for logic components

### 13. Capacitors 100µF (x2)
- **Role**: Stabilize voltage on the power supply lines

### 14. Breadboard + Jumper Wires
- **Role**: Prototyping and connections

## Schematics

![Schematic](schematicul.webp)


## Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| STM32 Nucleo-U545RE-Q | Main microcontroller | provided by lab |
| NEMA 17 Stepper Motor x2 | Move electromagnet on X and Y axes | ~60 RON |
| A4988 Stepper Driver x2 | Drive stepper motors | ~20 RON |
| Electromagnet 12V | Move chess pieces | ~15 RON |
| MOSFET IRF540N | Control electromagnet | ~5 RON |
| Diode 1N4007 | Flyback protection | ~1 RON |
| Hall Sensor SS49E x64 | Detect piece positions | ~80 RON |
| LED Matrix 8x8 | Display valid moves | ~10 RON |
| Resistors 220Ω x64 | Current limiting for LEDs | ~5 RON |
| LCD I2C 1602 | Chess timer display | ~15 RON |
| Power Supply 12V | Power motors and electromagnet | ~30 RON |
| Voltage Regulator LM7805 | Step down 12V to 5V | ~3 RON |
| Capacitors 100µF x2 | Voltage stabilization | ~2 RON |
| Breadboard + Jumper Wires | Prototyping | ~20 RON |
| **Total** | | **~266 RON** |

## Links

- [STM32 Nucleo-U545RE-Q Documentation](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)
- [embassy-rs](https://embassy.dev/)
- [PM-Rust Labs](https://embedded-rust-101.wyliodrin.com/)
- [A4988 Stepper Driver Datasheet](https://www.pololu.com/product/1182)
