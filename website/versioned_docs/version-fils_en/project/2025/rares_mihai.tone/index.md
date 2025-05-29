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
- **YS-27**: Wind speed (via propeller that has magnets attached to it, it keeps track of the propellers RPMs counting how many times the magnets pass the sensor)
- **YL-38**: Rainfall detection

Relevant labs/concepts applied:
- I2C communication
- Sensor calibration
- Web server development (Django)
- Real-time data visualization

## Motivation
I wanted to build a practical IoT device that bridges hardware and software, while contributing to environmental monitoring. This project challenges me to integrate multiple sensor protocols, design a user-friendly dashboard, and apply embedded systems knowledge from coursework, which i find very interesting. 

---

## Architecture
### Connections 
![Components](comp_diagram.webp)
### Schematic Diagram
![Electric Schematic](SmartWeatherStation.svg)


**Key Components**:
1. **Raspberry Pi**
   - Central controller for data collection and processing
   - Interfaces with all sensors via GPIO/I2C
2. **YS-27 module with a3144 hall effect sensor + Propeller**
   - **Interface**: GPIO with interrupts
   - **Role**: Measures wind speed and direction using the magnets attached to the propeller 
3. **DHT22 & BMP280 & YL-38**
   - **Interface**: I2C
   - **Role**: Core environmental sensing, like rain (YL-38), temperature and humidity (DHT22) and atmospheric pressure (BMP280)
4. **OLED FeatherWing 128x32**
   - **Interface**: I2C
   - **Role**: Display information locally
---

## Log 

### Week 1

The first week of the project was intense, but i got around to setting up the sensors, and even got around to working on the code. I hope the code will turn out nicely as well in the end. The only thing left for the hardware part is setting up the propeller, which im not sure yet how to do. I do have some ideas but i need to experiment. Also, i started working on the code for the other sensors, but i have some problems with the dht22 and the pressure bme280 sensor. The only one i got to work (other than the display) is the rain module, which displays if it's raining or not.
This is how it looks so far:
![first pic](hardware1.webp)
![second](hardware2.webp)

### Week 2

The second and third week were very stressful, i hit a lot of roadbumps that i am going to write about.
The first difficult step was wifi integration, which took me about one day, and after some extensive searching i found a repository in github that helped me a little bit. I will leave the link below, at the links tab. The second major step for me was creating a website to showcase my weather station's readings. The website was made in django and it wasn't that difficult to create. The problem, however, was that it was my first time using API's to store my sensor reading in, and i had a difficult time setting up the views.py of the project. After successfully displaying some test variables (i used CURL to write some values), i started the rust part of the connection, which took me a long time to accomplish, but after talking to class mates that had similar projects, i managed to finally make the connection between the pico and the API. Im going to attach a picture of the django frontend page, as well as the backend page from development, where i check the API.
![frontend](api.webp)
![backend](apisensor.webp)
After finishing the code, i started working on the hardware part some more. I wanted it to be more compact, so i got rid of the alternator i usually used to power the project, and replaced with some rechargeables batteries, with voltage up to 11.1 volts, plenty for my project. Also, i changed the windmill i had in the first place, and replaced it with a bigger one.
![windmill](windmill.webp)
This windmill made everything easier, given its large size. After using some neodym magnets, as you will see in the photos, the next challenge was setting up the hall effect sensor, which i ended up placing in a small box, and then attaching said small box using some zipties.
![hall](hall.webp)
The sensor picks on the magnetic pulses quite well, im proud of the way i improvised this. Overall, it was an exciting project, and im excited to present it at PMFair, i hope all will go well. This is how my project looks:
![finished](finished.webp)
![conf](configuration.webp)
---

## Hardware
### Bill of Materials
| Component                | Purpose                  | Price   | Link |
|--------------------------|--------------------------|---------|------|
| Raspberry Pi 2W          | Main controller          |  40 RON | https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=Raspberry+Pi+Pico+2W&results=26 |
| Debugger for Pico        | Debugger                 |  66 RON | https://www.optimusdigital.ro/ro/accesorii/12777-placa-pentru-depanare-raspberry-pi.html?search_query=Placa+pentru+Depanare+Raspberry+Pi&results=5 |
| DHT22 Sensor             | Temp/Humidity            |  25 RON | https://www.optimusdigital.ro/ro/senzori-senzori-de-temperatura/3157-senzor-de-temperatura-i-umiditate-dht22am2302b.html?search_query=dht22&results=6 |
| BMP280 Sensor            | Air pressure             |  35 RON | https://www.optimusdigital.ro/ro/senzori-senzori-de-presiune/1666-modul-senzor-de-presiune-barometric-bmp280.html?search_query=Modul+Senzor+de+Presiune+Barometric+BMP280+GY&results=3 |
| YS-27                    | Anemometer integration   |  15 RON | https://www.optimusdigital.ro/ro/senzori-senzori-hall/596-modul-cu-senzor-hall-ys-27.html?search_query=Modul+cu+Senzor+Hall+YS-27&results=17 |
| YL-38                    | Rainfall detection       |  20 RON | https://www.optimusdigital.ro/ro/senzori-senzori-de-umiditate/5775-modul-senzor-de-ploaie.html?search_query=Modul+Senzor+de+Ploaie&results=1 |
| FeatherWing OLED Display | Local display of info    | 100 RON | https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/3335-shield-featherwing-cu-ecran-oled-128-x-32-pentru-adafruit-feather.html?search_query=Shield+FeatherWing+cu+Ecran+OLED+128+x+32+pentru+Adafruit+Feather&results=1 |
---

## Software
| Tool/Library            | Purpose                          | Link |
|-------------------------|----------------------------------|------|
| Rust with Embassy       | Core programming language        | https://embassy.dev/                               |
| Django                  | Web dashboard framework          | https://www.djangoproject.com                      |
| Django Rest Framework   | flexible tool for building API's | https://www.django-rest-framework.org              |
---

## Links
1: https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/wifi_tcp_server.rs
2: https://medium.com/@michal.drozdze/setting-up-a-django-api-with-django-rest-framework-drf-a-beginners-guide-cee5d61f00a6
