# Runaway Alarm Clock
An autonomous runaway alarm clock that uses proximity sensors to detect the user and physically escape via motorized movement.

:::info 

**Author**: Alin Andrei Abahnencei \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-alinandrei2004

:::

<!-- do not delete the \ after your name -->

## Description

The project is an autonomous runaway alarm clock designed to eliminate oversleeping. Unlike traditional devices, this clock uses a dual-sensor array to manage its environment: one sensor dedicated to detecting the user's approach and another for active obstacle avoidance.

It uses real-time proximity data to distinguish between the user trying to silence the alarm and physical barriers like walls or furniture. By requiring the user to physically get out of bed to chase and capture the fleeing device, the project ensures that the user is fully awake through forced movement and environmental interaction.

## Motivation

The motivation for this project is to combat oversleeping by replacing the easily ignored "snooze" button with a physical activity that forces the user to wake up. I chose this specific application because it provides a practical way of learning how to process real-time sensor data within a dynamic, real-life environment, requiring the software to make navigational decisions based on live feedback to achieve a reliable "flee" response.

## Architecture 

Main Architectural Components:
1. User and Obstacle Detection (Input Data):
   - Role: Monitors surroundings to detect the user of pottential obstacles it may find in its way.
   - Components: 2x HC-SR04 Ultrasonic Sensors. One is towards the user (rear side of the robot) and the other is towards the front.
   - Logic: Signals are passed through a voltage divider (2K/1K resistors) to translate the 5V echo pin into 3V3 signals the board can accept.
2. The Central Logic Controller (Processing):
   - Role: The board executes the code logic, manages real-time interrupts from the sensors and calculates movement directions.
   - Async Task Manager: Uses asynchronous programming to simultaniously handle the alarm timing, sensor signals and motor adjustments without blocking the CPU.
3. Escape System (Output):
   - Role: Responsible for the physical movement of the clock.
   - Components: L298N Driver and 2x DC Motors.
   - Control Logic: The board sends PWM signals to the EnA/EnB pins to control the speed and digital signals to pins IN1-IN4 to set the direction.
4. Audio System (Output):
   - Role: Provides an alarm sound that forces the user to interact with the device.
   - Components: Passive Buzzer.
   - Logic: It is triggered by GPIO pins when the internal RTC matches the set alarm time.
5. Power Network:
   - Role: Power the circuit accordingly to each component's needs.
   - Logic: 1x external battery power the STM32 board and 4x AA batteries power the motors.

![Architecture Diagram](./images/diagramapm.drawio.svg)

## Log

<!-- write your progress here every week -->
### Week 13 - 19 April
    - Bought the hardware necessary for the project.
    - Defined the power managemt system.
### Week 20 - 26 April
    - Worked on the documentation, defining the architecture for the main features of the project.

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

1. STM32 board: Responsible for processing the sensor data and controlling the motors and buzzer.
2. HC-SR04 Ultrasonic Sensors: Used for detecting the user's presence and obstacles in the environment.
3. L298N Motor Driver: Controls the speed and direction of the DC motors.
4. DC Motors: Provide the physical movement for the clock to escape.
5. Passive Buzzer: Emits the alarm sound to wake the user.
6. Power Supply: A combination of an external battery for the STM32 board and AA batteries for the motors to ensure sufficient power for all components.

### Schematics

![Schematic](./images/project%20architecture%20resized.webp)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo U545RE-Q] | The microcontroller | [110 RON](https://eu.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D&utm_id=6470900573&utm_source=google&utm_medium=cpc&utm_marketing_tactic=emeacorp&gad_source=1&gad_campaignid=6470900573&gbraid=0AAAAADn_wf0GICp7OV9wICGSd4JiEPUkg&gclid=CjwKCAjwzLHPBhBTEiwABaLsSpfYEy1LvXzJXlwQr9pTOutAzfZ2l9gvivkogoet7YjGyN21xj5GexoCsvcQAvD_BwE) |
| [Chassis Kit] | The base for the robot | [48.40 RON](https://sigmanortec.ro/Kit-Sasiu-Smart-Car-2WD-p141489122) |
| [HC-SR04 Ultrasonic Sensor] | Used for user and obstacle detection | [16.26 RON](https://sigmanortec.ro/Kit-Sasiu-Smart-Car-2WD-p141489122) |
| [L298N Motor Driver] | Used to control the motors | [10.84 RON](https://sigmanortec.ro/Punte-H-Dubla-L298N-p125423236) |
| [Passive Buzzer] | Used for the alarm sound | [1.45 RON](https://sigmanortec.ro/Buzzer-pasiv-5v-p172425809) |



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [https://embedded-rust-101.wyliodrin.com/docs/acs_cc/category/lab](https://embedded-rust-101.wyliodrin.com/docs/acs_cc/category/lab)
