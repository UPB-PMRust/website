# Automotive Black Box
A black box that logs data on an SD card if the RC car has been in an accident.

:::info 

**Author**: Barbu Alexandru Daniel \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-AlexandruDanielBarbu

:::

## Description

This project serves two main purposes:
1. Detecting whether the RC car was involved in a crash.
2. Logging telemetry data, such as acceleration and orientation in space, onto an SD card in the event of an accident.

## Motivation

I am deeply passionate about cars. Many of my previous solo projects have revolved around automotive engineering, ranging from 3D modeling to simulating the physical forces that act upon a vehicle in motion.

## Architecture 

Project architecture schematic:
[draw.io schema of the project](docs/schema-block-black-box.drawio.webp)

The core of the system is a Kalman filter that performs sensor fusion between the accelerometer and the gyroscope. To eliminate noise, the filter records multiple readings over a 2-second interval, stabilizing the raw IMU signal before processing acceleration. Because these algorithms require complex linear algebra, a dedicated Rust mathematics crate handles all matrix multiplications.

Building upon this stabilized data, the system compares deceleration and impulsewith some thresholds to accurately detect an accident. The exact crash threshold was determined empirically by running test measurements, plotting the results, and analyzing the data. Once a crash is detected, the system immediately logs the car's acceleration and orientation to a CSV file on the SD card.

Additionally, the hardware includes two physical buttons to streamline testing and demo reruns: one safely ejects the SD card to prevent data corruption, while the other resets the IMU state to eliminate gyroscope drift and ensure a fresh start.

Below is the state machine diagram of the project:
[Image](docs/state-machine-release.drawio.webp)

Noteworthy graphs of my test runs can be found in the `README.md` of the software side of the project.

## Log

### Week 23 Feb - 29 Mar

Started by brainstorming and validating initial ideas. The first concept was too simple, so I went back to square one.

I then asked peers about problems my project could solve, but I was not satisfied with the suggestions.

Eventually, I researched aviation black boxes and how they record flight data during a crash. I adapted this concept for a smaller scope, choosing an RC car instead of a plane.

After validating this idea with the course staff, it was approved.

I then searched for and ordered the necessary components. Unfortunately, they were delivered to the wrong address, which caused additional delays.

### Week 30 Mar - 26 Apr

Worked on small-scale beginner projects on the STM board to get a feel for programming in Rust.

Continued waiting for the rest of the components to be delivered.

### Week 27 Apr - 10 May

The parts finally arrived at the correct address.

