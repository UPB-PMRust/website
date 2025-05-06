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

---

## Hardware
### Bill of Materials
| Component               | Purpose                  | Price   | Link |
|-------------------------|--------------------------|---------|------|
| Raspberry Pi 2W         | Main controller          |  40 RON | https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=Raspberry+Pi+Pico+2W&results=26 |
| Debugger for Pico       | Debugger                 |  66 RON | https://www.optimusdigital.ro/ro/accesorii/12777-placa-pentru-depanare-raspberry-pi.html?search_query=Placa+pentru+Depanare+Raspberry+Pi&results=5 |
| DHT22 Sensor            | Temp/Humidity            |  25 RON | https://www.optimusdigital.ro/ro/senzori-senzori-de-temperatura/3157-senzor-de-temperatura-i-umiditate-dht22am2302b.html?search_query=dht22&results=6 |
| BMP280 Sensor           | Air pressure             |  35 RON | https://www.optimusdigital.ro/ro/senzori-senzori-de-presiune/1666-modul-senzor-de-presiune-barometric-bmp280.html?search_query=Modul+Senzor+de+Presiune+Barometric+BMP280+GY&results=3 |
| Hall Sensor 3144        | Anemometer integration   |  15 RON | https://www.optimusdigital.ro/ro/senzori-senzori-hall/596-modul-cu-senzor-hall-ys-27.html?search_query=Modul+cu+Senzor+Hall+YS-27&results=17 |
| Wind Vane               | Direction measurement    |  5  RON | https://www.optimusdigital.ro/ro/mecanica-elice/421-elice-albastra-de-80-mm.html?search_query=Elice+Albastra+de+80+mm&results=3 |
| Rain Sensor             | Rainfall detection       |  20 RON | https://www.optimusdigital.ro/ro/senzori-senzori-de-umiditate/5775-modul-senzor-de-ploaie.html?search_query=Modul+Senzor+de+Ploaie&results=1 |
| Oled Display            | Local display of info    | 100 RON | https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/3335-shield-featherwing-cu-ecran-oled-128-x-32-pentru-adafruit-feather.html?search_query=Shield+FeatherWing+cu+Ecran+OLED+128+x+32+pentru+Adafruit+Feather&results=1 |
---

## Software
| Tool/Library            | Purpose                          | Link |
|-------------------------|----------------------------------|------|
| Python 3.10             | Core programming language        | https://www.python.org     |
| Adafruit_DHT            | DHT22 sensor library             | https://github.com/adafruit/DHT-sensor-library     |
| Django                  | Web dashboard framework          | https://www.djangoproject.com     |
| RPi.GPIO                | Hall sensor/anemometer interface | https://pypi.org/project/RPi.GPIO/       |
| Matplotlib              | Data graphing                    | https://matplotlib.org |

---

## Links
