# Eq-Detector
An earthquake detector with real-time seismic graph visualization.

:::info 

**Author**: Mitran Ramona Luminita \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-mnoramona

:::

## Description

Eq-Detector is an earthquake detector built using a Raspberry Pi Pico 2W and an ADXL335 accelerometer. It monitors 3-axis vibration levels in real time and triggers alerts when exceeding a predefined threshold. The project also includes a Python-based visualization component that plots seismic data live via serial communication.

The embedded software is written in **Rust**, using the `rp2040-hal` crate for hardware abstraction and `embedded-hal` for peripheral control. LCD communication is done via I2C, and accelerometer data is acquired using ADC pins.

## Motivation

Earthquakes pose a serious risk, especially in areas with poor infrastructure. This project aims to offer a low-cost, accessible, and educational way of building a basic seismic alert system. It merges electronics with real-time data visualization to provide a hands-on learning experience.

## Architecture 

The architecture of the Eq-Detector system is centered around the Raspberry Pi Pico 2W microcontroller, which interfaces with multiple peripheral devices to detect and respond to seismic activity.

An ADXL335 accelerometer is connected to the Pico via its ADC0 pin, providing analog data about vibrations across one or more axes. The Pico processes this data in real time.

If the detected vibration exceeds a predefined threshold, the Pico triggers two output signals:

A buzzer for an audible alert.

An LED connected via GPIO2 for a visual alert.

The system also includes a 16x2 I2C LCD display, which receives data through the I2C interface, showing real-time information such as vibration intensity or alert status.

Additionally, the Pico sends data via serial connection to a PC, where a Python application using Matplotlib can graphically visualize the vibration levels over time.

![Figure 1: System Architecture](Arhitectura.webp)

## Log

### Week 5 - 11 May
- Selected and ordered hardware components (Pico 2W, ADXL345, I2C LCD, buzzer, LED).
- Researched accelerometer communication protocols (I2C vs analog).
- Verified I2C compatibility between ADXL345 and LCD1602 on same bus.

### Week 12 - 18 May
- Built and refined the breadboard prototype.
- Connected all components (ADXL345, LCD I2C, buzzer, LED) to the Raspberry Pi Pico 2W.
- Verified power distribution and I2C wiring.
- Used Fritzing to document and organize all physical connections.
- Ordered additional components (secondary Pico, more wires, bigger breadboard)

### Week 19 - 25 May
TBC

## Hardware

- **Microcontroller:** Raspberry Pi Pico 2W (RP2040-based)
- **Sensor:** ADXL345 (3-axis digital accelerometer, communicates via I2C)
- **Output Devices:** 16x2 I2C LCD Display, Buzzer, Red LED
- **Other:** Breadboard, Jumper wires, USB cable

### Schematics
The connections:
1.	ADXL345 Accelerometer:
•	SDA (Serial Data): Connected to the GP0 pin on the Pico (I2C SDA).
•	SCL (Serial Clock): Connected to the GP1 pin on the Pico (I2C SCL).
•	GND: Connected to the Pico’s GND pin.
•	VCC: Connected to the Pico’s 3V3(OUT) pin for power.
This sensor transmits acceleration values along the X, Y, and Z axes using the I2C communication protocol.
2.	LCD 1602 I2C Display:
•	VCC: Connected to the Pico’s 3V3(OUT) pin.
•	GND: Connected to GND.
•	SDA: Connected to GP0 (shared with the ADXL345).
•	SCL: Connected to GP1 (shared with the ADXL345).
The LCD is used to display real-time acceleration data or status messages such as "NORMAL" or "ALERT!".
3.	Buzzer:
•	Positive terminal (+): Connected to GP2 through a 220ohm resistor.
•	Negative terminal (–): Connected to GND.
The buzzer emits a sound when the detected vibration exceeds the defined threshold.
4.	Led Indicator:
•	Anode (positive leg): Connected to GP3 through a 330-ohm resistor.
•	Cathode (negative leg): Connected to GND.
The LED turns on to visually indicate that an earthquake or vibration threshold has been breached.


![Figure 2: Electrical Schematic](Schematic.webp)
![Figure 3: Breadboard Connections](Breadboard.webp)

### Bill of Materials


| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/products/raspberry-pi-pico-2/)   | Rust-capable MCU              | [~40 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=raspberry+pi+pico+2w&results=26)  |
| [ADXL345 Accelerometer](https://www.analog.com/en/products/adxl335.html)  | Vibration sensor              | [~13 RON](https://www.optimusdigital.ro/ro/senzori-senzori-inertiali/97-modul-accelerometru-cu-3-axe-adxl345.html?search_query=Modul+accelerometru+3+axe+I2C/SPI+ADXL345&results=2)  |
| 16x2 I2C LCD           | Display output                | [~17 RON](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/2894-lcd-cu-interfata-i2c-si-backlight-albastru.html?gad_source=1&gad_campaignid=20868596392&gclid=Cj0KCQjw_dbABhC5ARIsAAh2Z-Rt1ZinLQ3VelBRdn2Qz-2a6QBfN8UXfIORv22csIZS3gs_NsfPR34aAgZaEALw_wcB)  |
| Buzzer                 | Audible alarm                 | [~2 RON](https://www.optimusdigital.ro/ro/audio-buzzere/634-buzzer-pasiv-de-5-v.html?search_query=buzzer&results=63)   |
| Red LED                | Visual indicator              | [~0.50 RON](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/696-led-rou-de-3-mm-cu-lentile-difuze.html?search_query=led&results=779) |
| Breadboard             | Prototyping                   | [~4 RON](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/13249-breadboard-300-puncte.html?search_query=breadboard&results=127)   |
| Jumper Wires (M-M; M-T; T-T)     | Wiring                        | [~4 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/91-fire-colorate-mama-mama-10p.html)   |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [rp2040-hal](https://github.com/UPB-PMRust-Students/proiect-mnoramona)         | RP2040 Hardware Abstraction Layer  | Used for peripheral control (ADC, GPIO, I2C) |
| [embedded-hal](https://github.com/UPB-PMRust-Students/proiect-mnoramona)       | Hardware abstraction traits        | Trait-based drivers for Rust embedded |
| [lcd1602-rs](https://github.com/UPB-PMRust-Students/proiect-mnoramona)         | LCD I2C display library            | Used for displaying sensor values  |
| [defmt + panic-probe](https://github.com/UPB-PMRust-Students/proiect-mnoramona) | Debug logging and panic handling   | Used for debugging on embedded target |
| [pyserial (Python)](https://github.com/UPB-PMRust-Students/proiect-mnoramona)  | Serial communication               | Reads data from Pico in Python     |
| [matplotlib (Python)](https://github.com/UPB-PMRust-Students/proiect-mnoramona)| Graph plotting                     | Real-time visualization of X/Y/Z axes |

## Links


1. [About Raspberry Pi Pico 2w](https://projects.raspberrypi.org/en/projects/getting-started-with-the-pico)
2. [About Rust](https://docs.rust-embedded.org/book/)
3. [Arduino Project, for inspiration only](https://how2electronics.com/arduino-earthquake-detector-accelerometer)
4. [ADXL335 Datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/ADXL335.pdf)
5. [Raspberry Pi Pico 2w Datasheet](https://datasheets.raspberrypi.com/picow/pico-2-w-datasheet.pdf)
