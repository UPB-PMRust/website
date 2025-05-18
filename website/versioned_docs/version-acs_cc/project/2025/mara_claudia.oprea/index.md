# Digital Touch Piano
A one line project description

:::info 

**Author**: Oprea Mara Claudia \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-mara291

:::

## Description

Digital Touch Piano is a "pocket" design for an electric piano which takes input using touch sensors. It includes features such as:
- sensors to play an entire octave
- learning instructions displayed on the screen
- buttons for recording and playing songs

## Motivation

Playing piano is one of my hobbies and I always liked the idea of being able to play even in places where I can't bring my piano. I also want to make learning seem easy for people without a music background, so I integrated in my project instructions teaching how to play popular piano songs. 

## Architecture 

 ![alt text](diagram.svg)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

- **Raspberry Pi Pico 2W** - the microcontroller, connects all parts together
- **MPR121 Capacitive Touch Sensor** - has 12 sensors which will be used as piano keys
- **Buttons** with the following functions:  
	-> **Mode** - change screen mode to on (shows instructions) or off (blank)  
	-> **Record** - starts recording song  
	-> **Stop** - stops recording song  
	-> **Play** - play recorded song  
- **LCD screen** - shows intstructions how to play
- **LED** - blinks when notes are played according to instructions
- **Buzzer** - generates a sound when a key is touched

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
| [Raspberry Pi Pico 2W x2](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [39.66 RON x2](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [MPR121](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Capacitive Touch Sensor | [8.99 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Button x4](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Buttons | [0.36 RON x4](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [LCD module](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Instructions screen | [14.99 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Buzzer](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Generates sound | [0.99 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Breadboard](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Breadboard | [8.99 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Jumper wires](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Connection between components | [4.99 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |


## Software

| Library | Description | Usage |
|--------|-------------|-------|
| [`rp-hal`](https://crates.io/crates/rp-hal) | Hardware Abstraction Layer | Used to control GPIO, PWM, I2C.. |
| [`embassy`](https://embassy.dev) | Asynchronous embedded framework | Used for running assynchronous tasks |
| [`embedded-hal`](https://crates.io/crates/embedded-hal) | Generic embedded traits for drivers | Provides interfaces for drivers |
| [`mpr121`](https://crates.io/crates/mpr121) | Driver for MPR121 capacitive touch sensor | Detects touch input from piano "keys" |
| [`lcd1602-rs`](https://crates.io/crates/lcd1602-rs) | Driver for LCD | Displays instructions on how to play |
| [`embedded-time`](https://crates.io/crates/embedded-time) | Time abstractions for embedded devices | Helps control timing for note duration |
| [`micromath`](https://crates.io/crates/micromath) | sin and cos waves | Used for generating wave samples |


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...


