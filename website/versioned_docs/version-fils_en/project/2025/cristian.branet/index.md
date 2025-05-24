# Parking Simulator
Small scale parking lot simulation

:::info 

**Author**: Cristian Branet \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-cristibtz

:::

## Description

The project consists of a small scale simulation of a parking lot, that has a barrier, motion sensor and a display.

It was built using three boards, each having a separate role.
- Raspberry Pi Pico 2 W(main board)
- 2 x Raspberry Pi Pico W(secondary boards)

Besides these boards, I have also used a servo motor, motion sensors, a display and WIFI connectivity for inter-boards communications.

The idea of this project is that tasks are delegated and each board does something useful: one board receives IR commands from a remote and sends them via WIFI to the main board which opens the parking lot barrier. After the car parks, motion sensors controlled by the main board detect the event and send the information, also via WIFI, to the other secondary board which will display the parking lot space status on a display. When a car leaves, the display reflects the parking lot space changes accordingly. 

Having delegated tasks and the boards working in a detachable manner, it makes the process of handling one board's failure easier and the functionality of the whole assembly doesn't stop working 100%.

## Motivation

I built this project with the idea of simulating something that is common in real life and creating a parking lot simulation was the idea that I was sure I can bring to life. I created it because it was very practical and by researching about this topic, learning how to design the project and how components are interconnected with each other at this scale, it helped me understand how more complex systems are created. Another reason is that I like the idea of multiple components actively communicating with each other, thus the use of three boards and WIFI

## Architecture 

![Architecture](architecture.svg)

There are 3 important components in this project.

**RP Pico 2 W**:
 - Receives data from the IR sensor regarding the desired state of the barrier, either closed or opened. 
 - Based on received data, it moves the servo motor.
 - Receives data from one or more motion sensors, if a parking space is occupied.
 - Sends motion sensor's data to the board controlling the display.
 - Modifies LEDs based on the barrier's and sensor's state.

**RP Pico W no. 1**:
 - Receives IR signals from the remote.
 - Processes the signal and sends command corresponding to the signal to the main Pico 2.

**RP Pico W no. 2**:
 - Receives data from the main board.
 - Updates displayed details based on received data.

**LEDs**:
 - Controlled by the main board.
 - They are turned on or off based on the state of the components associated with them(ex: opened barrier - light green LED or sensor that detected motion - light red LED)

**Display**:
 - Displays data about the state of the parking lot, more exactly when a place is freed or how many spaces are available.

**IR remote and receiver**:
 - Classic IR remote which sends IR signals.
 - The IR sensor receives the signals and sends them to the IR receiving board via wire.

:::info
All the boards communicate with between them using WIFI. Each one connects to a designated network via WIFI. The boards have a TCP server setup on them to listen for incoming connections. In this way, if a certain part stops working as it should, necessary data can be sent manually to keep the functionality working(ex: barrier can be opened by sending the proper code to the main board without IR). All other components are wired between each other and programmed to communicate accordingly in the software part.
:::

## Log

### Week 5 - 11 May

Uploaded a part of the documentation for the project. Started working on each individual component's functionality and very soon planning to integrate them together. Also, still testing some functionalities, like infrared and display related stuff. 

### Week 12 - 18 May

Having acquired all the necessary hardware, I continued working on creating IR receiver functionality in Rust, having previously used MicroPython, and succeeded(implementing more exactly, the NEC 8 bit protocol), I continued testing the main board's servo motor and motion detection sensors for proper functioning, then moved on to setting up the display. 

Having finished all the individual functionalities of the boards, I moved on to integrating them together and making sure they communicate properly via WIFI. I also defined the format for data that is transmitted and what every board expects to receive. Will continue testing and deciding on the components' arrangements on the breadboard and on the cardboard parking lot replica.

### Week 19 - 25 May

## Hardware

1. **Infrared Sensor VS1838B**  
   IR receiver module that decodes signals from remote controls.  
   Connected to a secondary board to receive parking commands.  
   Cheap, simple, and ideal for remote input tasks.

2. **Infrared Remote**  
   Standard remote control that emits IR signals to communicate commands.  
   Used to simulate a user requesting to open or lock the barrier.  
   Paired with the VS1838B sensor for command input.

3. **SG90 Servo Motor**  
   Miniature 180° servo motor for basic actuation tasks.  
   Used to physically open and close the parking barrier.  
   Controlled via PWM signals from the main board.

4. **Display SSD1306**  
   0.96" OLED display module with I2C interface.  
   Used to show real-time parking lot status (free/occupied spots).  
   Compact and energy-efficient visual output component.

5. **Infrared Obstacle Sensor**  
   Detects the presence of objects (e.g., parked cars) using reflected IR light.  
   Notifies the main board when a spot is occupied.  
   Essential for automatic status updates of parking spaces.

6. **LEDs**  
   Standard 5mm LEDs used for visual feedback.  
   Indicate system status like barrier state or space availability.  
   Color-coded for easy understanding of events.

7. **USB Hub**  
   Used to supply current to the boards.  
   Very useful to handle multiple USB connections to a Laptop while debugging the project.  
   Cheap and reliable.

