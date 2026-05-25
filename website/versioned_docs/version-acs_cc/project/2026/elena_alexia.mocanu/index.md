# Rhythm Game
Hardware Rhythm Game

:::info 

**Author**: Mocanu Elena Alexia \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-AlexiaElenaMocanu

:::

## Description

Rhythm game follows the standard mechanics of this genre: the player needs to hit the notes at the right time using the buttons on the breadboard. There are multiple types of notes: tap notes, hold notes, and motion notes triggered by moving the breadboard up/down on the table. Visual feedback is provided using LEDs, where the color represents the accuracy. \
Every hit note fills up the color match progress bar. Once the bar is full, the game enters a special mode called "Color Match". In this mode, the player needs to match the color of the LEDs with those shown on the screen using RGB channels. Failing to finish the game mode successfully will make the player lose one life. The player starts with three lives and can replenish one after completing a song. \
The game is accompanied by music, and players can customize which set tracks are playing and in what order. 

## Motivation

I chose this project because I like rhythm games. Combining them with hardware will be a fun challenge for players.

## Architecture

![](./arhitectura_diagrama.webp)

Main components:

*Central control unit*: 
- **STM32 Nucleo Board**
  - handles the core game logic and synchronizes all peripherals

*Graphics*:
- **LCD display** 
  - displays the game interface 
  - communicates through SPI

*Control*:
- **Push buttons** 
  -  detect tap/hold notes
  -  connected via GPIO pins   
-  **Gyroscope** 
   -  detects the breadboard movement
   -  communicates through I2C

*Feedback*:
- **LEDs** 
  - hit accuracy visual feedback
  - controlled via PWM pins
-  **Servomotor** 
   - color match progress bar visual feedback
     - when it reaches 180° it means the bar is full
   - controlled via PWM pins   

*Music*:
- **Mini player**
  - plays background tracks 
  - communicates through UART

*Sound effects*:
- **Buzzer**
  - game action sounds for audio feedback
  - controlled via PWM pins

## Log

### Week 13 - 19 April

I wrote the initial documentation.













