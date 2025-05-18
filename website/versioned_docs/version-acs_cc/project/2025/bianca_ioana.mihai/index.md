# Hand Motion System
A robotic hand that mimics user hand gestures via glove-mounted flex sensors.

:::info 

**Author**: Bianca-Ioana Mihai (332CC) \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-biancamih

:::

## Description

The project involves building a 3D-printed robotic hand controlled by flex sensors mounted on a glove. The robotic hand will replicate the movements of the user's hand in real-time, based on input from the sensors. These sensors send signals to a Raspberry Pi Pico 2 microcontroller, which interprets the data and drives individual servomotors for each finger of the robotic hand.

## Motivation

Hand gesture control offers an intuitive and accessible interface for robotic systems. This project aims to explore gesture-based interaction as a method for remote manipulation or accessibility tools for people with disabilities.

## Architecture 

The main architecture components and their interactions:

- **Input Layer**: Flex sensors on each finger measure bending angles.
- **Processing Layer**: Raspberry Pi Pico interprets sensor data using ADC and maps it to servo positions.
- **Actuation Layer**: Servos in the robotic hand mimic the finger positions.

## Log

### Week 5 - 11 May

- Initial hardware procurement
- Defined the project's scope and main functionalities.
- Selected the core hardware components.

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

- **Raspberry Pi Pico 2** – The core microcontroller for signal processing.
- **Flex Sensors** – Detect finger bending on the glove.
- **Servomotors** – Control finger movement on the robotic hand.
- **3D-Printed Robotic Hand** – Physical output mechanism.
- **Resistors, breadboard, jumper wires, USB cable** – Circuit prototyping.

### Schematics

Place your KiCAD schematics here.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [2 × Raspberry Pi Pico 2 W](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html) | Microcontrollers with Wi-Fi and Bluetooth for sensor processing and motor control | [40 RON each](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) |
| [5 × SG90 Micro Servo Motor](https://www.optimusdigital.ro/ro/motoare-servomotoare/2261-micro-servo-motor-sg90-180.html) | Actuators for robotic hand fingers | [12 RON each](https://www.optimusdigital.ro/ro/motoare-servomotoare/2261-micro-servo-motor-sg90-180.html) |
| 3D Printed Hand | Physical structure of the robotic hand | TBD |
| [5 x Force Sensitive Resistor](https://cdn.sparkfun.com/assets/c/4/6/8/b/2010-10-26-DataSheet-FSR406-Layout2.pdf) | Detects pressure or force from the user's finger | [~70 RON each](https://www.optimusdigital.ro/ro/senzori/2261-senzor-de-forta-fsr406.html) |
| [Jumper Wires](https://www.electronicwings.com/components/male-to-male-jumper-wire/1/datasheet) | This is used for wiring connections between modules and breadboard circuits | [23 RON](https://www.optimusdigital.ro/en/wires-with-connectors/12475-male-to-male-jumper-wires-40-pin-40cm.html) |
| [Breadboard](https://www.optimusdigital.ro/en/breadboards/13244-breadboard-175-x-67-x-9-mm.html) | Rapid prototyping without soldering | [15 RON](https://www.optimusdigital.ro/en/breadboards/13244-breadboard-175-x-67-x-9-mm.html) |
| [Resistors](https://www.plusivo.com/electronics-kit/117-plusivo-resistor-kit-250-pcs.html) | Used in voltage dividers and signal conditioning | [12 RON](https://www.optimusdigital.ro/en/resistors/10928-250-pcs-plusivo-resistor-kit.html) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy) | Async runtime for Raspberry Pi Pico | Used to control GPIO, ADC, and timers |
| [embassy-servo](https://github.com/embassy-rs/embassy) | Servo motor control | Sends PWM signals to SG90 servos |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Common hardware interface | Standard traits for pins, ADC, PWM |
| [defmt](https://github.com/knurling-rs/defmt) | Debug logging tool | Shows debug messages through RTT |


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Robotic Hand Controlled by Glove - Flex Sensor + Arduino](https://www.youtube.com/watch?v=Fvg-v8FPcjg) 
2. [DIY Robotic Arm using Flex Sensors and Arduino](https://www.youtube.com/watch?v=7J9GLTyKoxc) 
3. [Wireless Controlled Robotic Hand using Flex Sensors](https://www.youtube.com/watch?v=lWnlJzvybIs) 
4. [Interfacing Flex Sensor with Arduino](https://circuitdigest.com/microcontroller-projects/interfacing-flex-sensor-with-arduino) 