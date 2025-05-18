# Money counting machine with coin and bill storage



The project is about counting banknotes and coins which are identified through
sensors and then display the total sum inserted.

:::info

**Author**: Boncan Dragoş-Eduard-Gabriel

**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-Dragos112358

:::



## Description

The purpose of this project is to build an automated machine for counting
Romanian banknotes of various denominations (1, 5, 10, 50, 100, 200, and 500
lei). The device will automatically detect and recognize each inserted banknote
using specialized sensors that identify color, size or other distinctive
features. After recognition, the value of the banknote will be added to a
running total, which is displayed in real time on a 4-digit 7-segment display.
The system allows continuous insertion of multiple banknotes and when a reset
button is pressed, the total sum is reset to zero and the small door at the back
of the machine is unlocked, granting access to the storage compartment. Thus,
the machine functions both as a fast-counting system and as a temporary security
mechanism for the banknotes until the reset button is pressed.





## Motivation

I chose to develop this project because it combines several areas of personal
interest, such as electronics, programming, and real-time data processing. The
idea of creating a device capable of automatically recognize and process
banknotes is an exciting technical challenge that involves both circuit design
and the development of identification algorithms. Moreover, such a device has
practical applications in fields like commerce or financial security to reduce
human error and increase efficiency.





## Architecture

Main Components:

1) Banknote Identification System

– Responsible for identifying banknotes.

– At the hardware level, it includes printer rollers, a printer motor, a TCS230
color sensor with 4 LEDs connected to GPIO pins 6, 7, 8, and 9, and two presence
sensors connected to GPIO pins 14 and 15. The motor is powered at 12V and
receives control signals via GPIO pin 4.

2) Raspberry Pi Pico 2 Board

– Mounted on a 830-pin breadboard.

3) Display Module

– A 4-digit 7-segment display used to show the current amount (in Romanian leu
and bani).

4) Reset and Access Control Module

– A reset button that, when it is pressed, resets the displayed amount and
unlocks the back door. There will also be an on/off button to power the system
on or off.





Connections Between Components:

-The Pico board is mounted on an 830-point breadboard.

-All components are controlled from the Pico; the motor driving the rollers and
the lock are activated via 3.3V-controlled relays.

-GPIO pins 6, 7, 8, 9, and 10 are used for communication with the TCS230 color
sensor.

-GPIO pin 4 controls the motor that drives the rollers through Relay 1.

-GPIO pin 5 controls the electromagnetic lock through Relay 2.

-GPIO pin 15 receives data from the first presence sensor (when a banknote is
detected, the motor starts → GPIO 4 is set to high).

-GPIO pin 14 receives data from the second presence sensor (stops the banknote
inside the box for RGB reading).

-GPIO pins 18, 19, 20, and 21 are used for presence sensors for 1, 5, 10, and 50
bani coins, respectively (they increment the value when a coin passes).

-The 4-digit 7-segment display uses GPIO pins 22, 26, 27, and 28.

-GPIO pin 11 is an input pin for the ON/OFF button, helping reduce power
consumption.

-GPIO pin 12 is an input pin for the RESET button. When pressed, the rear door
unlocks, the displayed total resets to 0, and the user can collect the stored
money.



![image1.png](image1.svg)



## Log:


### Week 10 April – 16 April

-I purchased the Raspberry Pi Pico 2 board and debugger and connected them.

-I successfully ran the first cargo run, which displayed “Hello World!”.

-I configured the TCS230 color sensor (used GPIO pins 6, 7, 8, and 9 as outputs
and pin 10 as input).

-RGB values are correctly displayed in the program for different environments.



### Week 17 April –23 April:

-I wrote code to integrate presence sensors into the project (programmed GPIO
pins 14 and 15).

-I modified the code to implement the banknote counting machine as a state
machine (with states: Idle, Start, Stop, Read, and Eject).

-I used a relay to correctly control the motor that pulls banknotes using
rollers.



### Week 24 April – 1 May:

-I wrote the project documentation, including all used components and the KiCad
schematic.

-I tested the TCS230 color sensor in a dark environment (inside a box) to
consistently read the same color for banknotes, minimizing the effect of ambient
light.



### Week 1 May – 7 May:

-To be continued

## Hardware

The hardware design of the project includes a Raspberry Pi Pico 2 with 2 MB
Flash and 264 KB RAM, powered by a step-down 3.3V–5A power supply. An 8-channel
opto-isolated relay module (3.3V compatible) is used to control external
devices. Banknotes are recognized using a TCS230 color sensor, while detection
of coins and banknotes is done through 8 infrared obstacle sensors. Connections
between components are made with female-to-male jumper wires, and the security
of the collected banknotes is ensured by an electromagnetic lock.



## Schematics

![Kicad_schematics.PNG](Kicad_schematics.svg)


## Bill of materials



