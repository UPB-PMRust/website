# Hand Motion System
A robotic hand that mimics user hand gestures via pressure sensors.

:::info 

**Author**: Bianca-Ioana Mihai (332CC) \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-biancamih

:::

## Description

The project involves building a 3D-printed robotic hand controlled by pressure sensors that detect finger presses. The robotic hand will replicate the movements of the user's hand in real-time, based on input from the sensors. These sensors send signals to a Raspberry Pi Pico 2W microcontroller, which interprets the data and drives individual servomotors for each finger of the robotic hand.

## Motivation

Hand gesture control offers an intuitive and accessible interface for robotic systems. This project aims to explore gesture-based interaction as a method for remote manipulation or accessibility tools for people with disabilities.

## Architecture 

The main architecture components and their interactions:

- **Input Layer**: Pressure sensors detect finger presses.
- **Processing Layer**: Raspberry Pi Pico interprets sensor data using ADC and maps it to servo positions.
- **Actuation Layer**: Servos in the robotic hand mimic the finger positions.
![Diagram](./diagram.webp)

## Detailed Design

The software is structured into modular asynchronous components, each handling a specific part of the system’s logic:

### 1. Wi-Fi and TCP Communication
- **Libraries used**: `cyw43`, `embassy-net`, `embedded-io-async`
- **Responsibilities**:
  - Starts a Wi-Fi Access Point on the Raspberry Pi Pico 2W.
  - Opens a TCP socket that listens for incoming connections.
  - Parses user gesture commands sent as text (e.g., `"1"` = Peace).
  - Ensures reconnection and timeout handling.

### 2. Servo Controller
- **Libraries used**: `embassy-rp::pwm`, `fixed`
- **Responsibilities**:
  - Maps gestures to appropriate angles for each servo.
  - Applies calibration ranges for individual servos.
  - Uses PWM updates to move servos in real-time.

### 3. Sensor Monitoring Task
- **Libraries used**: `embassy-rp::gpio`, `embassy-time`
- **Responsibilities**:
  - Continuously reads five digital inputs from pressure sensors.
  - Automatic local control in the absence of TCP input.
  - Uses periodic 500ms polling for balancing speed and stability.

### 4. Command Parser
- **Libraries used**: `core::str`, `defmt`
- **Responsibilities**:
  - Parses TCP input as UTF-8 text.
  - Maps string commands `"1"`–`"8"` to gestures.
  - Logs activity and input validation using `defmt`.
![Functional Diagram](./functionaldiagram.webp)

## Log

### Week 5 - 11 May

- Initial hardware procurement
- Defined the project's scope and main functionalities.
- Selected the core hardware components.

### Week 12 - 18 May
- Documented all hardware components.
- Connected each pressure sensor to the Pico's ADC via voltage dividers.
- Assigned PWM outputs to individual GPIO pins.
- Finalized mechanical assembly of robotic hand.
- Tested individual servos with static duty cycles.
- Calibrated analog input readings from pressure sensors.

### Week 19 - 25 May
- Mapped individual pressure sensor inputs to corresponding servo angles.
- Fine-tuned servo movement ranges and logic based on physical orientation.
- Integrated TCP networking for communication with the hand controller.
- Calibrated pressure sensor thresholds to accurately detect low and high states for input handling.

## Hardware

- **Raspberry Pi Pico 2** – The core microcontroller for signal processing.
- **Pressure Sensors** – Detect user finger presses on a static input model.
- **Servomotors** – Control finger movement on the robotic hand.
- **3D-Printed Robotic Hand** – Physical output mechanism.
- **Resistors, breadboard, jumper wires, USB cable** – Circuit prototyping.

### Schematics

<!--Place your KiCAD schematics here. -->
![Schematic](./schematic.svg)

