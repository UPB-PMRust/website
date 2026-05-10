# Laser Harp - Rhythm Game
Interactive musical instrument and rhythm game implemented in Rust.

:::info 

**Author**: Marin Ioana-Alexia \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-Ioana-AlexiaMarin

:::

<!-- do not delete the \ after your name -->

## Description

The project is an interactive laser harp system implemented using Rust on an STM32 microcontroller. The system uses vertical laser beams, each representing a musical note. When a beam is interrupted, light sensors (phototransistors) detect the change and trigger the corresponding note via a buzzer. 
Additionally, the project includes a "Piano Tiles" inspired game mode where users must play notes in sync with a predefined sequence, receiving real-time feedback via LEDs and an LCD/OLED display.

## Motivation

The inspiration for this project dates back to my childhood when I first saw a laser harp on TV, it felt like something out of a science fiction movie. Now, having the technical background to understand the electronics behind it, I wanted to challenge myself by building my own version.
I decided to add the rhythm game component to make the experience more engaging and accessible. Growing up playing *Piano Tiles*, I realized that gamification is a great way for someone without a musical background to interact with an instrument in a fun, intuitive way. On the technical side, this project is my gateway into **Real-Time Signal Processing**. It allows me to explore how Rust and the **Embassy** framework handle low-latency tasks like simultaneous sensor polling, PWM audio generation, and UI rendering without skipping a beat.

## Architecture 

The system is designed as a high-speed, "real-time reactive" process that prioritizes low latency to ensure the harp feels like a real instrument. It all starts with the **STM32**’s high-speed GPIOs constantly monitoring the **phototransistors**. These sensors act as the "eyes" of the system, waiting for the exact moment a laser beam is interrupted. 
The core logic is managed by the **Embassy** async executor, which acts as a sophisticated conductor, multitasking between several time-sensitive events. When a hand "plucks" a light string, the system immediately jumps into action: it triggers a precise **PWM** signal to the **passive buzzer** to generate the corresponding musical note, while simultaneously checking the timing against the "Piano Tiles" game logic. This happens in microseconds, ensuring that the sound and the visual feedback are perfectly in sync without any perceptible lag.

## Log

<!-- write your progress here every week -->

### Week 23 - 29 March
- Defined the project concept and initial hardware requirements.
- Received the **STM32 NUCLEO** board on loan from the university.

### Week 30 - 5 April
- Received feedback from the lab assistant.
- Decided to add an OLED display and switch to a soldered protoboard for better reliability.

### Week 6 - 19 April
- Easter break and component research (selecting phototransistors for the best response time).
- Ordered the laser diodes and the OLED display.

### Week 20 - 26 April
- Set up the Rust development environment and the project page.
- Started testing basic GPIO interrupts with the STM32 board.

## Hardware

The hardware components are selected to ensure a low-latency and interactive musical experience:

- **STM32 NUCLEO-U545RE-Q:** The main processing unit that handles the real-time logic and manages audio-visual feedback through its high-performance peripherals.
- **Laser Diodes (5V, 650nm):** A set of 8 emitters that act as the physical "strings" of the harp, providing a steady and visible light source for the sensors.
- **L-932P3C Phototransistors:** High-speed sensors used to detect beam interruptions; they were chosen specifically for their microsecond response time, which is critical for a rhythm game.
- **Passive Buzzer:** Used for audio output; unlike active buzzers, this allows for frequency control via PWM to play different musical notes.
- **SSD1306 OLED Display:** A 128x64 screen that displays the game score, menus, and real-time status updates via the I2C interface.
- **PN2222 Transistor:** Used as a switch to drive the buzzer, ensuring the STM32's pins are protected from high current draw.
- **Soldered Protoboard:** Provides a stable and durable base for all components, replacing the breadboard for a more reliable final assembly.
- **Plusivo Soldering Kit:** A complete set of tools, including a temperature-controlled iron and a digital multimeter, used for circuit assembly and validation.s

### Schematics

![Diagrama](./laser_harp.svg)


### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
| :--- | :--- | :--- |
| STM32 NUCLEO-U545RE-Q | Main controller (Processing & Logic) | Borrowed |
| Laser Diodes 5V (8 pcs) | Creating the virtual harp strings | ~20 RON |
| Phototransistors L-932P3C (8 pcs) | High-speed beam interruption detection | ~16 RON |
| SSD1306 OLED Display (I2C) | UI, Score display, and Game feedback | ~15 RON |
| Passive Buzzer (3.3V) | Audio output for musical notes | ~5 RON |
| Plusivo Soldering Kit | Letcon, Multimeter, and tools for assembly | ~100 RON |
| Prototype PCB / Protoboard | Final hardware assembly (Soldered) | ~10 RON |
| Resistors & LEDs Set | Current limiting and visual feedback | ~40 RON |
| PN2222 / BC547 Tranzistor | Switching for the buzzer | ~2 RON |
| Jumper Wires / Solder Wire | Connections and assembly materials | ~10 RON |

## Software

| Library | Usage |
| :--- | :--- |
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | Hardware abstraction for GPIO, PWM, and I2C. |
| [embassy-time](https://crates.io/crates/embassy-time) | Handling precise game timing and delays. |
| [embedded-graphics](https://crates.io/crates/embedded-graphics) | Drawing the game interface and score on the OLED. |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. https://www.st.com/en/microcontrollers-microprocessors/stm32u5-series.html
2. https://github.com/embassy-rs/embassy
3. https://doc.rust-lang.org/book/
4. https://embassy.dev/book/index.html
5. https://docs.rs/ssd1306/latest/ssd1306/
6. https://docs.rs/embedded-graphics/latest/embedded_graphics/
