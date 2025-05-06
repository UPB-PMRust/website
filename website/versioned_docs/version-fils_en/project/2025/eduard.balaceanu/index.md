
# Matrix LED Adaptive Headlight System


A simplified replica of modern adaptive headlight systems found in vehicles, using a matrix of LEDs, real-time sensor input, and Rust-powered control logic running on a Raspberry Pi Pico 2W.


:::info

**Author:** Eduard-Andrei Balaceanu \
**GitHub Project Link:**: https://github.com/UPB-PMRust-Students/project-eduard1506

:::


## Description

This project showcases the knowledge acquired throughout the semester by designing a functional hardware system that runs software written entirely in Rust. The system replicates a simplified version of the adaptive headlight functionality found in modern vehicles, using a matrix of LEDs controlled by a Raspberry Pi Pico 2W microcontroller.

The setup includes multiple white LEDs arranged in a grid, which dynamically turn on/off or dim based on real-time sensor input. The system uses light sensors to detect the presence and direction of oncoming lights, and a distance sensor to identify nearby objects. It also features adaptive cornering simulation using a joystick input.

The goal is to demonstrate real-time hardware control, sensor input interpretation, and modular embedded design using the Rust programming language.


## Motivation

One of my biggest passions is cars—and everything they represent. I've always loved the feeling of a late-night drive, but to truly enjoy it, you need a powerful and intelligent lighting system. Matrix LED headlights represent the pinnacle of automotive lighting technology, adapting dynamically to changing road conditions, light sources, and traffic.

I’ve been fascinated by how this system works, how it improves both safety and comfort, and why it isn’t more widely adopted across all types of vehicles. Through this project, I wanted to explore the core principles behind adaptive lighting and bring a simplified version to life using accessible components and Rust-based control logic.



## Architecture

The system architecture consists of the following functional layers:

1. **Sensor Layer**  
   - Three ambient light sensors (TEMT6000) detect the direction and intensity of incoming light sources.
   - One ultrasonic sensor (HC-SR04) measures the distance to nearby objects.

2. **Input Layer**  
   - A joystick is used to simulate vehicle steering input for adaptive cornering.

3. **Control Layer**  
   - A Raspberry Pi Pico 2W reads input data from all sensors.
   - A custom control algorithm written in Rust determines which LEDs should be turned on, off, or dimmed.

4. **Output Layer**  
   - A matrix of white LEDs is powered and controlled via IRLZ44N MOSFETs.
   - The system dims or disables individual LEDs based on sensor input to avoid blinding oncoming traffic while preserving road visibility.

5. **Feedback Layer (Planned)**  
   - A small OLED display will be used to show real-time system feedback such as active LEDs, steering direction, and sensor status.

Each component is connected using breadboards and jumper wires, with careful power and grounding design to ensure stability. The entire system operates without external Wi-Fi or cloud dependency, functioning in a fully local, embedded environment.

## Diagram


![alt text](MatrixLED.svg)

## Weekly Log

### Week 8 – May 4th

Over the past few weeks, I’ve been gathering all the necessary components for the project. After pitching my idea to the professor and getting the green light, I decided to attempt building a simplified version of the Matrix LED adaptive headlight system used in modern vehicles.

This week marked the official start of the documentation process. I’ve already experimented with wiring the Pico 2W, tested the debugger, and ran a few simple LED blink tests in Rust to confirm everything’s working. Most of the hardware is now in place — the MOSFETs, LEDs, sensors, and breadboards — though I’m still waiting on the OLED screen and joystick module.

I also started working on the architecture diagram using diagrams.net. I’m aiming to create something clean and professional, so the structure makes sense both visually and logically. Once that’s done, I’ll export it and integrate it into the site. Documentation is coming together nicely so far.


## Hardware Design

The hardware implementation is still in progress, but the core setup has already been planned and partially assembled. The project is built around a Raspberry Pi Pico 2W microcontroller, connected to a matrix of white LEDs via IRLZ44N MOSFETs.

Sensor inputs include three ambient light sensors, an ultrasonic distance sensor, and a joystick to simulate adaptive cornering input. Power is delivered through a regulated breadboard power supply, and connections are made using standard jumper wires across two breadboards.

A schematic will be added once final wiring is complete.

## Bill of Materials

| **Device**             | **Usage**                                | **Price**        |
|------------------------|------------------------------------------|------------------|
| Raspberry Pi Pico W    | The microcontroller                      | 35 RON           |
| Breadboard             | The main breadboard                      | 10 RON           |
| Light Sensors (x3)     | Measure ambient light                    | 3 x 9.99 RON     |
| Ultrasonic Sensor      | Detect object distance                   | 6.49 RON         |
| Joystick Module        | Manual control for LED direction         | 11.99 RON        |
| MOSFETs (IRLZ44N x6)   | Drive high-power LEDs                    | 6 x 4.99 RON     |
| LED Assortment         | Visual output (matrix lighting)          | 26.99 RON        |
| OLED Display (0.96")   | Diagnostic feedback                      | 23.79 RON        |




##  Software 

| **Library**       | **Description**                                                        | **Usage**                                       |
|------------------|------------------------------------------------------------------------|------------------------------------------------|
| `heapless`       | Data structure library                                                  | Used for String vectors                        |
| `embassy_rp`     | Embassy HAL for the Raspberry Pi RP2040                                 | Used to program the Pico                       |
| `embassy_sync`   | Sync primitives and async data support                                  | Communicating through tasks                    |
| `embedded-hal`   | Hardware abstraction traits                                             | Common interface for embedded peripherals      |
| `ssd1306`        | OLED display driver                                                     | Drives the OLED via I2C                        |
| `defmt`, `defmt-rtt` | Lightweight logging + RTT interface                                | Debugging via the probe                        |
| `cortex-m`       | ARM Cortex-M microcontroller support                                    | Low-level control                              |
| `cortex-m-rt`    | Runtime support                                                         | Startup and interrupts                         |
| `panic-halt`     | Panic behavior                                                          | Safe stop on crash                             |
