# Ping-Pong Ball Levitation — Wind Tube PID Controller

:::info

**Author**: Oprea Horatiu Alexandru \
**Group**: 332CC \
**GitHub Project Link**: https://github.com/horatiuoprea/website.git

:::

## Description

The project maintains a Ping-Pong ball at a desired height inside a vertical wind tube. A laser distance sensor mounted at the top of the tube continuously measures the position of the ball. This measurement is fed into a PID regulator running on an STM32 microcontroller, which adjusts the speed of a propeller fan at the base of the tube to keep the ball stable at the target height.

The system supports two interaction modes. In **Auto mode**, the user sets the desired height via a dedicated potentiometer, and the PID controller autonomously stabilizes the ball at that height. In **Manual mode**, the user can adjust the three PID constants (Kp, Ki, Kd) individually using separate potentiometers, observing in real time how each parameter affects the system's behavior. An LCD display shows the current height and, depending on the active mode, the target height or the current PID constant values.

## Motivation

Embedded systems and control theory are two domains I wanted to explore simultaneously in a hands-on project. A wind tube ball levitation system is a classic control engineering problem that is visually intuitive, physically tangible, and technically rich. It requires real sensor integration, PWM motor control, I2C communication, and a properly tuned closed-loop PID controller — all implemented in Rust using the Embassy async framework on an STM32 microcontroller. The manual mode adds an educational dimension, allowing direct observation of how each PID term influences system stability, which reinforces the theoretical concepts from control systems courses.

## Architecture

The system is structured around four main architectural components that form a closed control loop:

**Sensor** — The VL53L0X laser Time-of-Flight sensor is mounted at the top of the tube and measures the distance to the ball via I2C. This gives the current height y(t).

**Controller** — The STM32 Nucleo-U545RE-Q runs the PID algorithm in an Embassy async task at a fixed 20ms cycle. It reads the current height from the sensor, computes the error relative to the target, and produces a control output u(t) in the range 0–100%.

**Actuator** — The Delta PFB0412EN-E fan receives a PWM signal at 25kHz directly on its dedicated blue wire. The duty cycle of this signal determines the fan speed, which controls the airflow and thus the ball's position.

**User Interface** — Four potentiometers provide analog inputs via ADC: one sets the target height, and three adjust Kp, Ki, Kd in Manual mode. A 1602 I2C LCD displays system state. A toggle switch selects between Auto and Manual mode.

## Closed Control Loop

![Closed Loop Diagram](Documentatie.drawio.svg)

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The project uses an STM32 Nucleo-U545RE-Q (Cortex-M33) as the main microcontroller, programmed in Rust with the Embassy async framework. The VL53L0X laser ToF sensor communicates over I2C and provides distance measurements with millimeter precision. The Delta PFB0412EN-E axial fan (40x40x28mm, 12V) features a native PWM control input, allowing direct speed control from the STM32 without a MOSFET driver stage. Four 10kΩ potentiometers provide analog inputs for height target and PID tuning. A 1602 LCD with I2C backpack shares the I2C bus with the sensor. A toggle switch selects the operating mode. The fan is powered by a dedicated 12V DC adapter; the STM32 and all logic components are powered via USB (5V), with the onboard regulator supplying 3.3V to the sensor and potentiometers. All grounds are tied to a common reference on the breadboard.

### Schematics

_KiCad schematic to be added here in SVG format._

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main microcontroller — runs PID algorithm in Rust/Embassy | already owned |
| [VL53L0X Laser ToF Sensor](https://www.optimusdigital.ro/ro/senzori-senzori-de-distanta/3309-modul-senzor-de-distanta-cu-laser-vl53l0x.html) | Measures ball height inside the tube via I2C | [25 RON](https://www.optimusdigital.ro) |
| [Delta PFB0412EN-E Fan 12V](https://www.digikey.com) | Propeller fan — controls airflow in the tube via PWM | already owned |
| [LCD 1602 I2C](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/2894-lcd-cu-interfata-i2c-si-backlight-albastru.html) | Displays current height and PID constants | [15 RON](https://www.optimusdigital.ro) |
| Potentiometer 10kΩ (x4) | Height target (x1) + Kp, Ki, Kd tuning (x3) | [5 RON x4](https://www.optimusdigital.ro) |
| Toggle Switch | Selects Auto / Manual operating mode | [3 RON](https://www.optimusdigital.ro) |
| Breadboard MB102 + YuRobot power module | Prototyping platform with integrated power supply | [20 RON](https://www.optimusdigital.ro) |
| DC Jack Module 5.5x2.1mm (breadboard) | Connects 12V adapter to breadboard for fan power | [7 RON](https://www.optimusdigital.ro) |
| Resistor 100Ω | Protects STM32 PWM pin from fan input impedance | [1 RON](https://www.optimusdigital.ro) |
| Jumper Wire Kit (120 pcs) | All signal and power connections on breadboard | [15 RON](https://www.optimusdigital.ro) |
| 12V 1A DC Adapter | Powers the fan independently from STM32 logic | [25 RON](https://www.optimusdigital.ro) |
| PVC Tube 40-45mm inner diameter, ~80cm | Wind tube housing for the ball | [12 RON](https://www.dedeman.ro) |
| Digital Multimeter | Verifying voltages and connections during assembly | [50 RON](https://www.optimusdigital.ro) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for STM32 microcontrollers | Peripheral access: PWM, I2C, ADC, GPIO |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor for embedded systems | Runs sensor, control and display tasks concurrently |
| [embassy-time](https://github.com/embassy-rs/embassy) | Timers and delays for Embassy | Fixed 20ms control loop timing |
| [vl53l0x](https://crates.io/crates/vl53l0x) | Driver for VL53L0X ToF sensor | Reads ball distance over I2C |
| [hd44780-driver](https://crates.io/crates/hd44780-driver) | Driver for HD44780-based LCD displays | Renders height and PID values on 1602 LCD |
| [defmt](https://github.com/knurling-rs/defmt) | Efficient logging framework for embedded Rust | Debug output during development via RTT |
| [defmt-rtt](https://github.com/knurling-rs/defmt) | RTT transport for defmt | Sends log messages to host over USB |
| [panic-probe](https://github.com/knurling-rs/panic-probe) | Panic handler for embedded Rust | Reports panics via defmt |

## Links

1. [Embassy — Async framework for embedded Rust](https://embassy.dev)
2. [VL53L0X — STMicroelectronics ToF sensor datasheet](https://www.st.com/resource/en/datasheet/vl53l0x.pdf)
3. [Delta PFB0412EN-E — Fan datasheet](https://www.delta-fan.com)
4. [PID Controller — Wikipedia](https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller)
5. [Ball in tube levitation — reference project](https://example.com)
6. [STM32U5 Reference Manual](https://www.st.com/resource/en/reference_manual/rm0456-stm32u5-series-advanced-armbased-32bit-mcus-stmicroelectronics.pdf)
7. [Source Code Repository](https://github.com/UPB-PMRust-Students/acs-project-2026-horatiuoprea.git)
