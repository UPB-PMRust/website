# Gamepad-Controlled GVS System
A system that uses a PS4 controller to control safe galvanic vestibular stimulation in real time.

:::info 

**Author**: Oprea Diana-Georgiana \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-dudisorr

:::

<!-- do not delete the \ after your name -->

## Description

This project aims to develop a system that allows real-time control of a Galvanic Vestibular Stimulation (GVS) setup using a PS4 controller.
The system reads controller input on a PC, processes it, and sends commands via Bluetooth to an STM32 microcontroller, which generates control signals for an analog circuit. This circuit produces a low, controlled current applied through two symmetrically driven electrodes to influence the user’s perception of movement and balance.
In addition, the system integrates a heart rate sensor and an LCD display that shows the measured pulse in real time. This allows monitoring of the user’s physiological response during stimulation, making it possible to observe whether changes in balance or perceived movement lead to increased heart rate, which can indicate stress or nervousness.

## Motivation

I chose this project because I am very interested in how technology can interact with human perception, especially in systems that directly influence the way we experience movement and balance.
The idea started when I saw an online video of someone actually experiencing Galvanic Vestibular Stimulation (GVS), where electrical stimulation was used to create the sensation of movement or tilt. Seeing a real person react to it made the concept much more tangible and interesting, so I started researching it in more detail.
While exploring the topic, I discovered that GVS is used not only in immersive technologies, but also in research and biomedical applications, such as studying the vestibular system, balance control, and human perception. This made the project more interesting to me, as it is not just a technical system, but something that directly interacts with the human body.
At the same time, I am also interested in biomedical devices and how electrical signals can be used safely to influence or measure physiological processes. This project combines these aspects, as it involves generating controlled electrical stimulation that affects perception, while requiring careful design to ensure safety.
Overall, this project allows me to explore the intersection between embedded systems, analog electronics, and biomedical engineering, while working with a concept that has both experimental and real-world applications.

## Architecture 

![Architecture Diagram](./diagram.svg)

## Log

<!-- write your progress here every week -->

### Week 1
- Discovered the project idea after seeing an experiment with Galvanic Vestibular Stimulation (GVS)

### Week 2 - 4
- Researched GVS in more detail
- Looked into possible applications and limitations
- Focused on safety aspects and current limits (3–4 mA range)
- Identified main challenges of the project

### Week 5 - 7
- Selected and ordered all required components
- Planned overall system architecture
- Reviewed datasheets for main components

### Week 8 - 9
- Started designing the current regulation circuit for the stimulation stage
- Studied different methods for limiting and stabilizing current
- Continued learning about GVS behavior and constraints

## Hardware

- Microcontroller: STM32U545RE Nucleo (core logic)
- Communication: HC-05 Bluetooth module (wireless UART communication)
- Analog Circuit: 2N3904 (NPN) and 2N3906 (PNP) transistors (current control)
- Power Supply (Analog): 4xAA battery pack + XL6009 boost converter
- Power Supply (Digital): Separate battery pack for STM32 and peripherals
- Sensor: Heart rate sensor (pulse monitoring)
- Display: LCD screen (real-time heart rate display)
- Input Device: PS4 controller (user input via PC)
- Connections: Jumper wires, connectors, electrodes (E1, E2)

