# Endless Motion
A robot arm that cleans liquid around itself.

:::info

**Author**: Robert-Mario Coman \
**Github Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-robertcoman1

:::

### Description
**Endless Motion** is a robotic arm system that receives input from sensors placed in specific locations to detect the presence of a liquid. It processes these signals to determine the target area that requires intervention. Based on this data, it calculates control commands, moves to the detected zone, and executes a wiping action. The system continuously updates its movement, repeating the process based on newly received signals.

### Motivation
The primary inspiration for this project comes from the famous contemporary artwork *"Can't Help Myself"* (by Sun Yuan & Peng Yu), which features an industrial robotic arm trapped in an endless, repetitive loop of trying to clean up a liquid that continuously spreads across the floor. Fascinated by this visual and conceptual piece, I decided to engineer a small-scale, functional replica of this behavior.

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

