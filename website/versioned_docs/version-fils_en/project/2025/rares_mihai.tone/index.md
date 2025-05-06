# Smart Weather Station
A Raspberry Pi-based weather monitoring system that collects temperature, humidity, air pressure, wind speed/direction, and rainfall data, displaying it locally and on a web dashboard.

:::info 
**Author**: Tone Rares-Mihai \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-toniyiy
:::

## Description
This project uses a Raspberry Pi to collect environmental data from multiple sensors:
- **DHT22**: Temperature and humidity
- **BMP280**: Atmospheric pressure
- **Hall Sensor 3144**: Wind speed (via anemometer)
- **Wind Vane**: Wind direction
- **Rain Sensor**: Rainfall detection

Relevant labs/concepts applied:
- SPI/I2C communication
- Sensor calibration
- Web server development (Django)
- Real-time data visualization

## Motivation
I wanted to build a practical IoT device that bridges hardware and software, while contributing to environmental monitoring. This project challenges me to integrate multiple sensor protocols, design a user-friendly dashboard, and apply embedded systems knowledge from coursework.

---

## Architecture
### Schematic Diagram
![Schematic]

**Key Components**:
1. **Raspberry Pi**
   - Central controller for data collection and processing
   - Interfaces with all sensors via GPIO/I2C/SPI
2. **Hall Sensor 3144 + Anemometer**
   - **Interface**: GPIO with interrupts
   - **Role**: Measures wind speed by detecting rotations of the anemometer cups
3. **Wind Vane**
   - **Interface**: Analog-to-Digital Converter (ADC)
   - **Role**: Determines wind direction using voltage output
4. **DHT22 & BMP280**
   - **Interface**: I2C
   - **Role**: Core environmental sensing

---

## Log 
### Week 1

### Week 2
![Prototype]
---

## Hardware
### Bill of Materials
| Component               | Purpose                  | Price  | Link |
|-------------------------|--------------------------|--------|------|
| Raspberry Pi 2W         | Main controller          | 40 RON | [Link] |
| Debugger for Pico       | Debugger                 | 66 RON | [Link] |
| DHT22 Sensor            | Temp/Humidity            | 25 RON | [Link] |
| BMP280 Sensor           | Air pressure             | 35 RON | [Link] |
| Hall Sensor 3144        | Anemometer integration   | 15 RON | [Link] |
| Wind Vane               | Direction measurement    | 5  RON | [Link] |
| Rain Sensor             | Rainfall detection       | 20 RON | [Link] |

---

## Software
| Tool/Library            | Purpose                          | Link |
|-------------------------|----------------------------------|------|
| Python 3.10             | Core programming language        | [Python]     |
| Adafruit_DHT            | DHT22 sensor library             | [GitHub]     |
| Django                  | Web dashboard framework          | [Django]     |
| RPi.GPIO                | Hall sensor/anemometer interface | [Docs]       |
| Matplotlib              | Data graphing                    | [Matplotlib] |

---

## Links