### Photos
![Photo1](./poza1.webp)
![Photo2](./poza2.webp)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [2 × Raspberry Pi Pico 2 W](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html) | Microcontrollers with Wi-Fi and Bluetooth for sensor processing and motor control | [40 RON each](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) |
| [5 × SG90 Micro Servo Motor](https://www.optimusdigital.ro/ro/motoare-servomotoare/2261-micro-servo-motor-sg90-180.html) | Actuators for robotic hand fingers | [12 RON each](https://www.optimusdigital.ro/ro/motoare-servomotoare/2261-micro-servo-motor-sg90-180.html) |
| [5 x Force Sensitive Resistor](https://www.sensor-test.de/assets/Fairs/2025/ProductNews/PDFs/SF15.pdf) | Detects force from the user's finger | [60 RON each](https://www.amazon.co.uk/Pressure-SF15-130-Resistance-Powerful-Sensitive/dp/B07PM64VN6/ref=sr_1_33?crid=23JOW43F2CRVQ&dib=eyJ2IjoiMSJ9.BsYERna4BQdO90ncctacvHuYw8Y8bmPwzhBNU39iD9Fs0iwHmTxn3dsXa-rzECqzjxf8yLEO-0gqvZCwWujpzSxcftJfuqD-CCeyiZtW59fuMhma60rKqXP6HycSpVhnJzaZSUSdLFk6-JHiZ4we0fXjotAT7qhXvNaReQzX4iG9-Cxa_yDB3en1HgmJJ8MgFnc2CAf0BCtEb8zMyYw8swU4CYHrix-XAQ9VLw9O8AcPStfTcNlkcBBCxzqG5Z0TNUOAknayjPyQWIkGME0S8tmfg0VxfLabDzy4cX-ZXhI.0wjQkt3wNfLhEmPA0whU_Vx8lJCTnHTzXJvrdHnJp-k&dib_tag=se&keywords=pressure+sensor&qid=1747589313&sprefix=pressure+sensor%2Caps%2C326&sr=8-33#) |
| [Jumper Wires](https://www.electronicwings.com/components/male-to-male-jumper-wire/1/datasheet) | This is used for wiring connections between modules and breadboard circuits | [23 RON](https://www.optimusdigital.ro/en/wires-with-connectors/12475-male-to-male-jumper-wires-40-pin-40cm.html) |
| [Breadboard](https://www.optimusdigital.ro/en/breadboards/13244-breadboard-175-x-67-x-9-mm.html) | Rapid prototyping without soldering | [15 RON](https://www.optimusdigital.ro/en/breadboards/13244-breadboard-175-x-67-x-9-mm.html) |
| [Resistors](https://www.plusivo.com/electronics-kit/117-plusivo-resistor-kit-250-pcs.html) | Used in voltage dividers and signal conditioning | [12 RON](https://www.optimusdigital.ro/en/resistors/10928-250-pcs-plusivo-resistor-kit.html) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy) | Async HAL for Raspberry Pi Pico microcontrollers | Used for GPIO control and driving PWM for servomotors |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Asynchronous task executor for embedded devices | Runs async tasks for sensor reading and servo control |
| [embassy-futures](https://github.com/embassy-rs/embassy) | Utilities for `async/await` in no_std environments | Used to perform concurrent waiting via `select` between input and timer |
| [embassy-time](https://github.com/embassy-rs/embassy) | Time management and delays | Used for non-blocking delays and timers |
| [embassy-net](https://github.com/embassy-rs/embassy) | Async network stack for embedded systems | Used for handling TCP connections over Wi-Fi |
| [cyw43](https://github.com/embassy-rs/embassy) | Driver for the CYW43 Wi-Fi chip | Connects the Pico 2W to a local network |
| [cyw43-pio](https://github.com/embassy-rs/embassy) | PIO backend for the CYW43 driver | Provides SPI communication for the Wi-Fi module |
| [embedded-io-async](https://github.com/embassy-rs/embedded-io) | Async traits for I/O | Used for TCP socket read/write operations |
| [heapless](https://github.com/japaric/heapless) | Fixed-capacity data structures without heap allocation | Used for static IP configuration |
| [defmt](https://github.com/knurling-rs/defmt) | Lightweight logging crate for embedded systems | Used for structured debug messages and runtime logs |
| [defmt-rtt](https://github.com/knurling-rs/defmt) | RTT backend for defmt logging | Sends `defmt` logs to the host machine via USB RTT |
| [panic-probe](https://github.com/knurling-rs/panic-probe) | Panic handler for embedded systems | Reports panics via defmt and safely stops the application |
| [static_cell](https://github.com/embassy-rs/static-cell) | Runtime-initialized static memory | Used to safely allocate static resources |
| [fixed](https://github.com/aggieben/fixed) | Fixed-point math library | Used to define PWM duty cycles accurately |


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Robotic Hand Controlled by Glove - Flex Sensor + Arduino](https://www.youtube.com/watch?v=Fvg-v8FPcjg) 
2. [DIY Robotic Arm using Flex Sensors and Arduino](https://www.youtube.com/watch?v=7J9GLTyKoxc) 
3. [Wireless Controlled Robotic Hand using Flex Sensors](https://www.youtube.com/watch?v=lWnlJzvybIs) 
4. [Interfacing Flex Sensor with Arduino](https://circuitdigest.com/microcontroller-projects/interfacing-flex-sensor-with-arduino) 