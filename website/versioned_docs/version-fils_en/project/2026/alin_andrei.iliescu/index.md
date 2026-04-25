# Breathalyzer

A device that measures the alcohol level in the air used mainly for combating drunk driving.

:::info
**Author:** Iliescu Alin-Andrei \
**Group:** 1221EA \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/fils-project-2026-13alinnn

:::

## Description

My breathalyzer device is an embedded project that meassures the amount of alcohol present in the users breath.

The output given by the breathalyzer is sent through an OLED display, a buzzer that recreates a police siren and an RGB LED that flashed blue and red, when alcohol is detected, everything physically connected through a breadboard. The program starts with an initial 2 minute "loading" time in which the MQ-3 alcohol sensor has to heat up.

## Motivation

When I was thinking of project ideas I wanted to make something related to music and I thought of a flute, in which you would blow into and based on the air preassure, a certain note would be played, then I remembered how you also blow into breathalyzers at police filters and I personally found this idea more unique, interesting and having more real world applications.

## Architecture

The project's architecture is thought of around the STM32 NUCLEO-U545RE-Q microcontroller. The workflow is:
* **Input:** The MQ-3 sensor constantly meassures the alcohol level in the air
* **Output:** The OLED Display, RGB LED and buzzers connected to the board give feedback to the user
* **Control:** The STM32 processes the input and based on its algorithm chooses what has to be done

How components connect:

```
MQ-3 Alcohol Sensor
     |
     | Analog (ADC)
     v
Sensor Acquisition Module
     |
     v
Processing Pipeline (STM32U545)
     |
     +--[ PWM ]------------> Alarm Controller (Buzzer)
     |
     +--[ PWM (x3) ]-------> Status Indicator (RGB LED)
     |
     v
Rendering Engine -> 1.3" OLED Screen
```

## Log

### Week 5
* Researched required components and made a list
* Planned how everything will connect
* Ordered most of the components

### Week 6
* Tested the STM32 board
* Ordered the last component, the screen

### Week 7
* Researched about the MQ-3 sensor
* Started learning how to 3D model for the casing of the device

## Hardware

The system is centered around the **STM32 NUCLEO-U545RE-Q** MCU. For output I am using an 1.3" OLED Display (SSD 1306 controller) using **I2C** for data transmission, a **passive buzzer** that gets a **PWM** signal, and an RGB LED that gets an oscilating **PWM** signal in the form of a 2D vector(since only red and blue are needed).

The OLED Display will have animations for the 2 minute waiting time and in the case of alcohol being detected.

## Schematics

<svg width="820" height="300" viewBox="0 0 820 300" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Breathalyzer project hardware schematic overview">
  <rect x="20" y="115" width="200" height="70" fill="none" stroke="black"/>
  <text x="120" y="155" text-anchor="middle" font-family="sans-serif" font-size="14">MQ-3 Sensor (Analog/ADC)</text>

  <rect x="290" y="100" width="220" height="100" fill="none" stroke="black"/>
  <text x="400" y="145" text-anchor="middle" font-family="sans-serif" font-size="16" font-weight="bold">STM32 NUCLEO-U545RE-Q</text>
  <text x="400" y="165" text-anchor="middle" font-family="sans-serif" font-size="12">Main Controller</text>

  <rect x="580" y="30" width="200" height="70" fill="none" stroke="black"/>
  <text x="680" y="70" text-anchor="middle" font-family="sans-serif" font-size="14">1.3" OLED Screen (I2C)</text>

  <rect x="580" y="115" width="200" height="70" fill="none" stroke="black"/>
  <text x="680" y="155" text-anchor="middle" font-family="sans-serif" font-size="14">Buzzer (PWM)</text>

  <rect x="580" y="200" width="200" height="70" fill="none" stroke="black"/>
  <text x="680" y="240" text-anchor="middle" font-family="sans-serif" font-size="14">RGB LED (3x PWM)</text>

  <line x1="220" y1="150" x2="290" y2="150" stroke="black" stroke-width="1.5"/>

  <line x1="510" y1="125" x2="580" y2="65" stroke="black" stroke-width="1.5"/>

  <line x1="510" y1="150" x2="580" y2="150" stroke="black" stroke-width="1.5"/>

  <line x1="510" y1="175" x2="580" y2="235" stroke="black" stroke-width="1.5"/>
</svg>

## Bill of Materials

| Device | Usage | Price |
| :--- | :--- | :--- |
| STM32 NUCLEO-U545RE-Q | The main microcontroller | ~160 RON |
| MQ-3 Alcohol Ethanol Sensor | The main sensor | ~15 RON |
| Passive Buzzer | For making siren noises | ~1.5 RON |
| Breadboard | For connecting the components | Previously owned |
| Display | For displaying alcohol levels | ~35 RON |
| RGB LED | For the police aesthetic | Previously owned |
| Jumper wires | For connecting | Previously owned |
| Resistors | For a voltage divider going from the sensor to the MCU | Previously owned |

## Software

| Library | Description | Usage |
| :--- | :--- | :--- |
| [`embassy-stm32`](https://crates.io/crates/embassy-stm32) | Hardware Abstraction Layer | Core hardware control |
| [`embassy-executor`](https://crates.io/crates/embassy-executor) | Async Executor | Running non-blocking tasks |
| [`ssd1306`](https://crates.io/crates/ssd1306) | Display driver | Controlling the OLED screen |
| [`embedded-graphics`](https://crates.io/crates/embedded-graphics) | 2D graphics library | Drawing text and shapes |

## Links

1. [Embassy-rs Documentation](https://embassy.dev/book/index.html) - The official book for the async framework used.
2. [STM32U545 Datasheet](https://www.st.com/en/microcontrollers-microprocessors/stm32u545re.html) - Technical details for our specific MCU.
3. [SSD1306 Rust Driver](https://docs.rs/ssd1306/latest/ssd1306/) - Documentation for the OLED screen library.
4. [Embedded Rust Book](https://docs.rust-embedded.org/book/) - General guide for Rust on microcontrollers.
