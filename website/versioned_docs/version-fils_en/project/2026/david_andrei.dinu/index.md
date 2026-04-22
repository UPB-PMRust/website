# Rust Autonomous Hovercraft
A simple, autonomous hovercraft controlled by a Raspberry Pi Pico 2 and Rust.

:::info 

**Author**: David-Andrei Dinu \
**GitHub Project Link**: https://github.com/UPB-PMRust/project-david-andrei-dinu

:::

## Description

The project consists of an autonomous hovercraft powered by a Raspberry Pi Pico 2 microcontroller. It uses dual motors—one for lift and one for propulsion/steering. The hovercraft will be able to navigate its environment autonomously, avoiding obstacles using distance sensors. The entire control logic, including motor PWM control and sensor data processing, will be written in Rust using the `embassy-rs` framework.

## Motivation

I chose this project because it perfectly combines hardware design, aerodynamics, and embedded programming. Building a hovercraft presents unique control challenges (such as lack of friction and drift), making the development of the control software in Rust an exciting and rewarding learning experience.

## Architecture 

The system architecture consists of a central Processing Unit (Raspberry Pi Pico 2) that interfaces with input and output components.
- **Processing Unit**: The core controller that runs the Rust firmware.
- **Sensors (Input)**: Ultrasonic distance sensors (e.g., HC-SR04) for obstacle detection.
- **Motor Controllers (Output)**: A dual motor driver module (e.g., L298N or TB6612FNG) that takes PWM signals to control the lift and thrust motors.
- **Power Management**: A battery pack supplying appropriate voltage to both the motors and the logic board.

*(You will need to replace this text with a diagram drawn in diagrams.net showing the connections between the Pico 2, Motor Driver, Motors, Sensors, and Battery).*

## Log

### Week 5 - 11 May
- Project topic selection and initial component research.
- Placed order for electronic components on Optimus Digital.

### Week 12 - 18 May
- *To be updated with hardware assembly progress*

### Week 19 - 25 May
- *To be updated with software implementation progress*

## Hardware

The hardware is based on the Raspberry Pi Pico 2 microcontroller. The mechanical structure of the hovercraft will be built using lightweight materials (like foam board). Two DC motors are used: a central downward-facing motor for the lift cushion, and a rear-facing motor for thrust and steering (via a servo-controlled rudder or differential thrust).

### Schematics

*(Place your KiCAD or similar schematics here in SVG format. For example: `![Schematics](./schematics.svg)`)*

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico 2](https://www.optimusdigital.ro) | The main microcontroller running Rust | ~30 RON |
| [DC Motors & Propellers](https://www.optimusdigital.ro) | For lift and thrust | ~50 RON |
| [Motor Driver (L298N / TB6612FNG)](https://www.optimusdigital.ro) | Controls motor speed and direction | ~15 RON |
| [Ultrasonic Sensor HC-SR04](https://www.optimusdigital.ro) | Obstacle detection | ~10 RON |
| [Battery Holder & Batteries](https://www.optimusdigital.ro) | Power supply for motors and Pico | ~25 RON |
| [Foam Board/Chassis Materials](https://www.optimusdigital.ro) | Hovercraft body | ~20 RON |

## Software

The software will be written entirely in Rust, utilizing the `embassy-rs` framework for asynchronous execution.

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rs](https://github.com/embassy-rs/embassy) | Async framework for embedded Rust | Main scheduling and execution |
| [embassy-rp](https://github.com/embassy-rs/embassy/tree/main/embassy-rp) | HAL for RP2040/RP2350 | Peripherals access (PWM, GPIO) |

## Links

1. [Rust on Raspberry Pi Pico](https://github.com/rp-rs/rp-hal)
2. [Embassy Framework Documentation](https://embassy.dev/)
3. [How to build a simple hovercraft](https://www.youtube.com/results?search_query=diy+hovercraft)