Researched Kalman filters. [Good information was found here](https://github.com/rlabbe/Kalman-and-Bayesian-Filters-in-Python).

Bought the RC car body and conducted further research on the specific type of Kalman filter needed, as well as how the car's chassis would affect the readings.

Wired the project hardware to the RC car.

Fixed an issue where the IMU sensor was not reading data, thanks to the lab assistants.

### Week 11 - 17 May

Added the external power supply.

Made significant progress on the software side:
- Refactored the lab code for the MPU6500 component to fit my specific needs.
- Implemented the code for the SD card logging system.
- Tested the buzzer, the two buttons, and the RGB LED.
- Tested sensor fusion and the Kalman filter implementation.
- Discovered two bugs:
  - `probe-rs` was not reading the board (caused by unplugging it from the PC at a bad time). This was eventually fixed.
  - When connecting the external power supply, the board lights up, but the code does not execute. This issue was resolved later.

### Week 18 - 24 May

Worked on the state machine for the test bench.

Sought help regarding the power supply issue and decided to use a powerbank. However, the powerbank kept shutting down due to the low current draw from the STM board. I tried plugging my phone into the second port to keep it awake, but the powerbank only had one USB-A port. To fix this, I went to Altex Orhideea and bought a 3-meter Hama data cable to keep the project wired to my laptop during tests. I expected signal degradation due to the cable's length, but it works perfectly, and the board is now properly powered.

### Week 25 - 27 May

Refactored Kalman Filter and crash detection.

Conducted several test runs to calibrate the crash detection system.

After calibration, performed additional tests to validate the new parameters.

Generated plots from the log files to analyze the gathered data.

Developed and tested the release build of the project.

Recorded the final project demonstration and posted it on YouTube [here](https://www.youtube.com/watch?v=s0vU-t5tHtw&list=PLj-KCdUICNfDgmbNKMuRmQCIzfgc7HheH&index=1).

Polished the overall project presentation.

## Hardware

Components:
- 6-axis IMU
- MicroSD card module
- STM board (provided by the university)
- Buzzer
- LED
- Wires
- Breadboard
- 4x AA battery power supply

### Schematics

[KiCad schema](docs/automotive-black-box.webp)

> [!NOTE]
> I am not entirely sure about the exact part numbers for the buttons, as they are generic hobbyist components, but I recreated them in the schematic to the best of my ability.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32U545](https://www.st.com/resource/en/user_manual/um3062-stm32u3u5-nucleo64-boards-mb1841-stmicroelectronics.pdf) | Main microcontroller | [113 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D&utm_id=6470900573&utm_source=google&utm_medium=cpc&utm_marketing_tactic=emeacorp&gad_source=1&gad_campaignid=6470900573&gbraid=0AAAAADn_wf1J6XpRotkoYj96_ZbUSaPnH&gclid=Cj0KCQjw77bPBhC_ARIsAGAjjV9JETny_HVaRTMCWUsjLF5mX_nrK4cA6P9VX1bEVQVYmCTCGeIwhOAaAlZUEALw_wcB) |
| [MPU-6500 6 axe](https://sigmanortec.ro/Modul-Accelerometru-Giroscop-I2C-MPU-6500-6-axe-p136248782) | Accelerometer and gyroscope for crash detection | 12 RON * 2 |
| [Suport baterii AA, 4AA](https://sigmanortec.ro/Suport-4-baterii-AA-cu-capac-si-intrerupator-p172447738) | Project power supply | 7 RON * 2 |
| [Modul MicroSD](https://sigmanortec.ro/Modul-MicroSD-p126079625) | Live telemetry logging | 5 RON * 2 |
| LED | Visual crash indicator | - |
| Buzzer | Auditory crash indicator | - |
| Wires | Interconnecting components | - |
| Breadboard | Prototyping circuit layout | - |
| RC car | Base chassis for the project | 130 RON |

> [!NOTE]
> Huge thanks to the lab assistants for providing detailed feedback on what hardware to buy!

> [!NOTE]
> I bought two of each component just in case one breaks.

## Software

> [!NOTE]
> These are subject to be changed.

| Library | Description | Usage |
|---------|-------------|-------|
| **embassy-stm32** | STM32 hardware driver | Controlling pins, timers, and peripherals |
| **embassy-time** | Time and delay management | Handling timeouts and periodic events |
| **embassy-sync** | Async sync primitives | Inter-task communication (Mutex, Channels) |
| **cortex-m** | Core processor access | Managing interrupts and CPU instructions |
| **cortex-m-rt** | Startup/Runtime for ARM | Initializing memory and the entry point |
| **defmt** | Low-overhead logger | Fast logging for embedded systems |
| **defmt-rtt** | RTT transport for logs | Transferring logs through debuggers |
| **embassy-embedded-hal** | HAL helper utilities | Adapting hardware traits for Embassy |
| **embassy-executor** | Async task scheduler | Running and managing async tasks |
| **embassy-futures** | Async helpers | Combining or waiting for multiple futures |
| **embassy-usb** | Async USB stack | Implementing USB device functionality |
| **embedded-hal-async** | Async hardware traits | Standard interface for non-blocking drivers |
| **panic-probe** | Debug panic handler | Reporting crashes via the probe |
| **minikalman** | `no_std` Kalman Filter | Core implementation for sensor fusion |
| **nalgebra** | General linear algebra | Handling 3D matrices and vector math |
| **micromath** | Fast embedded math | Efficient floating point (sin, cos, sqrt) |
| **ahrs** | Attitude/Heading Reference | Fusing IMU data into orientation vectors |

## Links

1. [Volvo car crash NCAP test](https://www.youtube.com/watch?v=jIuPI1k_kVU&t=10s)
2. [What is a black box](https://www.google.com/search?q=what+is+a+black+box&rlz=1C1GCEA_enRO1076RO1076&oq=what+is+a+black+box&gs_lcrp=EgZjaHJvbWUyCQgAEEUYORiABDIHCAEQABiABDIHCAIQABiABDIHCAMQABiABDIHCAQQABiABDIICAUQABgWGB4yCAgGEAAYFhgeMggIBxAAGBYYHjIICAgQABgWGB4yCAgJEAAYFhge0gEJMTQ5ODNqMGo0qAIAsAIA&sourceid=chrome&ie=UTF-8)
3. [Kalman filter - 1](https://kalmanfilter.net/)
4. [Kalman filter - 2](https://www.youtube.com/watch?v=IFeCIbljreY)
5. [KiCad tutorial](https://www.youtube.com/watch?v=vLnu21fS22s&t=551s)