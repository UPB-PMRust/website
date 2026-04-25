# 4WD Omni-Directional Smart Rover with Wi-Fi Teleoperation

A 4WD smart robotic rover powered by an STM32 Nucleo board, featuring mecanum wheels for lateral parking and Wi-Fi teleoperation.

:::info
**Author:** Dunărinţu Mihnea-Rafael \
**Group:** 1221ED \
**GitHub Project Link:** [https://github.com/UPB-PMRust-Students/fils-project-2026-mihnearafael](https://github.com/UPB-PMRust-Students/fils-project-2026-mihnearafael)
:::

## Description

The goal of this project is to build a 4WD smart robotic rover powered by an STM32 Nucleo board. The core software logic will be organized as a simple state machine, transitioning smoothly between the AUTONOMOUS_FORWARD, AUTONOMOUS_AVOID, MANUAL_CONTROL, SMART_REVERSE, LATERAL_PARKING, and STANDBY states based on sensor inputs, Wi-Fi commands, or physical hardware interrupts.

The car operates in two primary modes: an autonomous mode where it navigates independently using a front-facing ultrasonic sensor to detect and avoid obstacles, and a manual mode where it is piloted remotely. Thanks to the upgraded 4WD mecanum wheels, the rover is capable of omnidirectional movement, allowing for advanced maneuvers such as lateral parking. 

For teleoperation, an ESP8266 module acts as a Wi-Fi bridge, allowing the user to control the rover's movements using a custom joystick. Additionally, the rover features an acoustic parking assist. When reversing, a rear-facing ultrasonic sensor continuously monitors the distance to obstacles behind the car, triggering a passive buzzer to beep faster as it approaches a wall. The user can switch between autonomous and manual modes via the Wi-Fi app, alternatively a custom joystick or by pressing the physical blue USER button on the Nucleo board as an instant failsafe override.

## Motivation

I chose this project to practically apply embedded systems concepts, particularly concurrent task scheduling and state machine logic. Upgrading from a standard 2WD differential drive to a 4WD mecanum system introduces interesting challenges in motor control and PWM synchronization. The addition of lateral parking and a custom joystick makes the system far more interactive and closer to modern automotive assist technologies.

## Architecture

The system is organized into five functional layers, centered around the STM32 Nucleo board:

1. **Power:** Two 18650 Li-Ion batteries provide 7.4V. This powers the motor drivers directly, while an LM2596 buck converter steps it down to a stable 5V for the microcontroller and sensors.
2. **Mobility (4WD):** Four independent DC motors, equipped with mecanum wheels, are driven by two TB6612FNG motor controllers receiving PWM signals from the STM32.
3. **Perception:** Three HC-SR04 ultrasonic sensors (front, rear, lateral) detect obstacles. Their 5V Echo signals are safely stepped down to the Nucleo's 3.3V logic via a level converter.
4. **Communication:** An ESP-01S module establishes a Wi-Fi bridge, allowing the STM32 to receive remote control vectors from the custom joystick UI.
5. **Feedback:** A 0.96" OLED display provides visual state information, while a passive buzzer emits variable-frequency acoustic warnings during reverse maneuvers.

## Log

### Week 1 - 9
- Started with a 2WD Rover idea and eventually moved to 4WD and more complex features like parallel parking after receiving feedback.
- Throughout the weeks I have gathered the materials needed to assemble the hardware of the kit and its adjacent components needed for the behaviour of the rover.

### Week 12 - 18 May
### Week 19 - 25 May

## Hardware

The project relies on an STM32 Nucleo board as the main controller, communicating with an ESP8266 for remote commands. The chassis has been modified to support 4 independent DC motors and mecanum wheels for complex kinematics. Three ultrasonic sensors handle spatial awareness, while power is regulated via a buck converter to safely power the logic systems from the Li-ion batteries.

### Schematics

![Architecture Diagram](./architecture.svg)

### Bill of Materials

| Device | Usage | Price |
| :--- | :--- | :--- |
| STM32 Nucleo-U545RE-Q | Main microcontroller | 108.34 RON |
| 4WD Smart Car Chassis Kit | Base physical platform | 83.24 RON |
| 4x Omnidirectional (Mecanum) Wheels | Lateral/omnidirectional movement | 108.04 RON |
| 2x TB6612FNG Motor Drivers | PWM control for the DC motors | 20 RON |
| ESP-01S ESP8266 Wireless Module | Wi-Fi bridge for remote teleoperation | 18.99 RON |
| Bluetooth Module HC-06 | Alternative wireless communication | 30.41 RON |
| 3x HC-SR04 Ultrasonic Sensors | Obstacle detection (front & rear) | 40.12 RON |
| 2-Axis Joystick Module | Custom remote control input | 5.45 RON |
| OLED Display 0.96" I2C | Real-time status UI | 16.96 RON |
| 3V / 3.3V Passive Buzzer | Acoustic parking assist feedback | 0.99 RON |
| 2x Li-Ion Batteries & Battery Case | Main power supply | 72.09 RON |
| Li-Ion Battery Charger Module | Battery management & charging | 9.99 RON |
| LM2596 DC-DC Buck Converter | Voltage regulation (fixed 5V) | 12.99 RON |
| Breadboard Power Supply & 9V Connector | Logic power distribution | 6.20 RON |
| Prototyping PCBs & Breadboard | Component mounting & soldering | 22.98 RON |
| Silicone Wires, Jumpers, & Pin Headers | Component interconnections | 71.40 RON |
| Resistors, Capacitors, & Power Switch | Circuit filtering & power toggling | 22.88 RON |
| M2 Hex Pillars | Hardware mounting | 9.95 RON |

## Software

| Library | Description | Usage |
| :--- | :--- | :--- |
| `embassy-stm32` | Hardware Abstraction Layer | Required to interface directly with the STM32 hardware registers and pins. |
| `embassy-executor` | Async runtime | Responsible for scheduling concurrent, non-blocking tasks (driving motors while listening for Wi-Fi). |
| `embassy-time` | Timer module | Essential for creating precise microsecond timers to calculate obstacle distances via HC-SR04. |
| `embedded-hal` | Standard traits | Provides standard traits needed to communicate with external hardware safely. |
| `defmt` & `defmt-rtt` | Logging framework | Highly efficient logging tools for real-time debugging over USB. |
| `panic-probe` | Error handling | Catches critical firmware errors and prints backtraces. |
| `heapless` | Data structures | Manages UART data stream from ESP8266 without dynamic allocation. |

## Links

1. [STM32 Nucleo-U545RE-Q Documentation](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)
2. [Embassy framework documentation](https://embassy.dev/)