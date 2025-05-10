

# Snake Game on Raspberry Pi Pico 2W

A classic Snake game implemented on the Raspberry Pi Pico 2W using Rust and displayed on an LED matrix.

:::info

**Author**: Andrei Anca-Teodora \
**GitHub Project Link**: [link_to_github](https://github.com/UPB-PMRust-Students/proiect-Ancaa8)

:::

## Description

The project consists of implementing the classic Snake game on an embedded Raspberry Pi Pico 2W platform. The visual output is handled by a 16x16 LED matrix, which is assembled from four 8x8 modules arranged in a square grid. These modules are driven using a series of 74HC595 shift registers that allow efficient control of each individual LED through serial communication. User input is managed through four push buttons, each corresponding to a movement direction (up, down, left, right), and a buzzer is included to provide simple sound effects during gameplay. The game logic, input handling, and LED display updates are all implemented in the Rust programming language.

## Motivation

I chose this project because it combines embedded programming in Rust with hands-on experience in hardware design and control. It offers a practical and engaging way to explore how low-level code interacts with physical components like LEDs, buttons, and buzzers. Developing a visual and interactive game such as Snake makes the learning process more intuitive and rewarding. Additionally, using the Raspberry Pi Pico 2W provides a modern and affordable platform for experimenting with GPIOs, timing mechanisms, and shift registers, all within a real-time embedded environment.


## Architecture

![System Architecture](Diagrama.webp)

### Raspberry Pi Pico 2W  
**Role**: Main microcontroller responsible for game logic, managing input from buttons, sending data to the LED matrix through shift registers, and generating sound feedback via the buzzer.  
**Connections**:  
- Shift Registers (data, latch, clock pins via GPIO)  
- Buttons (4 directional input pins via GPIO)  
- Buzzer (output pin via GPIO)  
- Shared GND and power lines  

---

### LED Matrix 16x16 (composed of 4x 8x8 modules)  
**Role**: Visual output for displaying the Snake game.  
**Connections**:  
- Driven by 8 x 74HC595 shift registers connected in daisy-chain  
- Common power and ground  
- Controlled via serial data sent by the microcontroller  

---

### Shift Registers (74HC595, x8)  
**Role**: Serial-to-parallel conversion; extends GPIOs to control multiple LEDs across the matrix.  
**Connections**:  
- Connected in series (Q7' to DS of next register)  
- Data, Latch, and Clock lines connected to Pico GPIO pins  
- Outputs connected to LED matrix rows/columns  

---

### Buttons (x4)  
**Role**: User input for controlling the direction of the Snake (Up, Down, Left, Right).  
**Connections**:  
- One pin each to a dedicated GPIO on the Pico  
- Connected with 5.1kΩ pull-down resistors to ensure stable logic levels  

---

### Buzzer  
**Role**: Provides sound feedback for events such as collisions or scoring.  
**Connections**:  
- Connected to a GPIO pin
- Shared ground and power  


## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May


## Hardware


The project uses the Raspberry Pi Pico 2W as the main microcontroller responsible for handling all game logic, input processing, and LED matrix control. Four 8x8 LED matrix modules are combined to form a 16x16 display, providing a visual interface for the Snake game. These matrices are driven using 74HC595 shift registers, which allow serial-to-parallel conversion and enable control of a large number of LEDs using limited GPIO pins. Four buttons are connected to GPIO pins to allow directional input from the user, enabling gameplay interaction. Additionally, a buzzer is used to provide basic audio feedback, signaling events such as game start, collisions, or scoring.


## Schematics

![KiCAD Schematic](SchemaKicad.webp)



## Bill of Materials

| Device                  | Usage                             | Price     |
|-------------------------|------------------------------------|-----------|
| Raspberry Pi Pico 2W    | The microcontroller                | 35 RON    |
| LED Matrix 8x8 (x4)     | Game display                       | 60 RON    |
| Shift Register 74HC595  | Parallel control for LEDs          | 24 RON    |
| Buttons (x4)            | Direction control                  | 4 RON     |
| Buzzer                  | Sound feedback                     | 5 RON     |
| Resistors (220Ω, 5.1kΩ) | Current limiting, pull-down        | 10 RON    |
| Breadboard (x2)         | Rapid prototyping                  | 40 RON    |
| Connection wires        | GPIO wiring                        | 10 RON    |

## Software


| Library                                                                 | Description                             | Usage                                                 |
|-------------------------------------------------------------------------|-----------------------------------------|-------------------------------------------------------|
| [rp2040-hal](https://github.com/rp-rs/rp-hal)                          | Raspberry Pi Pico HAL                   | Provides low-level access to GPIOs and peripherals     |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal)          | Abstraction traits for embedded devices | Used to write hardware-agnostic code for LED and input|
| [bitbang-hal](https://github.com/eldruin/bitbang-hal)                  | Bit-banging I/O interface               | Used to drive shift registers connected to LED matrix |
| [heapless](https://crates.io/crates/heapless)                          | No-heap data structures                 | Used for storing game state in a `no_std` environment |
| [panic-halt](https://crates.io/crates/panic-halt)                      | Minimal panic handler                   | Used to safely halt on unrecoverable errors           |


## Links

1. [link ...](https://ocw.cs.pub.ro/courses/pm/prj2023/drtranca/snake)
