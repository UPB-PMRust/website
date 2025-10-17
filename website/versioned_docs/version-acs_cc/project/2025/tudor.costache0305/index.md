# Morse Code Door Lock

**Author**: Tudor-Cristian Costache

**GitHub Project Link**: (https://github.com/UPB-PMRust-Students/proiect-iJackT.git)

## Description

The project gives the user the possibility to unlock the door of a house using the Morse code. The input comes from the user using two buttons. It gives feedback to the user using:
+ Sound signals using a buzzer
+ Visual signals using GREEN and RED LEDs

## Motivation

I chose this project because it combines security with an alternative and creative method of entering a password – Morse code. It's an interesting approach that merges something we all use daily, a door lock, with an interactive way to unlock the door by using Morse code instead of a traditional key.

## Architecture 
The following diagram shows the system architecture:

![diagram](https://github.com/user-attachments/assets/44a96d33-9b22-457d-b9d3-9be17945bc4b)

The Raspberry Pi Pico 2W acts as the CNS of the whole system, managing everything.
The buttons are used by the user to input the code for unlocking the door.
The servomotor is used as the lock of the door.
The LEDs and the buzzer provide the user with feedback to let them know whether the inputted code is correct or not.

# Log

### Week 5-11 May
### Week 12-18 May
### Week 9-25 May

## Hardware Components

+ Raspberry Pico 2W – main controller of the system, reads the input and controls output to LEDs and buzzer
+ 1 buzzer – provides auditory feedback for incorrect code input
+ 2x LEDs – one green and one red, providing visual feedback
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

![IMG_6613](https://github.com/user-attachments/assets/f9d37fe5-58ca-4ecd-b86f-5e00f9a98f7e)
![IMG_6615](https://github.com/user-attachments/assets/7332fe71-4fbb-4c1a-a47a-ba76632d8cef)
![IMG_6617](https://github.com/user-attachments/assets/6da1ecf2-c034-4208-93e8-62515c6a0144)
![IMG_6614](https://github.com/user-attachments/assets/66487d98-becf-4d35-aa24-6cc0394ceaa9)
![IMG_6616](https://github.com/user-attachments/assets/2d4bf082-395d-4afa-a5d5-cf6beff45442)

# Final Photos

![IMG_6619](https://github.com/user-attachments/assets/2cdbb63c-3e14-4435-82fb-30afa97584dc)
![IMG_6621](https://github.com/user-attachments/assets/16001e65-c8f8-488e-9cac-a25656e4e493)
![IMG_6618](https://github.com/user-attachments/assets/5690a5f8-1052-4ebc-a795-7a69c32ad496)
![IMG_6623](https://github.com/user-attachments/assets/f50b6ef5-421b-49a3-a7c9-a8edfd7cbf79)

# Bill of Materials

| Device | Usage | Price | Quantity |
|--------|-------|-------------|----------|
| Breadboard | Connecting components | Already had it | 1 |
| Jumper Cables | Connecting the Pico to the components | Already had them | ~20 |
| Red LED | Signaling that the entered code is wrong | Already had it | 1 |
| Green LED | Signaling that the entered code is correct | Already had it | 1 |
| Buzzer | Auditory feedback in case of wrong code | Already had it | 1 |
| [Raspberry Pi Pico 2W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) | Main microcontroller | ~40 | 2 |
| Resistors | Used to ensure proper circuit operation | Already had them | ~6 |
| [Servomotor](https://www.optimusdigital.ro/ro/motoare-servomotoare/26-micro-servomotor-sg90.html?srsltid=AfmBOopNX7R_hjvSguzCn--QktQ5BoKMQ9fOrtjxloPUm0SnechT1Dbx) | Used to lock/unlock the door | ~13 | 1 |
| Total |  |  ~93 |  |

# Software

| Library | Description | Usage |
|:--------|:------------|:-------------------|
| [embassy-rp](https://docs.embassy.dev/embassy-rp/git/rp2040/index.html) | Library for peripheral access on the chip | Used to interact with the microcontroller |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Asynchronous task runtime | Entry point |
| [embassy-time](https://crates.io/crates/embassy-time) | Asynchronous timers and delays | Used for delays |

# Links
1. [Rust Programming for Beginners](https://github.com/Perlatecnica/getting-started-embassy-stm32f401re)
2. [Servomotor Interface with Raspberry Pico](https://randomnerdtutorials.com/raspberry-pi-pico-servo-motor-micropython/)
3. [Raspberry Pico 2 Book](https://datasheets.raspberrypi.com/pico/pico-2-datasheet.pdf)
   

