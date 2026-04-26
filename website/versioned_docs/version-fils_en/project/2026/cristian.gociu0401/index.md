# WidowMaker-Pico
A 4 legged 3 degrees of freedom per leg spider robot controlled via WiFi, phone app control, capable of walking in all directions and stabilizing itself.

:::info


**Author**: Gociu Cristian \
**Github Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-gociucristian

:::

## Description

A 4 legged spider robot controlled via a Flutter mobile app over WiFi with a TCP server for real time commands. The robot uses 12 MG90S servo motors (3 per leg) controlled by a PCA9685 PWM 16 channel controller with a Raspberry Pi Pico W running Rust firmware via EMbassy-rs. The robot will have a walking pattern with inverse kinematics, will be phone controlled with joystick movement, it will have some emote animations and body pose tilting via the MPU6050 gyroscope that I will also use for body orientation sensing and auto leveling.

## Motivation

I chose this project because I always was interested in the design and mechanical movement of robots, practically I wanted to combine mechanical design with embedded programming into this project. Building a walking robot would make me learn and understand inverse kinmatics, real time control and sensor fusion, all implemented in a Rust microcontroller, also the app should add a practical and fun user interface that makes the robot interactive.

## Architecture

![Arachne Architecture](./architecture_schematic.svg)

The system consists of three main components:

1. **Pico W**: Runs Embassy-rs async tasks for WiFi, gait engine, servo control, IMU reading
2. **I2C Peripherals**: PCA9685 servo driver, MPU6050 gyroscope, all on a shared I2C bus
3. **Flutter App**: TCP client with joystick, emote buttons and body pose control

## Log

### Week 5 - 8
I brainstormed ideas, chose my desired topic and got its approval. I ordered components. Verified Pico W blink and I2C channel.

### Week 9
Downloaded Fusion, learned how to use it and designed chassis and legs.

## Hardware

### Components

The system uses a Raspberry Pi Pico W (RP2040) as the main controller and for WiFi, a PCA9685 16-channel as a servo driver, 12 MG90S servos as leg joints, a MPU6050 (GY-521) gyroscope for body orientation sensing and a 7.4V 2S LiPo battery as power source.

## Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| Raspberry Pi Pico W | Main controller | 64 RON |
| Pca9685 Servo Driver | 16 channel PWM I2C servo driver |
| MG90S Servo Motor | 3 DOF movement for walking, rotating and stabilizing | 
| MPU6050 Module (GY-521) | IMU data | 
| 7.4V 2S LiPo Battery 1000mAh | Power source |
| 2s LiPo charger | Charger for battery |
| 5V 3A UBEC | Buck converter to reduce 7.4V down to ~6V for the servos |
| Protoboard | Board to solder all the power and communication rails |
| Pin Headers (male+female) | To help soldering the board and components |
| 3D Printing Filament | Used for the custom design of my project |
| Wires, heat shrinks | Soldering on the board and power insulation |
| Total | 

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| embassy-executor | Async task executor | Runs concurent tasks |
| embassy-rp | RP2040 HAL (GPIO, I2C, PIO) | Provides low level interface for the RP2040 GPIO, I2C buses and PIO state machines |
| embassy-time | Async timers and delays | Handles timing fot servo PWM updates and delays |
| embassy-net | TCP/IP networking stack | Allows the robot to communicate with the control interface via wireless network |
| embassy-sync | Channels, signals | Safe communication between async tasks (sending movement commands from the WiFi task to the gait engine) |
| cyw43 | WiFi driver for CYW43439 | Handles the WiFi radio and data transmission |
| cyw43-pio | PIO-based SPI for CYW43 | Handles high speed SPI communication with the WiFi chip |
| pwm-pca9685 | PCA9685 I2C servo driver | Drives the 16 channel PWM controller via I2C |
| mpu6050-dmp | MPU6050 gyro/accel driver | Retrieves orientation data, used for active stabilization algorithms |
| serde-json-core | no_std JSON parsing | Parses incoming JSON packets from the WiFi control wheel into Rust structures for movement commands |
| libm | Math functions for Inverse Kinematics | Trigonometry functions (sin, cos, atan2, sqrt) required for IK |
| defmt | Efficient logging for embedded | Highly efficient logging and formatting console |
| defmt-rtt | RTT transport for defmt logs| The transporter that sends defmt logs from the Pico W to the host PC |
| panic-probe | Panic handler for probe-rs | Panic handler that catches code failures and prints the file and line number where the error occurred |

## Links

1. [Embassy](https://embassy.dev)
2. [PCA9685 product data sheet](https://www.nxp.com/docs/en/data-sheet/PCA9685.pdf)
3. [Project inspiration](https://www.youtube.com/watch?v=xd8dKY6Ozrg)
