# Automated Drink Mixer

An automated drink mixer that precisely measures and dispenses custom juice recipes.

:::info
**Author:** Mario Emancipatu \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/project-2026-marioemancipatu
:::

## Description

The Automated Drink Mixer is an embedded project designed to automate the process of mixing beverages. The system uses an STM32 microcontroller to coordinate three mini submersible DC pumps. 

The user interface consists of an OLED display and a rotary encoder, allowing the user to select from a variety of predefined recipes. Once a drink is selected, the STM32 activates the corresponding pumps via a 4-channel relay module for a precise duration, ensuring the correct proportions for every glass. The entire system is built with a focus on asynchronous execution using the Rust Embassy framework to ensure a responsive UI while the hardware operations are in progress.

## Motivation

I chose this project because I wanted to build something that is not only fun to use but also serves a real-world purpose. It is a great way to explore how embedded systems can control physical hardware (pumps) based on digital user input. Also, I wanted to challenge myself by learning the Embassy framework in Rust, as it's a modern and safe way to handle multiple tasks at once, like managing a display while running timers for the pumps.

## Architecture


The project's architecture is built around the **STM32 NUCLEO-U545RE-Q** as the central brain. The workflow is as follows:
* **Input:** The **Rotary Encoder** sends signals to the STM32 to navigate the beverage menu.
* **Display:** The **OLED Display** provides visual feedback, showing the selected recipe and progress via the I2C protocol.
* **Control:** The STM32 processes the selection and triggers the **4-Channel Relay Module**.
* **Power & Output:** The relays act as electronic switches, closing the circuit between the **external battery pack** and the **Mini Submersible Pumps**.
* **Flow:** Each pump draws liquid from a container through **Silicone Tubing** and dispenses it into a common cup.

The workflow is as follows:

```text
Rotary Encoder
     |
     | Digital (GPIO)
     v
Input Acquisition Module
     |
     v
Processing Pipeline (STM32U545)
     |
     +--[ GPIO ]-----------> 4-Channel Relay Module
     |                              |
     |                              +--[ Power ]<-- External Battery Pack
     |                              |
     |                              v
     |                       Mini Submersible Pumps
     v
Rendering Engine -> [ I2C ] -> 0.96" OLED Screen
```


## Log

### Week 5 - 6
* Researched the required hardware components and placed orders.
* Analyzed the electrical requirements for the pumps and relay isolation.

### Week 7
* Tested each water pump individually using an external power source to ensure they are functional.
* Verified the relay switching logic to ensure safe operation with the microcontroller.

### Week 8 - 9
* Soldered the components.
* Built the initial physical circuit, wiring the STM32, relay module, and OLED screen.
* Tested all the components on their own.

## Hardware

The system is centered around an **STM32 NUCLEO-U545RE-Q** microcontroller. For user interaction, I am using a **0.96" OLED display** (SSD1306 controller) communicating via **I2C**, and a **KY-040 rotary encoder** connected through digital GPIOs with hardware interrupts for precise tracking. 

The fluid control system consists of **three 5V DC mini submersible pumps**, each driven by a channel of a **4-channel relay module**. The relays provide galvanic isolation between the MCU's logic and the inductive load of the pumps. Power is split into two domains: the STM32 is powered via USB, while the pumps are driven by an external **6V battery pack (4x AA)**. All connections are made using **Dupont wires** on a standard **830-point breadboard**, with additional current-limiting resistors where necessary.

## Schematics

