# Reaction time trainer
A device that measures and evaluates user reaction time using randomly generated stimuli

:::info 

**Author**: Ion Cristina-Gabriela \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-cristinaion0109

:::

<!-- do not delete the \ after your name -->

## Description
The project involves designing and implementing a device based on an STM32 microcontroller that tests user reaction speed using randomly generated visual, auditory and sensor-based stimuli.

The system measures the time between the appearance of a stimulus and the user’s response, detects incorrect actions, false starts, or delayed reactions, and applies penalties when necessary. After each round, the user receives immediate feedback on their performance, including the reaction time, while at the end of a session, the device displays overall statistics such as the best time, average time and total number of errors.



## Motivation
The main source of inspiration was the set of project ideas proposed by the professor, which I found both interesting and suitable for an embedded systems application. Additionally, I was particularly drawn to the concept of measuring reaction time, as I have always been curious to understand, beyond simple numbers, how quickly we can process visual or auditory stimuli.

This project allows me to work with different types of inputs and outputs, such as LEDs, sensors, and displays, while also implementing timing and user interaction, making it both practical and engaging.


## Architecture 

Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware
The system is based on an STM32 Nucleo board, which acts as the main controller. It handles the generation of stimuli, reads user inputs from various sensors and components, and measures the reaction time with high precision

Main parts:
- STM32 Nucleo Board - This is the main controller of the device. It generates stimuli, reads inputs from buttons and sensors, measures reaction time, and manages the game logic, including scoring, error detection and game mode selection
- LEDs - These are used as visual stimuli. When a specific LED lights up, the user must press the corresponding button as quickly as possible. They provide a simple and fast way to test reaction to visual signals
- Buttons - These are the main input devices for the user. Each button corresponds to a specific action or stimulus. The system detects button presses and checks whether the response is correct and within the allowed time
- Buzzer - This component generates audio stimuli. When a sound is played, the user must respond by rotating the potentiometer
- Potentiometer - This component is used to detect user input. In response to certain stimuli (sound), the user must rotate the potentiometer. The STM32 reads its value and determines whether the action was performed correctly
- Photoresistors - These sensors detect changes in light intensity. The system can display instructions such as covering a specific sensor, and the user must react
- Servo Motor: The servo is used to create a mechanical stimulus by raising a small flag. When the flag is lifted, the user must press a specific button
- Display (OLED): The display is used to show instructions, reaction times and game statistics. It also provides a menu for selecting the game mode and guides the user throughout the session


### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...