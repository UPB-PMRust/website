# Pet Sense
PetSense is a smart system that monitors temperature, light, and button status, triggering events and alarms based on sensor data.

:::info

**Author**: Dudui Ioana-Denisa \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-denisa31012005

:::

## Description

PetSense is a smart environmental monitoring system designed to help keep pets comfortable and safe in their living space. It uses sensors to track temperature and light levels, which are important factors in a pet’s environment—especially for small animals or those sensitive to heat and light.

An active buzzer alerts the owner with different sounds if the conditions become uncomfortable, like if it gets too hot or too bright. This helps pet owners take quick action to improve the environment.

The system uses a pub/sub communication model to efficiently manage and respond to these changes, making sure each component reacts when needed. For example, if the temperature sensor (connected via I2C) detects heat beyond a set level, the buzzer might sound and other parts of the system respond.

A real-time LCD screen shows the current temperature, so owners can easily check how things are going without needing to use a phone or computer.

To help manage the feeding process automatically, a servomotor is used.

A timer function also helps by checking conditions at regular intervals, ensuring the pet’s environment is monitored consistently without manual checks.

## Motivation

I chose this project because I wanted to create something that could monitor and react to real-world conditions, like temperature and light. It seemed like a challenge to use sensors to gather data and trigger alarms when something unusual happens.Also, I thought it would be a great way to improve my skills with embedded systems.I wanted to build something practical that could be useful in everyday situations, like keeping track of the environment in a room.

## Architecture 
![architecture](imagine.svg)

The Raspberry Pi Pico serves as the central control unit, managing all components within the PetSense system.

The temperature sensor(via I2C) monitors the ambient temperature and provides real-time data to the Pico for evaluation.

The light sensor measures the brightness of the surroundings and sends that information to the Pico, which decides what to do based on preset light level thresholds.

The button lets users interact with the system directly, whether to take manual control or reset the monitoring process when needed.

The active buzzer emits different alarms depending on changes in the environment—like too much light or an uncomfortable temperature—based on alerts sent through the system’s communication channel.

The LCD screen displays the current temperature readings,offering users a clear view of the environmental conditions.

The servomotor reacts to certain events.

A built-in timer keeps track of time intervals, allowing the system to run regular checks and carry out actions at set times automatically.



## Log

<!-- write every week your progress here -->

### Week 6 - 12 May
In the first week of my PetSense project,I bought the necesarry components for the project to keep everything organized.Then,I started working on the schematics,documentation and architecture, taking my time to plan out the design carefully while coding and testing the components to see if they were working accordingly.I also drew the initial KiCad schematic.

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

**Sensor Manager (Temperature & Light)**
- Role: Continuously gathers readings from the temperature (via I2C)and light sensors.

- Connection: Sends updated sensor values to the Event Publisher.

**Event Publisher & Event Channel**
- Role: The Event Publisher checks if sensor readings cross critical thresholds.If they do, it sends corresponding events to the Event Channel.

- Connection: Acts as a central dispatcher, delivering events to all registered subscribers.

**LCD Display**
- Role: Shows real-time temperature values.

- Connection: Subscribes to temperature updates from the Event Channel to refresh the screen regularly.

**Buzzer Controller**
- Role: Provides an audio warning signal when critical environmental limits are exceeded.

- Connection: Subscribes to alert-level events from the Event Channel to activate sound patterns accordingly.

**Servomotor Controller**
- Role: Mechanically responds in order to ensure the deeding process.

- Connection: Subscribes to specific threshold events to perform actuation.

**Button Interface**
- Role: Allows user-triggered manual interactions, such as forcing a reading

- Connection: Sends button events to the Event Publisher, which translates them into commands.


### Schematics

Place your KiCAD schematics here.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [BMP280](https://cdn-shop.adafruit.com/datasheets/BST-BMP280-DS001-11.pdf) | Temperature and Humidity Sensor | [7.99 RON](https://www.optimusdigital.ro/ro/senzori-senzori-de-temperatura/99-senzor-de-temperatura-si-si-umiditate-dht11.html) |
| [LDR](https://www.alldatasheet.com/datasheet-pdf/download/203054/TAOS/TSL2561.html) | Light Intensity Sensor | [6.95 RON](https://www.optimusdigital.ro/ro/senzori-senzori-optici/167-modul-cu-fotorezistor.html) |
| Active Buzzer |  AlertBuzzer | [1.5 RON](https://www.optimusdigital.ro/en/buzzers/12513-pcb-mounted-active-buzzer-module.html) |
| Breadboard HQ |  Breadboard | [4.56 RON](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/44-breadboard-400-points.html?search_query=Breadbo&results=129) |

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
