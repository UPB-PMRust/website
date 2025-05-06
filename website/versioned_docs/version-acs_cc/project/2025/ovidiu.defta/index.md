# AeroGuard: Solar-Powered Atmospheric Pollution Monitor with GPS
Fixed/portable air quality monitoring system, solar-powered, based on Raspberry Pi Pico 2 2350 W

:::info 

**Author**: DEFTA OVIDIU, 335CC \
**GitHub Project Link**: [AeroGuard Project](https://github.com/UPB-PMRust-Students/proiect-odefta)

:::

## Description

AeroGuard is a solar-powered air quality monitoring system that measures particulate matter, nitrogen oxides, volatile organic compounds and others.
The device detects pollution from combustion and traffic, stores data on a microSD card, and provides access to information through a web server with JSON API and a mobile application.

## Motivation

Air pollution is a serious health concern, and having a portable device that can collect accurate data can help people identify polluted zones and take appropriate measures.
The system can track pollution trends over time and provide useful information for research.

Another reason I picked this project is that even the sensors can be (are) quite expensive, similar commercial products can be even more expensive.
Professional air quality monitors with solar power can cost hundreds of dollars, and many require ongoing subscription fees. 
Plus, most commercial solutions only measure a limited set of parameters. By building the project myself, I can customize exactly which pollutants to measure based on local needs.

I also designed the project to be completely autonomous with its solar power and local data storage. 
This means it can work in places without constant electricity or internet access, which is really useful for monitoring remote locations or during power outages.

Finally, I built the system to be modular so it's easy to add new sensors or features as needed. 
If new pollution concerns emerge or better sensor technology becomes available, I can upgrade parts of the system without having to redesign everything from scratch.

## Architecture

![Architecture Diagram](arch.svg)

The AeroGuard architecture includes the following main components:
- **Sensor Module** - collects data about particulate matter, nitrogen oxides, volatile organic compounds, temperature, and humidity
- **GPS Module** - provides geolocation data for the measurements
- **Solar Power System** - ensures the energy autonomy of the device
- **Storage Module** - saves data on the microSD card for later analysis
- **Web Server** - provides access to data through JSON API
- **Mobile Application** - allows graphical visualization of data, statistics, and map location

Data is collected from sensors, associated with GPS coordinates, stored locally, and transmitted via WiFi to the mobile application or other systems.

## Log

### Week 24 April - 1 May
I ordered all the necessary components for the project, with most of them already delivered. 
I conducted initial testing on some components to verify their basic functionality.
I've begun developing the hardware schematic in KiCad, but due to the complexity and number of components involved, this work is still in progress. 
I have specified the interface types and pin connections for all components as detailed below in the hardware section.

**Challenges encountered:**
- Lack of appropriate KiCad symbols for the Raspberry Pi Pico 2 W 2350, as only symbols for older versions are readily available
- No Rust drivers are currently available for the MICS-4514 gas sensor (maybe developing a custom driver or implementing an FFI interface to existing C libraries?)
- Some uncertainty remains regarding the optimal breadboard connections for certain sensors, particularly those with non-standard pin layouts
- Still researching the exact design for the power management system to integrate the solar panel, TP4056 charging module, and battery with the Raspberry Pi Pico

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

AeroGuard uses a Raspberry Pi Pico 2 2350 W as the central microcontroller, connected to a series of specialized sensors for air quality monitoring. 
The system includes:

### Sensors
- **PMS5003** particulate matter sensor for PM1.0, PM2.5, and PM10 (UART interface, connected to GPIO0/1)
- **MICS-4514** dual-channel gas sensor for NOx/CO detection (Analog interface, connected to ADC0/ADC1)
- **SGP40** VOC sensor (I2C interface, connected to GPIO4/5)
- **BME280** temperature, humidity, and pressure sensor (I2C/SPI interface, shared I2C bus with SGP40)
- **NEO-6M GPS module (GYNEO6MV2)** for position (Lat/Long), altitude, and time/date (UART interface, connected to GPIO8/9)

### Power System
- **Solar Panel 12V, 350mA, 4.2W** (higher capacity than initially planned)
- **TP4056 Charger** with battery charging controller and over/under voltage protection
- **18650 Battery** (3.7V Li-Ion, ~3000mAh capacity) providing power supply for the system

### Storage
- **MicroSD Card Module** (SPI interface, connected to GPIO16-19) for storing pollution history logs, GPS coordinates tracking, and time series data

### Connectivity
- **WiFi Access Point** using the Pico 2 W's built-in wireless capabilities
- **Server Web & API JSON** endpoints for current data, historical data, battery status, and GPS location
- **Mobile Application** interface for pollution trend graphs, map visualization, battery/solar charging monitoring, and pollution level alerts

### Additional Components
- Breadboard for prototyping
- Jumper wires for connections
- Resistors (10kΩ, 4.7kΩ) for pull-ups and voltage dividers
- Capacitors (100nF, 10μF) for power stabilization
- Protective enclosure (TBD to accommodate all components)

### Schematics

The hardware schematic is being developed in KiCad.

### Bill of Materials

| Device | Usage | Price |
|----|----|----|
| [Raspberry Pi Pico 2 2350 W](https://www.raspberrypi.com/products/raspberry-pi-pico/) | Central microcontroller | [40 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?srsltid=AfmBOoo3VoaSxG2hFXm-a4rzixxQRCP1imo3My6atWk5yABGlb1mq7fR) |
| [PMS5003 Particle Sensor](https://www.adafruit.com/product/3686) | Measuring particulate matter (PM1.0, PM2.5, PM10) | [139 RON](https://www.emag.ro/senzor-de-puritate-a-aerului-pms5003-si-de-praf-pm2-5-elektroweb-50-x-38-x-20-mm-albastru-3-b-062/pd/DT5P7MYBM/) |
| [MICS-4514 NOx/CO Sensor](https://www.sgxsensortech.com/content/uploads/2014/08/0278_Datasheet-MiCS-4514-rev-16.pdf) | Detection of nitrogen oxides and carbon monoxide | [63 RON](https://ro.farnell.com/amphenol-sgx-sensortech/mics-4514/robust-mems-gas-detect-sensor/dp/3550711?srsltid=AfmBOooDpS4hLmMVFEw2MLMrhnpXEAY7rpTMGww7tMCeuKm-e0TwPmpB) |
| [SGP40 VOC Sensor](https://sensirion.com/products/catalog/SGP40/) | Measuring volatile organic compounds | [112 RON](https://www.electronic-mag.ro/alti-senzori/25545-senzor-atmosferic-calitate-aer-i2c-33-5vdc-gravity-ic-sgp40-5941101285192.html) |
| [BME280 Sensor](https://www.bosch-sensortec.com/products/environmental-sensors/humidity-sensors-bme280/) | Measuring temperature and humidity | [120 RON](https://www.optimusdigital.ro/ro/senzori/12977-senzor-de-temperatura-si-umiditate-gy-bme280-5v.html?search_query=GY-BME280-5V&results=1) |
| [NEO-6M GPS Module](https://www.dev.faranux.com/product/gyneo6mv2-gps-module-neo-6m-gy-neo6mv2/) | Geolocation | [70 RON](https://www.optimusdigital.ro/ro/gps/2137-modul-gps-gyneo6mv2-cu-antena-in-miniatura.html?search_query=GYNEO6MV2&results=1) |
| [12V/4.2W Solar Panel](https://www.amazon.com/Koieam-200x130mm-Conversion-Charger-Cellphone/dp/B0DS29VHY9) | Solar power | [70 RON](https://sigmanortec.ro/panou-solar-12v-350ma-42w-200x130mm) |
| [TP4056 Charging Module](https://quartzcomponents.com/products/tp4056-battery-charging-module) | Battery charging management | [15 RON](https://www.optimusdigital.ro/ro/electronica-de-putere-incarcatoare/219-incarcator-tp4056-cu-mini-usb-pt-baterie-lipo-1a-.html?search_query=TP4056+&results=4) |
| [18650 Li-Ion Battery] | Energy storage (3.7V, 2000mAh) | [35 RON](https://depozitulcubaterii.ro/home/151-acumulator-samsung-inr18650-35e-3500mah-li-ion-37v-182x650mm-8a.html) |
| [Modul MicroSD] | Data storage | [4 RON](https://sigmanortec.ro/Modul-MicroSD-p126079625) |
| [Breadboard](https://www.adafruit.com/product/64) | Prototyping | [30 RON](https://www.emag.ro/breadboard-1660-puncte-4-linii-alimentare-ai2097/pd/D30MN6YBM/?path=breadboard-1660-puncte-4-linii-alimentare-ai2097/pd/D30MN6YBM) |
| [Jumper Wires, Resistors, Capacitors, etc.] | Component connections | [60 RON](https://www.emag.ro/set-componente-electronice-breadboard-830-puncte-led-uri-compatibil-arduino-si-raspberry-pi-zz00044/pd/DRXG4XYBM) |

## Software

| Library | Description | Usage |
|----|----|----|
| [embassy-rs](https://github.com/embassy-rs/embassy) | Asynchronous framework for microcontrollers | Basic application architecture |
| [embassy-time](https://github.com/embassy-rs/embassy) | Time functionalities for Embassy | Management of timed operations |
| [embassy-rp](https://github.com/embassy-rs/embassy) | Support for Raspberry Pi Pico | Interfacing with Pico hardware |
| [embassy-net](https://github.com/embassy-rs/embassy) | Network stack for Embassy | WiFi communication |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstractions for embedded | Peripheral interfacing |
| [serde](https://github.com/serde-rs/serde) | Data serialization/deserialization | Data processing |
| [serde_json_core](https://github.com/rust-embedded-community/serde-json-core) | JSON support for embedded | JSON API |
| [heapless](https://github.com/japaric/heapless) | Data structures without dynamic allocation | Efficient memory management |
| [defmt](https://github.com/knurling-rs/defmt) | Logging for embedded | Debugging and monitoring |
| [embedded-sdmmc](https://github.com/rust-embedded-community/embedded-sdmmc-rs) | Support for microSD card | Data storage on SD card |
| [bme280](https://github.com/VersBinarii/bme280-rs) | Driver for BME280 sensor | Temperature/humidity/pressure measurement |
| [picoserve](https://github.com/sammhicks/picoserve) | Lightweight HTTP server | Serving data via web |
| [nmea0183](https://github.com/AeroRust/nmea) | Parser for GPS data | Processing location data |
| [sgp40-driver](https://docs.rs/sgp40/latest/sgp40/) | Driver for SGP40 sensor | Measuring volatile organic compounds |
| [pms5003-driver](https://crates.io/crates/pms5003) | Driver for PMS5003 sensor | Measuring particulate matter |

## Links

1. [Raspberry Pi Pico W Documentation](https://datasheets.raspberrypi.com/picow/pico-2-w-datasheet.pdf)
2. [Embassy-rs Framework](https://embassy.dev/)
3. [Embedded Rust Programming](https://docs.rust-embedded.org/book/)