# Study Assistant with Pomodoro

A system that helps users structure their study time by automatically managing focus and breack intervals, while also providing real-time feedback about the enviroment and system status to support better concentration.
:::info 

**Author**: Scarlat Andra-Nicole \
**Group**: 1221EA \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-Nykole25

:::

## Description

This project is an embedded study assistant built around the Pomodoro technique, implemented on an ESP32 microcontroller using Rust. Its main purpose is to support structured study sessions by combining a countdown timer with visual cues, sound alerts and environmental sensing.

The current session status and remaining time are shown on an OLED display, while a buzzer and LEDs provide clear feedback for events such as sessions start, completions and transitions between focus and break periods. In addition, it integrates environmental sensors to track conditions such as ambient light and air quality, which can help users understand and adjust their study environment for better focus.

The overall design focuses on creating a simple and distraction-free interactions flow, where the user only needs to use a few physical controls to manage their study sessions, while the system handels timing, feedback, and state transitions automatically in the background.

## Motivation

The main motivation behind this project comes from my own experience with studying and focus. I found that the Pomodoro technique genuinely helped me stay more productive, especially during long study sessions where it is easy to lose concentration or burn out. At the same time, I also realized that my study environment often had a big impact on how effective I actually was.

## Architecture 
```
                        +----------------------------------+
                        |            ESP32                |
                        |     (Central Controller)       |
                        |     Rust Embedded Firmware     |
                        +---------+-----------+----------+
                                  |           |
                                  |           |
                        +---------+           +-------------------+
                        |                                         |
                        v                                         v

              +---------------------+                +----------------------+
              |   OLED SSD1306      |                |   USB Power / PC     |
              |   (I2C Display)     |                |   (Optional Serial)  |
              |   SDA / SCL         |                |   Debug / Logging    |
              +----------+----------+                +----------+-----------+
                         |                                      |
                         |                                      |
                         v                                      v

        +----------------+----------------+        +--------------------------+
        |                                 |        |                          |
        v                                 v        v                          v

+------------------+           +------------------+        +------------------+
| LDR LIGHT SENSOR |           | MQ-135 GAS       |        | BUTTON INPUTS    |
| (Ambient Light)  |           | SENSOR           |        | Start / Reset    |
| ADC INPUT        |           | (Air Quality)    |        | GPIO + Pull-down |
+--------+---------+           +--------+---------+        +--------+---------+
         |                              |                            |
         |                              |                            |
         +--------------+---------------+----------------------------+
                        |
                        v

              +-----------------------------+
              |   SENSOR PROCESSING LAYER   |
              | (Sampling + Filtering ADC)  |
              +--------------+--------------+
                             |
                             v

              +-----------------------------+
              |  POMODORO STATE MACHINE     |
              |  Focus / Break / Idle       |
              +------+-----------+----------+
                     |           |
                     |           |
         +-----------+           +-------------------+
         |                                           |
         v                                           v

+----------------------+                 +----------------------+
| BUZZER (ACTIVE)     |                 | LED INDICATORS      |
| GPIO OUTPUT         |                 | Focus / Break       |
+----------+----------+                 +----------+----------+
           |                                        |
           |                                        |
           +-------------------+--------------------+
                               |
                               v

                    +---------------------------+
                    |   SYSTEM FEEDBACK LOOP    |
                    | OLED + BUZZER + LEDs      |
                    +---------------------------+

```     


## Log


### Week 13 - 19 April

I discussed the idea of the project and based on the feedback I improved the initial design by adding the air quality and light sensors. Also I added the OLED display in order to make the system more complete.

### Week 20 - 26 May

I did my reserch for all the necessary hardware components and ordered them. Once parts arrived, I started testing the basic functionality of individual components to ensure everything was working correctly.

### Week 27 - 3 May

## Hardware

The system uses an ESP32 board as the main processing unit, responsible for running the Pomodoro logic and coordinating all connected peripherals. Time tracking and system status are shown on an SSD1306 OLED screen, while alerts are handled through a 5V active buzzer and LED indicators that signal different states of the study session.

Interaction with the device is done using a few physical push buttons connected to GPIO pins, giving control over the timer flow. To enrich the experience the setup includes a LDR sensor for measuring ambient light and an MQ-135 sensor for monitoring air quality, providing context about the surrounding environment. Communication with peripherals is handled through standard ESP32 interfaces such as GPIO and I2C.

Hardware components:

Microcontroller: ESP32-WROOM-32D

Input: LDR light sensor + MQ-135 air quality sensor + Push Buttons

Output: SSD1306 OLED Display + Active Buzzer +LED Indicators

Interface: GPIO/I2C

### Schematics

