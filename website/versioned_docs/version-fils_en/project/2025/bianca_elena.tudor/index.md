# ClimaControl
Monitors sensors,displays temperature, and controls motor and buzzer based on threshold logic.

:::info

**Author**:Tudor Bianca Elena \
**GitHub Project Link**:https://github.com/UPB-PMRust-Students/project-biancatudor1234

:::

## Description

This smart system keeps track of temperature, light, and a button input, and uses that information to control things like a fan, buzzer, and LEDs, while also showing the temperature on an LCD screen. It reads data from temperature and light sensors and lets you control some features manually with a button. The LCD shows the current temperature in real time. The fan can turn on by itself when it gets too hot or can be turned on manually. If the temperature gets too high, a buzzer goes off as a warning. There are two LEDs to show what’s going on: green means everything’s okay, and red means there’s an alert or problem. The system checks sensors all the time and changes the outputs as they are needed to help keep the environment comfortable.

## Motivation

The motivation behind this project is to create a smart system that makes the environment more comfortable by automatically monitoring and adjusting things like temperature and lighting. I chose this project because having a system that reacts to changes in temperature or light without needing constant input is practical  and efficient. The LCD display is helpful for showing the real-time temperature, and the fan and buzzer make sure the environment stays just right. It’s designed to make your space adapt to you effortlessly and keep things comfortable.


## Architecture 
![architecture](imagine.svg)

 The Raspberry Pi Pico 2W acts as the central controller for all functions. It collects data from sensors and controls all connected components based on programmed logic.

The temperature and light sensors constantly check the surroundings and send updated values to the Raspberry Pi Pico, which then reacts accordingly.

The LCD is used to display the current temperature in real time, helping users monitor the environment .

The button is included for manual control, it allows the user to turn the fan on and off no matter the current temperature.

The fan, which runs on a DC motor, is starts automatically once the temperature goes above a certain level, but it can also be switched on manually through the button.

The buzzer works as a warning system and makes a sound when the temperature gets too high, letting users know it’s getting too hot.

There are two LEDs that show what’s going on: the green one lights up when everything is working fine, and the red one turns on if there is a problem .
## Log

<!-- write every week your progress here -->

### Week 6 - 12 May
 I bought the components needed for the project to make sure I had everything ready to start. Then, I worked on the initial documentation and architecture. I also drew the first version of the KiCad schematic, connecting the components based on my design. After that, I started coding and testing the sensors, buzzer, and LEDs to check that they were working properly and i integrated them in the project.


### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

__*Raspberry Pi Pico:*__

-Purpose: Main control unit.
-Function: Manages and controls all connected components, such as sensors, the fan, buzzer, LEDs, and the LCD display based on the data it receives.

__*Temperature Sensor:*__

-Purpose: Measures the surrounding temperature.
-Function: Continuously sends temperature data to the Raspberry Pi Pico, which uses the information to decide when to activate other components.

__*Light Sensor:*__

-Purpose: Detects ambient light levels.
-Function: Sends light readings to the Pico, which can use this data to influence system behavior (like adjusting LED indicators).

__*Button:*__

-Purpose: Allows manual control.
-Function: Lets the user manually turn the fan on or off, giving control in addition to the automatic temperature-based activation.

__*DC Motor with Fan:*__

-Purpose: Cools the system when it's too hot.
-Function: Turns on automatically when the temperature passes a certain threshold or manually through the button.

__*Active Buzzer:*__

-Purpose: Provides a sound alert.
-Function: Beeps when the temperature is too high to warn the user of overheating.

__*Green and Red LEDs:*__

-Purpose: Show system status.
-Function: The green LED lights up during normal operation, while the red LED turns on when there’s an alert or issue.

__*LCD Display:*__

-Purpose: Displays sensor data.
-Function: Shows the current temperature in real time so the user can easily monitor the environment.

### Schematics


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
| [BMP280 Sensor](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmp280-ds001.pdf) |The Temperature and Humidity Sensor DHT11 | [7,99 RON](https://www.optimusdigital.ro/ro/senzori-senzori-de-temperatura/99-senzor-de-temperatura-si-si-umiditate-dht11.html) |
| LDR Sensor | The Light Intensity Sensor| [6,95 RON] |
| Breadboard HQ  | Breadboard HQ | [4,56 RON] |
| Active Buzzer | The Buzzer| [1,50 RON] |
| DC Motor | DC Motor | [3,50 RON] |
| LEDS | LED with Diffused Lens | [0,80 RON] |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy/tree/main/embassy-rp) | RP2040 Peripherals | Used for accessing the peripherals|
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...
