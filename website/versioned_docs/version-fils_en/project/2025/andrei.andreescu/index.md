# SentinelGuard


:::info

**Author**: Andreescu Andrei-Vlad \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-AndreiAndreescu

:::
## Description

i want to make a project that activates a system alarm when something enters in a zone where is a movement sensor a laser sensor and a light sensor and then it gives me an email with the the time spend in the zone at the time that the alarm activated 

## Motivation

For me an alarm system is very interesting so to do one is absolutly grandious 

## Architecture 

![architecture](architecture.webp)
## Log

<!-- write every week your progress here -->

### Week 6 - 12 April 
Research for components
### Week 13 - 19 April 
Buying the components
### Week 19 - 25 April  
Documentation 
### Week 12 - 16 May
Hardware

## Hardware
![Hardware](poza1.webp)
![Hardware](poza2.webp)
Raspberry Pi Pico W (SC0918): Microcontroller board based on the RP2040 microcontroller chip.

APDS-9960 Light Sensor: Digital RGB, ambient light, and gesture sensor with I2C interface.

VL53L0X Distance Laser Sensor: Time-of-flight (ToF) laser-ranging sensor for measuring distances with an I2C interface.

RGB LED: Light-emitting diode capable of producing various colors, typically with common cathode/anode configuration.

Buzzer: Electromechanical component that produces sound when activated, usually by applying an alternating current.

### Schematics

![kicad schematics](kicad.webp)

### Bill of Materials

Rapspberry Pi Pico W
Light sensor and movement sensor
Laser distance sensor
Rgb led
Buzzer
Power supply 


| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
[Light sensor and movement sensor](https://www.emag.ro/senzor-de-lumina-rgb-detector-de-gesturi-apds-9960-multicolor-apds9960-mod/pd/DLR798MBM/?utm_campaign=share%20product&utm_source=mobile%20app&utm_medium=ios)  | |[21 RON] |
[Laser distance sensor](https://www.emag.ro/senzor-de-distanta-laser-tof-vl53l0x-aalbastru-vl53l0x-gy530-blue/pd/DQR798MBM/?utm_campaign=share%20product&utm_medium=ios&utm_source=mobile%20app)  | |[16 RON] |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...
