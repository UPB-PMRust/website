# Automated Plant Watering System 


Automates plant watering based on soil moisture and provides monitoring via a mobile application


:::info 

**Author**: Manolache Alexandra Claudia

**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-alexandra33312

:::

## Description
This project automates plant watering using a Raspberry Pi Pico W microcontroller and a mobile application for remote monitoring and control. A soil moisture sensor detects the moisture level, and when the soil becomes too dry, the Pico W activates a relay module that powers a water pump.
The mobile app communicates with the Pico W via a simple REST API, displaying real-time data and allowing manual watering from anywhere.


## Motivation
I chose this project to optimize water usage and ensure plant health by eliminating the risk of overwatering or underwatering. Integrating a mobile app offers maximum flexibility and an intuitive way to monitor soil conditions and watering activity. Additionally, the system can be scheduled to water at specific time intervals.

## Architecture 

The system architecture is divided into four main logical components:

**Sensing Layer** : Soil Moisture Sensor Module: Reads analog moisture values from the YL-69 sensor and converts them into digital readings via the Pico W’s ADC.

**Control Layer** - Raspberry Pi Pico W: Hosts the core logic, compares sensor readings against thresholds, and decides when to activate the irrigation. Exposes a REST API over Wi-Fi for status reporting and remote control.

**Actuation Layer**- Relay & Pump Module: Receives digital control signals from the Pico W GPIO to switch the relay, which powers the water pump.
Feedback Indicators: LED and buzzer provide local visual and audible status alerts (e.g., watering in progress, errors).

**Presentation & Management Layer** - Mobile Application (Android/iOS): Polls the Pico W REST API to display real-time moisture data, pump status, and watering history. Allows manual triggering of irrigation via POST requests.

**Data Flow & Integration** : 
- The sensor module measures soil moisture and sends the analog signal to the Pico W.
- The control logic on the Pico W evaluates the reading and, if dry, activates the relay to power the pump.
- The pump waters the plants, and the Pico W logs the event.
- The Pico W REST API provides endpoints such as /status (GET moisture, pump state) and /water (POST to trigger watering).
- The mobile app periodically polls /status and displays data in the UI; a manual button invokes /water to start irrigation on demand.

![Architecture Diagram](schematics/architecture.svg)
Schematics

## Log 

### Week 5 - 11 May
Set up development environment for Raspberry Pi Pico W (VS Code + Pico SDK)
Researched YL‑69 soil moisture sensor and tested basic analog readings on Pico W
Drew initial block diagram of system architecture
Make  Kicad schematics

### Week 12 - 18 May
Calibrated moisture sensor thresholds (dry vs. wet) through voltage measurements
Implemented and tested ADC reading code on Pico W
Integrated 5 V relay module; wrote GPIO control routines to switch pump on/off.

### Week 19 - 25 May
Developed simple REST API on Pico W (endpoints /status and /water)
Created basic KiCad schematic for sensor, relay, and power connections
Began prototyping mobile app UI in React Native (status screen and manual control button)

## Hardware
The following hardware components are used in this project to sense soil moisture, control irrigation, and provide power and feedback:
Raspberry Pi Pico W: Main microcontroller with integrated Wi‑Fi and 12‑bit ADC (for sensor reading)
YL‑69 Soil Moisture Sensor Module: Analog sensor probes + comparator module for measuring soil moisture
5 V Relay Module: Drives the water pump by switching its power on/off via GPIO signal
DC Water Pump (3–6 V): Pumps water through tubing to irrigate plants
6 mm Silicone Tubing: Carries water from the pump to the plant pots
10 kΩ Resistor: Forms a voltage divider with the YL‑69 sensor for stable ADC readings
LED (Red/Green): Visual feedback for system status (e.g., watering, idle, error)
5V Buzzer: Audible alert for errors or watering start/stop
Jumper Wires & Breadboard: Prototyping and connections between components
5V 3A Power Supply: Powers the relay module and pump; Pico W is powered via the same supply (through breadboard power lines)

### Schematics

Place your KiCAD schematics here 

[Work in progress...]

### Bill of Materials


