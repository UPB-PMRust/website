# Environment Tracker 

A STM32-based environment monitoring system that tracks data about temperature, humidity and Ambiental light, displaying it locally and on an LCD.

:::info

Author: Julean Diana-Gabriela
GitHub Project Link:

:::

## Description

The Smart Environment Tracker is an embedded system designed to measure and respond to environmental conditions in real time using an STM32 microcontroller programmed in Rust. It uses a DHT22 sensor to monitor temperature and humidity, and a photoresistor (LDR) to detect ambient light levels. Three LEDs visually indicate temperature ranges — green for normal, yellow for warm, and red for hot — while an LCD display presents live sensor readings.
An integrated buzzer alarm system, controlled by a timer, activates when temperature or humidity exceeds predefined thresholds, providing an audible warning for unsafe conditions. A button should silence the buzzer when the user is noticed of the alert.
Another LED is turned on when the light read from the photoresistor is reading a value beneath a set threshold. 

## Components Overview

•	DHT11 – to read and display temperature and humidity

•	LDR – to display light levels

•	LEDs – to show the temperature level and to act as a lamp 

•	LCD – display temperature, humidity and light level

•	Buzzer – alarm when temperature exceeds predefined thresholds

•	Button – to stop the alarm

