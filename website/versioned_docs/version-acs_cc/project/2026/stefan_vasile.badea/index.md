# DeskCal - Smart Desk Calendar
A smart desk calendar that displays weather, news, local sensor data, and 
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
microcontroller. It receives real-time data from a laptop via USB-UART —
including current time, weather conditions, Calendar events, and live
news headlines — all serialized as JSON by a companion Python script.

A BME280 sensor measures local temperature, humidity, and atmospheric pressure.
A 2.8" ILI9341 TFT touchscreen display renders five navigable screens: a
summary overview, a full-screen clock with upcoming meetings, a local air
quality dashboard, an outdoor weather page, and a live news feed. Navigation
between screens is done via swipe gestures on the touchscreen.

A WS2812B RGB LED ring displays animated color patterns — configurable via a
companion web application — with a dedicated alert mode when a meeting is
approaching. A passive buzzer emits an audio alert before
events start.

The system supports multiple visual themes, selectable in real time from the 
companion web app which communicates with the Python bridge over HTTP. The 
device is housed in a custom 3D-printed enclosure.
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

**DeskCal Controller (Web App)** — a local web application running in the
browser that allows the user to select visual themes, configure LED ring
behavior, adjust brightness, and push quick meetings to the device. Communicates
with the Python bridge over HTTP at localhost:5000.

**Python Bridge Script** — runs on the laptop and acts as the central data
aggregator. Fetches weather data from OpenWeatherMap, events from
Calendar, and news headlines from an RSS feed. Receives theme and configuration
changes from the web app and forwards everything to the STM32 as JSON packets
over USB-UART. Sends time synchronization packets every second
and full data updates every 60 seconds.

**STM32 Nucleo-U545RE-Q** — main controller running embassy-rs. Contains a
dedicated async UART task that receives and parses incoming JSON packets and
dispatches them to the main loop via a channel.

**Display Subsystem** — ILI9341 2.8" TFT touchscreen over SPI renders five
screens: Summary (screen 0), Clock + Meetings (screen 1), Air Quality (screen
2), Weather (screen 3), and News (screen 4). Navigation between screens is
performed via swipe gestures detected on the XPT2046 touchscreen
controller. When a meeting is within 5 minutes, a full-screen alert overlay
appears with a dismiss button.

**Alert Subsystem** — WS2812B LED ring driven via one-wire protocol (using SPI
peripheral for precise timing) displays eight configurable animation modes:
Off, Dim, Solid, Breathe, Pulse, Fast, Strobe, and Chase. The ring switches
to a dedicated alert animation mode when a meeting is approaching. A passive
buzzer driven by PWM emits a tone. Both the
normal ring color/animation and the alert color/animation are configurable in
real time from the web app.
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

### Week 8 & 9
Completed project documentation page. Set up embassy-rs project skeleton for
STM32U545. Verified all components arrived and functional.

### Week 11 - Hardware Milestone
Connected and tested all hardware components: ILI9341 display via SPI,
BME280 sensor via I2C, WS2812B LED ring via SPI, passive buzzer via PWM,
XPT2046 touchscreen, potentiometer via ADC. Designed and 3D printed enclosure.
Implemented basic firmware with display rendering, sensor reading, touch input,
buzzer alerts and LED ring animations.

![Project Wired](./images/wires_project.webp)
![Fusion Enclosure Project](./images/fusion_project.webp)

## Hardware

The project uses an STM32 Nucleo-U545RE-Q as the main microcontroller, running
at 160MHz via PLL configured in firmware.

A 2.8" ILI9341 TFT display with integrated XPT2046 touchscreen controller is
connected via SPI1 (PA5=SCK, PA7=MOSI, PA6=MISO). The display uses a dedicated
SPI chip select (PC9) and control pins for data/command (PC8) and reset (PC6).
The touchscreen controller shares the SPI bus with a separate chip select (PB4)
and an interrupt line (PC13) for touch detection.

A BME280 barometric sensor is connected via I2C1 (PB6=SCL, PB7=SDA) and
provides local temperature, humidity, and atmospheric pressure readings.

A WS2812B 16-LED RGB ring is driven via one-wire protocol using the SPI2
peripheral (PC3=MOSI, PB13=SCK) at 6.4MHz for precise signal timing. Each bit
is encoded as a single SPI byte (0xC0 for logical 0, 0xF8 for logical 1).

A passive buzzer is driven via PWM on TIM1 channel 1 (PA8), producing
configurable frequency tones for meeting alerts.

A 50kΩ potentiometer is connected to ADC1 (PA0) and read periodically. A
tactile button with integrated blue LED provides user interaction via GPIO.

All components are interconnected via jumper wires on a mini breadboard and
housed in a custom 3D-printed PLA enclosure designed in Fusion 360.
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
![KiCad schematic](./images/kikad-deskcal.webp)

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

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for STM32 | Peripheral drivers: SPI, I2C, UART, ADC, PWM, GPIO |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor | Running concurrent tasks on the microcontroller |
| [embassy-sync](https://github.com/embassy-rs/embassy) | Sync primitives | Mutex and Channel for inter-task state sharing |
| [embassy-embedded-hal](https://github.com/embassy-rs/embassy) | Embedded HAL bridge | SpiDevice wrapper for shared SPI bus |

* TODO: the rest will be added as I develop the code

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
8. [KiCad EDA for schematics](https://www.kicad.org)
