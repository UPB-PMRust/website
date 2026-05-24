# Acoustic Material Sorter
A smart environmental sorting device that classifies materials (glass, plastic, metal) by analyzing the acoustic signature of their impact using digital signal processing and machine learning.

:::info

**Author**: Bogdan Octavian Grecu \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/acs-project-2026-grecubog2001](https://github.com/UPB-PMRust-Students/acs-project-2026-grecubog2001)

:::
## Description

The Acoustic Material Sorter is a smart classification system that differentiates objects based on the sound they make when dropped. Items slide down a custom-built ramp and hit a hard impact plate. A piezoelectric contact microphone taped beneath the plate captures the raw audio waveform of the impact. The system rapidly processes this audio burst, extracting frequency data to feed into a machine-learning model that predicts whether the material is Glass, Plastic, or Metal.

A master switch determines whether we are in the data aquisition phase or the inference phase.

If we are in the data aquisition phase, we collect impact data from our materials. In order to manually label the material, we use a 4x4 keypad.

If we are in the inference phase, the system checks whether a model exists in our root folder.

If it doesn't, then it trains the model from scratch using our recorded data. The display shows the iteration number and the loss.

If it does, or after it trains the model, it waits for impact data, then, using the model, it classifies the material. The predicted material is displayed on the screen. Several SG90 micro servo automatically move a sequence of gates to guide the item into the correct bin. 

## Motivation

Traditional optical sorting machines used in recycling facilities are expensive, highly complex, and often struggle with transparent materials (like clear glass vs. clear plastic) or dirty surfaces. I chose this project to explore a cost-effective, robust alternative: acoustic sorting.

Every material has a unique resonant frequency and acoustic "fingerprint." By relying on sound rather than sight, we bypass the limitations of optical sensors. Additionally, this project serves as an excellent sandbox for learning about edge computing, as it perfectly blends hardware actuation, digital signal processing (DSP), and embedded machine learning all in one integrated Rust-based environment.

## Architecture

- Acoustic Acquisition Subsystem: A piezoelectric disc acts as a contact microphone, triggering an ADC capture on the microcontroller the exact moment an impact occurs, recording a rapid, 1-second burst of raw audio data.

- Signal Processing Engine: The raw time-domain audio data is transformed into the frequency domain (a spectrogram representation) using the rustfft library, extracting the distinct acoustic features of the drop.

- Classification Engine: The extracted frequency features are fed into a trained machine learning model (powered by the smartcore library) which outputs a prediction and a confidence score.

- Display System: A 1602 I2C character LCD provides real-time feedback, showing the active prediction (e.g., "Metal").

- Actuation System: Several SG90 micro servos are attached to a sequence of gates. This physically directs the material to the corresponding collection bin (Glass, Plastic, or Metal).

## Log

## Hardware

The project utilizes a Raspberry Pi Pico 2 as the main microcontroller, leveraging its powerful RP2350 architecture to handle digital signal processing and machine learning inference at the edge. A piezoelectric disc is taped to the impact plate and connected to one of the Pico's ADC pins to measure the vibration and sound of the falling object. A 1602 character LCD module, connected via the I2C interface, serves as the user interface to display the classification results and confidence levels. 4 SG90 micro servos, each controlled via a PWM pin, mechanically routes the objects into their respective bins. A 4x4 numpad facilitate manual data labelling. All electronic components are interfaced using a breadboard and standard jumper wires.

<img width="721" height="960" alt="image" src="https://github.com/user-attachments/assets/0449ff2a-d726-461a-aae9-35ec7e84a3a2" />
<img width="721" height="960" alt="image" src="https://github.com/user-attachments/assets/fabf8dd4-b936-468f-ab6a-8ab9c649f94d" />
<img width="721" height="960" alt="image" src="https://github.com/user-attachments/assets/16c5d53f-491b-4a55-8a9e-0aa5b2c6eea9" />


### Schematics

<img width="970" height="553" alt="Screenshot 2026-05-24 151729" src="https://github.com/user-attachments/assets/1b2c01ed-03c4-4476-a64d-724f0b23b2ea" />


### Bill of Materials
| Device | Usage | Price |
|--------|--------|-------|
|Raspberry Pi Pico 2|"Main microcontroller board handling DSP, ML, and hardware control"| 270 ron (kit)
|1602 I2C LCD|Displays the predicted material type and ML confidence score| included in kit
|Piezoelectric Disc|Contact microphone used to capture the impact sound| 25 ron 
|4x SG90 Micro Servo|Actuator that rotates the funnel to sort items into bins| 35 ron
|4x4 keypad|Input for manual material classification and disposal| included in kit
|Breadboard & Wires|Prototyping connections| included in kit
|Ramp & Impact Plate|Physical structure for dropping and funneling items| 100 ron

## Software

| Library / Interface | Description | Usage |
| --- | --- | --- |
| `cortex-m` | ARM Cortex-M support library for low-level processor access. | Access core peripherals and use `Peripherals::take()` in `src/main.rs`. |
| `cortex-m-rt` | Runtime support crate for Cortex-M microcontrollers. | Provides the entry point and startup/runtime support for the embedded firmware. |
| `embedded-hal` | Hardware abstraction traits for embedded devices. | Provides `InputPin` and `OutputPin` traits used by keypad, LED, and GPIO logic. |
| `embedded-hal-0-2` | Compatibility adapter for older embedded-hal v0.2 traits. | Provides `PwmPin` and `adc::OneShot` traits used by servo and ADC code. |
| `defmt` | Efficient logging framework for `no_std` embedded Rust. | Used for debug logging in firmware and tests. |
| `defmt-rtt` | RTT transport layer for `defmt` logging. | Enables runtime debug output over RTT during development. |
| `usb-device` | USB device stack for embedded targets. | Handles USB polling and device management for serial communication. |
| `usbd-serial` | USB CDC serial implementation. | Provides the virtual serial port used to send and receive waveform and control messages. |
| `heapless` | Fixed-capacity data structures for `no_std` environments. | Uses `heapless::String` for LCD text formatting and serial line buffering. |
| `lcd-lcm1602-i2c` | I2C driver for 16x2 LCD modules. | Drives the display used for mode prompts, instructions, and inference results. |
| `micromath` | Math helpers for `no_std` floating-point computation. | Provides `F32Ext` for CNN inference and training math operations. |
| `panic-probe` | Panic handler that reports panic information via probe. | Used on ARM targets to surface panic messages during development. |
| `rp235x-hal` | HAL for RP2350-based Raspberry Pi Pico 2 boards. | Provides GPIO, ADC, PWM, I2C, timer, and USB support for the RP2350 target. |




[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/yNS7YZsE)
