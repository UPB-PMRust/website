# Interactive Robotic Flower

A robotic flower that reacts to user presence and sound through dynamic movement.

**Author:** Ilie Irina-Elena  
**Project GitHub Link:** https://github.com/UPB-PMRust-Students/fils-project-2026-ilieirina9990-creator

---

## Description

This project consists of an interactive robotic flower built using an STM32 microcontroller and programmed in Rust. The system combines mechanical movement, sensor input, and user interaction in a compact and visually engaging design. The flower reacts to nearby users and ambient sound, creating a dynamic and responsive behavior.

---

## Motivation

The main motivation behind this project is to explore how embedded systems can be used to create interactive and expressive physical objects. The idea was inspired by combining simple robotics with creative design, resulting in a system that behaves in a natural and engaging way. Additionally, the project aims to develop practical skills in microcontroller programming, sensor integration, and real-time system behavior.

---

## Architecture

The system is structured as a set of interacting modules:

- **Input Module** – handles data from the ultrasonic sensor (distance) and sound sensor  
- **Processing Module** – implemented on the STM32, processes sensor data and controls behavior  
- **Control Logic** – a finite state machine that determines system states (idle, interaction, audio-reactive, reset)  
- **Output Module** – controls the servo motors and OLED display  

The modules communicate through the microcontroller, which acts as the central unit coordinating all operations.

---

## Journal

### Week 5 – 11 May
- Project idea selection  
- Initial research and component selection  

### Week 12 – 18 May
- System design and architecture definition  
- Hardware planning and documentation  

### Week 19 – 25 May
- Implementation planning  
- Preparing development environment  

---

## Hardware

The system uses the following hardware components:

- STM32 Nucleo-U545RE-Q (main controller)  
- 2x SG90 Micro Servo Motors (movement of the stem)  
- HC-SR04 Ultrasonic Sensor (user detection)  
- KY-038 Sound Sensor (audio input)  
- SSD1306 OLED Display (visual feedback)  
- Breadboard and jumper wires  
- External power supply  
- Mechanical structure components  

---

## Schematics

The system is based on a simple embedded architecture where sensors provide input to the microcontroller, which processes the data and controls actuators.

![System Diagram](diagram.svg)

---

## Bill of Materials (Estimated Cost)

- STM32 Nucleo-U545RE-Q – ~120 RON  
- 2x SG90 Micro Servo Motor – ~20 RON  
- HC-SR04 Ultrasonic Sensor – ~10 RON  
- KY-038 Sound Sensor Module – ~5-10 RON  
- SSD1306 OLED Display – ~25 RON  
- Breadboard – ~20 RON  
- Jumper wires – ~15 RON  
- Power supply – ~20 RON  
- Mechanical materials – ~50-60 RON  

**Total estimated cost: ~285–300 RON**
