
# Project Name Solar Tracking "Sun Stirling" System
A renewable energy project utilizing adaptive mirrors and a Stirling engine to generate electricity from solar heat. 

:::info 
**Author** Deaconescu David Mihai \

**GitHub Project Link**: https://github.com/UPB-PMRust/fils-project-2026-david.deaconescu

:::

## Description 
The project focuses on a renewable energy system where adaptive mirrors focus sunlight onto a Stirling engine. The resulting heat puts the engine in motion, which can then drive an alternator attached to the crankshaft to produce electrical energy. For the current demonstration, the system focuses on the control logic for the mirrors rather than a functioning engine or alternator. 

## Motivation

As the global demand for electrical energy grows, finding renewable and sustainable supply methods is increasingly vital. This project aims to offer a cost-effective alternative to traditional photovoltaic panels by lowering the cost per kilowatt-hour through solar thermal concentration. 
+1

## Architecture 

The system architecture relies on two main microcontrollers communicating via UART: 

STM32: Acts as the primary controller for hardware interaction, managing the mirrors and processing data from the directional light sensor. 

ESP32: Serves as the network layer, handling the Wi-Fi connection and fetching weather data. 


## Log
### Week 5 - 11 May
 Discussed the project idea at the lab and started work on the 3d models in Fusion

### Week 12 - 18 May
Bought components (Servo motors, LCD screen, Joystick, Esp32 and the STM32)

### Week 19 - 25 May
Started work on the code in rust and assembled the directional light sensor

## Hardware
The hardware setup includes an STM32 for motor and sensor control, an ESP32 for connectivity, and a variety of sensors and actuators to track light and display system status. 
+1

### Schematics

/Users/ddmihai/Documents/website/website/versioned_docs/version-fils_en/project/2026/david.deaconescu/imagine.webp

### Bill of Materials

|Device|	Usage	|Price|
|------|------------|-----|
|STM32 U545RE-Q	|Controls the servo motor, takes inputs from the joystick and the photoresistors| 125 lei 
|SSD1306 Display	|Used to display current processes	|18 lei |
|Joystick	|Makes the interaction between the user and the Stm32 possible  |5 lei| 
|Esp32 Devkit	|Used as a network layer to access weather data	|   42 lei |
|3D printed items	|Includes the directional light sensor frame and the servo motor housings	|Approx. 25 lei| 
|Servo motors (sg90)	|Move the mirrors according to lighting conditions	|30 lei| 
|Photoresistors	|Used to detect light from different angles	|4 lei |
|Miscellaneous	|Wires, buttons and LEDs for connecting components together	|10 lei| 
|Total|		|259 lei |

## Software
|Library	|Description	|Usage|
|___________|_______________|_____|
|Embassy_time	|Async tasks and delays	|Timers for defining intervals at which tasks will be performed |
|Embassy_executor|	Used for concurrent tasks	|Used for running concurrent tasks |
|Wi-fi.h	|Wifi library for the esp32	|Used to connect to the internet on the Esp32 |
|Ssd1306	|Display driver for the 0.96 inch oled	|Used to interact with the ssd1306 display |
|defmt	|Structured logging framework	|Real time monitoring of processes |
|heapless	|Library for vectors of strings	|Used to define vectors with a fixed maximum size |
|Embedded-graphics	|2D graphics library	|Used to create graphics on the oled display |

## Links

https://auto.howstuffworks.com/stirling-engine.htm