<svg width="820" height="300" viewBox="0 0 820 300" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Smart Dispenser project hardware schematic overview">
  <rect x="20" y="115" width="200" height="70" fill="none" stroke="black"/>
  <text x="120" y="145" textAnchor="middle" fontFamily="sans-serif" fontSize="14" fontWeight="bold">Rotary Encoder</text>
  <text x="120" y="165" textAnchor="middle" fontFamily="sans-serif" fontSize="12">(CLK, DT, SW - GPIO)</text>

  <rect x="290" y="100" width="220" height="100" fill="none" stroke="black"/>
  <text x="400" y="145" textAnchor="middle" fontFamily="sans-serif" fontSize="16" fontWeight="bold">STM32 NUCLEO-U545RE-Q</text>
  <text x="400" y="165" textAnchor="middle" fontFamily="sans-serif" fontSize="12">Main Controller</text>

  <rect x="580" y="30" width="200" height="70" fill="none" stroke="black"/>
  <text x="680" y="60" textAnchor="middle" fontFamily="sans-serif" fontSize="14" fontWeight="bold">0.96" OLED Screen</text>
  <text x="680" y="80" textAnchor="middle" fontFamily="sans-serif" fontSize="12">(I2C - SDA, SCL)</text>

  <rect x="580" y="115" width="200" height="70" fill="none" stroke="black"/>
  <text x="680" y="145" textAnchor="middle" fontFamily="sans-serif" fontSize="14" fontWeight="bold">Relay Module</text>
  <text x="680" y="165" textAnchor="middle" fontFamily="sans-serif" fontSize="12">(Digital / GPIO)</text>

  <rect x="580" y="200" width="200" height="70" fill="none" stroke="black"/>
  <text x="680" y="230" textAnchor="middle" fontFamily="sans-serif" fontSize="14" fontWeight="bold">Water Pumps</text>
  <text x="680" y="250" textAnchor="middle" fontFamily="sans-serif" fontSize="12">(Ext. Battery Powered)</text>

  <line x1="220" y1="150" x2="290" y2="150" stroke="black" strokeWidth="1.5"/>
  
  <line x1="510" y1="150" x2="580" y2="65" stroke="black" strokeWidth="1.5"/>
  
  <line x1="510" y1="150" x2="580" y2="150" stroke="black" strokeWidth="1.5"/>
  
  <line x1="680" y1="185" x2="680" y2="200" stroke="black" strokeWidth="1.5"/>
</svg>

## Bill of Materials

| Device | Usage | Price |
| :--- | :--- | :--- |
| STM32 NUCLEO-U545RE-Q | The main microcontroller | ~160 RON |
| 3x Mini Submersible Pumps 5V | Pumping liquid from jars | ~30 RON |
| 1x 4-Channel Relay 5V | Isolating and switching the pumps | ~15 RON |
| OLED Display 0.96" I2C | User interface screen | ~17 RON |
| KY-040 Rotary Encoder | Menu navigation knob | ~6 RON |
| 4x AA Battery Holder | Powering the pumps | ~7 RON |
| Silicone Tubing 3x5mm | Transporting the liquid | ~5 RON |
| 40x Dupont wires (male-male) | Circuit connections | ~8 RON |

## Software

| Library | Description | Usage |
| :--- | :--- | :--- |
| [`embassy-stm32`](https://crates.io/crates/embassy-stm32) | Hardware Abstraction Layer | Core hardware control |
| [`embassy-executor`](https://crates.io/crates/embassy-executor) | Async Executor | Running non-blocking tasks |
| [`ssd1306`](https://crates.io/crates/ssd1306) | Display driver | Controlling the OLED screen |
| [`embedded-graphics`](https://crates.io/crates/embedded-graphics) | 2D graphics library | Drawing text and shapes |
| [`rotary-encoder-hal`](https://crates.io/crates/rotary-encoder-hal) | Hardware driver | Decoding knob turns |


## Links

1. [Embassy-rs Documentation](https://embassy.dev/book/index.html) - The official book for the async framework used.
2. [STM32U545 Datasheet](https://www.st.com/en/microcontrollers-microprocessors/stm32u545re.html) - Technical details for our specific MCU.
3. [SSD1306 Rust Driver](https://docs.rs/ssd1306/latest/ssd1306/) - Documentation for the OLED screen library.
4. [Embedded Rust Book](https://docs.rust-embedded.org/book/) - General guide for Rust on microcontrollers.
