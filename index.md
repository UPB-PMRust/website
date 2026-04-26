\# Building Control System (STM32 Smart Monitoring \& Automation)



\## Description

This project is an embedded system based on an STM32 microcontroller that monitors and controls environmental parameters in a building. It integrates temperature monitoring, gas detection, and staircase lighting automation using sensors, logic circuits, and relay-based control. System status is displayed in real time on an LCD screen.



\---



\## Motivation

The purpose of this project is to design a smart building control system using embedded systems principles. It demonstrates real-time data acquisition, signal processing (Kalman filtering), and automated control using STM32 microcontrollers.



\---



\## Functionality



The system performs the following tasks:

\- Reads data from multiple temperature sensors

\- Applies a Kalman filter to improve temperature accuracy

\- Monitors gas levels for safety simulation

\- Controls staircase lighting using a logic-based trigger system

\- Automatically turns off staircase lights after 60 seconds

\- Displays runtime, temperature, and gas levels on an LCD



\---



\## Architecture



The system is composed of:

\- STM32 microcontroller (main control unit)

\- Temperature sensors (3 units)

\- Gas sensor module

\- LCD display

\- Relay modules for load control

\- Logic gate / capacitive trigger circuit for staircase input



\### System Layers

\- Hardware layer (sensors, relays, LCD)

\- Processing layer (Kalman filter, control logic)

\- Application layer (automation logic and display updates)



\---



\## Components



\- STM32 microcontroller

\- 3x temperature sensors

\- 1x gas sensor

\- LCD display module

\- Relay modules

\- Logic gate / staircase trigger circuit

\- Power supply unit



\---



\## Hardware Design



\- Temperature sensors connected via ADC or I2C

\- Gas sensor connected via analog/digital input

\- LCD connected via I2C interface

\- Relays controlled via GPIO pins

\- Staircase trigger connected to digital input with logic gating



Power is distributed to all modules through a regulated supply.



\---



\## Software Design



The firmware handles:

\- Real-time sensor reading

\- Kalman filtering for temperature estimation

\- State machine for staircase lighting control

\- Timer interrupt for 60-second auto shutoff

\- LCD display updates



\---



\## Bill of Materials



| Component            | Quantity | Description |

|---------------------|----------|-------------|

| STM32 Microcontroller | 1      | Main control unit |

| Temperature Sensor   | 3        | Environmental monitoring |

| Gas Sensor           | 1        | Safety monitoring |

| LCD Display          | 1        | Output interface |

| Relay Module         | 6        | Load control (lights/devices) |

| Logic Gates          | 3        | Staircase trigger logic |

| Power Supply         | 1        | System power |



\---





