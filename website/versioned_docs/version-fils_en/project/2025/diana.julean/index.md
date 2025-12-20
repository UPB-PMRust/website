# Environment Tracker 

A STM32-based environment monitoring system that tracks data about temperature, humidity and Ambiental light, displaying it locally and on an LCD.

:::info 

**Author**: Julean Diana-Gabriela \
**GitHub Project Link**:  

:::

## Description

The Smart Environment Tracker is an embedded system designed to measure and respond to environmental conditions in real time using an STM32 microcontroller programmed in Rust. It uses a DHT22 sensor to monitor temperature and humidity, and a photoresistor (LDR) to detect ambient light levels. Three LEDs visually indicate temperature ranges — green for normal, yellow for warm, and red for hot — while an LCD display presents live sensor readings.
An integrated buzzer alarm system, controlled by a timer, activates when temperature or humidity exceeds predefined thresholds, providing an audible warning for unsafe conditions. A button should silence the buzzer when the user is noticed of the alert.
Another LED is turned on when the light read from the photoresistor is reading a value beneath a set threshold. 

## Components Overview

- DHT11 – to read and display temperature and humidity
- LDR – to display light levels
- LEDs – to show the temperature level and to act as a lamp 
- LCD – display temperature, humidity and light level
- Buzzer – alarm when temperature exceeds predefined thresholds
- Button – to stop the alarm

## Motivation

This project is developed to create a simple and reliable system for monitoring environmental conditions in real time. By combining temperature, humidity, and light sensing with visual and audible alerts, it aims to demonstrate how embedded systems can improve comfort, safety, and automation in environments such as homes, labs, or offices. 

## Architecture 

![hardware used](schematics.webp)

The STM32 reads sensor data, processes it and controls the LEDs, buzzer and LCD. 

Input devices (send data to the MCU): 
- LDR - STM reads its value via ADC pin, one-way input
- Button - user input device

Output devices: 
- LEDs
- ST7735 - MCU sends sensor readings to be displayed
- Buzzer - MCU activates an alarm using a timer
- DHT11

## Log

### Week 17 - 23 Nov
I drafted my idea for the project and decided upon the components I wanted to use. Overall, I spent this week documenting my project and searching for options of display. 

### Week 24 - 31 Nov
I started testing some of the project components, such as the LDR, the LEDs and DHT11 sensor. Implemented the logic for the LDR sensor, to read the light level and to light up an LED when light dropped below the 300 level. I made the architecture of the project schematics. 

### Week 1 - 7 Dec
This week I searched for a crate for the DHT sensor. I tried to test it, but after all code errors were solved, the DHT11 still gave me a reading error. I first tested it on arduino to see if there was something wrong with the hardware part, but as the sensor worked just fine there, I concluded it was a code problem. I also tested my initial display, an LCD1602, which unfortunately didn't work, neither on Rust nor with other programming languages. I bought another display with an LM1602 I2C.  

### Week 14 - 21 Dec
With help received from the lab, I got over the DHT11 problem. In the meantime I tested the new LCD with the integrated I2C. It didn't work either, so I decided to use a ST7735 instead. 

## Hardware

### Schematics

### Bill of materials

## Software


