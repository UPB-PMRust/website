# Dual-Axis Solar Tracker
Active dual-axis light tracking system using STM32 and embedded Rust.

:::info 

**Author**: Marcea Radu Andrei \
**GitHub Project Link**: [Github](https://github.com/UPB-PMRust-students/acs-project-2026-MarceaRadu)

:::

<!-- do not delete the \ after your name -->

## Description

The Dual-Axis Solar Tracker is a hardware and software system that automatically orients a central platform towards the strongest light source in a given environment. It uses four light-dependent resistors (LDRs) to measure ambient light intensity across four quadrants. A microcontroller processes these inputs to calculate light differentials and drives two servo motors to adjust the platform's horizontal (pan) and vertical (tilt) alignment, ensuring it remains perpendicular to the light source.

## Motivation

My main motivation was to build an autonomous system, and a dual-axis solar tracker felt like the perfect challenge. Since I've never built a hardware project from the ground up, I saw this as a great way to get hands-on experience with servo motors, LDRs, and microcontrollers. Blending physical hardware with asynchronous Rust code brings the project to life, creating something real that actively reacts to its surroundings.

## Architecture 

![System Architecture Diagram](./arhitecture.svg)

The project is divided into 4 main components: Input, Processing, Output, and Power.

**Main Components:**

* **Input:** The four Light Dependent Resistors (LDRs) send analog voltage signals to the processing unit via the ADC (Analog-to-Digital Converter), and the UI Button sends calibration or reset signals via GPIO.
* **Processing:** The STM32 Nucleo board reads these signals asynchronously, calculates the horizontal and vertical light differentials, and determines the necessary angle adjustments. It then translates these actions into control signals (PWM) and sends them directly to the servo motors.
* **Output:** The Pan and Tilt servo motors receive the PWM commands and execute the mechanical motion to physically align the solar tracker. Additionally, a Status LED receives GPIO signals to provide visual feedback on the system's state.
* **Power:** There is a 5V power supply for the Processing Unit and the sensor array, and an external power source to safely provide the higher current required by the servo motors.

## Log

<!-- write your progress here every week -->

### Week 20 - 26 April

* Researched project requirements and finalized the hardware component list.
* Ordered the Pan-Tilt bracket, SG90 servos, LDRs, and required electronic components.
* Finalized documentation draft.

### Week 27 April - 4 May

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The project relies on the STM32 Nucleo-U545RE-Q as the core controller. The mechanical movement is achieved using a prefabricated Pan-Tilt plastic bracket housing two SG90 micro servos. The sensing array consists of four standard photoresistors (LDRs) paired with 10kΩ resistors acting as pull-downs in a voltage divider setup, assembled on a mini breadboard.

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The main microcontroller unit | Provided by faculty |
| [Pan-Tilt Bracket with 2x SG90 Servos](https://sigmanortec.ro/montura-servomotor-suport-camera-2-axe-pt-antivibratii-ptz-pentru-sg90-mg90s) | Mechanical 2-axis movement | 8.46 RON |
| [4x Photoresistor (5537) 5mm](https://sigmanortec.ro/Fotorezistor-5537-5mm-p160378607) | Light intensity detection (LDRs) | 6.76 RON |
| [Resistor kit](https://sigmanortec.ro/kit-rezistori-30-valori-20-bucati) | Contains the 4x 10kΩ resistors needed for the voltage dividers | 15.16 RON |
| [Breadboard 400 points](https://sigmanortec.ro/Breadboard-400-puncte-p129872825#) | Prototyping electronic circuits | 6.62 RON |
| [Set of Jumper Wires](https://sigmanortec.ro/Set-Jumper-breadboard-140-p136286416) | Connecting components | 11.72 RON |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Lab](https://embedded-rust-101.wyliodrin.com/docs/fils_en/category/lab)
