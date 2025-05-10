# Guitar Tuner
A tuning device for electric and acoustic guitar


:::info

**Author**: Ianis-Gabriel Armencea

**GitHub Project Link**: [repository](https://github.com/UPB-PMRust-Students/proiect-iamc03)

:::


## Description

This guitar tuner helps you tune your guitar into standard tuning (EADGBe). It uses a microphone (with a built in
amplifier) which listens to the guitar chord being played and identifies if it is too flat or too sharp from the
reference frequency. It displays the current note on the oled display.

## Motivation

Being a guitar player myself i thought it would be a fun idea to integrate a hobby into a school project. Also,
if the project turns out well i will be using the device myself to tune my guitar before playing.

## Architecture
### Diagram
![alt text](architecture_pm.webp)

## Log
### Week 5 - 11 May
### Week 12-18 May
### Week 19-25 May

## Hardware

1. **Raspberry Pi Pico 2W**: The microcontroller responsible for computing and processing the data and for communicating with all of the components.
2. **MAX4466**: The microphone used to detect the sound coming from the guitar. Comes with a built-in amplifier so it can detect quieter sounds.
3. **LEDs**: Used to guide the guitar player by indicating whether the note is too sharp, too flat or in perfect pitch.
4. **Button**: Used for switching between the notes in the E-standard tuning.
5. **OLED display**: Displays the current note that is being tuned to.

## Schematics
![alt text](schematic.webp)


## Bill of Materials
| Device | Usage | Price |
| ------ | ----- | ----- |
| Raspberry Pi Pico 2W | The microcontroller | 40 RON |
| MAX4466 | Mic with amplifier built-in | 15 RON |
| Button | Changing note | 1 RON |
| OLED Display 0.96" | Displaying notes | 50 RON |
| Resistor kit | Filtering | 15 RON |
| Breadboard | Component connections | 5 RON |
| Jumper wires set | Connections | 8 RON |
| Micro-usb cable | Power | 4 RON |
| LED | Note flat, sharp or perfect pitch | 0.39 RON | 

## Software

| Library | Description | Usage |
| ------- | ----------- | ----- |
| embassy | Framework for embedded programming | Writing embedded Rust applications for microcontrollers |
| spectrum_analyzer | An easy to use and fast no_std library (with alloc) to get the frequency spectrum of a digital signal (e.g. audio) using FFT. | Converting audio signal to frequency |
| ssd1306 | Display driver | Display text on screen |

## Links
