# IoT house
A IoT device monitoring the air quality in a house and sends alerts to the user if the air quality is bad.

:::info 

**Author**: Mihăescu Răzvan-Andrei \
**GitHub Project Link**: [link_to_github](https://github.com/UPB-PMRust-Students/proiect-mihaescurazvan.git)

:::

## Description

A device that monitors the air quality in a house using multiple sensors and sends alerts to the user if the air quality is bad. The device will be connected to a Raspberry Pi Pico 2W and will use a display to show the data. 

The indexes that will be monitored are:
- **temperature**
- **humidity**
- **CO**
- **pressure**
- **noise**
- **ultraviolet radiation**
- **fine dust**
- **fire risk**

These indexes will be shown on the display and it will be used an MicroSD card to store the data to be able to show graphs of the data over time. The device will be connected to a Wi-Fi network and will send alerts to the user if the air quality is bad.

In case of bad air quality, the device will start a buzzer to alert the user, and in case of not being at home, the device will send a notification to the user using a server. 

Also, in case of high temperature or humidity, the device is able to turn on a fan to cool the room if the user sets this option.

The LCD display will be use to show the indexes in real time, but also graphs of the data over time. The indexes will be shown in different colors depending on the value of the index. For example, if the temperature is too high, the text will be red, if it is normal, it will be green and if it is too low, it will be blue. The same will be done for all the indexes.

## Motivation

I choose this project because it could have a real usage in my life. I am a person that is very careful about the air quality in my house and I want to be able to monitor it in real time. 

Also, I want to be able to see the data over time and be able to see if there are any changes in the air quality. I also want to be able to receive alerts if the air quality is bad, so I can take action and improve it. 

In this modern world we live in, where air pollution and climate change are becoming more and more of a problem, I want to be very careful about the air quality in my house and prevent any problems that might occur so my health is not affected.

## Architecture 

The **Rasperry Pi Pico 2W** will be the main microcontroller of the project. It will be connected to multiple sensors that will monitor the air quality in the house. 

The **sensors** will be connected to the microcontroller using I2C or SPI communication. The data from the sensors will be read by the microcontroller and will be displayed on the **LCD display**. This data will be stored on a MicroSD card to be able to show graphs of the data over time on the **LCD display**. The microcontroller will also be connected to a Wi-Fi network and will send alerts to the user if the air quality is bad. 

There will also be mechanisms to prevend the rise of the temperature and humidity in the house, like **a fan** that will be turned on automatically if the temperature or humidity is too high.

The device will also have a **buzzer** that will be used to alert the user if the air quality is bad. The buzzer will be turned on automatically if the air quality is bad (eg. if the CO level is too high, if a fire is detected, if dust is too high etc.).

For very high risk, like fire, high measurements of CO or very high temperature, the device will send a notification to the user using **a server** through the Wi-Fi network to alert him even if he is not at home.

![diagram](diagrama-bloc.svg)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

1. **Raspberry Pi Pico 2W**:
- **Function**: Acts as the main controller, coordinating the operations of sensors, buzzer, fan and the LCD display. Also, used to connect to the Wi-Fi network, send alerts to the user and process the data from the sensors.

2. **Raspberry Pi Pico 2W as Debugger**:
- **Function**: Acts as a secondary microcontroller, used to test the code and components before integrating them into the main microcontroller. This will help to identify any issues with the code or components before they are used in the final project.

3. **Prototype board**:
- **Function**: Acts as a base for the project, used to connect all components together.
- **Model**: 10x15cm.

4. **LCD display**:
- **Function**: Acts as a user interface, used to display the data from the sensors in real time and show graphs of the data over time. The LCD will display the data in different colors depending on the value of the index. For example, if the temperature is too high, the text will be red, if it is normal, it will be green and if it is too low, it will be blue. The same will be done for all the indexes.
- **Model**: ILI9341 & XPT2046 (240x320 px).

5. **Temperature and humidity sensor**:
- **Function**: Calculates the temperature and humidity in the house. The data will be displayed on the LCD display and stored on the MicroSD card.
- **Model**: DHT22.

6. **Pressure & Temperature sensor**:
- **Function**: Calculates the pressure and temperature in the house. The data will be displayed on the LCD display and stored on the MicroSD card.
- **Model**: BMP280.

7. **Real-time clock**:
- **Function**: Keeps track of the time and date. The data will be displayed on the LCD display and stored on the MicroSD card.
- **Model**: DS3231.

8. **Carbon monoxide sensor**:
- **Function**: Calculates the CO level in the house. The data will be displayed on the LCD display and stored on the MicroSD card. If the CO level is too high, the buzzer will be turned on to alert the user.
- **Model**: MQ-7.

9. **Noise sensor**:
- **Function**: Calculates the noise level in the house. The data will be displayed on the LCD display and stored on the MicroSD card.

10. **Dust particle sensor**:
- **Function**: Calculates the dust level in the house. The data will be displayed on the LCD display and stored on the MicroSD card.
- **Model**: GP2Y1014AU0F.

11. **Fire sensor**:
- **Function**: Detects flame and smoke in the house. If a fire is detected, the buzzer will be turned on to alert the user and a notification will be sent to the user using the server.
- **Model**: KY-026.

12. **Ultraviolet radiation sensor**:
- **Function**: Calculates the UV radiation level in the house. The data will be displayed on the LCD display and stored on the MicroSD card.
- **Model**: GUVA-S12SD.

13. **Mini fan**:
- **Function**: If the temperature or humidity is too high, the fan will be turned on to cool the room. The user can set this option in the code.

14. **Buzzer module**:
- **Function**: If the air quality is bad (eg. if the CO level is too high, if a fire is detected, if dust is too high etc.), the buzzer will be turned on to alert the user. The buzzer will be turned on automatically if the air quality is bad.

### Schematics

Place your KiCAD schematics here.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico 2W](https://datasheets.raspberrypi.com/picow/pico-2-w-datasheet.pdf) | The microcontroller | [39,66 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html?search_query=raspberry+pi+pico+2&results=36) |
| [Rapspberry Pi Pico 2W](https://datasheets.raspberrypi.com/picow/pico-2-w-datasheet.pdf) | Used as debugger | [39,66 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html?search_query=raspberry+pi+pico+2&results=36) |
| Prototype board | Used to connect all components together | [6,99 RON](https://www.optimusdigital.ro/ro/prototipare-altele/12552-placa-pcb-pentru-prototipare-10x15cm-pas-de-254-mm.html?search_query=Placa+PCB+pentru+prototipare+10x15cm+pas+de+2.54+mm&results=1) |
| [LCD display](https://cdn-shop.adafruit.com/datasheets/ILI9341.pdf) | Used to display the data from the sensors | [69,99 RON](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/3544-modul-lcd-spi-de-28-cu-touchscreen-controller-ili9341-i-xpt2046-240x320-px.html?search_query=%09Modul+LCD+SPI+de+2.8%27%27+cu+Touchscreen+-+Controller+ILI9341+%C8%99i+XPT2046+%28240x320+px%29&results=2) |
| [Temperature and humidity sensor](https://cdn.sparkfun.com/assets/f/7/d/9/c/DHT22.pdf) | Used to measure the temperature and humidity in the house | [22,99 RON](https://www.optimusdigital.ro/ro/senzori-senzori-de-temperatura/1199-senzor-de-temperatura-i-umiditate-dht22.html?search_query=Senzor+de+Temperatura+%C8%99i+Umiditate+DHT22&results=6) |
| [Pressure & Temperature sensor](https://cdn-shop.adafruit.com/datasheets/BST-BMP280-DS001-11.pdf) | Used to measure the pressure and temperature in the house | [8,49 RON](https://www.optimusdigital.ro/ro/senzori-senzori-de-presiune/1666-modul-senzor-de-presiune-barometric-bmp280.html?search_query=Modul+Senzor+de+Presiune+Barometric+BMP280+GY&results=3) |
| [Real-time clock](https://www.analog.com/media/en/technical-documentation/data-sheets/ds3231.pdf) | Used to keep track of the time and date | [18,99 RON](https://www.optimusdigital.ro/ro/altele/1102-modul-cu-ceas-in-timp-real-ds3231.html?search_query=Modul+cu+Ceas+in+Timp+Real+DS3231&results=3) |
| [Carbon monoxide sensor](https://www.pololu.com/file/0j313/mq7.pdf) | Used to measure the CO level in the house | [32,13 RON](https://www.emag.ro/senzor-detectie-monoxid-de-carbon-mq-7-cl208/pd/DFFSGJBBM/?utm_source=cns_payment_accepted&utm_medium=email&utm_campaign=cns_status_update&utm_content=cns_product_title&ref_id=1777940605) |
| [Noise sensor](https://www.openimpulse.com/blog/wp-content/uploads/wpsc/downloadables/Sound-Sensor-Schematic.pdf) | Used to measure the noise level in the house | [4,99 RON](https://www.optimusdigital.ro/ro/senzori-altele/12325-modul-senzor-sunet-fara-cablu.html?search_query=Modul+Senzor+Sunet+%28fara+cablu%29&results=1) |
| [Dust particle sensor](https://www.socle-tech.com/doc/IC%20Channel%20Product/Sensors/Dust%20Sensor/GP2Y1014AU0F_SPEC.pdf) | Used to measure the dust level in the house | [29,99 RON](https://www.optimusdigital.ro/ro/senzori-senzori-optici/2447-senzor-optic-de-particule-de-praf-gp2y1010au0f.html?search_query=Senzor+Optic+de+Particule+de+Praf+GP2Y1014AU0F&results=1) |
| [Fire sensor](https://moviltronics.com/wp-content/uploads/2019/10/KY-026.pdf?srsltid=AfmBOor8DJEfJC36RB7NFECCiRJe-YsMCXLLZZN4_MH0TMZMwkindiLl) | Used to detect flame and smoke in the house | [6,99 RON](https://www.bitmi.ro/electronica/modul-senzor-flacar-ir-ky-026-10657.html) |
| [Ultraviolet radiation sensor](https://cdn-shop.adafruit.com/datasheets/1918guva.pdf) | Used to measure the UV radiation level in the house | [6,20 £](https://thepihut.com/products/adafruit-analog-uv-light-sensor-breakout-guva-s12sd) |
| Mini fan | Used to cool the room if the temperature or humidity is too high | [7,80 £](https://thepihut.com/products/grove-mini-fan-v1-1) |
| Buzzer module | Used to alert the user if the air quality is bad | [4,98 RON](https://www.optimusdigital.ro/ro/altele/1102-modul-cu-ceas-in-timp-real-ds3231.html?search_query=Modul+cu+Ceas+in+Timp+Real+DS3231&results=3) |



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy/tree/main/embassy-rp) | RP2350 Peripherals | Used for accessing the peripherals |
| [embedded-hal](https://crates.io/crates/embedded-hal) | Embedded Hardware Abstraction Layer | Used for accessing the hardware |
| [cortex-m](https://github.com/rust-embedded/cortex-m) | Provides low-level APIs for ARM Cortex-M processors | Interrupt handling and system control |
| [embassy-executor](https://github.com/embassy-rs/embassy/tree/main/embassy-executor) | Asynchronous executor for embedded systems | Used for managing tasks and scheduling |
| [ili9341](http://crates.io/crates/ili9341) | ILI9341 LCD driver | Used for controlling the LCD display |
| [xpt2046](https://crates.io/crates/xpt2046) | XPT2046 touch controller | Used for controlling the touch screen |
| [bmp280](https://crates.io/crates/bmp280) | BMP280 driver | Used for controlling the BMP280 sensor |
| [dht22-driver](https://crates.io/crates/dht22-driver) | DHT22 driver | Used for controlling the DHT22 sensor |
| [ds323x](https://crates.io/crates/ds323x) | DS3231 driver | Used for controlling the DS3231 sensor |



## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