### Schematics

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE-Q](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D) | Main microcontroller | 144 RON |
| [HC-05 Bluetooth Module](https://www.emag.ro/modul-bluetooth-hc-05-3874784221244/pd/DMZGZKYBM/) | Wireless communication (PC ↔ STM32) | 48 RON |
| [XL6009 DC-DC Boost Converter (x2)](https://www.emag.ro/modul-dc-dc-boost-xl6009/pd/DR9P6GBBM/) | Voltage boost for analog circuit and power management | 36 RON |
| [4xAA Battery Holder (x2)](https://www.emag.ro/suport-4-baterii-aa-cu-comutator-r6-6v-negru-rosu-2-a-003/pd/D4YGKLMBM/) | Power supply for analog and digital sections | 34 RON |
| [Transistor Kit (2N3904 / 2N3906 included)](https://www.emag.ro/kit-tranzistoare-aystoaysr-to-92-cu-cutie-de-depozitare-multicolor-ays-351/pd/DWZL503BM/) | Analog stimulation circuit components | 34 RON |
| [Resistor Kit](https://www.emag.ro/set-600-rezistori-sinbinta-set-rezistori-1-4w-10-1m-30-tipuri-toleranta-1-20-de-bucati-pentru-fiecare-valoare-ideal-pentru-proiecte-electronice-si-ingineri-metal-130-22-80mm-multicolor-ww-462/pd/DGZ93K3BM/) | Biasing and current control | 45 RON |
| [3.5mm Jack Terminal Adapter (Goobay 76746)](https://www.emag.ro/adaptor-bloc-terminal-cu-3-pini-tip-regleta-cu-surub-la-jack-3-5-mm-mama-3-pini-stereo-goobay-76746/pd/D65B7H3BM/) | Connection interface for electrodes | 16 RON |
| [Electrode Cables (3.5mm)](https://www.emag.ro/set-2-cabluri-conexiune-jack-3-5mm-pentru-electrozi-snap-on-3-5mm-compatibile-cu-aparatele-electrostimulare-tens-si-ems-lungime-150-cm-culoare-alb-cbl2/pd/DZQL9RYBM/) | Connection between circuit and electrodes | 31 RON |
| [Electrodes (TENS pads)](https://www.emag.ro/set-20-electrozi-paduri-cu-gel-pentru-aparat-electrostimulare-reutilizabili-alb-21-tens-pads/pd/DYZ39TYBM/) | GVS stimulation interface | 35 RON |
| [Heart Pulse Sensor (AI206)](https://www.emag.ro/modul-senzor-puls-cardiac-ai206-s104/pd/DXX2YRBBM/) | Heart rate monitoring | 15 RON |
| [LCD 16x2 with I2C](https://www.emag.ro/display-lcd-2-x-16-cu-convertor-i2c-80-x-35-mm-verde-albastru-negru-2-e-001/pd/DHRJ0LMBM/) | Real-time heart rate display | 23 RON |
| **Total** |  | **461 RON** |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [gilrs](https://github.com/Arvamer/gilrs) | Gamepad input library for Rust | Used to read input from the PS4 controller on the PC side |
| [serialport](https://github.com/serialport/serialport-rs) | Serial communication library for Rust | Used to send data from the PC application to the HC-05 Bluetooth module |
| [tokio](https://github.com/tokio-rs/tokio) | Asynchronous runtime for Rust | Used to manage asynchronous tasks in the PC application |
| [serde](https://github.com/serde-rs/serde) | Serialization framework for Rust | Used to structure and serialize control data sent to STM32 |
| [egui](https://github.com/emilk/egui) | GUI library for Rust | Used to create the PC-side interface |
| [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) | Application framework for egui | Used to run the desktop application |
| [embassy-stm32](https://github.com/embassy-rs/embassy) | STM32 support for Embassy (Rust embedded framework) | Used for hardware control on the STM32 |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor for embedded Rust | Used to manage concurrent tasks on STM32 |
| [embassy-time](https://github.com/embassy-rs/embassy) | Timing utilities for embedded systems | Used for delays and scheduling |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer | Used for interfacing with peripherals (GPIO, UART, etc.) |
| [defmt](https://github.com/knurling-rs/defmt) | Logging framework for embedded systems | Used for debugging on STM32 |
| [panic-probe](https://github.com/knurling-rs/panic-probe) | Panic handler for embedded Rust | Used for error handling |
| [hd44780-driver](https://github.com/JohnDoneth/hd44780-driver) | Driver for HD44780-compatible LCD displays | Used to control the 16x2 LCD display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Galvanic Vestibular Stimulation (GVS) - Wikipedia](https://en.wikipedia.org/wiki/Galvanic_vestibular_stimulation)  
2. [Scientific study on GVS and balance control](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC3449488/)  
3. [STM32U545 Datasheet (STMicroelectronics)](https://www.st.com/resource/en/datasheet/stm32u545ce.pdf)  
4. [Inspiration video – GVS experiment](https://www.instagram.com/p/DUjTwyxiH6j/) 
...