|               Device               |                            Usage                                 | Unit Price (Ron) | Quantity | Total Price (Ron) |
|:----------------------------------:|:--------------------------------------------------------------:|:-----------------:|:--------:|------------------:|
| [Raspberry Pi Pico 2](https://www.tme.eu/ro/details/sc1632/raspberry-pi-sisteme-incorporate/raspberry-pi/raspberry-pi-pico-2-with-header/)               | Microcontroller                                                 |        27         |     1    |         27        |
| [Debugger for Raspberry Pi](https://www.tme.eu/ro/details/sc0889/raspberry-pi-accesorii/raspberry-pi/debug-probe/)         | Hardware-level debugging and coding                             |        55         |     1    |         55        |
| [8 channel relay module](https://ampul.eu/cs/rele-magneticke-kontakty/3941-modul-8-rele-s-optickym-oddelenim-33v)            | To control the lock and the motor that drives the rollers.     |        85         |     1    |         85        |
| [3.3V 5A DC-DC buck converter](https://www.emag.ro/sursa-de-alimentare-profesionala-12v-5a-100-240v-50-60hz-valabil-si-pentru-internet-modem-mv5a12v/pd/DCZX48YBM/?X-Search-Id=8411c119cfae6a7e6c46&X-Product-Id=218016933&X-Search-Page=1&X-Search-Position=24&X-Section=search&X-MB=0&X-Search-Action=view)      | Power microcontroller when not connected to PC                  |        18         |     1    |         18        |
| [Color sensor TCS230](https://www.optimusdigital.ro/en/optical-sensors/111-tcs230-color-sensor-module.html?search_query=tcs230&results=2)              | Recognize banknotes based on their color (RGB)                  |        39         |     1    |         39        |
| [Infrared obstacle detection sensor](https://www.optimusdigital.ro/en/optical-sensors/4514-infrared-obstacle-sensor.html?search_query=infrared&results=156)| 4 for coins (4 types of coins), 2 for banknotes and 1 for my electromagnetic lock | 3.5 | 8 | 28 |
| [Male-female wires (40 x)](https://www.optimusdigital.ro/en/wires-with-connectors/92-female-male-wire40p-20-cm.html?search_query=0104210000001792&results=1)          | To connect all my circuits, including Pico with all the sensors |        8          |     2    |         16        |
| [Electromagnetic lock](https://www.emag.ro/broasca-electrica-pin-retractabil-12v-24x55x28-mm-2-e-052/pd/DXXN01MBM/)              | To secure all the money inserted while reset button is not pressed | 12        |     1    |         12        |
| [Plexiglass](https://www.dedeman.ro/ro/placa-hobbyglas-transparent-1000-x-500-x-4-mm/p/6019007)                        | For the enclosure housing the hardware components               |        70         |     1    |         70        |
| [4-digit 7 segment display](https://www.emag.ro/afisaj-cu-7-segmente-8-cifre-spi-83x15-mm-max7219-digits-red/pd/D7Z798MBM/)         | Display the total sum stored in the safe                        |        18         |     1    |         18        |
| [830 points Breadboard](https://www.emag.ro/set-componente-electronice-breadboard-830-puncte-led-uri-compatibil-arduino-si-raspberry-pi-zz00044/pd/DRXG4XYBM/)             | For prototyping the entire circuit                             |         20         |     1    |    20        |
| [All purpose leds](https://www.emag.ro/set-componente-electronice-breadboard-830-puncte-led-uri-compatibil-arduino-si-raspberry-pi-zz00044/pd/DRXG4XYBM/)                  | Status indicators (e.g., power, validation, errors)            |         0.7        |     20   |             14        |
| [Buzzer](https://www.emag.ro/set-componente-electronice-breadboard-830-puncte-led-uri-compatibil-arduino-si-raspberry-pi-zz00044/pd/DRXG4XYBM/)                            | For audio feedback (e.g., valid/invalid banknote or coin)      |         5          |      1   |             5         |
| **Total cost**                    |                                                                |                   |          |       **407**     |







## Software



The code I wrote controls a system on the Raspberry Pi Pico 2 for detecting
banknotes. It uses presence sensors to detect when a banknote is inserted, and a
TCS230 color sensor to read its RGB values. The motor is started or stopped
based on the banknote’s position, and the RGB frequencies are normalized to
identify the note’s color. The code handles concurrent tasks and delays using
libraries like embassy_executor and embassy_time.


|     Library      |                      Description                       |                            Usage                            |
|:----------------:|:------------------------------------------------------:|:-----------------------------------------------------------:|
| [embassy_executor](https://docs.embassy.dev/embassy-executor/git/std/index.html) | Asynchronous runtime for microcontrollers              | Run async fn main() and defines asynchronous tasks, such as bancnote_task and monede_task |
| [embassy_time](https://docs.rs/embassy-time/latest/embassy_time/)     | Time management: delays, timers, time instants.        | Used for delays with Timer::after() and time measurements with Instant::now(). |
| [defmt](https://docs.rs/defmt/latest/defmt/)            | Efficient logging library for embedded systems.        |                             defmt is used to log internal states                                |
| [panic_probe](https://docs.rs/panic-probe/latest/panic_probe/)      | Panic handler – displays debug information on panic in embedded mode. | Handles potential runtime panics during execution.      |
| [embassy_rp::gpio](https://docs.embassy.dev/embassy-rp/git/rp2040/index.html) | GPIO pin control on the Raspberry Pi Pico (RP2040)     | Defines and controls pins as output (using Output) or input (using Input). |
| [embassy_rp::init](https://docs.embassy.dev/embassy-rp/git/rp2040/index.html) | Initializes RP2040-specific peripherals               | let p = init(...) — make objects for all pins and peripherals. |







## Links:

https://s3-sa-east-1.amazonaws.com/robocore-lojavirtual/889/TCS230%20Datasheet.pdf

https://www.youtube.com/watch?v=7ILHtAPY29I


