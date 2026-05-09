# Pico Reaction Gamepad

A reaction-based gamepad using Raspberry Pi Pico 2 W, buttons, LEDs, and Rust.

:::info

**Author**: Furkan Korkmaz  
**GitHub Project Link**: GitHub Project Link: https://github.com/playstationnetwork987456-commits/Pico-Gamepad

:::

<!-- do not delete the \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ after your name -->

## Description

Pico Reaction Gamepad is a small embedded hardware project based on Raspberry Pi Pico 2 W. The project is designed as a simple gamepad-like reaction game. The device uses push buttons as user inputs and LEDs as visual feedback.

The main idea is simple: one LED turns on randomly, and the player must press the matching button as quickly as possible. The microcontroller measures the reaction time and calculates a score. The project will run software written in Rust on the Raspberry Pi Pico 2 W.

## Motivation

I chose this project because it is simple to understand but still includes important microcontroller concepts. It uses GPIO input, GPIO output, timing, button handling, and embedded Rust programming.

The project is also flexible. It can start with a basic version using buttons and LEDs, and later it can be improved with a buzzer, OLED display, score memory, or more game modes.

## Architecture

The Raspberry Pi Pico 2 W is the central controller of the system. The buttons are connected to GPIO input pins, and the LEDs are connected to GPIO output pins. The software reads the button states and controls the LEDs according to the game logic.

Main architecture components:

* Input module: push buttons connected to GPIO pins
* Output module: LEDs connected to GPIO pins through resistors
* Game logic module: selects a random LED and checks the pressed button
* Timing module: measures the reaction time of the player
* Score module: calculates and updates the score

Functional architecture:

`text
+------------------------+
| Raspberry Pi Pico 2 W  |
|                        |
|  Rust Game Logic       |
|  Timing + Score        |
+-----------+------------+
            |
     +------+------+
     |             |
 GPIO Inputs   GPIO Outputs
     |             |
  Buttons         LEDs
     |             |
 Player input   Visual feedback

 Log
Week 5 - 11 May

The project idea was selected. I decided to build a reaction-based gamepad using Raspberry Pi Pico 2 W. The first version of the project will use buttons and LEDs.

Week 12 - 18 May

The hardware architecture was planned. The buttons will be used as GPIO inputs, and the LEDs will be used as GPIO outputs. The main software logic was also planned.

Week 19 - 25 May

The documentation page was created. The project description, motivation, architecture, hardware plan, and software plan were written.

Hardware

The project will use Raspberry Pi Pico 2 W as the main microcontroller. The prototype will be built on a breadboard using buttons, LEDs, resistors, and jumper wires.

Planned hardware components:

Hardware components:

- Raspberry Pi Pico 2 W: 1 piece, main microcontroller.
- Breadboard: 1 piece, used for building the prototype circuit.
- Push buttons: 3 or 4 pieces, used as player input.
- LEDs: 3 or 4 pieces, used as visual feedback.
- Resistors: 3 or 4 pieces, used for LED current protection.
- Jumper wires: several pieces, used for making connections.
- Buzzer: 1 optional piece, used for sound feedback.
- OLED display: 1 optional piece, used for showing the score.

Schematics

The first prototype will connect each button to a GPIO input pin and each LED to a GPIO output pin. Each LED will use a resistor for current protection.

A more detailed schematic will be added after the physical circuit is built and tested.

Software

The software will be written in Rust and will run directly on the Raspberry Pi Pico 2 W.

The program will follow these steps:

Initialize the GPIO pins.
Configure buttons as inputs.
Configure LEDs as outputs.
Turn off all LEDs.
Select one LED for the current round.
Turn on the selected LED.
Start measuring time.
Wait for the player to press a button.
Check if the correct button was pressed.
Calculate reaction time.
Update the score.
Start a new round.

Start
  |
Initialize GPIO
  |
Turn off LEDs
  |
Select random LED
  |
Turn on LED and start timer
  |
Wait for button press
  |
Check button
  |
Calculate reaction time
  |
Update score
  |
Repeat

Current Status

The project topic, motivation, initial architecture, hardware plan, and software plan are defined. The next step is to build the first breadboard prototype and test GPIO input and output using Rust.

![image](./Images/Ekran görüntüsü 2026-05-09 010937.webp)
![image](./Images/Ekran görüntüsü 2026-05-09 011422.webp)
![image](./Images/Ekran görüntüsü 2026-05-09 011503.webp)
![image](./Images/Ekran görüntüsü 2026-05-09 011601.webp)
![image](./Images/Ekran görüntüsü 2026-05-09 011615.webp)
![image](./Images/Ekran görüntüsü 2026-05-09 011631.webp)
![image](./Images/Ekran görüntüsü 2026-05-09 011736.webp)
![image](./Images/Ekran görüntüsü 2026-05-09 011937.webp)
![image](./Images/Pico_Gamepad.svg)



