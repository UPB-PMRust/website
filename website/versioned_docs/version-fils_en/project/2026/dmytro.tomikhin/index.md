# Smart Package Monitoring System
A device used for real-time monitoring of environmental and physical conditions of packages during transit.

:::info 

**Author**: Tomikhin Dmytro \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-tomikhin

:::

## Description

The project aims to create a "black box" for shipping. It monitors mechanical shocks, orientation changes, temperature, humidity, and light exposure. Data is processed in real-time, displayed on an OLED screen, and logged to a microSD card to identify potentially harmful events during transportation.

## Motivation

The motivation for this project comes from a blend of personal curiosity and practical necessity. Having a spare Arduino Sensor Kit at home, I wanted to move beyond basic tutorials and apply these sensors to a meaningful, real-world scenario.
In today’s global economy, we often ship high-value or fragile items without knowing how they are handled once they leave our hands. I believe that package condition monitoring is crucial for ensuring accountability in logistics. Interestingly, part of the inspiration came from the game Death Stranding, where the core mechanic revolves around the careful delivery of cargo and monitoring its integrity. This project is my attempt to bring that "cargo monitor" concept to life using embedded programming.

## Architecture 

Main layers:
- **Sensor Polling Tasks:** Independent asynchronous tasks for ADC (Light), I2C (Weather & IMU), and SPI (SD Card).
- **Data Aggregator (Central Logic):** Consolidates raw sensor data into a unified system state.
- **Display Manager:** A dedicated task for updating the OLED UI via an Embassy Channel.
- **Logging Service:** A task that monitors the data stream for critical events and writes them to the microSD card.

![Architecture Diagram](architecture.svg "Architecture")

## Log

### Week 2 - 4: Conceptualization and Project Design
- Defined the core idea of the "Smart Package Monitoring System" and drafted the system requirements, inspired by the cargo monitoring mechanics in *Death Stranding*.
- Researched environmental factors critical for logistics (shocks, humidity, light exposure).

### Week 5 - 7: Market Research and Component Procurement
- Identified and sourced compatible sensors (AHT20 + BMP280, BMI160) and the OLED display along with the MicroSD module.
- Verified compatibility between the Arduino Sensor Kit components and the 3.3V logic of the STM32 board.

### Week 8 - 9: Initial Hardware Integration and Sensor Calibration
- Successfully set up the development environment using *probe-rs* and *defmt*.
- **Assembly and Soldering:** Performed precision soldering of the AHT20+BMP280 sensor module and the OLED display headers. Using a dedicated soldering station allowed for clean, high-quality joints, ensuring signal integrity on the I2C buses and mechanical durability for the "black box" prototype.
- **Light Sensing:** Integrated the KY-018 photoresistor via ADC. Resolved a hardware issue where the initial pinout caused a power dip, successfully re-calibrating the analog values for dark/light transitions.
- **Weather Monitoring:** Implemented I2C communication for the AHT20+BMP280 sensor.
- **Architecture Milestone:** Developed a dual-bus I2C architecture. Isolated the OLED display on I2C1 and the sensors on I2C2 to ensure high-speed display updates without interference from sensor polling tasks.
- **Software Foundation:** Implemented asynchronous tasks using Embassy, allowing the system to poll light and weather data concurrently while updating the UI.

## Hardware

Components:
- **MCU:** STM32 Nucleo-U545RE-Q.
- **Sensors:**
    - KY-018 Photoresistor (Analog/ADC).
    - AHT20 + BMP280 (Temperature, Humidity, Pressure - I2C).
    - BMI160 (3-axis Accelerometer & 3-axis Gyroscope - I2C/SPI).
- **Display:** 0.96-inch OLED SSD1306 (I2C).
- **Storage:** MicroSD Card Adapter (SPI) + 16GB MicroSD Card.

