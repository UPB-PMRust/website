# Rusty Cube
A Rubik's cube solving robot 

:::info 

**Author**: Porojan Mihai-Iulian \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-PorojanMihai

:::

<!-- do not delete the \ after your name -->

## Description

This project is a Rubik's cube solving robot using a 4 clamp servo powered system along with a color sensing module. My goal is to reduce the workload of the microcontroller by generating an optimal solution 
with Kociemba's algorithm on my laptop to allow for a much faster solve (< 20 moves). 

## Motivation

I chose this project because I was fascinated by the MIT robot that managed to solve a scrambled cube in 0.38 seconds and decided to create my own version using a more budget friendly approach with servomotors
instead of stepper motors and a manual color sensing module instead of cameras.
## Architecture 
![Architecture](architecture.svg)
## Log

<!-- write your progress here every week -->

### Week 5 
Decided on the final project idea and ordered 2 servos and a color sensing module to allow for initial testing.

### Week 6 - 8
Received the STM32U5 microcontroller from the lab.
Tested the SG90 servo to make sure it can turn the face of the cube before ordering more.
Tested the color sensor, encountered issues in detecting the orange and white face, the module is highly influenced by ambient lightning so i tried to find a solution without altering the cube itself.
I plan on 3d printing a small box with a hole for the color sensing module on the bottom, this will allow me to place the cube on top of the sensor with no external interference, if this is unsuccessfull
I will have to resort to using a sharpie on the orange and white face of the cube.

### Week 9 
Decided on using Kociemba's algorithm on my laptop to generate the solution because of memory constraints.

## Hardware
My project is based around the STM32U5 provided at the lab to take the solution generated on my computer using Kociemba's algorithm and control the rotating clamps. The 4 clamps are designed with a pair of 
SG90 servos each, one servo moves the clamp away and towards the cube, while the other rotates the face of the cube 90 or 180 degrees.
The color of each piece of the cube is detected using the TCS34725 color sensor seated in a small box separate from the main mechanism to ensure no external interference with the sensor.


### Schematics
Todo

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | Borrowed |
| [TCS34725 Color Sensor](https://www.makerguides.com/tcs34725-rgb-color-sensor-with-arduino/) | Color detection |[49.82 RON](https://ardushop.ro/ro/senzori/1454-modul-senzor-de-culoare-tcs34725-6427854021342.html) |
| [SG90 Servomotor (8x)](https://www.friendlywire.com/projects/ne555-servo-safe/SG90-datasheet.pdf) | Rotating the cube's faces and moving the clamps |[9.49 RON](https://sigmanortec.ro/Servomotor-SG90-limit-switch-p141662062) |
| [Electronics Kit](https://www.optimusdigital.ro/ro/kituri/12026-kit-plusivo-pentru-introducere-in-electronica-0721248990075.html?search_query=Kit+Plusivo+pentru+Introducere+in+Electronica&results=3) |Breadboard,wires,resistors etc. |[39.99 RON](https://www.optimusdigital.ro/ro/kituri/12026-kit-plusivo-pentru-introducere-in-electronica-0721248990075.html?search_query=Kit+Plusivo+pentru+Introducere+in+Electronica&results=3) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | Hardware Abstraction Layer | Controls the pins, the USB/UART connection to the laptop, and the I2C bus |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async Executor | Used for drawing to the display |
| [kociemba](https://crates.io/crates/kociemba) | Algorithm crate | Used for generating the solution on the computer |
| [embassy-time](https://docs.embassy.dev/embassy-time/0.5.1/default/index.html) | Timer | Handles the pause between servo movements |
| [tcs3472](https://crates.io/crates/tcs3472) | Color sensor driver | Read the colors from the detection module |
| [defmt](https://defmt.ferrous-systems.com) | Logging framework | Debugging |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [MIT .38 second solve](https://news.mit.edu/2018/featured-video-solving-rubiks-cube-record-time-0316)
2. [1 axis solver](https://www.instructables.com/Rubik-Cube-Solver-Robot-With-Raspberry-Pi-and-Pica/)
...
