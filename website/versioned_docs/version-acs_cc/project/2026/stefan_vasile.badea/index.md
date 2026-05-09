# DeskCal - Smart Desk Calendar
A smart desk calendar that displays weather, local sensor data, and Google 
Calendar events on a TFT screen, with animated LED ring alerts for upcoming 
meetings.

:::info 

**Author**: BADEA Stefan-Vasile \
**GitHub Project Link**: [UPB-PMRust-Students/acs-project-2026-stefanbadea-sudo](https://github.com/UPB-PMRust-Students/acs-project-2026-stefanbadea-sudo)

:::

<!-- do not delete the \ after your name -->

## Description

DeskCal is a smart desk calendar running on an STM32 Nucleo-U545RE-Q 
microcontroller. It receives weather data and Google Calendar events from a 
laptop via USB-UART, processed by a Python script. A BME280 sensor provides 
local temperature, humidity, and atmospheric pressure. A TFT color display shows 
four navigable pages: current clock and date, weather conditions, local sensor 
data with historical temperature graph, and upcoming calendar events. A
WS2812B RGB LED ring displays animated color patterns indicating meeting 
proximity. A passive buzzer emits timed audio alerts before events. A physical
button snoozes active reminders. The device is housed in a custom 3D-printed 
enclosure.

## Motivation

As a student with a busy schedule of classes, lab sessions, and project 
deadlines, I often find myself missing meetings or losing track of environmental 
conditions in my workspace. I wanted to build a device that consolidates all 
this information in one place on my desk, without requiring me to constantly 
check my phone or laptop. The project also gave me the opportunity to explore 
embedded Rust development with embassy-rs, work with multiple hardware 
peripherals simultaneously, and design a complete end-to-end system from 
hardware to software to physical enclosure.

## Architecture 

<!-- Add here the schematics with the architecture of your project. Make sure to 
include:
 - what are the main components (architecture components, not hardware 
 components)
 - how they connect with each other -->

![System Architecture Diagram](./images/project-pm-diagram.drawio.svg)

The system is organized around four main components:
 
**Host Python Script** — runs on the laptop, fetches weather from OpenWeatherMap 
and events from Google Calendar, serializes to JSON and sends over USB-UART 
every 60 seconds.
 
**STM32 Nucleo-U545RE-Q** — main controller running embassy-rs async tasks: 
UART reception, BME280 reading, display rendering, LED ring and buzzer control.
 
**Display Subsystem** — ILI9341 2.8" TFT over SPI renders four pages selected
via potentiometer (ADC).
 
**Alert Subsystem** — WS2812B LED ring (SPI) + passive buzzer (PWM) produce 
visual and audio alerts based on meeting proximity; tactile button handles 
snooze via GPIO interrupt.



## Log

<!-- write your progress here every week -->

### Week 5
Finalized project idea and had the theme approved by the lab coordinator.

### Week 7
Ordered all hardware components from Optimus Digital and eMAG. Started reading 
embassy-rs documentation and experimenting with basic GPIO and UART on the 
Nucleo board. Started enclosure design in Fusion 360.

### Week 8

Stating writing the documentation page.

## Hardware

The project uses an STM32 Nucleo-U545RE-Q as the main microcontroller.
A 2.8" ILI9341 TFT display connected via SPI renders the user interface across
four pages. A BME280 sensor connected via I2C measures local temperature, 
humidity, and atmospheric pressure. A WS2812B 16-LED ring connected via SPI 
provides animated RGB visual alerts. A passive buzzer driven by PWM emits 
audio alerts. A 50kΩ potentiometer on an ADC pin handles page navigation. 
A tactile button on GPIO pins handle snooze and interaction. 
All components are housed in a custom 3D-printed PLA enclosure.

### Schematics

* TODO: KiCad schematic to be added at Hardware Milestone (Week 11)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```
-->


| Device | Usage | Price |
|--------|-------|-------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main microcontroller | Provided by faculty |
| [ILI9341 2.8" SPI TFT Display](https://www.optimusdigital.ro/en/lcds/3544-modul-lcd-spi-de-28-cu-touchscreen-controller-ili9341-i-xpt2046-240x320-px.html) | Displays clock, weather, sensor data and calendar pages | ~90 RON|
| [BME280 Barometric Sensor Module](https://www.optimusdigital.ro/en/pressure-sensors/1354-modul-senzor-barometric-de-presiune-bme280.html) | Measures local temperature, humidity and pressure | ~34 RON |
| [WS2812B RGB LED Ring (16 LEDs)](https://www.optimusdigital.ro/en/others/749-inel-de-led-uri-rgb-ws2812-cu-16-led-uri.html) | Visual meeting alerts with animated color patterns | ~20 RON|
| [Passive Buzzer 3.3V](https://www.optimusdigital.ro/ro/audio-buzzere/12247-buzzer-pasiv-de-33v-sau-3v.html) | Audio alerts before and during meetings | ~13 RON|
| Potentiometer Mono 50kΩ | Page navigation via ADC | ~7 RON|
| [Tactile Button 6x6x6mm](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1119-buton-6x6x6.html) (x2) | Snooze reminder and UI interaction | ~3 RON|
| [Jumper Wires M-F 40p 20cm](https://www.optimusdigital.ro/ro/fire-fire-mufate/92-fire-colorate-mama-tata-40p.html) | Component interconnections | ~10 RON|
| [Jumper Wires F-F 10p 20cm](https://www.optimusdigital.ro/ro/fire-fire-mufate/91-fire-colorate-mama-mama-10p.html) (x2) | Component interconnections | ~10 RON|
| [Breadboard 170 points](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/246-mini-breadboard-colorat.html) | Prototyping connections | ~3 RON|
| 3D-printed PLA enclosure | Houses all components | TBD |


## Software

* TODO: the rest will be added as I develop the code

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [embassy-rs documentation](https://embassy.dev)
2. [ILI9341 datasheet](https://cdn-shop.adafruit.com/datasheets/ILI9341.pdf)
3. [WS2812B datasheet](https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf)
4. [BME280 datasheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bme280-ds002.pdf)
5. [Google Calendar API Python quickstart](https://developers.google.com/calendar/api/quickstart/python)
6. [OpenWeatherMap API](https://openweathermap.org/api)
7. [embedded-graphics examples](https://github.com/embedded-graphics/examples)
