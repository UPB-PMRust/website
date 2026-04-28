# Interactive Robotic Flower with Audio-Reactive Behavior
A robotic flower that reacts to sound and nearby objects using sensors and servo motors.

:::info 

**Author**: Ilie Irina-Elena \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-ilieirina9990-creator

:::

## Description

This project consists of an interactive robotic flower built using an STM32 microcontroller and programmed in Rust. The system combines mechanical movement, sensor input, and user interaction in a compact and visually engaging design. The flower reacts to nearby users and ambient sound, creating a dynamic and responsive behavior.

## Motivation

The main motivation behind this project is to explore how embedded systems can be used to create interactive and expressive physical objects. The idea was inspired by combining simple robotics with creative design, resulting in a system that behaves in a natural and engaging way. Additionally, the project aims to develop practical skills in microcontroller programming, sensor integration, and real-time system behavior.

## Architecture 

The system is based on a modular architecture centered around the STM32 microcontroller. The system behavior is implemented using a finite state machine that manages interaction and audio-reactive responses.

Main components:
- Input module (ultrasonic sensor, sound sensor)
- Processing module (STM32 microcontroller)
- Output module (servo motors, OLED display)

Connections:
- Sensors send data to the microcontroller
- The microcontroller processes the input signals
- Based on the processed data, it controls the servo motors and the display

![Architecture Diagram](./ilie_irina.svg)

## Log

### Week 5 - 11 May
Project idea selection, initial research, and component identification.

### Week 12 - 18 May
System design and implementation of sensor reading and actuator control.

### Week 19 - 25 May
Integration of all components and testing of the full system behavior.

## Hardware

The system uses the following hardware components:
- STM32 microcontroller  
- Ultrasonic sensor  
- Sound sensor  
- 2x SG90 Servo motors  
- OLED display  
- Breadboard and jumper wires  

### Schematics

(Add your KiCAD schematics here if available)

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| STM32 Nucleo board | Main controller | ~120 RON |
| Ultrasonic Sensor (HC-SR04) | Distance detection | ~10 RON |
| Sound Sensor (KY-038) | Audio input | ~5-10 RON |
| SG90 Servo Motors (x2) | Stem movement (2 axes) | ~20 RON |
| ⁠SSD1306 OLED Display | Visual output | ~25 RON |
| Breadboard | Prototyping | ~20 RON |
| Jumper wires | Connections | ~15 RON |
| Power Supply (5V) | Powers servo motors and system | ~20 RON |
| Mechanical materials (structure, support, glue) | Physical structure of the flower | ~50–60 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| STM32 HAL | Hardware abstraction layer | Used for peripheral control and GPIO/PWM |
| embedded-hal | Hardware abstraction in Rust | Used for interfacing with components |
| embedded-graphics | Graphics library | Used for OLED display rendering |

## Links

1. https://github.com/UPB-PMRust-Students/fils-project-2026-ilieirina9990-creator
2. https://www.st.com/en/microcontrollers-microprocessors/stm32.html
3. https://lastminuteengineers.com/ultrasonic-sensor-arduino-tutorial/
4. https://lastminuteengineers.com/servo-motor-arduino-tutorial/
5. https://github.com/embedded-graphics/embedded-graphics
