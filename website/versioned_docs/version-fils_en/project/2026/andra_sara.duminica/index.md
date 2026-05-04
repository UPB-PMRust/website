# Embedded Rhythm Music Box Game

A real-time embedded rhythm game where the user must press buttons in sync with LED cues and music.

:::info

**Author:** Andra-Sara-Maria Duminica  \
**Project GitHub Link:** https://github.com/UPB-PMRust-Students/fils-project-2026-Asmd-44  

:::

---

## Description

This project implements a real-time embedded rhythm game using an STM32 microcontroller. The system extends a simple music box into an interactive game where the user must follow the rhythm of a melody.

The device plays an 8-bit melody using a buzzer while LEDs light up in a predefined sequence corresponding to the rhythm. Each LED is mapped to a specific button, and the user must press the correct button at the correct time.

The system evaluates the user’s input in real time, checking both correctness and timing accuracy. Based on performance, a scoring system calculates metrics such as correct hits, missed inputs, and overall accuracy. At the end of the song, the final score is displayed on an OLED screen.

The system uses **three interchangeable cartridges**, each corresponding to a different melody, allowing easy switching between songs and future extensibility.

---

## Motivation

The motivation behind this project is to explore real-time embedded systems through an interactive and engaging application. It combines hardware control, precise timing, and user interaction, making it a practical way to understand synchronization, input processing, and modular system design.

---

## Architecture

The system is structured as a set of interacting modules:

- **Main Controller** – coordinates the entire system  
- **Cartridge Decoder** – detects which of the three cartridges is inserted  
- **Melody Manager** – handles song playback and timing  
- **LED Controller** – generates visual rhythm cues  
- **Button Handler** – reads and debounces user input  
- **Rhythm Checker** – verifies timing accuracy of button presses  
- **Score System** – computes performance metrics  
- **Buzzer Controller** – generates audio output  
- **OLED Controller** – displays feedback and final results  

The modules communicate through the microcontroller, which acts as the central unit coordinating all operations.

---

## Journal

### Week 5 – 11 May
 

### Week 12 – 18 May
 

### Week 19 – 25 May


---

## Hardware

The system uses the following hardware components:

- STM32 Nucleo board  
- Passive buzzer  
- LEDs (multiple colors)  
- Push buttons (3–4)  
- SSD1306 OLED display (I2C)  
- Breadboard and jumper wires  
- Header pins for cartridge system  

---

## Schematics

![System Diagram](final_fiag.svg)


---

## Bill of Materials (Estimated Cost)

- STM32 Nucleo board – ~120–180 RON  
- Passive buzzer – ~5–15 RON  
- LEDs – ~10–20 RON  
- Resistors – ~5–10 RON  
- Push buttons – ~10–20 RON  
- SSD1306 OLED display – ~30–60 RON  
- Breadboard – ~15–30 RON  
- Jumper wires – ~10–20 RON  
- Header pins (cartridge system) – ~5–10 RON  

**Total estimated cost: ~200–350 RON**
