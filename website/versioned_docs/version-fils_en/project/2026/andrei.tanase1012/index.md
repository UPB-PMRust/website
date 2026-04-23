# Smart Air Purifier
An STM32-powered smart air purifier that monitors real-time PM2.5 levels and environmental conditions to automate a 12V HEPA filtration system.

:::info

**Author**: TANASE Andrei-Nicolae \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-Andrei-Tase

:::

## Description

This project focuses on building an intelligent environment control system that bridges the gap between air quality sensing and automated filtration. By utilizing a high-precision laser PM2.5 sensor, the system detects particulate levels in real-time and dynamically adjusts the fan's speed through a custom-built MOSFET power stage. The goal is to create an efficient, responsive device that maintains air safety while providing detailed environmental feedback on a dedicated OLED display.

## Motivation

I chose this project because I’ve been fascinated by how we can use embedded intelligence to make the "invisible" parts of our environment, like air quality, manageable through technology. Building a smart purifier gives me a hands-on way to master the STM32 ecosystem while bridging the gap between digital logic and analog power control.

It’s an ideal challenge, forcing me to handle two very different types of hardware: sensitive laser-based sensors that require precise timing, and high-power 12V fans that need a custom-built MOSFET stage for safe operation.

## Architecture

The STM32 acts as the central hub, coordinating four main functional modules (Sensing, Processing, Actuation, and Display) through the following interfaces:

- **UART**: The Plantower PMS5003 air quality sensor sends real-time serial data packets containing PM2.5 concentrations.
- **I2C Bus**: The AHT20/BMP280 combo (temperature, humidity, and pressure) and the 0.96" OLED display share the same bus using different addresses for environmental data capture and visual feedback.
- **PWM**: Drives the IRLZ44N MOSFET gate in the custom control circuit to regulate the 12V fan's power through Pulse Width Modulation.
- **Custom Motor Control Stage**: Bridges the low-power MCU logic to the high-power 12V fan circuit, utilizing a 1N4007 flyback diode for inductive surge protection and a pull-down resistor for gate stability.

The firmware logic manages a continuous cycle of data acquisition and responsive actuation. It parses incoming UART packets to monitor particle levels while concurrently polling I2C sensors for environmental context. The control algorithm then calculates the necessary fan intensity, updating the OLED UI and adjusting the PWM output to match the current air quality requirements.

**Component Connections:**
```
[POWER SECTION]                         [CONTROL & SENSING SECTION]

+-------------------------+                 +----------------------------+
|  12V DC Wall Adapter    |                 |          HOST PC           |
| (Main System Power)     |                 |       (Flash/Debug)        |
+-----------+-------------+                 +-------------+--------------+
            |                                             ^
    [DC Barrel Jack Adapter]                            [USB]
            |                                             |
            | (12V Rail)                                  v
            +------------------+           +----------------------------+
            |                  |           |    NUCLEO STM32U545RE-Q    |
            v                  |           |   (Main Microcontroller)   |
+--------------+               |           +---+--------+--------+------+
| 12V DC Fan   |               |               |        |        |
| (Purifier)   |               |               |        |        |
+------+-------+               |               |      [I2C]   [UART]
       |                       |               |      (Bus)   (Point)
       | [Fan Negative Path]   |               |        |        |
       v                       | [12V to VIN]  |        v        v
+--------------+               +-------------->|    +-------+ +---------+
|Custom MOSFET |                               |    |SSD1306| | PMS5003 |
|Stage(IRLZ44N)|<--- [PWM Signal from STM32] --|    | OLED  | | PM2.5   |
+--------------+                               |    +-------+ +---------+
       |                                       |        ^
       |                                       |        | [5V or 3.3V]
       v                                       |        | (Power from 
 [Common Ground] <-----------------------------+--------+  Nucleo pins)
                                               |        |
                                               |        v
                                               |    +-----------+
                                               +--> | AHT20/BMP |
                                                    | Combo Brk |
                                                    +-----------+
```

## Development Log

### Week 2-4
I researched various air quality sensing technologies and compared laser-based PM2.5 sensors with standard MOS sensors, ultimately choosing the STM32 Nucleo platform for its robust real-time processing capabilities. This period involved defining the project scope and establishing the functional logic for an automated HEPA filtration system based on real-time particulate matter concentration.

### Week 5-6
I studied the technical documentation and datasheets for the PMS5003 UART sensor and the I2C-based AHT20/BMP280 modules to prepare the necessary firmware communication protocols. I also designed the custom MOSFET control circuit for the 12V fan and finalized the full bill of materials before ordering all necessary hardware components and specialized filters.

### Week 7-8
I began the hardware assembly phase by verifying each component's integrity upon arrival and setting up the initial breadboard prototype. This involved establishing the first communication links between the sensors and the STM32 via I2C and UART, while testing the MOSFET switching logic to ensure the high-power fan could be safely regulated by the microcontroller's PWM signal.

## Hardware

The system is built around an STM32 Nucleo-U545RE-Q microcontroller, which interfaces with a Plantower PMS5003 laser sensor via UART for high-precision PM2.5 monitoring. Environmental context is provided by an AHT20 + BMP280 combo breakout (temperature, humidity, and pressure) and a 0.96" OLED display (SSD1306), both communicating over a shared I2C bus. The filtration is handled by a 12V DC fan and a HEPA filter, controlled through a custom motor drive circuit featuring an IRLZ44N logic level MOSFET, a 1N4007 flyback diode, and a 10kΩ pull-down resistor. The entire setup is powered by a 12V DC adapter and integrated on a standard 830 point breadboard.

