---
sidebar_position: 1
---

# Scanny, the GuardBot

An autonomous robot that patrols a parking lot and scans license plates.

:::info
**Author**: Anastasiu Antonia-Maria \
**GitHub Project Link**: https://github.com/antoanastasiu96-dotcom/WebsiteScanny
:::

## Description

Scanny is an autonomous patrol robot designed to scan and verify license plates in a private parking lot, running firmware written in Rust on two Raspberry Pi Pico 2W microcontrollers. When a button is pressed, the robot begins patrolling. It uses an ultrasonic sensor to detect nearby objects. Once close enough to one, it stops and the servo motor rotates the camera for up to 10 seconds, scanning for a license plate. If no plate is detected, the robot assumes the object is not a vehicle and continues. If a plate is found, the ESP32-CAM captures a photo and the system checks the number against a registered vehicles database. A valid plate triggers a green LED, a short beep and a "NUMBER VALID" message on the display. An unregistered plate triggers a red LED, a longer beep, a "NUMBER NOT FOUND" message and the plate is automatically saved to a separate database. The robot then continues its patrol regardless of the outcome.

## Motivation

The inspiration for this project came from the security guard at my campus, who spends his days and nights patrolling between parked cars, writing plate numbers in a small notebook and checking them manually against a list. The work is repetitive and exhausting. I wanted to build something that would take over the most demanding part of that routine, so he could focus on managing the situation rather than doing the physical patrol himself. Beyond the practical side, the project gave me the chance to work with embedded systems, wireless communication, computer vision and real-time control all at once, which made it a genuinely challenging and rewarding build.

## Architecture

The system uses a dual-controller architecture supported by an external vision server to manage high-latency tasks without interrupting robot movement.

**Control & Debugging**: The primary Raspberry Pi Pico 2W runs an asynchronous Embassy executor to manage motor PWM, sensor polling, and feedback logic. A secondary Pico 2W is configured as a Picoprobe, providing a dedicated hardware debugging interface for real-time logging.

**Vision & Logic**: An ESP32-CAM captures images and transmits them via Wi-Fi to a Python Flask server. The server runs OCR processing and SQLite database queries, returning the validation result to the Pico via UART to trigger the appropriate LEDs and buzzer responses.

## Log

### Week 1 - 4
* Thought about what kind of project I wanted to build.
* Explored several directions and looked into different component options.
* Studied how license plate recognition works and how ESP32-CAM handles image capture and WiFi communication.

### Week 5 - 6
* Settled on the final concept and placed the orders.
* Once the components arrived, started with the easier assembly parts like mounting the wheels and fitting the chassis together.

### Week 7 - 8
* Moved on to integrating the accumulators, the push button, the L298N motor driver module and the HC-SR04 ultrasonic sensor.
* By the end of this period the robot was able to move on its own, navigating with the help of the ultrasonic sensor without any manual intervention.

## Hardware

**The Prototype Chassis**: The robot is built on a 4WD transparent chassis with DC motors driven by an L298N motor driver. Two Raspberry Pi Pico 2W boards handle control and feedback respectively. An ESP32-CAM with OV2640 2MP camera is used for image capture. An HC-SR04 ultrasonic sensor detects obstacles, while an SG90 servo motor rotates the camera during scanning.

**Feedback & Power**: A 0.96" OLED display (128x64) shows status messages. Red and green LEDs and a passive buzzer provide immediate visual and audio feedback. Power comes from two Samsung INR18650-35E 3450mAh batteries in a dual holder. A CH340G USB-to-UART converter is used for uploading code to the ESP32-CAM.

### Schematics

### Bill of Materials

