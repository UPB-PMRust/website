# Automatic BPM Detector for Music  
A portable device that detects and displays the Beats Per Minute (BPM) of music from the environment.

:::info 

**Author**: Harpa George-Alexandru (Group 1221EA)  
**GitHub Project Link**: https://gitlab.cs.pub.ro/george.harpa/pmrust.pages.upb.ro

:::

## Description

This project implements a real-time BPM (Beats Per Minute) detector using two microcontrollers. It captures ambient music using a microphone, processes the audio signal to detect beat patterns, and displays the BPM on an OLED or LCD screen.

## Motivation

I chose this project because of my interest in music technology and signal processing. The idea of creating a portable, real-time tempo analysis tool offers a good mix of embedded hardware, digital signal processing, and display interfacing challenges — all while being visually demonstrable and engaging.

## Architecture 

The system includes the following main architecture components:

- **Audio Front-End**: Captures the music using a microphone.
- **BPM Processor (Primary MCU)**: Processes the signal and calculates the BPM.
- **Debugging MCU**: Collects internal logs and performance data for analysis.
- **User Interface**: Displays the current BPM on a screen and optional LED indicators.
- **Interconnection**: Microphone feeds audio into the primary MCU. The debug MCU receives serial data over UART from the primary MCU.


## Log

### Week 5 - 11 May  
- Defined project scope and main components  
- Started component research and selection  
- Drafted initial architecture  
- Created GitLab fork and added base files

### Week 12 - 18 May  
- Prepare audio acquisition setup  - TO DO
- Build amplifier circuit for microphone - TO DO
- Begin testing ADC and audio signal sampling  - TO DO

### Week 19 - 25 May  
- Integrate BPM detection logic  - TO DO
- Display integration  - TO DO
- Debug communication setup between MCUs  - TO DO

## Hardware

- Raspberry Pi Pico W (MCU with Wi-Fi)  
- Electret microphone with MAX9814 amplifier  
- 128x32 OLED SPI display  
- Breadboard, jumper wires, USB power supply

### Schematics

*(Insert KiCad schematic here — image or link)*

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| [Raspberry Pi Pico W](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) | Main microcontroller with Wi-Fi | 34.50 RON |
| [Electret Microphone Amplifier - MAX9814](https://www.optimusdigital.ro/en/others/1194-electret-microphone-amplifier-max9814-with-auto-gain-control.html) | Captures ambient music | 26.99 RON |
| [OLED 128x32 SPI Display](https://www.optimusdigital.ro/en/lcds/8660-modul-display-monocrom-128x32-oled-spi-adafruit.html) | Displays BPM | 39.99 RON |
| [Breadboard 830 Points](https://www.optimusdigital.ro/en/breadboards/8-breadboard-hq-830-points.html) | Prototyping | 19.99 RON |
| [Male to Male Jumper Wires (40pcs)](https://www.optimusdigital.ro/en/wires-with-connectors/12475-male-to-male-jumper-wires-40-pin-40cm.html) | Connections | 9.99 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [MicroPython](https://micropython.org/) | Lightweight interpreter for MCUs | Used to program Pico W |
| [ssd1306](https://github.com/micropython/micropython/blob/master/drivers/display/ssd1306.py) | OLED driver | Display BPM data |
| [network](https://docs.micropython.org/en/latest/library/network.html) | Wi-Fi connectivity | Connects Pico W to local network |
| [socket](https://docs.micropython.org/en/latest/library/socket.html) | Web server | Hosts BPM data online |
| [array / ustruct](https://docs.micropython.org/en/latest/library/ustruct.html) | Signal processing | Handles sample buffers and math |

## Links

1. [Beat tracking algorithm overview](https://www.ee.columbia.edu/~dpwe/papers/Ellis07-beattrack.pdf)  
2. [Real-time BPM Detection Techniques](https://www.researchgate.net/publication/345912338_Real-Time_BPM_Detection_in_Music)  
3. [MicroPython for Pico W](https://www.raspberrypi.com/news/micropython-on-raspberry-pi-pico-w/)  
4. [SSD1306 OLED Display Tutorial](https://learn.adafruit.com/monochrome-oled-breakouts/python-usage)  
5. [MAX9814 Microphone Datasheet](https://datasheets.maximintegrated.com/en/ds/MAX9814.pdf)

