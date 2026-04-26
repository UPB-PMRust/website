# Embedded Rhythm Music Box Game

A real-time embedded rhythm game where the user must press buttons in sync with LED cues and music.

Author : Andra-Sara-Maria Duminica  
Project GitHub Link : 

---

## Description

This project implements a real-time embedded rhythm game using an STM32 microcontroller. It extends a classic music box into an interactive system where the user must follow the rhythm of a melody. 

The device plays an 8-bit melody through a buzzer while LEDs light up in a predefined sequence corresponding to the rhythm. Each LED is associated with a specific button, and the user must press the correct button at the correct time.

The system evaluates the user’s input in real time, checking both correctness and timing accuracy. Based on performance, a scoring system computes metrics such as correct hits, missed inputs, and overall accuracy. At the end of the song, the final score is displayed on an OLED screen.

The system uses a cartridge-based selection mechanism with three different cartridges, each corresponding to a different melody. This allows easy switching between songs and makes the system modular and extensible.

---

## Motivation

I chose this project because it combines real-time embedded programming with interactive gameplay. It allows practical exploration of timing, synchronization, and hardware control, while also creating an engaging user experience.

---

## Architecture

The system is structured into the following software components:

- Main Controller – coordinates all components
- Cartridge Decoder – identifies which of the three cartridges is inserted
- Melody Manager – handles playback and timing
- LED Controller – generates visual rhythm cues
- Button Handler – reads and processes user input
- Rhythm Checker – compares input timing with expected beats
- Score System – calculates performance metrics
- Buzzer Controller – generates audio output
- OLED Controller – displays feedback and results

### Component interaction:

Cartridge → Cartridge Decoder → Melody Manager  
Melody Manager → LED Controller  
Melody Manager → Buzzer Controller  
Melody Manager → Rhythm Checker  
Button Handler → Rhythm Checker  
Rhythm Checker → Score System  
Score System → OLED Controller  

---

## Journal

### Week 5 - 11 May


### Week 12 - 18 May
 

### Week 19 - 25 May


---

## Hardware

- STM32 Nucleo board  
- Buzzer  
- LEDs  
- Push buttons  
- OLED display  
- Cartridge connectors (for 3 cartridges)  

---

## Schemes

The electrical schematics will be designed using KiCAD and will be added later.

##Bill of Materials (estimation)

- Passive buzzer – ~5–15 RON  
- LEDs – ~10–20 RON  
- Resistors – ~5–10 RON  
- Push buttons – ~10–20 RON  
- SSD1306 OLED display – ~30–60 RON  
- Breadboard – ~15–30 RON  
- Jumper wires – ~10–20 RON  
- Header pins (cartridge system) – ~5–10 RON  

Total estimated cost: ~200–350 RON