The KiCad schematic will be added here.

### Bill of Materials


| Device | Usage | Price |
|--------|--------|-------|
| [ESP32-WROOM-32](https://documentation.espressif.com/esp32-wroom-32_datasheet_en.html) | The microcontroller | [68 RON](https://www.emag.ro/modul-esp32-devkitc-v4-esp32-wroom-32d-wifi-bluetooth-microusb-48-x-28-x-8-mm-ah8/pd/D8V9R1YBM/) |
| [OLED SSD1306 1C2 0.96 inch disply](https://app.readthedocs.org/projects/ssd1306/downloads/pdf/latest/) | OLED display | [32 RON](https://www.emag.ro/afisaj-oled-alb-0-96-inch-rezolutie-128x64-pixeli-i2c-spi-tensiune-3-5v-controler-ssd1306-compatibil-cu-arduino-stm32-msp430-dimensiuni-27x27x4mm-lizibilitate-excelenta-c3/pd/D767C1YBM/) |
| [25 LEDs 5mm Set](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/13608-set-25-led-uri-5mm.html?search_query=LED+5mm&results=65) | Visual Feedback| [5 RON](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/13608-set-25-led-uri-5mm.html?search_query=LED+5mm&results=65) |
| [5V Active Buzzer](https://www.net4web.de/downloads/datasheets/datasheet_active_buzzer_5v_english_190523.pdf) | Sound notifications | [7 RON](https://www.emag.ro/buzzer-activ-5v-12mm-x-10mm-arduino-ai151-s126/pd/DTPXQGMBM/?ref=sponsored_products_search_f_b_1_1&recid=recads_1_8c58b0a8052b9df7b1010db04ca79e120eb12b2d55b877f3172f9cdd0717e37a_1776945261&aid=664d1e70-3c92-11f1-801c-06eaf0d4245d&oid=57727870&scenario_ID=1#opensearch) |
| [LDR Light Sensor](https://www.optimusdigital.ro/ro/senzori-senzori-optici/167-modul-cu-fotorezistor.html?gad_source=1&gad_campaignid=20868596392&gbraid=0AAAAADv-p3CazgL_VTW2ESDQIFWPOsawe&gclid=CjwKCAjwhqfPBhBWEiwAZo196hrLffxefJQd8QHEDV9NBfnIrsjff0fYDwHfm5t914PI5Yu-fNfK5RoCQRQQAvD_BwE) | Measures ambient light level | [7 RON](https://www.optimusdigital.ro/ro/senzori-senzori-optici/167-modul-cu-fotorezistor.html?gad_source=1&gad_campaignid=20868596392&gbraid=0AAAAADv-p3CazgL_VTW2ESDQIFWPOsawe&gclid=CjwKCAjwhqfPBhBWEiwAZo196hrLffxefJQd8QHEDV9NBfnIrsjff0fYDwHfm5t914PI5Yu-fNfK5RoCQRQQAvD_BwE) |
| [MQ-135 Air quality sensor](https://www.winsen-sensor.com/d/files/PDF/Semiconductor%20Gas%20Sensor/MQ135%20(Ver1.4)%20-%20Manual.pdf) | Measures the air quality | [20 RON](https://www.emag.ro/modul-senzor-de-calitate-a-aerului-mq-135-elektroweb-5-v-2-m-086/pd/DTWC35MBM/) |
| [Jumper Wires + Breadboard + MicroUSB cable]| Conecting the hardware components| [ 33RON](https://www.emag.ro/breadboard-h-hct-tronic-830-puncte-de-conectare-abs-200x630-puncte-034-066/pd/DBNQ7R3BM/) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) | Hardware abstraction layer for ESP32 | Used for GPIO, ADC and peripheral control |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Generic embedded Rust traits | Provides standard interface for hardware drivers |
| [ssd1306](https://docs.rs/ssd1306/latest/ssd1306/) | OLED display driver | Used to control the SSD1306 display |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing text and UI on the OLED |
| [anyhow](https://docs.rs/anyhow/latest/anyhow/) | Error handling library | Simplifies runtime error management |
| [log](https://docs.rs/log/latest/log/) | Logging framework | Used for debugging and system logs |


## Links

1. [ESP32-WROOM-32 documentation](https://documentation.espressif.com/esp32-wroom-32_datasheet_en.pdf)
2. [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/index.html)
3. [SSD1306 OLED Display Driver Documentation](https://docs.rs/ssd1306/latest/ssd1306/)
4. [Embedded Graphics Library](https://github.com/embedded-graphics/embedded-graphics)
5. [Embedded HAL](https://docs.rs/embedded-hal/latest/embedded_hal/)
