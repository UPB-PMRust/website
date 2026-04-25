# The Lie Detector

A biometric analyzer powered by an STM32 that correlates real-time heart rate and skin conductivity via ADC to detect deception markers.

:::info
**Author**: Sahin Leyla-Kubra \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/fils-project-2026-leylakubrasahin](https://github.com/UPB-PMRust-Students/fils-project-2026-leylakubrasahin)
:::

## Description

The Lie Detector is an asynchronous embedded system designed to monitor physiological stress indicators often associated with deception. Using an STM32 Nucleo-U545RE-Q microcontroller, the device establishes a physiological baseline for a subject's heart rate and skin conductivity (GSR). During active questioning, the system uses its ADC to monitor real-time sensor data, triggering visual PWM-controlled RGB LED transitions and audible buzzer alarms if simultaneous spikes above the baseline thresholds are detected.

## Motivation

I was inspired by the modern criminal investigation tool, the "Polygraph", as I am a fan of crime documentaries and have always wanted to experiment with one. Therefore, I decided that I could recreate my own version of it by using the STM32 Nucleo-U545RE-Q microcontroller as the main component.

## Architecture

Main software and system components:

* **Physiological Data Acquisition Module**: Reads skin conductance from the Grove GSR sensor and heart rate from the Pulse sensor via ADC channels.
* **Stress Analysis Module**: Processes the physiological baseline and current readings using an asynchronous logic brain to detect deception spikes.
* **Shared State Module**: Manages safe data access between concurrent tasks using an `embassy-sync` Mutex.
* **Visual Feedback Module**: Controls the RGB LED via PWM to transition colors from Green to Red based on stress intensity.
* **Audible Alert Module**: Drives the passive buzzer using PWM to emit alarms that vary in pitch as deception scores rise.
* **Logging and Debug Module**: Sends real-time sensor data and system status to a PC via USB for graphing and monitoring.

![Architecture Diagram](./lie_detector.svg)

## Log

### Week 1-7
* Dedicated this period to brainstorming and evaluating potential embedded system concepts.
* Explored various ideas before finalizing the "Lie Detector" based on the STM32 Nucleo-U545RE-Q.
* Researched physiological sensors (GSR and Pulse) and their integration with asynchronous Rust.

### Week 8
* Focused on sourcing the specific hardware required for the project.
* Obtained the necessary components like the Pulse sensor, RGB LEDs, and an active buzzer.

### Week 9
* Completed the gathering of all secondary hardware, including the USB debugging cables and safety resistors.
* Completed the project documentation milestone.

## Hardware

The project uses a mix of analog physiological sensors and asynchronous digital processing to achieve real-time deception detection. The hardware setup is centered around the STM32 Nucleo-U545RE-Q, which acts as the high-speed data aggregator.

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

| Device | Usage | Price |
| :--- | :--- | :--- |
| [STM32 NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Central Controller | 128 RON |
| [Grove GSR Sensor](https://wiki.seeedstudio.com/Grove-GSR_Sensor/) | Stress Monitor | 40.18 RON |
| [Analog Pulse Sensor](https://robocraze.com/blogs/post/what-is-pulse-sensor) | Heart Rate Monitor | 15.4 RON |
| [RGB LED](https://www.elprocus.com/what-is-three-rgb-led-and-its-working/) | Visual Indicator | 0.92 RON |
| [Passive Buzzer](https://www.circuitbasics.com/what-is-a-buzzer/) | Audible Alarm | 1 RON |
| Resistors (220Ω) | Safety | 2 RON |
| Breadboard & Wires | Connection | 30 RON |

## Software

| Library | Description | Usage |
| :--- | :--- | :--- |
| [embassy-stm32](https://github.com/embassy-rs/embassy) | STM32 Hardware Abstraction Layer | Controls the ADC for sensors and PWM for indicators. |
| [embassy-time](https://github.com/embassy-rs/embassy/tree/main/embassy-time) | Timekeeping library | Manages the baseline calibration timing. |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task manager | Allows simultaneous monitoring of multiple sensors. |
| [fixed](https://github.com/tomhoule/fixed) | Fixed-point math | Calculates percentage spikes without floating-point overhead. |
| [defmt](https://github.com/knurling-rs/defmt) | Efficient logging | Sends real-time sensor data to the PC via RTT. |

## Links

1. [Embassy-rs Documentation](https://embassy.dev/book/index.html)
2. [Rust Embedded Book](https://docs.rust-embedded.org/book/)
3. [GSR Sensor Guide](https://wiki.seeedstudio.com/Grove-GSR_Sensor/)