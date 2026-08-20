# mp3 player
A bluetooth mp3 player

:::info 

**Author**: Cihodaru Valentin-Alexandru \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-Vulisu

:::

## Description

An mp3/wav player that reads the song from a sd card and plays it on a bluetooth speaker. The mp3 
player also has a potentiometer that is used to change the volume, a play/pause button and skip 
song and backwards buttons. 
## Motivation

I chose this project because I think an mp3 bluetooth player that has the songs stored offline can be helpful when there is no internet access.
## Architecture 
![Schematic diagram](schema.webp)


## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Detail in a few words the hardware used.

### Schematics

![Schematic diagram](kicad.webp)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->
### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico W](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) | Main microcontroller for decoding and logic (x2) | 79.32 RON |
| [Bluetooth Module](https://sigmanortec.ro/modul-transmitator-audio-bluetooth-41-ble) | Handles wireless audio transmission to speakers | 38.31 RON |
| [PAM8403 Amplifier Modul](https://www.optimusdigital.ro/ro/module-audio/13699-modul-amplificator-audio-cu-potentiometru-pam8403.html) | Amplifies audio signal for local output | 6.99 RON |
| [Breadboard HQ830 Kit](https://www.optimusdigital.ro/ro/breadboard-uri/1352-kit-breadboard-hq830-cu-fire-si-sursa.html) | Prototyping base with power supply module | 22.00 RON |
| [Potentiometer 50k Mono](https://www.optimusdigital.ro/ro/potentiometre/18387-potentiometru-mono-50k.html) | Analog volume control input | 1.49 RON |
| [Buttons (6x6x6)](https://www.optimusdigital.ro/ro/butoane-si-comutatoare/10862-buton-6x6x6.html) | Song control (Play/Pause, Skip, Back) (x10) | 3.60 RON |
| [Power Button with Red LED](https://www.optimusdigital.ro/ro/butoane-si-comutatoare/20628-buton-de-pornire-cu-led-rosu.html) | Main power switch with visual indicator | 7.75 RON |
| [Jumper Wires (M-F)](https://www.optimusdigital.ro/ro/fire-conectori-si-socluri/3864-fire-colorate-mama-tata-40p-10-cm.html) | Connecting components to the Pico | 5.99 RON |
| [Headers (Male/Female)](https://www.optimusdigital.ro/ro/fire-conectori-si-socluri/11944-header-de-pini-verde-254-mm-40p.html) | Soldering to the Pico for breadboard use | 11.88 RON |
| [LED Pack (R/V/A)](https://www.optimusdigital.ro/ro/led-uri/6186-led-rosu-de-3-mm-cu-lentile-difuze.html) | Status indicators (Power, BT Link, SD Activity) | 1.04 RON |
| [Resistors 220Ω](https://www.optimusdigital.ro/ro/rezistori/10053-rezistor-025w-220.html) | Current limiting for LEDs (x8) | 0.80 RON |
| [DAC](https://www.optimusdigital.ro/ro/audio-altele/5658-modul-audio-stereo-dac-pcm5102a-cu-interfaa-pcm-32-bii-384-khz.html) | Digital to analog for sound | 94.99 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rs](https://github.com/embassy-rs/embassy) | Embassy-rs | Async GPIO, PWM, PIO, and ADC |
| [Synphonia](https://github.com/pdeljanov/Symphonia) | Audio decoding | Decode the mp3 to be able to be played |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [raspberry pi documentation](https://www.raspberrypi.com/documentation/microcontrollers/)
2. [synphonia documentation](https://docs.rs/symphonia/latest/symphonia/)
3. 
...
