# Ferris Goes Vroom

A fast autonomous line-following car

**Author:** Sorana-Ioana Ulmeanu

**GitHub Project Link:** https://github.com/UPB-PMRust-Students/acs-project-2026-soranaulm

## Description

Ferris Goes Vroom is an autonomous line-following car that tracks a black line on a white surface, aiming for the fastest possible lap time. The car uses a row of IR sensors mounted under the chassis to detect the line position, and a PID controller continuously adjusts the speed of the two DC motors to stay on track and minimize lap time. A small OLED display shows real-time information such as current speed and system state.

What sets this project apart is the use of **Rust** with the **Embassy** async framework on an STM32 microcontroller — an uncommon choice in the embedded world, but one that brings memory safety and high performance. A set of colored LEDs provides real-time visual feedback about the system state: line detected, line lost, or maximum speed reached.

## Motivation

Growing up, I always wanted a remote control car. I loved watching them speed around and dreamed of having one
of my own. So when this project came along, I saw the perfect opportunity to finally get my car, just built by my own hands this time.

What makes this project special to me is how interactive and alive it feels. Watching a small car zoom along
a track on its own, reacting to the world around it in real time, feels almost like it has a personality.
There is something deeply satisfying about building something that moves, that responds, that feels human in
its own little way.

## Architecture

The system is organized into four main modules that run concurrently as Embassy async tasks:

**Sensor Subsystem:** Three to five TCRT5000 IR sensors are mounted transversally under the car. They read the surface reflectivity and return digital values depending on whether they detect the black line or the white background. The combined readings are used to compute the line position error relative to the car's center.

**PID Control Engine:** The control engine receives the error value from the sensor subsystem every few milliseconds and computes a correction using proportional, integral, and derivative terms. This correction is applied differentially to the two motors: if the car drifts right, the left motor speeds up and the right motor slows down, and vice versa. The three PID constants (Kp, Ki, Kd) are tuned experimentally on the real track.

**Motor Drive Subsystem:** An L298N dual H-bridge driver receives PWM signals from the STM32 and translates them into motor voltages. The two DC motors are independently controlled, allowing the car to steer by varying the speed difference between left and right.

**LED Feedback Module:** Three LEDs connected to GPIO output pins indicate the current system state. Green lights when the line is detected and tracking is active. Red lights when the line is lost and the car is searching. Blue lights when the car is running at maximum speed.

**Display Module:** A 0.96" SSD1306 OLED display connected via I2C shows real-time information including current speed, system state (tracking / line lost), and lap time.

### Architecture Diagram

```
+--------------------------------------------------+
|              STM32 Nucleo-U545RE-Q               |
|                                                  |
|   +------------------+   +--------------------+  |
|   |  PID Controller  |   |  LED Feedback Task |  |
|   |  (error → PWM)   |   |  (state → GPIO)    |  |
|   +--------+---------+   +---------+----------+  |
|            |                       |             |
+------------|------------------------|-------------+
             |                       |
             v                       v
   +------------------+       +--------------+
   |  L298N Motor     |       |  LEDs x3     |
   |  Driver (PWM)    |       |  Green/Red/  |
   +--------+---------+       |  Blue        |
            |                 +--------------+
     +------+------+
     v             v     +------------------+
 +--------+   +--------+ |  OLED SSD1306    |
 | Motor  |   | Motor  | |  (I2C)           |
 | Left   |   | Right  | +------------------+
 +--------+   +--------+
     ^
     |
 +------------------+
 | IR Sensors       |
 | TCRT5000 x3-5    |
 | (GPIO Input)     |
 +------------------+
```

### Communication Protocols

| Protocol | Usage |
|----------|-------|
| **GPIO Input** | Reading IR sensors, computing line position |
| **GPIO Output** | Controlling LEDs |
| **ADC** | Reading analog IR sensor values for precise line position |
| **PWM (TIM)** | Controlling motor speed via L298N |
| **I2C** | Communication with SSD1306 OLED display |

## Log

### Week 1
Defined the project concept and architecture. Researched and selected the hardware components needed for the build. Started working on the project documentation.

### Week 2
*(coming soon)*

### Week 3
*(coming soon)*

### Week 4
*(coming soon)*

## Hardware

