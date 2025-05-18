# Smart parking sistem
A parking sistem that use infrared sensors, ultrasound sensors and Raspberry Pi Pico 2W

:::info

**Author**: Gabriel Constantin \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-pachirishu

:::

## Description

The parking sistem use ultrasound sensors that detect when a car wants to enter and a servomotor will activate as a barrier. Inside there are 3 parking slots that use led rgb modules to generate the use for each spot and infrared sensors that check the availability. At the entrance there is a lcd module that will tell the number of available spots and the closest one to the entrance. The parking also use wifi connection, so the driver can see online how many spots are left.

## Motivation

I chose this project because it addresses a real-world problem related to the organization and management of parking spaces, especially in small or private areas such as driveways, company lots, or residential complexes. The proposed system offers an automated, cost-effective, and scalable solution for access control and parking availability display, without relying on expensive equipment like cameras or commercial barriers.

## Architecture

![alt text](DiagramaPico.drawio.svg)

The **Raspberry Pi Pico 2W** acts as the central controller, managing all data exchange and operations between input and output modules.
The **ultrasonic sensors** detect vehicle presence near the entry and exit, triggering barrier actions.
The **micro servomotors** lift or lower the entry and exit barriers based on sensor data.
The **infrared sensors** detect car presence in each parking spot.
The **RGB LED modules** indicate the occupancy status: green for available, red for occupied.
The **LCD 1602** displays the number of available parking spaces in real time.
The built-in **Wi-Fi module** allows the Pico to transmit parking status data to a remote server, enabling remote monitoring.
The **PC** is used for programming and optional debugging, while the battery supplies power to the entire system.

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

### Schematics

### Bill of materials

| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico 2W](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html#pico-2-family) | Microcontroller | [39.66 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=raspberry+pi+pico+2w&results=26) |
| [Rapspberry Pi Pico](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html#pico-1-family) | Debugger | [22.49 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12024-raspberry-pi-pico-728886755172.html?srsltid=AfmBOopzfbrSkCAOiZWVeX2qe2_Jwcdpe1hIj6UWpEmNRnZYO2zb_Jat) |
|[Modul Led RGB](https://arduinomodules.info/ky-009-rgb-full-color-led-smd-module/)| Led RGB | [4.99 RON](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/737-modul-cu-led-rgb.html?search_query=modul+cu+led&results=338)|
|[Micro Servomotor SG90 180°](http://www.ee.ic.ac.uk/pcheung/teaching/DE1_EE/stores/sg90_datasheet.pdf)| Micro Servomotor | [11.99 RON](https://www.optimusdigital.ro/ro/motoare-servomotoare/2261-micro-servo-motor-sg90-180.html)|
| [Male to male jumper wires](https://media.digikey.com/pdf/Data%20Sheets/Digi-Key%20PDFs/Jumper_Wire_Kits.pdf) | Wires | [4.99 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/884-set-fire-tata-tata-40p-10-cm.html?search_query=fire&results=429) |
| [Male to female jumper wires](https://media.digikey.com/pdf/Data%20Sheets/Digi-Key%20PDFs/Jumper_Wire_Kits.pdf) | Wires | [7.99 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/92-fire-colorate-mama-tata-40p.html) |
| [Infrared Sensor ](https://studylib.net/doc/25541782/arduino-ir-infrared-obstacle-avoidance-sensor-module) | Infrared Sensor | [3.49 RON](https://www.optimusdigital.ro/ro/senzori-senzori-optici/4514-senzor-infrarosu-de-obstacole.html) |
| [Ultrasonic Sensor ](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf) | Ultrasonic Sensor | [6.49 RON](https://www.optimusdigital.ro/ro/senzori-senzori-ultrasonici/12897-senzor-ultrasonic-hc-sr04-.html?search_query=senzor+ultrasunet&results=7) |
| [LCD 1602 ](https://handsontec.com/dataspecs/module/I2C_1602_LCD.pdf) | LCD | [14.99 RON](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/62-lcd-1602-cu-interfata-i2c-si-backlight-galben-verde.html?search_query=ecran+16+2&results=192) |
| [Breadboards ](https://components101.com/sites/default/files/component_datasheet/Breadboard%20Datasheet.pdf) | Breadboard | [4.56 RON](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/44-breadboard-400-points.html?search_query=breadboard&results=127) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://docs.embassy.dev/embassy-rp/git/rp2040/index.html)| Peripheral access crate for the RP2040 chip used in Raspberry Pi Pico | Used to interact with the microcontroller's hardware|
| [embassy-executor](https://docs.embassy.dev/embassy-executor/git/std/index.html)| Asynchronous task executor for embedded systems| Enables non-blocking async execution|
| [embassy-time](https://embassy.dev/book/dev/time_keeping.html)| Time management library| Used for implementing delays and measuring durations|
| [gpio](https://docs.embassy.dev/embassy-stm32/git/stm32c011d6/gpio/index.html)| General-purpose input/output abstraction| Used to read sensor data (e.g., IR sensor) or control devices like LEDs|
| [pwm](https://docs.embassy.dev/embassy-nrf/git/nrf52840/pwm/index.html)| Pulse-width modulation control| Used to control RGB LED brightness and SG90 servo positioning|
| [log](https://docs.embassy.dev/embassy-usb-logger/git/default/index.html)| Logging facade for embedded Rust| Used to add structured logging|
| [embassy-net](https://docs.embassy.dev/embassy-net/git/default/index.html)| Asynchronous embedded networking stack built on smoltcp| Used for network connection|
| [embassy-net-driver](https://docs.embassy.dev/embassy-net-driver/git/default/index.html)| Network interface drivers for embassy-net| Used to connect embassy-net with hardware like the CYW43 Wi-Fi chip on the Raspberry Pi Pico 2W|

## Links

-