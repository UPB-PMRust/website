# Mini Air Defence System

An automated miniature turret system that monitors the environment via ultrasonic scanning and engages detected targets using automated or semi-automated firing modes.

:::info 
**Author**: Vlad-Ştefan CONDREA  
**Laboratory Assistant**: Irina Bradu  
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-vladcondrea
:::

## Description

My project consists of a mini air defense system that continuously monitors the surrounding space using ultrasonic sweeping. The system processes distance data in real-time to identify intrusions within its perimeter and reacts by orienting a launcher towards the target. 

The device operates in two distinct modes:
1. **Semi-automatic mode**: The system detects the target, calculates the necessary coordinates, and aligns the launcher on the target's axis. It then waits for an external command (a laptop keystroke) transmitted via UART to initiate the firing sequence.
2. **Automatic mode**: The internal processing unit identifies the target's trajectory and immediately aligns the launcher. Once aligned, it sequentially activates the propulsion motors and the launch mechanism without requiring human intervention.

To ensure precision and stability, the control mechanism uses the STM32's advanced hardware timers to generate precise PWM signals for the exact positioning of the servomotors. Furthermore, I have implemented debouncing algorithms and software filtering for the ultrasonic sensor data. This filters out the inherent noise and compensates for the 15-30 degree measurement cone of the sensor, allowing the system to approximate the true center of the target rather than firing blindly at the edge of the detected cone.

## Motivation

I chose this project because it perfectly combines complex hardware architecture with critical real-time software processing. It tackles real-world engineering challenges such as power distribution management, sensor noise filtering, and latency reduction in serial communication. Building a mechatronic system from scratch allows me to deeply understand the timing constraints of STM32 microcontrollers and how to seamlessly integrate peripherals like UART, PWM timers, and external interrupts (EXTI).

## Architecture 

![Architecture](./architecture.webp)

## Log

### Week 20 - 27 May
* Conducted research on the necessary components and their compatibility, specifically focusing on the STM32 microcontroller's timer capabilities for PWM generation.
* Set up the development environment using STM32CubeIDE and configured the clock tree and peripheral initializations using STM32CubeMX.
* Analyzed the power consumption risks. I realized that the servomotors draw high peak currents on startup, which would cause voltage drops and continuously reset the STM32 if powered from the same source. Decided to use a separate power supply and a buck converter for the motors.


## Hardware

* **STM32 Microcontroller** – The main control board processing sensor data, managing interrupts, and controlling outputs.
* **Ultrasonic Sensor (HC-SR04)** – Performs the spatial sweep and measures distance to targets.
* **Pan/Tilt Servomotors** – Orients the launcher on the X and Y axes.
* **DC Propulsion Motors** – Spins the flywheels to launch the projectile.
* **Dual Motor Driver Module L298N** – Controls the high-current DC propulsion motors.
* **Step-down Buck Converter (LM2596)** – Regulates a separate, stable power supply for the motors to prevent STM32 brown-out resets.
* **External Power Supply (LiPo Battery)** – Provides sufficient current for the entire system.

![Hardware](image1.webp)
![Hardware](image2.webp)
![Hardware](image3.webp)

### Schematics

![Schematic](schematic.svg)

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| [STM32 Development Board](https://www.optimusdigital.ro/ro/placi-stm/123-placa-de-dezvoltare-stm32f103c8t6.html) | The main microcontroller | ~25.00 RON |
| [HC-SR04 Ultrasonic Sensor](https://www.optimusdigital.ro/ro/senzori-ultrasonici/13-senzor-ultrasonic-hc-sr04.html) | Target detection and distance measurement | ~6.50 RON |
| [MG996R Servomotors (x2)](https://www.optimusdigital.ro/ro/motoare-servomotoare/272-servomotor-mg996r.html) | Pan and tilt positioning | ~50.00 RON |
| [DC Motors (x2)](https://www.optimusdigital.ro/ro/motoare-de-curent-continuu/85-motor-de-curent-continuu-3v-6v.html) | Projectile propulsion | ~15.00 RON |
| [Dual Motor Driver L298N](https://www.optimusdigital.ro/ro/drivere-de-motoare-cu-perii/145-driver-de-motoare-dual-l298n.html) | Driving the DC motors | ~10.99 RON |
| [LM2596 Buck Converter](https://www.optimusdigital.ro/ro/surse-de-alimentare-coboratoare/66-modul-sursa-coboratoare-lm2596s.html) | Power regulation for motors | ~8.50 RON |

## Software

| Library / Tool | Description | Usage |
|---|---|---|
| **STM32CubeIDE** | Integrated Development Env. | The primary IDE used for writing, compiling, and debugging the C code. |
| **STM32 HAL** | Hardware Abstraction Layer | Core library generated by CubeMX for configuring peripherals and clocks. |
| **TIM (Timers)** | Hardware Timers / PWM | Generating precise PWM signals for servo positions and measuring sensor echoes. |
| **UART** | Universal Asynchronous Rx/Tx | Handling the communication between the PC and STM32 for the semi-auto mode. |
| **EXTI** | External Interrupts | Triggering immediate actions based on specific sensor inputs or button presses. |

## Links

1. [STM32 Timer and PWM Tutorial](https://controllerstech.com/pwm-in-stm32/)
2. [Interfacing HC-SR04 with STM32](https://controllerstech.com/interface-ultrasonic-sensor-hc-sr04-with-stm32/)
3. [Managing Servomotor Current Spikes](https://forum.arduino.cc/t/servo-motor-causing-reset/)