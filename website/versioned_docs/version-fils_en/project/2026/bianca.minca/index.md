# SmartPark

An intelligent parking management system based on STM32, utilizing ultrasonic sensors for detection and an LCD for real-time monitoring.

:::info 
**Author:** Minca Bianca-Mihaela \
**Group:** 1221EA \
**GitHub Project Link:** [https://github.com/UPB-PMRust-Students/fils-project-2026-biancamnc](https://github.com/UPB-PMRust-Students/fils-project-2026-biancamnc)
:::

## Description
This project implements an automated parking barrier system that integrates a keypad for secure PIN-based access. The system detects an approaching vehicle using an IR/proximity sensor and control the barrier arm via a PWM-controlled servomotor. When a valid PIN is entered on the keypad, a signal triggers the servo motor to open the gate. Once the vehicle enters the parking facility, a dedicated LED system guides the user to their specific reserved spot. Furthermore, each individual parking space is equipped with an IR sensor connected to color-coded LED indicators. These provide real-time visual feedback on the occupancy status of every spot. 

## Motivation
Witnessing the daily frustration of drivers searching for available spots and the inefficiencies of manual entry systems inspired me to build an end-to-end automated parking solution from scratch. From a technical standpoint, this project allows me to combine multiple core microcontroller peripherals—such as PWM for precise servo actuation, GPIO matrices for keypad input, and External Interrupts for real-time IR sensing—into a single, cohesive ecosystem.
Instead of relying on isolated components, I want the STM32 to act as a centralized gateway that bridges the gap between a digital web reservation platform and physical hardware validation. This challenge requires synchronizing several concurrent events: validating secure PIN entries, monitoring vehicle proximity, and managing timed barrier movements.Building this system was a great opportunity to dive into async embedded development. It challenged me to manage complex hardware events simultaneously while ensuring the high level of memory security that Rust and Embassy provide.

## Architecture

The system architecture revolves around the **STM32 microcontroller (Nucleo board)**, managing multiple tasks concurrently:

```text
[ Power / USB Debug ]
               |
               V
      +-----------------------+
      |  NUCLEO STM32 Board   | <--- (Central Controller)
      +-----------------------+
         ^          |          |
         |          |          V
      [ Inputs ]    |     [ Outputs ]
         |          |          |
      +----------+  |    +--------------+
      | 4x4      |  |    | Servo Motor  |
      | Keypad   |--+    | (Barrier)    |
      | (PIN)    |       +--------------+
      +----------+             |
         ^                     V
         |               +--------------+
      +----------+       | Multi-LED    |
      | IR       |------>| Indicators   |
      | Sensors  |       | (Status)     |
      | (Spots)  |       +--------------+
      +----------+             |
                               V
                         +--------------+
                         | 0.96" OLED   | <--- (I2C Interface)
                         | Display (UI) |
                         +--------------+
```

The system architecture revolves around the STM32 microcontroller (Nucleo board):
1. **Input:**User credentials are captured via a Keypad matrix for secure PIN entry. Vehicle proximity and parking spot occupancy are monitored asynchronously using IR sensors connected via External Interrupts (EXTI) to ensure zero-latency detection.
2. **Processing:** The STM32 validates the entered PIN against reservation data and manages the parking state logic. Using the Embassy framework, the system concurrently tracks the barrier's position, processes keypad input, and updates the real-time availability of parking spots.
3. **Output:** Movement is triggered via PWM (Pulse Width Modulation) to the Servomotor, which precisely actuates the barrier arm. Visual feedback is provided through a Multi-LED system: status indicators for the main gate and individual Red/Green LEDs to signal occupancy for each parking spot.

## Shematics
**KiCad Schematic** 
![SmartPark Schematic](./schematic.svg)

## Hardware
The system is centered around an STM32 Nucleo MCU. For input, I am using IR proximity sensors for vehicle detection and a 4x4 Matrix Keypad for secure PIN entry. Outputs consist of an SG90 Servo Motor driven by a PWM signal to actuate the barrier, alongside Red and Green LEDs for real-time status and occupancy feedback via digital GPIO.
The firmware uses the Embassy framework to handle these components concurrently, ensuring responsive motor control and low-latency sensor monitoring.

## Log
### Week 4 - Idea research
 * Explored various themes within smart city infrastructure
 * Initially considered a simple ultrasonic-based barrier, then decided to expand the scope to a full-stack system including a web reservation platform and secure PIN entry.
 * Researched commercial automated parking systems gate controllers to understand industrial standards for safety delays and user feedback.

### Week 5 - 6 - Component research & Procurment
 * Researched hardware requirements for precise actuation and reliable proximity sensing: STM32 Nucleo board, SG90 servo motor, IR obstacle sensors, and a 4x4 matrix keypad.
 * Compared detection methods (Ultrasonic vs. IR), ultimately choosing Infrared (IR) sensors for their faster response times and simpler digital logic in parking spot occupancy detection.

### Week 7 - 8 - Components arrive & Initial testing
 * Started testing the USB
 * Started testing the components to see if works

### Week 9 
 * Encountered a problem, the ultrasonic sensor didn't work, looking for a substitute


## Bill of materials
| Device | Usage | Price |
| :--- | :--- | :--- |
| STM32 Nucleo | System Brain | ~125 RON |
| Ultrasonic Sensor HC SR-04P | Vehicle Detection | ~10 RON |
| Servo Motor | Barrier Control | ~18 RON |
| I2C LCD | Status Display | ~18 RON |
| IR sensors | Parking spot detection car | ~14 RON |
| LED set | For parking spots status and reserved spots | ~5 RON |
| Resistors | For the LED's | ~15 RON |
| M-T wires | Connections | ~33 RON |
| M-M wires | Connections | ~16 RON |
| Programator UART USB ESP-01 | Debugging | ~10 RON |
| Breadboard | Component connections | ~12 RON |
| Keypad | PIN entering | ~7 RON |
| WIFI module ESP-01 | Connectivity | ~20 RON |

## Software
| Library | Description | Usage |
| :--- | :--- | :--- |
| `embassy-stm32` | Async HAL for STM32 microcontrollers | Handles hardware drivers for **PWM** (Servo), **GPIO** (LEDs & Sensors), and **UART** (Debug). |
| `embassy-executor` | Async task executor for Embassy | Runs concurrent tasks: monitoring the **Keypad**, reading **IR Sensors**, and moving the **Servo** simultaneously. |
| `embassy-time` | Async timers and delays | Handles the 5-second delay for the barrier and software **debouncing** for the keypad. |
| `embassy-sync` | Mutexes and channels for shared state | Safely shares the "Parking Spot Status" or "PIN Code" between different async tasks. |
| `embedded-hal` | Hardware abstraction traits | Provides a common interface for GPIO and PWM, ensuring compatibility with driver crates. |
| `defmt` & `defmt-rtt` | Efficient logging framework | Sends real-time messages like "Vehicle Detected" or "Correct PIN" to your PC console. |
| `panic-probe` | Panic handler for embedded | Helps you find exactly where the code crashed during testing. |
| `ssd1306` | I2C/SPI driver for the display | Sends raw data to the screen using the I2C protocol. |
| `embedded-graphics` | 2D graphics library | Allows you to write text, draw lines, or small icons. |

## Links
1. [Embedded Rust](https://docs.rust-embedded.org/book/)
2. [Nucleo-U545RE-Q](https://www.st.com/resource/en/user_manual/um3062-stm32u5-nucleo64-board-mb1841-stmicroelectronics.pdf)

