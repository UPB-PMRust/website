# Digital Fairy Companion
A one line project description

:::info 

**Author**: Ciupitu Alexandra-Isabela\
**GitHub Project Link**: [link_to_github](https://github.com/SabeCiupi/website)

:::

<!-- do not delete the \ after your name -->

## Description

The Digital Fairy Companion functions as a reactive embedded system that translates environmental data into interactive emotional responses to simulate a sentient digital presence. By continuously monitoring real-time inputs from light, motion, sound, color sensors and by capacitive touch interaction, the system utilizes a complex Finite State Machine to manage over seventeen distinct behavioral states. These inputs are processed by an STM32 Nucleo microcontroller which autonomously coordinates visual animations on a 2.4-inch LCD, synchronized audio responses stored on a microSD card, and dynamic ambient lighting via RGB LEDs. In practice, the system adapts its persona based on environmental triggers, such as entering a sleep state in low light or initiating a productivity timer when specific colors are detected, providing a comprehensive multi-sensory user experience.

## Motivation

I chose this project because of a deep conviction that mood and mental well-being are fundamental pillars for personal growth and resilience in daily life. By merging the technical rigor of embedded systems with the whimsical charm of childhood nostalgia, I aimed to recreate the sense of magic that is often absent in standard modern technology. This project serves as a bridge between the complex mechanics of real-time sensor processing and the emotional human experience, providing a supportive companion that acknowledges and reacts to its user's environment. Ultimately, developing this system from the ground up allows me to explore how hardware and software can be harnessed not just for utility, but to foster a more mindful and enchanting atmosphere within a personal workspace.

## Architecture 

<!--Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other-->

![Shematic](arhitectura.png) 

## Log

<!-- write your progress here every week -->
### Week 2 - 8 March
I choosed my project idea and I thought the details of the project.

### Week 13 - 19 April
I wrote the initial documentation.

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Each component was selected to balance high performance with the low-power requirements of an always-on companion device:
* **Microcontroller (MCU)**

  The NUCLEO-U545RE-Q was chosen for its ultra-low-power capabilities and sufficient processing power to handle multiple serial protocols and complex FSM logic concurrently.

* **Display & Storage**

  The ILI9341 LCD module was selected for its vibrant color reproduction and integrated microSD slot, which simplifies the wiring for the SPI bus while providing ample storage for pixel-art assets.

* **Sensors**

  The TCS34725 provides superior color accuracy via I2C, while the MPU 6050 (GY-521) offers 3-axis precision for detecting physical interaction or "dizziness". The TTP223 capacitive touch sensor allows for a seamless "petting" interface without mechanical wear.

* **Audio Subsystem**

  To provide clear auditory feedback, the PAM8403 Class D amplifier is used to drive a compact 1W 8ohm speaker, delivering high-efficiency sound reproduction directly from the board's DAC.

* **Prototyping & Power**

  The system is assembled on an MB102 830-point breadboard using a combination of male-to-female and male-to-male jumper wires. Power is supplied through the USB connection of the Nucleo board, ensuring a stable 5V and 3.3V supply for all modules.

### Schematics

<!--Place your KiCAD or similar schematics here in SVG format.-->

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|-------|-------|
| ILI9341 2.4" LCD with Touch and SD Slot | Display for pixel-art animations and SD card storage | 66,99 RON |
| TCS34725 Color Recognition Sensor | Detects ambient colors for state transitions | 19,99 RON |
| GY-521 MPU-6050 Gyroscope + Accelerometer | Motion and rhythm detection | 24,19 RON |
| PAM8403 Class D Audio Amplifier Module | Drives the speaker for audio feedback | 5,99 RON |
| High Sensitivity Microphone Sound Sensor Module | Detects ambient sound levels | 13,92 RON |
| WS2812 RGB5050 LED Module | Status-based ambient lighting | 3,99 RON |
| LM393 Photodiode Sensor Module | Detects ambient light intensity | 3,65 RON |
| TTP223 Capacitive Touch Sensor | Touch/petting interaction interface | 1,98 RON |
| MB102 830-Point Breadboard | Prototyping platform | 13,99 RON |
| 40x Dupont Wires Male-Female 20cm | Connecting modules to microcontroller | 6,99 RON |
| 40x Dupont Wires Male-Male 20cm | Breadboard connections | 8,99 RON |
| Mini Speaker 1W 8Ω | Audio output | ? RON |
| MicroSD Card | Storing graphical frames and audio clips | ? RON |
| Tactile Push Button 6x6mm | Reset and color detection triggers | ? RON |
| 1x40 Pin Header 2.54mm | Connecting PAM8403 to breadboard | ? RON |
| NUCLEO-U545RE-Q | Main microcontroller running the FSM logic | - RON |


## Software
<!--
| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |
-->

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

<!--1. [link](https://example.com)
2. [link](https://example3.com)
...-->
