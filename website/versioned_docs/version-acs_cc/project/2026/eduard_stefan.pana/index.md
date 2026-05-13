# NucleoPod: High-Fidelity Digital Audio Player

:::info

**Author:** Eduard-Ștefan Pană  \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/acs-project-2026-editheman

:::

---

## Description

NucleoPod is a standalone, portable digital audio player inspired by classic media players. Built around the STM32U545RE microcontroller and programmed entirely in Rust, the device reads audio files from an external MicroSD card, applies software-based Digital Signal Processing (DSP) for a 3-band equalizer, and outputs high-fidelity audio via an I2S DAC. It features a capacitive touch interface (click-wheel style) for navigation and a color TFT display for a hierarchical user interface (UI). The system is fully portable, powered by a rechargeable Li-Po battery circuit.

## Motivation

The motivation behind this project is rooted in a childhood dream. Growing up, I always wanted an original iPod, but I never had the opportunity to own one. Now that the iconic device has been officially discontinued, I decided to fulfill that wish by engineering my own functional tribute from scratch. Beyond this personal fulfillment, the project serves as a comprehensive technical challenge. Recreating the seamless experience of a classic media player allows me to dive deep into the embedded Rust ecosystem (specifically the Embassy async framework), master real-time Digital Signal Processing (DSP), and handle complex hardware synchronization (I2S, SPI, I2C) required for high-fidelity audio playback.

## Architecture

The software architecture is entirely asynchronous, built on the Embassy executor. It uses several concurrent tasks coordinated through async channels:

* **Audio Task:** Runs on TIM6 hardware interrupt at the WAV file's sample rate. The interrupt handler reads samples directly from a buffer and writes them to the internal DAC registers via PAC. This provides true hardware-driven audio output independent of CPU scheduling.
* **SD Reader Task:** Reads audio data from `.wav` files via SPI2 in 8KB chunks, using a double-buffering (ping-pong) technique to ensure continuous playback without underruns.
* **UI & Input Task:** Polls the MPR121 capacitive touch sensor via I2C1, detects click-wheel gestures (scroll up/down, select), and updates the menu state machine. Pushes UI updates to the TFT display over SPI1.
* **Haptic Task:** Triggers the vibration motor via GPIO on each scroll event to provide tactile feedback.

## Block Diagram

* **Power Subsystem:** External 5V Power Bank ➔ STM32 Nucleo USB-C port (`5V` rail).
* **Processing:** STM32U545RE (Nucleo-64), clocked at 160MHz via PLL.
* **Inputs:**
  * MicroSD Card Module ➔ connected via SPI2 (PB13/PB14/PB15, CS on PB5).
  * MPR121 Capacitive Touch Sensor ➔ connected via I2C1 (PB6/PB7).
  * Tactile push-button (center select) ➔ connected to GPIO PB4.
* **Outputs:**
  * Internal 12-bit DAC (DAC1_OUT1 on PA4) ➔ RC filter (100Ω + 100nF) ➔ 3.5mm Audio Jack.
  * TFT LCD Display (ILI9341, 320x240) ➔ connected via SPI1 (PA5/PA6/PA7, DC/RST/CS on PC6/PC7/PC9).
  * Vibration motor module (haptic feedback) ➔ connected to GPIO PB10.

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
* **Week May 4 - May 10:**
  * Verified Nucleo board power rails and tested each component individually.
  * Successfully integrated and tested the ILI9341 display via SPI1 (with mirror correction via `flip_horizontal`).
  * Validated SD card communication on SPI2 and verified FAT32 file listing.
  * Tested MPR121 touch sensor on I2C1 — confirmed touch detection by reading raw electrode capacitance values.
  * Tested vibration motor on GPIO PB10 for haptic feedback.
  * Validated push-button input on PB4 with internal pull-up.
* **Week May 11 - Present:**
  * Discovered that SAI/I2S peripheral pins are not exposed on Nucleo-64, blocking the PCM5102A I2S path. Pivoted to using STM32's internal DAC on PA4.
  * Resolved DAC clock configuration by routing it through the LSE oscillator and increasing system clock to 160 MHz via PLL.
  * Built initial WAV playback prototype using a single-task busy-wait loop. Identified audio glitches caused by SD card read latency interleaved with sample output.
  * Implemented double-buffering (ping-pong) between two 8 KB / 16 KB / 32 KB buffers.
  * Tested an Embassy async task split (audio task + SD task) with a `Channel`, but confirmed that Embassy's cooperative scheduling on a single core could not eliminate the interruption while `embedded-sdmmc` performs blocking SPI transfers.
  * Currently implementing TIM6 hardware interrupt for audio output via PAC, decoupling sample timing from the async executor and allowing the main task to handle SD I/O without affecting audio continuity.

## Hardware Overview

The core of the system is the **STM32 Nucleo-64 (STM32U545RE)**, chosen for its ARM Cortex-M33 core with hardware FPU, built-in 12-bit DAC, and good Embassy support. The system clock is configured at 160 MHz using HSI + PLL to provide enough processing headroom for parallel SD reads and audio output. The hardware design is modular: each peripheral (display, SD, touch, motor) is on its own breakout board, connected via standard SPI/I2C/GPIO buses.

### Schematics

![Schematic](./images/edi_schematic.webp)

## Components

* **STM32U545RE Nucleo-64:** Main microcontroller and development board.
* **Internal 12-bit DAC (PA4):** Generates analog audio signal, AC-coupled via 100Ω + 100nF to the headphone jack.
* **PCM5102A Module (jack reuse only):** Provides the 3.5mm headphone jack and AGND reference; its I2S DAC chip is unused.
* **MicroSD SPI Module:** Mass storage for `.wav` audio files (FAT32 formatted, max 32 GB SDHC).
* **2.4" Color TFT LCD (ILI9341, 320x240):** Displays the menu UI, track list, and playback status.
* **MPR121 Touch Sensor:** Reads capacitive touch on copper-tape electrodes arranged in a circle, simulating a click-wheel.
* **Tactile Push-Button (6x6mm):** Center "Select / Play-Pause" button.
* **Vibration Motor Module:** Provides haptic feedback on scroll events.
* **5V USB Power Bank:** Portable power source.

## Bill of Materials (Hardware)

1. 1x STM32 Nucleo-64 Development Board (STM32U545RE)
2. 1x PCM5102A Module (used only for its 3.5mm jack)
3. 1x MicroSD Card Reader Module (SPI, 5V supply, 3.3V logic) + MicroSD card formatted FAT32 GroundStudio
4. 1x 2.4" Color TFT LCD Display (ILI9341 driver)
5. 1x MPR121 Capacitive Touch Module
6. 1x Tactile push-button 6x6mm (THT)
7. 1x Vibration motor module (3.0–5.3V DC, ~60 mA)
8. 1x USB Power Bank (5V output)
9. 2x 100Ω resistors, 2x 100nF (104) ceramic capacitors for DAC output coupling
10. 1x Breadboard (830 tie-points) and assorted Dupont jumper wires (M-M, M-F)
