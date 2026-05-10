# VitalBox
A compact STM32-based biomedical monitoring prototype that measures heart rate, SpO₂, and ECG data, then displays and transmits the results in real time.

:::info

**Author**: Cojocaru Miruna-Andreea \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-miruna1205

:::


## Description

VitalBox is a compact embedded IoT system developed in Rust for monitoring basic physiological parameters in real time. The project is built around the STMicroelectronics NUCLEO-U545RE-Q board, which coordinates sensor acquisition, signal processing, local visualization, alert handling, and wireless communication.

The system uses a MAX30102 pulse oximeter module to measure heart rate and blood oxygen saturation (SpO₂), and an AD8232 ECG module to acquire a simplified electrocardiogram signal through electrodes. The measured values are shown on an SSD1306 OLED display, while the most relevant data can also be transmitted through a Wi-Fi module to a browser-based monitoring interface.

VitalBox is intended as an educational embedded systems prototype, not as a certified medical device.

## Motivation

I chose this project because it combines embedded Rust, real-time sensor acquisition, analog signal sampling, local display rendering, and IoT communication in one practical system. Instead of using each biomedical sensor separately, VitalBox integrates pulse, oxygen saturation, and ECG monitoring into a single compact device. This makes the project technically interesting because it requires working with several microcontroller peripherals at the same time: I2C for sensors and display, ADC for ECG acquisition, UART for Wi-Fi communication, and GPIO for status signals and alerts.

## Architecture

Main software and system components:
- Sensor Acquisition Module: reads heart rate and SpO₂ data from the MAX30102 sensor over I2C.
- ECG Sampling Module: samples the analog ECG output from the AD8232 module using the STM32 ADC.
- Signal Processing Module: filters and interprets raw measurements, prepares BPM, SpO₂, and simplified ECG waveform data.
- Shared State Manager: stores the latest valid measurements so that display, communication, and alert tasks can access the same data safely.
- Display Manager: updates the SSD1306 OLED with heart rate, SpO₂, ECG waveform preview, and sensor status.
- Alert Manager: activates an LED or buzzer when abnormal values or sensor errors are detected.
- Wi-Fi Communication Module: sends measurements through an ESP32-C3 or ESP8266 module connected by UART.
- Browser Monitoring Interface: receives and displays the transmitted measurements on another device connected to the same network.
- UI Control Module: uses an optional push button to switch screens, mute alerts, or reset displayed warnings.

How components connect:

```
MAX30102 Pulse Oximeter
(Heart Rate + SpO₂)
        |
        | I2C
        v
Sensor Acquisition Module
        |
        v
Shared State Manager <----------------------+
        ^                                   |
        |                                   |
AD8232 ECG Sensor                           |
(Analog ECG + Lead-Off Pins)                |
        |                                   |
        | ADC + GPIO                        |
        v                                   |
ECG Sampling Module                         |
        |                                   |
        v                                   |
Signal Processing Module -------------------+
        |
        +---------------------> Display Manager -> SSD1306 OLED Display
        |
        +---------------------> Alert Manager -> LED / Buzzer
        |
        +---------------------> Wi-Fi Communication Module -> ESP32-C3 / ESP8266
                                                               |
                                                               | Wi-Fi
                                                               v
                                                     Browser Monitoring Interface

Optional Push Button
        |
        | GPIO
        v
UI Control Module
```

## Log


### Week 5 - 11 May
- Finalized the VitalBox project idea and defined the main monitored values: heart rate, SpO₂, and ECG.
- Selected the STM32 NUCLEO-U545RE-Q as the main development board.
- Researched the MAX30102 pulse oximeter module and the AD8232 ECG module.
- Defined the first version of the system architecture and the communication interfaces needed for each component.

### Week 12 - 18 May
- Planned the hardware wiring: I2C bus for MAX30102 and OLED, ADC input for AD8232 ECG output, UART for Wi-Fi, and GPIO for alerts.
- Defined the software tasks for sensor acquisition, ECG sampling, display updates, wireless transmission, and alerts.
- Prepared the Rust/Embassy project structure and identified the libraries needed for I2C, ADC, UART, GPIO, timing, and display rendering.
- Planned the first bring-up tests: I2C scan, OLED test screen, MAX30102 read test, and ADC sampling test.

### Week 19 - 25 May
- Planned integration order for the final prototype: display first, then pulse oximeter, then ECG, then Wi-Fi transmission.
- Designed the initial OLED screen layout for BPM, SpO₂, ECG waveform preview, and sensor status messages.
- Defined basic alert thresholds and error cases, such as disconnected ECG leads or invalid sensor readings.
- Prepared the final hardware schematic overview and bill of materials for the project page.

## Hardware

The system uses an STM32 NUCLEO-U545RE-Q as the central microcontroller board. The MAX30102 pulse oximeter module is connected through I2C and provides heart rate and SpO₂ measurements. The AD8232 ECG module provides an analog ECG output that is sampled by the STM32 ADC, while its lead-off detection pins can be connected to GPIO pins for detecting disconnected electrodes.

