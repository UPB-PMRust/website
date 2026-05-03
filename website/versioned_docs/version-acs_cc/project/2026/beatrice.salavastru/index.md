# Laser Harp
A laser-based musical instrument that converts interrupted light beams into sound in real time.

:::info 

**Author**: Beatrice-Elena Sălăvăstru \
**GitHub Project Link**: [link_to_github](https://github.com/UPB-PMRust-Students/acs-project-2026-beatriceelena5)

:::

<!-- do not delete the \ after your name -->

## Description

A laser harp system with 8 laser beams acting as strings. When a beam is interrupted, a sensor detects the change and the microcontroller generates the corresponding musical note. The sound can be played through a speaker or sent to a computer for external audio playback.

## Motivation

As someone interested in both music and technology, I want to build an interactive system that translates physical gestures into sound. This project allows me to explore working with optical sensors, handling real-time input from multiple sources, and generating audio signals on a microcontroller. It also gives me the opportunity to integrate hardware and software into a complete system, while creating a visually engaging and intuitive user experience.

## Architecture 

![Architecture Diagram](images/laser_harp_architecture.svg)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 April
Designed and built the wooden frame for the laser harp. Planned the layout of laser beams and sensors to ensure proper alignment and stability.

### Week 12 - 18 April
Ordered all hardware components from TME and Sigmanortec. 


## Hardware

The project uses an STM32 Nucleo-U545RE-Q as the main microcontroller. Laser modules act as light sources, while phototransistors detect beam interruptions. The signals are processed using ADC inputs. An LM386-based audio module drives a speaker for sound output. A toggle switch selects between local audio and external output, while a rocker switch controls power. The system is powered by a 5V supply and mounted on a custom wooden frame.

<!-- ### Schematics

Place your KiCAD or similar schematics here in SVG format. -->


### Bill of Materials  

| Device | Usage | Price |
|--------|--------|-------|
| Nucleo U545RE | Microcontroller | ~120 RON |
| Laser modules x10 | Light beams (strings) | 20 RON total |
| TEPT4400 x11 | Light detection | 16.76 RON total |
| LM386 module | Audio amplification | 4.14 RON |
| Speaker (8Ω) | Sound output | 23.02 RON |
| Resistors 10kΩ | Signal conditioning | 10.38 RON |
| Capacitors 100µF | Power filtering | 4.07 RON |
| Capacitors 100nF | Noise filtering | 6.47 RON |
| Capacitors 0.1µF | Decoupling | 8.16 RON |
| Rocker switch | Power ON/OFF | 4.43 RON |
| Toggle switch (SPDT) | Mode selection | 5.58 RON |
| Power supply 5V 2A | Power source | 22.61 RON |
| DC jack connector | Power connection | 3 RON |



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | STM32 HAL for Embassy | Used for GPIO, ADC and timers/PWM |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor | Runs the main application logic |
| [embassy-time](https://github.com/embassy-rs/embassy) | Timekeeping and delays | Used for timing and sensor polling |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer | Standard interface for peripherals |
| [defmt](https://github.com/knurling-rs/defmt) | Lightweight logging framework | Used for debugging and logging |
| [defmt-rtt](https://github.com/knurling-rs/defmt) | RTT logging transport | Sends logs to the PC |
| [panic-probe](https://github.com/knurling-rs/probe-run) | Panic handler | Used for debugging crashes |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Arduino-Laser-Harp](https://www.instructables.com/Arduino-Laser-Harp-1/)

