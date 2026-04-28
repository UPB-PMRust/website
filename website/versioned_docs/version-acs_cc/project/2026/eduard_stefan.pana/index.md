# NucleoPod: High-Fidelity Digital Audio Player

:::info
**Author:** Eduard-Ștefan Pană  
**GitHub Project Link:** [eduard_stefan.pana/website](https://github.com/editheman/website/tree/project/eduard_stefan.pana)
:::

---

## Description

NucleoPod is a standalone, portable digital audio player inspired by classic media players. Built around the STM32U545RE microcontroller and programmed entirely in Rust, the device reads audio files from an external MicroSD card, applies software-based Digital Signal Processing (DSP) for a 3-band equalizer, and outputs high-fidelity audio via an I2S DAC. It features a capacitive touch interface (click-wheel style) for navigation and a color TFT display for a hierarchical user interface (UI). The system is fully portable, powered by a rechargeable Li-Po battery circuit.

## Motivation

The motivation behind this project is rooted in a childhood dream. Growing up, I always wanted an original iPod, but I never had the opportunity to own one. Now that the iconic device has been officially discontinued, I decided to fulfill that wish by engineering my own functional tribute from scratch. Beyond this personal fulfillment, the project serves as a comprehensive technical challenge. Recreating the seamless experience of a classic media player allows me to dive deep into the embedded Rust ecosystem (specifically the Embassy async framework), master real-time Digital Signal Processing (DSP), and handle complex hardware synchronization (I2S, SPI, I2C) required for high-fidelity audio playback.

## Architecture

The software architecture is entirely asynchronous, avoiding blocking operations. It uses an async executor to run several concurrent tasks:

* **Audio Task:** Manages the DMA transfers to the I2S peripheral using a double-buffering (ping-pong) technique to ensure a continuous audio stream.

* **SD Reader Task:** Reads data chunks from the `.wav` files via SPI into RAM buffers.
* **DSP Task:** Applies Biquad filters (Low-pass, Band-pass, High-pass) utilizing the Cortex-M33 hardware Floating-Point Unit (FPU).
* **UI & Input Task:** Polls the capacitive touch sensor via I2C, updates the internal state machine for the menu, and pushes UI updates to the TFT display over SPI.

## Block Diagram

* **Power Subsystem:** 3.7V Li-Po Battery ➔ TP4056 (Charger/Protection) ➔ 5V Step-Up Boost ➔ STM32 Nucleo `5V` Pin.
* **Processing:** STM32U545RE (Nucleo-64).
* **Inputs:** * MicroSD Card Module ➔ connected via SPI.
* MPR121 Capacitive Touch Sensor ➔ connected via I2C.
* **Outputs:** * PCM5102A DAC Module ➔ connected via I2S ➔ 3.5mm Audio Jack.
* TFT LCD Display (ST7789) ➔ connected via SPI.

## Log

* **Week Mar 30 - Apr 5:** * Project ideation and initial brainstorming. Evaluated multiple concepts (e.g., smart solar tracker, automated sorter) before settling on the digital audio player (NucleoPod) concept.
* Verified the core capabilities of the STM32U545RE microcontroller to ensure it has the necessary processing power (FPU) and peripherals for audio processing.
* **Week Apr 6 - Apr 12:** * Hardware research and compatibility checks. Investigated the necessary external modules for high-fidelity audio (I2S), storage (SPI), and user input (I2C).
* Made the architectural decision to exclusively use the STM32 Nucleo board, discarding the initially proposed Raspberry Pi Pico to streamline development and focus entirely on the STM32 ecosystem.
* **Week Apr 13 - Apr 19:** * Software architecture planning and scope management. Researched the Rust `embassy-stm32` framework for handling asynchronous tasks and DMA transfers.
* Consulted with the laboratory assistant to refine the project scope.
* **Week Apr 20 - Apr 26:** * Component selection and finalization of the Bill of Materials (BoM).
* Identified specific, compatible breakout boards (PCM5102A DAC, ST7789 TFT display, MPR121 Touch sensor) and designed the theoretical portable power subsystem (Li-Po battery, TP4056 charger, and 5V Boost converter).
* **Week Apr 27 - Present:** * Drafting the official project documentation and Moodle proposal.
* Initializing the GitHub repository and setting up the basic Rust toolchain for the target architecture (`thumbv8m.main-none-eabihf`). Currently preparing to order the hardware components to begin physical prototyping.

## Hardware Overview

The core of the system is the **STM32 Nucleo-64 (STM32U545RE)**. This microcontroller was chosen for its ultra-low power capabilities, ARM Cortex-M33 core, and built-in FPU, which is essential for executing the DSP equalizer math without lagging the system. Its advanced DMA controllers and native I2S support make it ideal for high-fidelity audio streaming. The hardware design is strictly modular, separating storage, user input, graphical output, and audio generation into dedicated breakout boards.

## Components

* **PCM5102A DAC Module:** Receives digital audio data via I2S and converts it into a clean analog signal for headphones.
* **MicroSD SPI Module:** Acts as the mass storage drive for the audio files.
* **TFT LCD Display (ST7789/ST7735):** A 1.8" screen used to display the currently playing track, volume, equalizer settings, and battery status.
* **MPR121 Touch Sensor:** Reads up to 12 capacitive touch inputs. Arranged in a circular pattern, it simulates the rotational physical input of a classic media player click-wheel.
* **Power Subsystem:** Ensures the board and modules receive a stable 5V (stepped down to 3.3V by the Nucleo's internal regulator) while providing safe charging for the lithium battery.

## Bill of Materials (Hardware)

1. 1x STM32 Nucleo-64 Development Board (STM32U545RE)
2. 1x PCM5102A I2S Audio DAC Module (with 3.5mm Jack)
3. 1x MicroSD Card Reader Module (SPI logic level 3.3V) + MicroSD Card
4. 1x 1.8" Color TFT LCD Display (ST7789 or ST7735 driver)
5. 1x MPR121 Capacitive Touch Module
6. 1x 3.7V Li-Po Battery (Flat pouch, approx. 1000mAh - 1500mAh)
7. 1x TP4056 Battery Charging Module (with battery protection circuits)
8. 1x 5V Step-Up (Boost) Converter Module
9. 1x Breadboard (830 tie-points) and assorted Dupont jumper wires (M-M, M-F)