An SSD1306 OLED display is connected through the same I2C bus and provides local visualization of the measured values. Wireless communication is handled by an ESP32-C3 or ESP8266 Wi-Fi module connected through UART. Additional components include a breadboard, jumper wires, ECG electrodes, a status LED, an optional buzzer for alerts, an optional push button for user control, and USB power.

### Schematics

KiCad Schematics will be inserted later.

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| [STM32 NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main microcontroller board | ~125 RON |
| [MAX30102 Pulse Oximeter Module](https://www.analog.com/en/products/max30102.html) | Heart rate and SpO₂ sensing over I2C | ~25 RON |
| [AD8232 ECG Sensor Module](https://www.analog.com/en/products/ad8232.html) | ECG signal acquisition through analog output; Electrodes and Cables included for Body connection for ECG module | ~45 RON |
| [SSD1306 OLED Display 0.96 inch I2C](https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf) | Local display for BPM, SpO₂, ECG preview, and status | ~30 RON |
| [ESP32-C3 Module](https://www.espressif.com/en/products/socs/esp32-c3) or [ESP8266 Module](https://www.espressif.com/en/products/socs/esp8266) | Wi-Fi communication through UART | ~25 RON |
| Breadboard | Prototype assembly | ~15 RON |
| Jumper Wires | Electrical connections between modules | ~15 RON |
| LED + Resistor | Visual status and alert indication | ~3 RON |
| Active Buzzer | Optional audible alert | ~5 RON |
| Push Button | Optional user input for screen switching or alert mute | ~3 RON |
| USB Cable / USB Power Supply | Power and development connection | ~20 RON |
| Enclosure / Project Box | Physical case for the VitalBox prototype | ~30 RON |

## Software

| Library / Tool | Description | Usage |
|----------------|-------------|-------|
| [Rust](https://www.rust-lang.org/) | Systems programming language | Main firmware implementation |
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for STM32 microcontrollers | Access to I2C, ADC, UART, GPIO, interrupts, and timers |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async embedded task executor | Runs concurrent tasks for sensors, display, Wi-Fi, and alerts |
| [embassy-time](https://github.com/embassy-rs/embassy) | Embedded timing library | Periodic sampling, display refresh, and communication intervals |
| [embassy-sync](https://github.com/embassy-rs/embassy) | Synchronization primitives for embedded async Rust | Shared state between acquisition, processing, display, and communication tasks |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction traits | Common embedded interfaces for I2C, SPI, GPIO, and ADC-style drivers |
| [embedded-hal-async](https://github.com/rust-embedded/embedded-hal) | Async embedded HAL traits | Async-compatible sensor and peripheral access |
| [ssd1306](https://github.com/rust-embedded-community/ssd1306) | SSD1306 OLED display driver | Controls the I2C OLED display |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Draws text, values, icons, and simplified ECG waveform on the OLED |
| [max3010x](https://crates.io/crates/max3010x) | Driver family for MAX3010x pulse oximeter sensors | Reads raw optical data from the MAX30102 sensor |
| [heapless](https://github.com/rust-embedded/heapless) | Fixed-capacity data structures for no_std Rust | Stores ECG samples, messages, and buffers without dynamic allocation |
| [serde-json-core](https://crates.io/crates/serde-json-core) | no_std JSON serialization/deserialization | Formats measurement packets for Wi-Fi transmission |
| [defmt](https://github.com/knurling-rs/defmt) | Embedded logging framework | Debug messages during development |
| [defmt-rtt](https://github.com/knurling-rs/defmt) | RTT transport for defmt logs | Sends logs from the board to the host computer |
| [panic-probe](https://github.com/knurling-rs/probe-run/tree/main/panic-probe) | Panic handler for embedded Rust | Reports firmware panics during debugging |
| [probe-rs](https://probe.rs/) | Flashing and debugging tool | Uploads and debugs firmware on the STM32 board |
| ESP AT Firmware / UART AT Commands | Wi-Fi command interface for ESP8266 or ESP32-C3 | Connects to Wi-Fi and sends measurement data to the browser interface |
| HTML / JavaScript Browser Page | Lightweight monitoring interface | Displays BPM, SpO₂, ECG preview, and alert status remotely |

## Links


1. [Embassy framework](https://embassy.dev/)
2. [STM32 NUCLEO-U545RE-Q product page](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)
3. [MAX30102 pulse oximeter and heart-rate sensor](https://www.analog.com/en/products/max30102.html)
4. [AD8232 ECG front-end sensor](https://www.analog.com/en/products/ad8232.html)
5. [SSD1306 OLED display driver crate](https://github.com/rust-embedded-community/ssd1306)
6. [embedded-graphics crate](https://github.com/embedded-graphics/embedded-graphics)
7. [probe-rs debugging tool](https://probe.rs/)
8. [ESP8266 AT Command Set](https://docs.espressif.com/projects/esp-at/en/latest/esp8266/)
9. [ESP32-C3 documentation](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/)

