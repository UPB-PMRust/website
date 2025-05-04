# Wearable Health-Monitoring System
A watch-like device that tracks a person's health.

:::info 

**Author**: Danaila Andreea-Valentina \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-andreeadanaila20

:::

## Description

The project consists on a watch-like device that can be put on a person's wrist that measures and displays key biometric data in real time. Built around a Raspberry Pi Pico 2W board, the system integrates two sensors: MAX30102 for the heart rate and SpO2, and the AD8232 for a basic monitoring of ECG levels. The display presents the measurements directly on the wearable device, eliminating thus the need for an external app. The project is powered by a rechargeable Li-Po battery, and a button is used to turn the system on or off.

## Motivation

My motivation for choosing this project was to create a simple health monitoring system that anyone can use, regardless of their medical expertise. Continuous health tracking is becoming more and more vital for the early detection of issues, while also promoting personal wellness. Today however, most devices that can be bought are either too expensive or rely on external apps. This project aims to create a low-cost and easy to use alternative, thus the use of a button for ON/OFF. 

## Architecture 

### Components 
The architecture is designed of a few layers. Firstly, we have the processing layer, which consists of the Raspberry Pi Pico 2W board. Then, we have the input layer which connects to the two sensors, MAX30102 and AD8232. The output layer connects to the LCD display. The control layer is the button for turning the system on and off. Lastly, the power management layer, which consists of the Li-Po Battery.

### Connections
The heart rate and SpO2 sensor (MAX30102) and the ECG sensor (AD8232) collect the health data and send it to the Pico over I2C.
The Pico takes the data and sends it to the display to show the vitals using I2C.
The button is pressed and the Pico turns the system on or off.

## Log

### Week 8 - 14 April
I researched the suitable components for my device and ordered them. I researched the best way to make the project wearable, looking into PCBs as an option. In the same week, I soldered the pins on my two sensors and on the Raspberry Board.

### Week 9 - 24 April
I made the initial connection between all the components - Raspberry board, MAX30102, AD8232, and the display, on a breadboard. 

### Week 19 - 25 May

## Hardware

Detail in a few words the hardware used.

### Schematics

Place your KiCAD schematics here.

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
|  |
|  |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...