*2 minutes demo of the current version:* [Demo](https://www.youtube.com/watch?v=qNot_UbnDCE)

**Below, there are some images taken during the development process:** 

Main board
![main](main.webp)

Infrared receiver board
![ir](ir.webp)

Display board
![display](display.webp)

**Photos when the components are integrated together:**
![p1](done.webp)

*Left - main board and motion sensor, Center - IR receiver board, Right - Display board*

![p2](done2.webp)

![p3](done3.webp)

### Schematics

![Kicad](kicad.svg)

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [2 x Raspberry Pi Pico WH](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The secondary boards | [2 x 56 RON](https://ardushop.ro/ro/raspberry-pi/1945-raspberry-pi-pico-wh-wirelessheaders-6427854029621.html) |
| [Raspberry Pi Pico 2 W](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html#pico-2-family) | The main board | [40 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) |
| [Infrared Sensor VS1838B](https://www.sivago.com.cn/upload/pdf/2022/VS1838B.pdf)| Receive IR signals | [1.77 RON](https://ardushop.ro/ro/comunicatie/758-receptor-infrarosu-vs1838b-6427854009722.html?)
| [Infrared Remote](https://ardushop.ro/ro/comunicatie/2358-kit-ir-telecomanda-receptor-cablu-6427854032461.html) | Send IR signals | [10 RON](https://ardushop.ro/ro/comunicatie/2358-kit-ir-telecomanda-receptor-cablu-6427854032461.html)|
| [SG90 Servo Motor](http://www.ee.ic.ac.uk/pcheung/teaching/DE1_EE/stores/sg90_datasheet.pdf) | Used to move the barrier | [12 RON](https://www.optimusdigital.ro/ro/motoare-servomotoare/2261-micro-servo-motor-sg90-180.html)
| [Display SSD1306](https://www.mouser.com/datasheet/2/1398/Soldered_333099-3395096.pdf?srsltid=AfmBOorDRNE8qWa15NdQ-YsWSvowft21MKXJtvTkFzpYuBffTD88NhD5) | Display parking space info | [34 RON](https://www.emag.ro/afisaj-oled-ssd1306-oled-i2c-compatibil-arduino-si-raspberry-pi-27x27x4-mm-albastru-c9/pd/D3C7C1YBM/?utm_medium=ios&utm_source=mobile%20app&utm_campaign=share%20product) | 
| [Infrared Obstacle Sensor](https://www.handsontec.com/dataspecs/sensor/IR%20Obstacle%20Detector.pdf) | Detects a parked car | [3.5 RON](https://www.optimusdigital.ro/ro/senzori-senzori-optici/4514-senzor-infrarosu-de-obstacole.html)|
| [LEDs](https://ardushop.ro/ro/led-uri/293-467-led-5mm.html#/) | Used to signal certain events | [0.30 RON](https://ardushop.ro/ro/led-uri/293-467-led-5mm.html#/)|
| [Resistances](https://ardushop.ro/ro/componente-discrete/465-801-rezistor-1-4w-1-buc-alege-valoarea.html#) | Fundamental in every circuit | [0.10 RON](https://ardushop.ro/ro/componente-discrete/465-801-rezistor-1-4w-1-buc-alege-valoarea.html#)|

## Software

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [cyw43](https://github.com/embassy-rs/embassy/tree/main/cyw43) | Rust driver for the CYW43439 WiFi+Bluetooth chip | Used for WiFi communications between the boards. |
| [embassy-rp](https://docs.embassy.dev/embassy-rp/git/rp235xb/index.html) | Access to the Raspberry Pi Pico W peripherals | Used for GPIO, PWM, and other peripheral interactions. |
| [embassy-net](https://docs.embassy.dev/embassy-net/git/embassy-net/index.html) | Networking stack for embedded systems | Used for TCP socket communication and network stack initialization. |
| [embassy-time](https://docs.embassy.dev/embassy-time/git/embassy-time/index.html) | Time management for embedded systems | Used for delays and timers in tasks. |
| [defmt](https://docs.rs/defmt/latest/defmt/) | Logging framework for embedded systems | Used for logging events and debugging information. |
| [fixed](https://docs.rs/fixed/latest/fixed/) | Provides fixed-point arithmetic support | Used for configuring PWM with precise timing values. |
| [heapless](https://docs.rs/heapless/latest/heapless/) | Provides heapless data structures | Used for creating fixed-size strings for sensor state messages. |
| [static_cell](https://docs.rs/static_cell/latest/static_cell/) | Provides statically allocated memory | Used for allocating resources like the network stack. |
| [embedded-graphics](https://docs.rs/embedded-graphics/latest/embedded_graphics/) | Graphics library for embedded systems | Used for rendering text and graphics on the OLED display. |
| [ssd1306](https://docs.rs/ssd1306/latest/ssd1306/) | Driver for SSD1306 OLED displays | Used for controlling the OLED display to show parking lot status. |
## Links

1. [First time trying sensors on RP](https://murraytodd.medium.com/using-rust-embedded-to-capture-sensor-data-37db1f726d5c)
2. [Learning about IR receiver](https://www.youtube.com/watch?v=ddceKBDMKsE)
3. [Demo](https://www.youtube.com/watch?v=qNot_UbnDCE)