## Schematics

(KiCAD or similar schematics will be added here)


## Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | [~120 RON](https://www.digikey.ro/en/products/detail/stmicroelectronics/NUCLEO-U545RE-Q/22106570) |
| [Plantower PMS5003](https://cdn-shop.adafruit.com/product-files/3686/plantower-pms5003-manual_v2-3.pdf) | PM2.5 air quality sensing (UART) | [~172,83 RON](https://ro.mouser.com/ProductDetail/485-3686) |
| [AHT20 + BMP280](https://sigmanortec.ro/modul-senzor-aht20-si-bmp280-temperatura-umiditate-presiune-28-5v) | Temperature, humidity, and pressure (I2C) | [~12,26 RON](https://sigmanortec.ro/modul-senzor-aht20-si-bmp280-temperatura-umiditate-presiune-28-5v) |
| [0.96" OLED Display](https://sigmanortec.ro/display-oled-096-i2c-iic-alb) | Visual data interface (I2C) | [~16,95 RON](https://sigmanortec.ro/display-oled-096-i2c-iic-alb) |
| [12V PC Case Fan](https://www.emag.ro/ventilator-12v-80x80x25mm-rulment-long-life-76-4m3-h-ee80251bx-a99/pd/DTBDCBYBM/) | Air circulation and filtration | [~47 RON](https://www.emag.ro/ventilator-12v-80x80x25mm-rulment-long-life-76-4m3-h-ee80251bx-a99/pd/DTBDCBYBM/) |
| [HEPA Filter](https://filtru-aspirator.compari.ro/irobot/roomba-800-900-series-filtru-hepa-p802881633/#) | Particulate removal | [~11 RON](https://filtru-aspirator.compari.ro/irobot/roomba-800-900-series-filtru-hepa-p802881633/#) |
| [IRLZ44N MOSFET](https://ro.mouser.com/datasheet/3/70/1/Infineon_IRLZ44N_DataSheet_v01_01_EN.pdf) | PWM fan power control | [~7,65 RON](https://ro.mouser.com/ProductDetail/942-IRLZ44NPBF) |
| [1N4007 Diode](https://diotec.com/request/datasheet/1n4001.pdf) | Circuit protection (flyback) | [~0,65 RON](https://ro.mouser.com/ProductDetail/637-1N4007) |
| [10kΩ Resistor](https://ro.mouser.com/datasheet/3/508/1/YAGEO_CFR_DATASHEET.pdf) | MOSFET gate pull-down | [~0,43 RON](https://ro.mouser.com/ProductDetail/603-CFR-25JT-52-10K) |
| [12V DC Power Adapter](https://cdn.sparkfun.com/datasheets/Prototyping/18742.pdf) | Main system power supply | Already owned |
| [474-PRT-10811 DC Barrel Jack Adapter](https://cdn.sparkfun.com/datasheets/Prototyping/18742.pdf) | Power supply connection | [~4,28 RON](https://ro.mouser.com/ProductDetail/474-PRT-10811) |
| [Breadboard MB102](https://sigmanortec.ro/breadboard-mb102-830-puncte-transparent) | Hardware prototyping platform | [~16,66 RON](https://sigmanortec.ro/breadboard-mb102-830-puncte-transparent) |
| [Male to Female Jumper Wires PRT-12794](https://cdn.sparkfun.com/datasheets/Prototyping/ACCA-0268%20Model.pdf) | Component interconnects | [~11,92 RON](https://ro.mouser.com/ProductDetail/474-PRT-12794) |
| [Male to Male Jumper Wires](https://sigmanortec.ro/40-Fire-Dupont-20cm-Tata-Tata-p210851325) | Component interconnects | [~8,97 RON](https://sigmanortec.ro/40-Fire-Dupont-20cm-Tata-Tata-p210851325) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Hardware Abstraction Layer | Configuring pins and peripherals |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor | Managing concurrent system tasks |
| [embassy-time](https://github.com/embassy-rs/embassy) | Time and delay library | Handling delays and timeouts |
| [ssd1306](https://github.com/adafruit/adafruit_ssd1306) | Display driver for OLED | Driving the visual display |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Rendering UI and text |
| [aht20](https://github.com/Seeed-Studio/Seeed_Arduino_AHT20) | Environmental sensor driver | Reading humidity and temperature |
| [bmp280-rs](https://github.com/pietgeursen/bmp280-rs) | Atmospheric pressure driver | Extracting barometric pressure data |
| [pms5003](https://github.com/janjongboom/mbed-pms5003) | UART sensor parser | Processing PM2.5 concentration data |
| [defmt](https://github.com/knurling-rs/defmt) | Embedded logging framework | Efficiently logging debug data |
| [defmt-rtt](https://github.com/knurling-rs/defmt) | RTT log transport | Sending logs to debugger |

## Links

1. [Embedded Rust 101 course labs](https://embedded-rust-101.wyliodrin.com/docs/fils_en/lab/01)
2. [Embassy book](https://embassy.dev/book/#_what_is_embassy)
3. [Nucleo-U545RE-Q user manual](https://www.st.com/resource/en/user_manual/um3062-stm32u5-nucleo64-board-mb1841-stmicroelectronics.pdf)