| Device | Usage | Price |
| :--- | :--- | :--- |
| [Raspberry Pi Pico 2W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=Raspberry+Pi+Pico+2W&results=24) (x2) | Main microcontroller and dedicated debugger | 79,32 RON |
| [ESP32-CAM](https://3dstar.ro/modul-esp32-cam?search=ESP32-CAM) | Image capture and WiFi communication | 53,89 RON |
| [Wheels with Motors (4x)](https://electronicmarket.ro/6v-250-rpm-motor-si-roti?search=roti) | Robot mobility | ~40-50 RON |
| [L298N Dual Motor Driver](https://www.optimusdigital.ro/ro/drivere-de-motoare-cu-perii/145-driver-de-motoare-dual-l298n.html?search_query=Modul+cu+Driver+de+Motoare+Dual+L298N+Rosu&results=1) | DC motor control | 10,99 RON |
| [SG90 Servo Motor](https://www.optimusdigital.ro/ro/) | Camera rotation | ~15 RON |
| [HC-SR04 Ultrasonic Sensor](https://www.optimusdigital.ro/ro/) | Obstacle detection | ~7 RON |
| [0.96" OLED Display (I2C)](https://www.emag.ro/afisaj-grafic-oled-128x64-0-96-inch-galben-albastru-3874784221572/pd/DGTRPXYBM/) | Status messages | ~20 RON |
| [LEDs (Red + Green)](https://www.optimusdigital.ro/ro/) | Visual feedback | ~3 RON |
| [Passive Buzzer](https://www.optimusdigital.ro/ro/) | Audio feedback | ~3 RON |
| [Push Button](https://www.optimusdigital.ro/ro/) | Starts the patrol | ~2 RON |
| [Samsung 18650 Battery (x2)](https://3dstar.ro/samsung-inr18650-35e-3450mah) | Power supply | 34,49 RON |
| [Dual 18650 Battery Holder](https://www.optimusdigital.ro/ro/suporturi-de-baterii/941-suport-de-baterii-2-x-18650.html?search_query=Suport+de+Baterii+2+x+18650&results=13) | Battery housing | 3,99 RON |
| [18650 Dual Charger](https://www.optimusdigital.ro/ro/incarcatoare-de-baterii/11021-incarcator-1865026650-dublu-cu-cablu-usb-pentru-baterii-cu-litiu-ion.html?search_query=Încarcator+18650%2F26650+Dublu+cu+Cablu+USB%2C+pentru+Baterii+cu+Litiu-Ion&results=1) | Battery charging | 19,99 RON |
| [CH340G USB to UART](https://www.optimusdigital.ro/ro/interfata-convertoare-usb-la-serial/13390-convertor-ch340g-la-uart.html?search_query=Convertor+USB+la+UART+CH340G+%28include+fire%29&results=1) | ESP32-CAM programming | 9,99 RON |
| [Jumper Cables (40-Pin)](https://www.optimusdigital.ro/ro/) | Interconnections | 8,28 RON |
| [Breadboard](https://www.optimusdigital.ro/ro/) | Prototyping | ~10 RON |
| [Resistors](https://www.optimusdigital.ro/ro/) | Circuit protection | ~5 RON |

## Software

| Library | Description | Usage |
| :--- | :--- | :--- |
| [embassy-rp](https://docs.embassy.dev/embassy-rp/git/rp2040/index.html) | HAL for RP2350 | Core peripheral access for GPIO, I2C, PWM, UART on the Pico 2W |
| [embassy-executor](https://docs.embassy.dev/embassy-executor/git/cortex-m/index.html) | Async task executor | Runs concurrent tasks such as motor control and sensor reading |
| [embassy-time](https://docs.embassy.dev/embassy-time/git/default/index.html) | Time management | Handles delays, timers and timeouts |
| [embassy-sync](https://docs.embassy.dev/embassy-sync/git/default/index.html) | Synchronization primitives | Safe communication between async tasks |
| [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/) | Hardware abstraction traits | Standard interface for embedded peripherals |
| [embedded-hal-async](https://docs.rs/embedded-hal-async/latest/embedded_hal_async/) | Async HAL traits | Non-blocking peripheral communication |
| [cyw43](https://docs.embassy.dev/cyw43/git/default/index.html) | CYW43439 WiFi driver | Manages WiFi on the Pico 2W |
| [cyw43-pio](https://docs.embassy.dev/cyw43-pio/git/default/index.html) | PIO-based SPI for CYW43 | Handles SPI communication between RP2350 and the WiFi chip |
| [embassy-net](https://docs.embassy.dev/embassy-net/git/default/index.html) | Async networking stack | TCP/IP communication with the laptop server |
| [defmt](https://docs.rs/defmt/latest/defmt/) | Logging framework | Debug output from the Pico |
| [defmt-rtt](https://crates.io/crates/defmt-rtt) | RTT transport for defmt | Transfers logs over debug probe |
| [ssd1306](https://crates.io/crates/ssd1306) | OLED display driver | Controls the 0.96" OLED display over I2C ||

## Links

1. [Raspberry Pi Pico 2W Datasheet](https://pip-assets.raspberrypi.com/categories/610-raspberry-pi-pico/documents/RP-008307-DS-1-pico-datasheet.pdf?disposition=inline)
2. [Embassy Book](https://embassy.dev/book/#_what_is_embassy)
3. [EasyOCR Documentation](https://github.com/JaidedAI/EasyOCR)
4. [ESP32-CAM Getting Started](https://randomnerdtutorials.com/esp32-cam-video-streaming-face-recognition/)
5. [Embedded Rust Book](https://docs.rust-embedded.org/book/)
