# WaterBerry

Automatic irrigation system for more plant pots simultaneously.

:::info 

**Author**: Diaconu Oana-Ruxandra \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-ruxandradiaconu

:::

## Description

WaterBerry – Plant Feeder is an automated watering system for two plant pots, based on soil moisture measurements.  The Raspberry Pi Pico 2W controls two water pumps connected through relay modules, decides pump activation based on sensor readings, and displays the moisture level on an LCD screen. 2 RGB leds provide visual feedback and 2 buttons are there for manual control of the system.



## Motivation


This idea seemed like a suitable one for me, as a person that sometimes forgets that my plants also need to be healthy and fed, so I wanted to create a project that would be useful for me even after this semester. This automated device would be very helful for people who don't know how to really take care of house plants or are too busy for them, as it does the most important part of caring for a pot.



## Architecture

![diagrama bloc](diagrama_proiect.svg)
- **Raspberry Pi Pico 2W** : acts as the main controller of the system, it reads the sensor data, makes decisions based on soil moisture levels, controls the water pumps through relay modules, updates the LCD, and manages LED indications and manual watering buttons

- **Soil Moisture Sensors** : continuously measure the moisture levels in the soil of each plant pot, in order to determine if the soil is too dry or moist enough

- **Water Pumps**: physically water the plants when the soil is detected to be too dry, ensuring individualized watering control

- **Distance Sensor** : measures the distance between the top of the water tank until it reaches with an echo the water level and helps measuring how much water is still left

- **Relay Modules**: they allow the Pico to turn the pumps on/off without being directly exposed to the high current needed by the pumps

- **LCD Display**: displays real-time data about the status of each plant

- **RG LED**: indicates the soil moisture status: green if the soil is moist, red otherwise

- **Push Buttons**: allow the user to manually do watering for each plant, regardless of the soil moisture level

- **Power Supply**: ensures power for some of the components used, providing enough current



## Log

### Week 5 - 11 May
- acquired most components for the project
- connected the sensors and LEDs
- read the input values from the humidity sensors and adjusted them for further display printing(%) 
- implemented the logic for LEDs 
- managed to ruin 1 Pico
### Week 12 - 18 May
- assemble the components together for hardware milestone
- develop the logic and test the functionality of relay modules and water pumps together
- acquired a level shifter for correct control between Pico 2W(3V3) and relay(5V)
- implemented the button logic for activating the relay and the water pump whenever the button is pressed
- started to develop the code for independent functioning
- updated the schematic and the diagram
### Week 19 - 25 May
- acquired diodes and condenser for stability and solved the pumps problems
- implemented logic for the ultrasonic sensor measuring the water level left in the tank and the display
- finished the source code so the two pots are managed individually through different tasks

## Hardware

- 2x Raspberry Pi Pico 2W: controls the components, reads data from the soil moisture sensors, processes the information, controls the relays and pumps, updates the display, and manages LED indicators and button inputs; The second Pico is used for debugging 
- 2x Soil moisture sensors: measure the moisture level in the soil
- 2x Water pumps: deliver water to the plants when the system detects that the soil is too dry
- Distance sensor: measures the distance from the top of the water tank to the level of water, measuring the distance between them
- 2x Relay modules: electronic switches used to safely control the pumps
- LCD display with I2C module: text display used to show the real-time moisture readings of the two plants and how much water is in the water tank
- 2x Red-green LED: shows the status of the soil moisture to be visible by the user
- 2x Push buttons: allows the user to manually control the watering to whenever is wanted
- Breadboard Power Supply: provides stable 3.3V/5V voltage and sufficient current to power some of the peripherals
- Logic Level Shifter: provides correct connection between different voltages(3.3V and 5V)
- Breadboard and jumper wires: used to connect all the components together
- Diodes + condenser : used for stabilizing the power 

![](photo1.webp)
![](photo2.webp)
![](photo3.webp)



## Schematics
![schematic](schematic_proiect.svg)


## Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| [Raspberry Pi Pico 2W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=Raspberry+Pi+Pico+2W&results=26) | Main microcontroller & debugger | 40 RON * 2 (+1) |
| [Soil Moisture Sensor](https://www.optimusdigital.ro/ro/senzori-senzori-de-umiditate/73-senzor-de-umiditate-a-solului.html?search_query=Modul+cu+Senzor+de+umiditate+a+solului&results=1) | Measuring soil moisture | 4 RON * 2 |
| [Relay Module](https://www.optimusdigital.ro/ro/relee/1897-modul-releu-1-canal-5v.html) | Controlling pumps | 5 RON * 2 |
| [LCD 1602 Display with I2C](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/2894-lcd-cu-interfata-i2c-si-backlight-albastru.html?search_query=LCD+1602+cu+Interfata+I2C+si+Backlight+Albastru&results=2) | Displaying sensor values | 16 RON |
| [Red-Green LED](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/704-led-bicolor-de-3-mm-rosu-si-verde-cu-catod-comun.html) | Visual status indication | 1 RON * 2 |
| [Push buttons](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1115-buton-cu-capac-rotund-alb.html?search_query=buton&results=213) | Manual watering control | 2 RON * 2 |
| [Distance sensor](https://www.optimusdigital.ro/ro/senzori-senzori-de-distanta/8150-senzor-de-distana-ultrasonic-hc-sr04p-3-55-v.html?search_query=senzor+distanta&results=180) | Measure left water in tank | 7 RON |
| [Water pump](https://www.bitmi.ro/electronica/mini-pompa-de-apa-submersibila-10452.html) | Deliver water | 8 RON * 2|
| [Breadboard Power Supply](https://www.optimusdigital.ro/en/linear-regulators/61-breadboard-source-power.html) | Power supply | 5 RON |
| [12V 1A Alimentator](https://www.optimusdigital.ro/ro/electronica-de-putere-alimentatoare-priza/2885-alimentator-stabilizat-12v-1000ma.html) | Power supply | 19 RON |
| [Logic Level Shifter](https://www.optimusdigital.ro/ro/interfata-convertoare-de-niveluri/1380-convertor-de-niveluri-logice-bidirecional-pe-8-bii-txs0108e.html?search_query=Convertor+de+Niveluri+Logice+Bidirecțional+pe+8+Biți+TXS0108E&results=1) | Power supply | 6 RON |
| [Resitors 220/22k ohms](https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/10958-rezistor-05w-220.html?search_query=Rezistor+0.5W+220Ω&results=1) | Resitors | ~ 1 RON |
| [Jumper wires](https://www.optimusdigital.ro/ro/fire-fire-mufate/12-set-de-cabluri-pentru-breadboard.html?search_query=Set+Fire+pentru+Breadboard&results=37) | Connections | 8 RON |
| [Breadboard](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/44-breadboard-400-points.html?search_query=Breadboard+HQ+%28400+Points%29&results=1) | Connections | 5 RON * 3 | 


## Software

| Library | Description | Usage |
|---------|-------------|-------|
|embassy-rp | Peripherals access | Initialization and interaction with ADC, GPIO, I2C|
|embassy-executor | Async tasks executor | Non-blocking execution of tasks |
|embassy-time | Non-blocking waits | Delays |
|defmt |Logging | Debugging|
|panic-probe | Panic handler | Print panics|
|hd44780_driver | Output | Print on display|



## Links
- [Setup](https://pmrust.pages.upb.ro/docs/acs_cc/lab/01)
- [Peripherals control](https://pmrust.pages.upb.ro/docs/acs_cc/lab/02)
- [ADC control](https://pmrust.pages.upb.ro/docs/acs_cc/lab/03)
- [Async development](https://pmrust.pages.upb.ro/docs/acs_cc/lab/04)
- [I2C for Display](https://pmrust.pages.upb.ro/docs/acs_cc/lab/06)
- [Inspo 1](https://pmrust.pages.upb.ro/docs/fils_en/project/2024/hadasa.jercau)
- [Inspo 2](https://pmrust.pages.upb.ro/docs/fils_en/project/2024/ana_maria.comeaga)
- [Ultrasonic sensor usage](https://www.youtube.com/watch?v=JvQKZXCYMUM)
- [Power Supply](https://www.youtube.com/watch?v=1er6XQ-BMp4)
- [Sensors reading](https://blog.theembeddedrustacean.com/embedded-rust-embassy-analog-sensing-with-adcs)
- [Display printing](https://crates.io/crates/hd44780-driver)
- [Ultrasonic sensor measurement](https://blog.theembeddedrustacean.com/esp32-embedded-rust-at-the-hal-timer-ultrasonic-distance-measurement)