The hardware architecture of the Smart Package Monitoring System utilizes an STM32 Nucleo-U545RE-Q microcontroller as its high-performance processing core, managing concurrent data streams through a multi-bus configuration. Environmental sensing is handled by an AHT20+BMP280 module for precise climate tracking and a KY-018 photoresistor for real-time light exposure detection via ADC. The system features a dedicated I2C1 bus for the SSD1306 OLED display to ensure smooth status visualization, and an independent I2C2 bus for the sensor cluster to prevent data contention. Reliable event logging is facilitated through a MicroSD adapter interfaced via SPI, providing non-volatile storage for comprehensive trip analysis.

*The system architecture is designed to be modular. Thanks to the asynchronous nature of the Embassy executor, new components/functions might be integrated in future sprints with minimal changes to the core logic.

### Schematics

To be added soon.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | Borrowed |
| [AHT20](https://www.aosong.com/userfiles/files/media/Data%20Sheet%20AHT20.pdf) + [BMP280](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmp280-ds001.pdf)| Weather monitoring (temperature, humidity, pressure) | [~11 RON](https://www.temu.com/ro/kit-de-pornire-pentru-proiecte-de-%C3%AEnv%C4%83%C8%9Bare-%C8%99i-dezvoltare-de-%C3%AEnalt%C4%83-calitate-2-%C3%AEn-1-aht20-bmp280-breadboard-pentru--esp32---g-605755960863487.html?_oak_mp_inf=EP%2FV%2BObo3YkBGiBiMmU4YjgzYzUyN2E0YmQ5YWI0MDFjMWI3MTVkZDg4ZiD43vuz3DM%3D&top_gallery_url=https%3A%2F%2Fimg.kwcdn.com%2Fproduct%2Fopen%2F254db806f4564513be7dd6abb1029dbb-goods.jpeg&spec_gallery_id=32373972813&refer_page_sn=10009&freesia_scene=2&_oak_freesia_scene=2&_oak_rec_ext_1=MTA5NA&_oak_gallery_order=572369713%2C164505595%2C1265333921%2C2036364154%2C718611760&search_key=aht20%20bmp280&refer_page_el_sn=200049&_x_sessn_id=0fux1nuu9k&refer_page_name=search_result&refer_page_id=10009_1777151701851_6tccsz3ydw) |
| [BMI160 IMU](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmi160-ds000.pdf) | Shock and orientation detection (Accelerometer & Gyroscope) | [~11 RON](https://www.temu.com/ro/bmi160-gy-bmi160-modul-senzor-de-accelera%C8%9Bie-gravita%C8%9Bional%C4%83-%C8%99i-giroscop-cu-6-axe---de--iic-i2c-spi-3-5v-g-601099964832113.html?_oak_mp_inf=EPGyg%2B%2Bn1ogBGiAxYmYyMDJlMjdlMjI0MTIyYWQ1MmJiNTY2YzNjNDJiZSC2xoS03DM%3D&top_gallery_url=https%3A%2F%2Fimg.kwcdn.com%2Fproduct%2Ffancy%2F7f73bfc0-83a6-4010-9dc4-0741ab33c6e2.jpg&spec_gallery_id=50764&refer_page_sn=10009&freesia_scene=2&_oak_freesia_scene=2&_oak_rec_ext_1=MTA3Mg&_oak_gallery_order=1068455885%2C465490554%2C75149157%2C1451834463%2C577536460&search_key=bmi%20160&refer_page_el_sn=200049&_x_sessn_id=0fux1nuu9k&refer_page_name=search_result&refer_page_id=10009_1777151845993_qi31ms0lhm) |
| [KY-018 Photoresistor](https://docs.keyestudio.com/projects/KT0193F/en/latest/KT0193F.html#project-17-photocell-sensor) | Detecting light exposure| Already owned |
| [0.96](https://learn.adafruit.com/monochrome-oled-breakouts/downloads) "[OLED SSD1306](https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf) | Real-time local status visualization | [~16 RON](https://www.temu.com/goods.html?_bg_fs=1&goods_id=601102215209339&sku_id=17603589383653&_oak_page_source=501&_x_sessn_id=gark5ntimf&refer_page_name=shopping_cart&refer_page_id=10037_1777158466659_m7lvx3k7vt&refer_page_sn=10037) |
| [MicroSD Adapter](https://www.silicon-power.com/knowledge-detail/microsd-spec/) | Interface for data logging to SD card | [~24 RON](https://www.emag.ro/2-bucati-micro-sd-tf-adaptor-pentru-carduri-viteza-mare-adaptor-convertor-pentru-carduri-micro-sd-transflash-tf-la-sd-sdhc-cititor-de-carduri-memory-stick-pentru-camera-foto-computer-memorii-interne-c/pd/D75GFJ3BM/) |
| [16GB MicroSD Card](https://www.verbatim-europe.com/en/memory-cards/products/premium-u1-microsdhc-card-16gb-plus-adapter-44082) | Non-volatile storage for trip logs | Already owned |
| [Breadboard (400 points)](https://en.wikipedia.org/wiki/Breadboard) | Prototyping | Already owned |
| [Jumper Wires Set (Male-Male, Male-Female, U-Shape Preformed)](https://en.wikipedia.org/wiki/Jump_wire) | Wiring | Already owned |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Hardware Abstraction Layer (HAL) for STM32 | Used for low-level peripheral control: GPIO, ADC, I2C, SPI, and DMA configuration |
| [embassy-executor](https://github.com/embassy-rs/embassy) | An async/await executor for embedded systems | Manages concurrent tasks without a traditional RTOS |
| [embassy-time](https://github.com/embassy-rs/embassy) | Timekeeping and delay provider | Handles precise delays for sensor initialization and task intervals |
| [embassy-sync](https://github.com/embassy-rs/embassy) | Synchronization primitives for async tasks | Used for inter-task communication via Channels (sending sensor data to the display) |
| [ssd1306](https://github.com/jamwaffles/rust-ssd1306) | Driver for the SSD1306 OLED display | Handles I2C communication and buffer management for the 0.96" OLED screen |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library for embedded systems | Used for drawing text and UI elements on the OLED display |
| [aht20-driver](https://github.com/anglerud/aht20-driver) | Driver for the AHT20 humidity and temperature sensor | Manages the initialization and data reading from the AHT20 sensor via I2C |
| [defmt](https://github.com/knurling-rs/defmt) | Efficient deferred logging framework | Provides real-time debugging information and sensor readings via RTT |
| [embassy-futures](https://github.com/embassy-rs/embassy) | Future Utilities | Provides tools to handle multiple asynchronous events simultaneously |
| [defmt-rtt](https://github.com/knurling-rs/defmt) | RTT Transport | Provides the physical transport layer for logs, allowing them to be viewed on the PC |
| [cortex-m](https://github.com/rust-embedded/cortex-m) | Processor Core Support | Provides low-level access to the ARM Cortex-M33 core |
| [cortex-m-rt](https://github.com/rust-embedded/cortex-m-rt) | Startup Runtime | Handles the reset handler and memory initialization (stack, heap) before calling main |
| [panic-probe](https://crates.io/crates/panic-probe) | Panic Handler | Catches code errors (panics) and prints the file and line number to the terminal for debugging |
| [static_cell](https://crates.io/crates/static_cell) | Static Allocation Utility | Safely allocates static memory for task arguments and long-lived drivers at runtime |

## Links

1. [Rust Embedded Book](https://docs.rust-embedded.org/book/)
2. [Embassy Framework](https://embassy.dev/)
3. [STM32U545 Datasheet](https://www.st.com/resource/en/data_brief/nucleo-u545re-q.pdf)
4. [Arduino | 37 in 1 Sensors Kit Explained](https://www.instructables.com/Arduino-37-in-1-Sensors-Kit-Explained/)
5. [AHT20+BMP280 Guide](https://docs.cirkitdesigner.com/component/3d8e6da8-841a-47a6-a1aa-0aa902c905e1/aht20bmp280)
6. [BMI160 Explained](https://www.flywing-tech.com/blog/exploring-the-bmi160-a-powerful-inertial-measurement-unit-chip/)
7. [The Rusty Bits](https://www.youtube.com/@therustybits)
8. [Embedded Rust course](https://www.youtube.com/playlist?list=PLL2SCPK5xSRWBPj-nKOVYIhxRw7C4kYeI)