| Device                  | Usage                           | Price |
|-------------------------|---------------------------------|-------|
| Raspberry Pi Pico W     | Main microcontroller            | 70x2  |
| Soil Moisture Senzor    |Soil moisture measurement        | 3.99  |
| 5V Relay Module         |Switches pump power for          | 4.99  |
|                         |irrigation                       |       |
| DC Water Pump (3–6V)    |Pumps water through tubing       | 16    |
| 6 mm Silicone Tubing    |Transports water to plant        | 10    |
| 10 kΩ Resistor          |Voltage divider for sensor       | 1     |
| Jumper Wires            |Connections between components   | 12    |
| Breadboard              |Prototyping platform             | 10    |
| 5V 3A Power Supply      |Powers Pico W and pump           | 24    |
| Red/Green LED           |Visual feedback indicator        | 1     |
| 5V Buzzer                |Audible alert                   | 1     |


Raspberry Pi Pico W https://www.emag.ro/placa-de-baza-raspberry-pi-pico-2w-ai2237/pd/D1YX6B3BM/?X-Search-Id=f0dcce3aa6dd09c15077&X-Product-Id=226605089&X-Search-Page=1&X-Search-Position=2&X-Section=search&X-MB=0&X-Search-Action=view

Soil Moisture Senzor https://www.optimusdigital.ro/ro/senzori-senzori-de-umiditate/73-senzor-de-umiditate-a-solului.html?search_query=umiditate&results=165

5V Relay Module      https://www.optimusdigital.ro/ro/cautare?controller=search&orderby=position&orderway=desc&search_query=5v+releu&submit_search=

DC Water Pump (3–6V) https://www.emag.ro/pompa-de-apa-3-6v-cl88/pd/D91Z5JBBM/#reviews-section

6 mm Silicone Tubing https://claumarpescar.ro/tub-siliconic-mostiro-1m-0.6mm.html?srsltid=AfmBOorzXBO0eFCGJUn9feYSHYxty-69NSNSLf_W9DOd_fCkD1tnU4bg

10 kΩ Resistor       https://www.optimusdigital.ro/ro/kituri/4745-set-de-rezistoare-asortate-optimus-digital-intre-10-1-m-600-buc-0616639927610.html?search_query=rezistor&results=110

Jumper Wires         https://www.optimusdigital.ro/ro/fire-fire-mufate/891-set-fire-tata-tata-10p-30-cm.html?search_query=fire&results=429
                     https://www.optimusdigital.ro/ro/toate-produsele/877-set-fire-mama-tata-40p-15-cm.html?search_query=Fire+Colorate+Mama-Tata+%2840p%29&results=5

Breadboard           https://www.optimusdigital.ro/ro/kituri/9698-kit-plusivo-pi-4-fara-placa-i-fara-card.html?search_query=KIT&results=324

5V 3A Power Supply   https://www.emag.ro/sursa-de-alimentare-5v-3a-pa5-3/pd/DJYZ6TMBM/

Red/Green LED        https://www.optimusdigital.ro/ro/kituri/9698-kit-plusivo-pi-4-fara-placa-i-fara-card.html?search_query=KIT&results=324

5V Buzzer            https://www.optimusdigital.ro/ro/kituri/9698-kit-plusivo-pi-4-fara-placa-i-fara-card.html?search_query=KIT&results=324



## Software
 
Firmware (Rust / Pico W)

| Library                                        | Description                                         | Usage                                            |
|------------------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| `rp2040-hal`                                   |Hardware Abstraction Layer pentru RP2040             |GPIO, ADC, PWM, UART and I²C control              |
| `embassy` + `embassy-rp`                       |Async executor & suport specific RP2040              |Concurrent task scheduling                        |
| `embedded-hal`                                 | Standard HAL traits                                 | Unified API for GPIO/ADC interactions            |
| `embedded-graphics`                            | Grafica 2D pe display                               | Drawing text and icons on the ST7789 display     |
| `st7789`                                       | Driver display ST7789                               | Low-level control of the SPI-driven display      |
| `defmt`                                        | Framework de logging eficient                       | Lightweight debug output over serial/USB         |
| `serde` + `serde_json`                         | Serializare JSON                                    |JSON parsing and formatting for the REST API      |





Mobile APP

| Library                                        | Description                                         | Usage                                            |
|------------------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| `react-native`                                 | Cross-platform mobile framework                     |  Builds Android & iOS user interfaces            |
| `react-navigation`                             | Routing and navigation                              |  Screen and stack management                     |
| `axios`                                        | Promise-based HTTP client                           | Fetching GET /status and POST /water             |
| `victory-native`                               | Charting library                                    | Rendering moisture history graphs                |
|`react-native-push-notification`                | Push notifications                                  | Alerts when moisture is below critical level     |



## Links

https://datasheets.raspberrypi.com/picow/pico-2-w-datasheet.pdf

