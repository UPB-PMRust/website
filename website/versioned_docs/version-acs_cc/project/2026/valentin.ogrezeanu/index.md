# DeskBot - Desktop Robot with Personality

A small autonomous desk robot with a mood engine, face animations, and wireless control.

:::info 

**Author**: Ogrezeanu Valentin Alexandru \
**GitHub Project Link**: https://github.com/valeogre/website

:::

## Description

DeskBot is a small 2WD desk robot with its own personality. An OLED display acts as its face, showing different animations based on an internal mood engine — curious, happy, alert or idle — which evolves over time based on interactions. It navigates autonomously, avoids obstacles, detects desk edges, and tracks sound sources. It can also be controlled manually via a wireless IMU-based handheld controller that receives live telemetry back from the robot.

## Motivation

I chose this project after watching videos of small interactive robots and finding the concept genuinely interesting. The idea of a robot that feels alive through animations and behavioral states — rather than just moving around — made it stand out. Even in its first iteration, rougher and bigger than ideal, it has the potential to be a fun study buddy for a desk setup. It also covers a wide range of embedded concepts: state machines, PID control, wireless communication, sensor fusion and real-time display rendering, all in Rust.

## Architecture 

**Main components:**

- **STM32 Nucleo-U545RE-Q** - central microcontroller, runs all logic
- **State Machine** - core behavioral engine, manages transitions between IDLE, ROAMING, ALERT, HAPPY, CONTROLLED states
- **Mood Engine** - internal variables (energy, curiosity, sociability) that evolve based on interactions and environment
- **Navigation Module** - ultrasonic sweep on servo + IR edge detection + LDR + closed-loop PID on encoders
- **Sound Tracker** - two microphones, detects sound direction and orients the robot toward it
- **OLED Face** - renders animations and info dashboard based on current state
- **Wireless Link** - nRF24L01 bidirectional communication between robot and handheld
- **Handheld Controller** - MPU6050 IMU for tilt-based gesture control + OLED for live telemetry

**Connections:**
State machine owns the global robot state. All other modules communicate with it via Embassy channels and signals — no shared state directly between tasks.

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware


### Schematics

*KiCAD schematics to be added*

## Software



## Links

1. https://www.youtube.com/watch?v=ktWnwJ-e-_w
2. https://www.youtube.com/watch?v=3hjvpyjxPsk