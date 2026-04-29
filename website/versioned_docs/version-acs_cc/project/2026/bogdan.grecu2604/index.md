# Acoustic Material Sorter
A smart environmental sorting device that classifies materials (glass, plastic, metal) by analyzing the acoustic signature of their impact using digital signal processing and machine learning.

:::info

**Author**: Bogdan Octavian Grecu \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/acs-project-2026-grecubog2001](https://github.com/UPB-PMRust-Students/acs-project-2026-grecubog2001)

:::
## Description

The Acoustic Material Sorter is a smart classification system that differentiates objects based on the sound they make when dropped. Items slide down a custom-built ramp and hit a hard impact plate. A piezoelectric contact microphone taped beneath the plate captures the raw audio waveform of the impact. The system rapidly processes this audio burst, extracting frequency data to feed into a machine-learning model that predicts whether the material is Glass, Plastic, or Metal.

The predicted material and the model's confidence percentage are displayed on a 1602 I2C LCD screen. If the confidence threshold is met, an SG90 micro servo automatically rotates a funnel to guide the item into the correct bin. If the confidence is too low, the system pauses and asks a human to manually classify the item by pressing one of four buttons (Glass, Plastic, Metal, or Disposal).

## Motivation

Traditional optical sorting machines used in recycling facilities are expensive, highly complex, and often struggle with transparent materials (like clear glass vs. clear plastic) or dirty surfaces. I chose this project to explore a cost-effective, robust alternative: acoustic sorting.

Every material has a unique resonant frequency and acoustic "fingerprint." By relying on sound rather than sight, we bypass the limitations of optical sensors. Additionally, this project serves as an excellent sandbox for learning about edge computing, as it perfectly blends hardware actuation, digital signal processing (DSP), and embedded machine learning all in one integrated Rust-based environment.

## Architecture

- Acoustic Acquisition Subsystem: A piezoelectric disc acts as a contact microphone, triggering an ADC capture on the microcontroller the exact moment an impact occurs, recording a rapid, 1-second burst of raw audio data.

- Signal Processing Engine: The raw time-domain audio data is transformed into the frequency domain (a spectrogram representation) using the rustfft library, extracting the distinct acoustic features of the drop.

- Classification Engine: The extracted frequency features are fed into a trained machine learning model (powered by the smartcore library) which outputs a prediction and a confidence score.

- Human-in-the-loop Override: A state machine checks the confidence score. If it falls below the required threshold, automation halts, and the system waits for manual input from a 4-button array to categorize or dispose of the tricky item.

- Display System: A 1602 I2C character LCD provides real-time feedback, showing the active prediction (e.g., "Metal - 94%") and prompting the user when manual intervention is required.

- Actuation System: A PWM-driven SG90 micro servo is attached to a guiding funnel. It physically rotates to the appropriate angle to drop the item into the Glass, Plastic, or Metal bin.

## Log

## Hardware

The project utilizes a Raspberry Pi Pico 2 as the main microcontroller, leveraging its powerful RP2350 architecture to handle digital signal processing and machine learning inference at the edge. A piezoelectric disc is taped to the impact plate and connected to one of the Pico's ADC pins to measure the vibration and sound of the falling object. A 1602 character LCD module, connected via the I2C interface, serves as the user interface to display the classification results and confidence levels. An SG90 micro servo, controlled via a PWM pin, mechanically routes the objects into their respective physical bins. Four momentary push buttons are connected to digital GPIO pins with internal pull-up resistors to facilitate manual human-in-the-loop overrides. All electronic components are interfaced using a breadboard and standard jumper wires, while the mechanical ramp and sorting bins are constructed from 3D-printed and cardboard materials.

### Schematics



### Bill of Materials
| Device | Usage | Price |
|--------|--------|-------|
|Raspberry Pi Pico 2|"Main microcontroller board handling DSP, ML, and hardware control"| - 
|1602 I2C LCD|Displays the predicted material type and ML confidence score|-
|Piezoelectric Disc|Contact microphone used to capture the impact sound|-
|SG90 Micro Servo|Actuator that rotates the funnel to sort items into bins|-
|Push Buttons (x4)|Input for manual material classification and disposal|-
|Breadboard & Wires|Prototyping connections|-
|Ramp & Impact Plate|Physical structure for dropping and funneling items|-

## Software

| Library / Interface | Description | Usage |
|---------------------|-------------|-------|
|rustfft|High-performance Fast Fourier Transform library|Used to convert the raw time-domain audio from the piezo sensor into frequency-domain data for feature extraction.
|smartcore|Machine learning library for Rust|"Used to train and run the classification model (e.g., Random Forest) that predicts the material type based on the acoustic signature."



[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/yNS7YZsE)
