# Ping Pong Ball Launcher
Variable speed ping pong ball launcher

:::info

**Author**: Roescu Matei Florin \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/acs-project-2026-matei78](https://github.com/UPB-PMRust-Students/acs-project-2026-matei78)

:::

<!-- do not delete the \ after your name -->

## Description
This project presents an automatic ping pong ball launcher designed for training purposes. The system is capable of launching balls at variable speeds, which can be adjusted through user input via a control button. A separate trigger button allows the user to launch a ball on demand, offering precise control over each shot. The launching mechanism is based on two continuously rotating wheels that grip and accelerate the ball upon contact, ensuring consistent speed and direction.

## Motivation

The main idea for this project was inspired by my passion for ping pong. Since I play this sport regularly, I have always been interested in finding ways to improve my skills. This led me to the concept of designing a training machine that could simulate real gameplay situations. Building a ping pong ball launcher felt like a natural and exciting challenge, as it combines my interest in the sport with practical engineering and problem-solving.
## Architecture
![Diagram](Architecture.drawio.webp) 

Main Components:
- **Microcontroller (STM32 Nucleo)**: It's the brain of the project
- **Buttons**: One is used to cycle through different predefined launching speeds and the other is used to launch the ball
- **DC Motors**: Used for spinning two wheels continuously
- **Motor Driver**: Controls the speed and direction of the two motors
- **Power Supply**: The board does not support enough power for high motor speeds, so a power supply is needed
- **Servo Motor**: Used for releasing the ball when launching button is pressed

## Log

<!-- write your progress here every week -->
### Week 20 - 26 April
Created documentation and bill of materials

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Hardware used:
- **Microcontroller (STM32 Nucleo)**: Controls all the other components
- **Buttons**: One is used to cycle through different predefined launching speeds and the other is used to launch the ball
- **DC Motors**: 3V DC Motors used for spinning two wheels continuously
- **Motor Driver**: Controls the speed and direction of the two motors
- **Power Supply**: The board does not support enough power for high motor speeds, so a power supply is needed
- **Servo Motor**: 90 degree Motor used for releasing the ball when launching button is pressed

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

| Device | Usage | Price |
| ------------------------------------------ | -------------------------------------------------------- | ----------------|
| STM32 | Microcontroller | [156 RON](https://ro.farnell.com/stmicroelectronics/nucleo-u545re-q/development-brd-32bit-arm-cortex/dp/4216396?CMP=e-email-sys-orderack-GLB) |
| Motor driver L9110S | Controls DC Motors| [14,99 RON](https://www.optimusdigital.ro/ro/drivere-de-motoare-cu-perii/8246-modul-driver-de-motoare-cu-4-canale-l9110s.html) |
| 30mm wheels | Launches balls | 2 x [3,90 RON](https://www.optimusdigital.ro/ro/mecanica-roti/347-roata-de-20-mm-cu-cauciuc-pentru-ax-de-2-mm.html) |
| Motor DC F130 3V | Spins wheels | 2 x [3,99 RON](https://www.optimusdigital.ro/ro/motoare-altele/13612-motor-dc-f130-3v.html) |
| Micro Servomotor SG90 90 | Deploys ball | [13,99 RON](https://www.optimusdigital.ro/ro/motoare-servomotoare/26-micro-servomotor-sg90.html) |
| Buttons | Pressed for actions | 2 x [1,99 RON](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1114-buton-cu-capac-rotund-rou.html) |
| Batteries, plastics, other | Building materials | 20 RON |


## Software

| Library       | Description                                                                 | Usage                                                                 |
|---------------------------|-----------------------------------------------------------------------------|-----------------------------------------------------------------------|
| [embassy-executor](https://docs.rs/embassy-executor/latest/embassy_executor/)          | Lightweight async executor for running embedded tasks                      | Runs tasks for motor and servo control without blocking         |
| [embassy-time](https://docs.embassy.dev/embassy-time/)              | Time management                              | Creates delays for PWM or timing during launch           |
| [embassy-stm32::pwm](https://docs.embassy.dev/embassy-stm32/git/stm32f4/pwm/index.html)        | PWM module for STM32                                           | Controls DC motor speed and servo position                |
| [embassy-stm32::gpio](https://docs.embassy.dev/embassy-stm32/git/stm32f4/gpio/index.html)       | GPIO pin control                                                          | Reads buttons and controls digital signals                       |

## Links

1. [Reddit ping pong launcher](https://www.reddit.com/r/arduino/comments/1oecjqm/help_with_ping_pong_ball_launcher/)
2. [Youtube ball launcher](https://www.youtube.com/watch?v=cs02VoRqgPg)
3. [Powerful ball launcher](https://www.youtube.com/watch?v=-1ND3IfWjNY)
