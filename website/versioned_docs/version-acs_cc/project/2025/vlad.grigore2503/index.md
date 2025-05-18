# Room Thermometer
Displays room temperature or outside weather on a LCD screen, logs measured temperature

:::info 

**Author**: Vlad Grigore \
**GitHub Project Link**: [github.com/UPB-PMRust-Students/proiect-v-grigore](https://github.com/UPB-PMRust-Students/proiect-v-grigore)

:::

## Description

Thermometer that uses a LCD screen to show either room temperature or outside weather. Features:
- Room temperature measuring and display using BMP280 sensor
- Outside weather display using API calls through WIFI connection
- High resolution RGB screen
- Integrated touchscreen for ease of use
- USB module for temperature logging

## Motivation

I usually like to check the outside weather when going out to find out what I should wear. This means opening my phone and waiting for the weather app to refresh every time I want to get dressed. For the project assignment, I wanted to do something practical that I could use in my day to day life, and having a screen that tells me the temperature directly any time I need it is useful to me. I also haven't bought a thermometer for my room yet, another reason to build one for myself.

## Architecture 

![Architecture](./images/architecture.drawio.svg)

- **Raspberry Pi Pico 2W**
    - Central processing unit for all the other components
- **LCD Display with Touchscreen**
    - Used for weather/temperature display
    - Input/Output interface for changing display settings, connecting to WIFI, selecting weather locations etc.
- **BMP280 sensor**
    - Measures local temperature
- **USB Module**
    - The user can supply a storage unit over USB to log and display recorded data
- **LED Button**
    - Push button that acts as a ON/OFF switch for the device

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

- **Raspberry Pi Pico 2W:** the microcontroller
- **3.5'' LCD Module with Touchscreen:** display used for I/O
- **USB CH376S Module:** used for storing recorded temperature values
- **BMP280 Barometric Pressure Sensor:** used for measuring temperature
- **Power Button with Green LED:** ON/OFF switch
- **Micro USB Cable:** power supply
- **Breadboard, Wires:** wiring

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
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [40 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html) |
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Debug probe | [40 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html) |
| [3.5'' LCD Module with Touchscreen](https://www.hpinfotech.ro/ILI9486.pdf) | Display | [100 RON](https://www.optimusdigital.ro/en/lcds/13507-35-lcd-module-with-touchscreen-with-ili9486-and-xpt2046-controller-320x480-px.html) |
| [USB CH376S Module](https://www.mpja.com/download/ch376ds1.pdf) | USB Read-Write module | [18 RON](https://www.optimusdigital.ro/en/others/12543-ch376s-usb-module-u-disk-read-write-module.html) |
| [BMP280 Barometric Pressure Sensors](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmp280-ds001.pdf) | Pressure and temperature sensors | [17 RON](https://www.optimusdigital.ro/en/pressure-sensors/1666-modul-senzor-de-presiune-barometric-bmp280.html) |
| Breadboard HQ 830 Points | Breadboard | [10 RON](https://www.optimusdigital.ro/en/breadboards/8-breadboard-hq-830-points.html) |
| Male-Male Wires | Wires | [5 RON](https://www.optimusdigital.ro/en/wires-with-connectors/884-set-fire-tata-tata-40p-10-cm.html) |
| Male to Female Wires | Wires | [8 RON](https://www.optimusdigital.ro/en/all-products/877-15-cm-40p-male-to-female-wire.html) |
| Pin Headers | Wiring | [2 RON](https://www.optimusdigital.ro/en/pin-headers/85-40p-254-mm-pin-header-200-pcs.html) |
| Power Button with Green LED | ON/OFF Switch | [8 RON](https://www.optimusdigital.ro/en/buttons-and-switches/2182-power-button-with-green-led.html) |
| Micro USB Cable | Connect to power supply | [4 RON](https://www.optimusdigital.ro/en/usb-cables/11939-micro-usb-black-cable-1-m.html) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [reqwless](https://github.com/drogue-iot/reqwless) | Rust async HTTP client for embedded/no_std | Fetch weather data from online API

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...
