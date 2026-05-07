# Reaction Timer Game

:::info
**Author:** Matei Zamfir \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/fils-project-2026-matinator1510-lgtm
:::

## Description

The project is an advanced reaction timer game built on a Raspberry Pi Pico (RP2040) microcontroller. The device uses visual stimuli (an LED) and mechanical user input (push buttons) to evaluate how quickly a user can respond. The system runs a structured "Best of 3" game format where it measures reaction times across randomized delay periods, calculates the average performance score in real time, and actively monitors for premature responses (false starts). All menus, session data, and statistical feedback are provided visually through an OLED display.

## Motivation

I chose this project in order to explore embedded systems through a highly interactive application focused on precise timing and state management. It combines real-time hardware input processing with microsecond-level hardware timers to accurately measure human reflexes. This makes it both a rigorous technical exercise in building a responsive, non-blocking state machine in Rust, and a practical, engaging tool for measuring reaction speed under varying conditions.

## Architecture

The core of the system is the Raspberry Pi Pico. It acts as the central control unit, managing the application state machine (Menu > Game > Results).
* **Visual Output:** An OLED display connected via I2C/SPI provides the user interface, while a discrete LED serves as the primary reaction stimulus.
* **User Input:** Two tactile push buttons are used to handle menu navigation and capture the exact moment of the user's physical response.
* **Timing & Logic:** The internal random number generator creates unpredictable delays, and the hardware timers calculate the delta between the LED illuminating and the button press.

## Log

**Week 30 March - 5 April**
Explored multiple project ideas and decided on the concept for the reaction timer game.

**Week 13 - 19 April**
Researched the necessary components, defined the project scope, and finalized the hardware bill of materials.

**Week 20 - 26 April**
Drafted the initial project documentation, set up the repository, and planned the software architecture.

## Hardware

The system uses a Raspberry Pi Pico (RP2040) development board as the main microcontroller. Visual feedback is provided by an SSD1306 OLED display and a standard 5mm LED. User interaction is handled through two momentary push buttons. The system is powered via the Pico's USB connection and assembled on a standard prototyping breadboard using jumper wires.

## Schematics

Work in progress..  
Place your KiCAD or similar schematics here in SVG format.

## Bill of Materials

### Hardware

| Device | Usage | Price |
| :--- | :--- | :--- |
| Raspberry Pi Pico | The microcontroller | ~30.00 RON |
| OLED Display | Display - menu and statistical feedback | ~25.00 RON |
| Push buttons (x2) | User input - menu control and reaction response | ~4.00 RON |
| Standard LED | Visual stimulus | ~1.00 RON |
| Resistor Kit | Circuit protection | ~15.00 RON |
| Breadboard & Wires | Prototyping and connections | ~15.00 RON |

### Software

| Library | Description | Usage |
| :--- | :--- | :--- |
| `rp2040-hal` | Hardware abstraction layer for RP2040 | Used for configuring peripherals (GPIO, Timers, I2C/SPI) |
| `cortex-m-rt` | Runtime support | Initializes the microcontroller and defines the entry point |
| `embedded-hal` | Hardware abstraction traits | Provides standard interfaces for interacting with the LED and buttons |
| `ssd1306` | Display driver | Used to send initialization commands and data to the OLED screen |
| `embedded-graphics` | 2D graphics library | Used for drawing the UI text and shapes to the display |
| `rand` | Random number generator | Used to create the unpredictable delay for the visual stimulus |