The project is built around the **STM32 Nucleo-U545RE-Q**, featuring an ARM Cortex-M33 core running at 160MHz with 256KB of SRAM and 2MB of Flash, and an integrated ST-LINK/V3E debugger for easy flashing and RTT logging from Rust.

Three to five **TCRT5000 IR reflectance sensors** are mounted in a row under the front of the chassis. Each sensor emits infrared light and reads back the reflection — black surfaces absorb IR (digital LOW) and white surfaces reflect it (digital HIGH). The combined pattern of readings is used to compute the signed error of the line position relative to the car center.

An **L298N dual H-bridge motor driver** receives PWM signals from two STM32 timer channels and independently drives the two **DC motors with gearboxes**. The speed difference between the motors is the steering mechanism — no servo is needed.

Three **5mm LEDs** (green, red, blue) with 220Ω current-limiting resistors are connected to GPIO output pins and provide live system state feedback.

A **0.96" SSD1306 OLED display** is connected via I2C and shows real-time data: current speed, system state, and lap time.

A **LiPo 7.4V battery** powers the system autonomously. The L298N has a built-in 5V regulator that powers the logic side, while the motors are driven directly from the battery voltage.

The car chassis is either a commercial kit or a custom 3D-printed frame that holds all components in a compact and balanced layout. All components are connected on a breadboard with jumper wires.

## Schematics

![Schematic](Schematic_pm.svg)

## Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| STM32 Nucleo-U545RE-Q | Main microcontroller | Provided by university |
| TCRT5000 IR Sensor x5 | Line detection | ~15 RON |
| L298N Motor Driver | Dual DC motor control | ~15 RON |
| DC Motor with gearbox x2 | Car propulsion | ~30 RON |
| OLED Display SSD1306 0.96" | Real-time info display | ~20 RON |
| LED 5mm x3 (green, red, blue) | Visual feedback | ~3 RON |
| Resistors 220Ω x3 | LED current limiting | ~1 RON |
| LiPo Battery 7.4V | Autonomous power supply | ~40 RON |
| Car chassis kit | Mechanical structure | ~35 RON |
| Breadboard + jumper wires | Prototyping connections | ~15 RON |
| **Total** | | **~174 RON** |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| `embassy` | Async runtime for embedded Rust | Task management and async executor |
| `embassy-stm32` | Embassy HAL for STM32 | GPIO, PWM (TIM), I2C control |
| `embassy-time` | Timer abstractions | Delays and PID loop timing |
| `embassy-sync` | Async synchronization primitives | Coordinating sensor, PID, display and LED tasks |
| `embedded-hal` | Hardware Abstraction Layer | Generic hardware interfaces |
| `ssd1306` | OLED display driver | Drawing text and graphics on the SSD1306 via I2C |
| `embedded-graphics` | 2D graphics library | Rendering text and shapes on the OLED display |
| `defmt` | Efficient embedded logging | Debugging via RTT |
| `cortex-m` | Low-level ARM Cortex-M utilities | Low-level control |
| `cortex-m-rt` | Runtime for ARM Cortex-M | Entry point and interrupt setup |

## Links

- [STM32 Nucleo-U545RE-Q Documentation](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)
- [Nucleo-U545RE-Q User Manual](https://www.st.com/resource/en/user_manual/um3062-stm32u3u5-nucleo64-boards-mb1841-stmicroelectronics.pdf)
- [Labs](https://embedded-rust-101.wyliodrin.com/docs/acs_cc/category/lab)
- [SSD1306 Driver Documentation](https://docs.rs/ssd1306/latest/ssd1306/)
- [embedded-graphics Documentation](https://docs.rs/embedded-graphics/latest/embedded_graphics/)
- [TCRT5000 Datasheet](https://4donline.ihs.com/images/VipMasterIC/IC/VISH/VISH-S-A0023695319/VISH-S-A0023695322-1.pdf?hkey=61A2E4C270F0397D049F8F05BD4F1054)
- [L298N Datasheet](https://4donline.ihs.com/images/VipMasterIC/IC/SGST/SGST-S-A0019207369/SGST-S-A0019207369-1.pdf?hkey=61A2E4C270F0397D049F8F05BD4F1054)
- [SSD1306 Datasheet](https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf)
