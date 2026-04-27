# Automotive Black Box
A black box that logs data on an SD card if the RC car has been in an accident.

:::info 

**Author**: Barbu Alexandru Daniel \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-AlexandruDanielBarbu

:::

<!-- do not delete the \ after your name -->

## Description

The project will do 2 things:
1. Detect if the RC car was involved in a crash
2. If a crash had occurred log data such as acceleration, speed etc on an SD card.

## Motivation

I am passionate about cars. Many of my solo projects had involved cars from modeling to simulating the forces that act on a car.

## Architecture 

Add here the schematics with the architecture of your project. Make sure to include:
 - i will do a Kalman filter so i need a crate for handling math and matrix multiplication
 - i will have to do some custom graphs to determine when an accident happened so another rust crate there
 - from what i understand i also need a filter that stabilizes the signal from the IMU sensor

<!-- TODO:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other -->

## Log

<!-- write your progress here every week -->

### Week 23 Feb - 29 Mar

Looking for an idea, validating idea, idea is too simple so back to square 1.

Asking people for problems that my project could solve, I am not satisfied with the ideas.

Finding out about back boxes and how they record data for a plane crash, adapting the idea for a smaller scope so i chose a car instead of a plane.

Validating idea with the course staff, idea is good.

Looking for components, finding components, ordering them, they get delivered to the wrong address, waiting for components some more time.

### Week 30 Mar - 26 Apr

Doing small scale beginner project on the board to get the feel of how to program in rust on the STM board.

Waiting for the rest of the components to be delivered.

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Components:
 - IMU 6 axis
 - MicroSD card module
 - STM board (provided by the university)
 - buzzer
 - LED
 - wires
 - breadboard
 - power supply 4 AA batteries

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32U545](https://www.st.com/resource/en/user_manual/um3062-stm32u3u5-nucleo64-boards-mb1841-stmicroelectronics.pdf) | The microcontroller | [113 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D&utm_id=6470900573&utm_source=google&utm_medium=cpc&utm_marketing_tactic=emeacorp&gad_source=1&gad_campaignid=6470900573&gbraid=0AAAAADn_wf1J6XpRotkoYj96_ZbUSaPnH&gclid=Cj0KCQjw77bPBhC_ARIsAGAjjV9JETny_HVaRTMCWUsjLF5mX_nrK4cA6P9VX1bEVQVYmCTCGeIwhOAaAlZUEALw_wcB) |
| [MPU-6500 6 axe](https://sigmanortec.ro/Modul-Accelerometru-Giroscop-I2C-MPU-6500-6-axe-p136248782) | Accelerometer and gyroscope for crash detection | 12 RON * 2 |
| [Suport baterii AA, 4AA](https://sigmanortec.ro/Suport-4-baterii-AA-cu-capac-si-intrerupator-p172447738) | Power up the project | 7 * 2 RON |
| [Modul MicroSD](https://sigmanortec.ro/Modul-MicroSD-p126079625) | For live telemetry | 5 * 2 RON |
| LED | Visual effect of the crash | - |
| Buzzer | Auditorial effect of the crash | - |
| Wires | - | - |
| Breadboard | Link components together | - |



> [!NOTE]
> I bought 2 of each part just in case one breaks.

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

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Volvo car crash NCAP test](https://www.youtube.com/watch?v=jIuPI1k_kVU&t=10s)
2. [What is a black box](https://www.google.com/search?q=what+is+a+black+box&rlz=1C1GCEA_enRO1076RO1076&oq=what+is+a+black+box&gs_lcrp=EgZjaHJvbWUyCQgAEEUYORiABDIHCAEQABiABDIHCAIQABiABDIHCAMQABiABDIHCAQQABiABDIICAUQABgWGB4yCAgGEAAYFhgeMggIBxAAGBYYHjIICAgQABgWGB4yCAgJEAAYFhge0gEJMTQ5ODNqMGo0qAIAsAIA&sourceid=chrome&ie=UTF-8)
3. [Kalman filter - 1](https://kalmanfilter.net/)
4. [Kalman filter - 2](https://www.youtube.com/watch?v=IFeCIbljreY)
