# Morse Code Door Lock

**Author**: Tudor-Cristian Costache
**GitHub Project Link**: (https://github.com/UPB-PMRust-Students/proiect-iJackT.git

## Description

The project give the user the possibility to unlock the door of a house unsing the Morse code. The input comes from the user using two buttons. It gives feeback to the user using:
+ Sound signals using a buzzer
+ Visual signals using GREEN and RED LEDs

## Motivation

I chose this project because it combines security with an alternative and creative method of entering a password – Morse code. It's an interesting approach that merges something we all use daily, a door lock, with an interactive way to unlock the door by using Morse code instead of a traditional key.

## Architecture 
The following diagram shows the system architecture:

![diagram](https://github.com/user-attachments/assets/44a96d33-9b22-457d-b9d3-9be17945bc4b)

The Raspberry Pi Pico 2W acts as the CNS of the hole system, managing everything.
The buttons are used by the user to input the code for unlocking the door.
The servomotor is used as the lock of the door.
The LEDs and the buzzer provide the user feedback to let them know whether the inputede code is correct or not.

## Hardware Components

+ Raspberry Pico 2W – main controller of the system, reads the input and controls ouput to LEDs and buzzer
+ 1 buzzer – provides auditory feedback for incorrect code input
+ 2x LEDs – one green and one red, provideing visual feedback
+ 1x servomotor – used for locking/unlocking the door
+ 2x buttons – one for "dot" and the other for "dash", corresponding to Morse code
+ Resistors – used to protect components and ensure proper circuit operation

## Hardware Description:
The two microcontrollers are connected according to the laboratory diagram for debugging. The components are connected as follows:
- Pins 4-7 are used for debugging
- Pin 39 is used for power supply
- Pin 38 is connected to GND
- The servo motor is connected to 3V3, GND, and pin 2 of the Raspberry Pi
- Pin 9 is connected to the button for the respective function
- Pin 10 is connected to the button for the respective function
- Pins 11 and 12 are used for the green and red LEDs, respectively
- Pin 14 is used to control the buzzer

## Operation
The user presses buttons to enter the code. The microcontroller checks whether the password matches the one stored. If the entered sequence matches the preset one, the servo motor will unlock the door, and a green LED will light up. Otherwise, the red LED will turn on, and the buzzer will sound to alert that the code was entered incorrectly.

## Schematics

![Proiect_PM](https://github.com/user-attachments/assets/64b10ff1-c110-44ee-9abd-21c814ea9434)

# Photos



# Biil of Materials


| Device | Usage | Price | Quantity |
|--------|-------|-------------|----------|
| Breadboard | Connecting components | Already had it | 1 |
| Jumper Cables | Connecting the Pico to the components | Already had them | ~20 |
| Red LED | Signaling inputed code is wrong | Already had it | 1 |
| Green LED | Signaling inputed code is correct | Already had it | 1 |
| Buzzer | Auditory feedback in case of wrong code | Already had it | 1 |
| [Raspberry Pi Pico 2W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) | Main microcontroller | ~40 | 2 |
| Resistors | Used to ensure proper circuit operation | Already had them | ~6 |
| [Servomotor](https://www.optimusdigital.ro/ro/motoare-servomotoare/26-micro-servomotor-sg90.html?srsltid=AfmBOopNX7R_hjvSguzCn--QktQ5BoKMQ9fOrtjxloPUm0SnechT1Dbx) | Used to lock/unlock the door | ~13 | 1 |
| Total |  |  ~93 |  |

