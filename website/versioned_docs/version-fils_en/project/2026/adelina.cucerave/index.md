# Smart Training Station
A smart reaction-training station that uses lights and sensors to measure hit force while logging scores online in real-time.

:::info 

**Author**: Cucerave Adelina-Maria \
**GitHub Project Link**: (https://github.com/UPB-PMRust-Students/fils-project-2026-AdelinaMariaCucerave)

:::

<!-- do not delete the \ after your name -->

## Description

This project is an interactive training station that uses a network of sensors to track physical hits and share the results over the internet. The system is built with several "interaction nodes" that connect to a main controller and a central dashboard.

The station works by using piezo sensors to measure exactly how hard and how fast a user hits a target. Because hitting these sensors can create high-voltage spikes, the system includes a protection circuit (Schottky-clamping). This keeps the sensitive electronics safe while ensuring the hit data is recorded accurately.

Users get instant feedback through three different channels:

    1. Visual: Bright RGB LED rings change colors to show the intensity of the hit.

    2. Audio: A buzzer plays different sounds for successful hits or errors.

    3. Data: A color screen displays live scores and menus, while a knob (potentiometer) lets the user manually adjust how sensitive the sensors are.

The "brain" of the system is the STM32U545RE-Q microcontroller. It runs on a modern software framework called Embassy-rs, which allows the code to handle many tasks at once without slowing down. Finally, an ESP32-C3 Wi-Fi module acts as a bridge, sending the performance data to an online dashboard so users can track their progress over time.

## Motivation

I have always been interested in how technology can be used to measure and improve human performance. Reaction time and physical precision are important in many areas, such as professional sports, physical therapy, and even industrial safety.

By building a system that gives instant feedback through lights and sounds, users can train their reflexes in a more engaging and effective way. Additionally, by adding Wi-Fi connectivity to the station, performance data can be tracked and saved over time. This allows users to stay aware of their progress, set personal goals, and find specific areas where they can improve their speed and accuracy.

## Architecture 

 Main Architectural Components:

1. Signal Acquisition Unit (Input)

    Role: Detects physical impacts and measures the force of each hit.

    Components: 3x Piezo Diaphragms and 1x Potentiometer (used for adjusting sensor sensitivity).

2. User Interface and Dashboard (I/O)

    Role: Manages the menu system and displays live scores and training data.

    Components: 1.8" SPI TFT Display and 3x Tactile Push Buttons (for navigation and selection).

3. Haptic Feedback System (Output)

    Role: Provides immediate visual and audio alerts to the user when a target is hit.

    Components: 3x 16-LED RGB Rings and a Passive Buzzer.

4. Wireless Telemetry Bridge (Connectivity)

    Role: Sends performance results and hit data to a remote web dashboard via Wi-Fi.

    Components: ESP32-C3 (RISC-V) Module.

<center>
![Diagram](diagram.svg)
</center>

## Log


### Week 5:
- Research, decided on an idea
- Asked lab assistant about it, decided to improve it

### Week 6:
- Started to look at components 
- Decided on what I wanted to buy

### Week 7: 
- Waited for lab feedback
- Placed the order

### Week 8:
- Some components came, started to look into Ubuntu 
- Having problems with Rust but fixing them

## Hardware

Detail in a few words the hardware used.

### Schematics

Place your KiCAD or similar schematics here in SVG format.

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
