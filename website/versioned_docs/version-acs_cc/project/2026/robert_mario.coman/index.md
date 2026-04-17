# Endless Motion:
A robot arm that cleans liquid around itself.

### Description
**Endless Motion** is a robotic arm system that receives input from sensors placed in specific locations to detect the presence of a liquid. It processes these signals to determine the target area that requires intervention. Based on this data, it calculates control commands, moves to the detected zone, and executes a wiping action. The system continuously updates its movement, repeating the process based on newly received signals.

### Motivation
This project explores the control of a robotic arm within a real-time reactive system. The main idea is to adapt an automated industrial behavior (responding to external stimuli) into a small-scale application. The project combines data acquisition electronics, asynchronous embedded programming in Rust, and mechanical modeling, demonstrating the capability to read real-time sensory data and translate it into precise mechanical actions.

### Architecture

![System Architecture Diagram](./images/diagram_pm.svg)

Based on the system diagram, the architecture is divided into the following logical and hardware blocks:

* **Liquid Level Sensors (Input):** The physical endpoints that detect the presence of liquid and send raw analog or digital signals to the microcontroller.
* **GPIO/ADC Input Driver:** The low-level hardware abstraction layer on the STM32 that reads the electrical signals from the sensors and translates them into software trigger events.
* **Async Task Manager (Rust Embassy):** The core logic router. It uses asynchronous tasks to handle sensor triggers without blocking the CPU, safely managing the overall state of the robot (e.g., Idle, Moving, Wiping).
* **Kinematics & Motion Controller:** Receives target zone data from the Task Manager and calculates the specific angles required for the robotic arm to reach the active area and execute the cleaning sequence.
* **Hardware Timers & PWM Output:** Converts the mathematically calculated angles into highly precise PWM electrical signals.
* **Servomotors (Output):** The physical actuators that interpret the PWM control signals to execute the physical movement of the arm.
* **External Power Supply:** A dedicated power source strictly for the servomotors.

### Log
