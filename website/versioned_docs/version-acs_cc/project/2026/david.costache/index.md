# VERIT-SCAN
A multi-sensor fresh fruit quality analyzer with data fusion from 4 physical domains

**Author**: Costache David \
**GitHub Project Link** : [Github](https://github.com/costache-david/VERIT-SCAN)

## Description
VERIT-SCAN is a non-destructive fresh fruit quality analyzer that fuses measurements from four independent physical domains — aroma (VOC + humidity), firmness (elastic deformation under controlled force), apparent density (mass / laser-estimated volume), and electrical conductivity — into a single Overall Quality Score. The results are displayed locally on an OLED screen with RGB LED feedback and streamed via UART to a host PC for logging and plotting.

## Motivation
Fruit quality assessment is traditionally subjective: we squeeze, sniff, and judge by appearance. These signals are real but hard to quantify, compare across samples, or log over time. VERIT-SCAN replaces subjective judgment with four quantifiable measurements drawn from distinct physical domains — chemistry (VOCs), mechanics (elasticity), geometry + mass (density), and electrochemistry (ionic conductivity). The project is an exercise in multi-sensor data fusion: no single sensor tells the whole story, but together they build a meaningful quality score. It also explores a non-destructive approach to firmness testing, replacing the traditional penetrometer (which damages the sample) with an elastic-deformation measurement that leaves the fruit intact.

## Architecture 
The system is structured around an async embedded runtime (Embassy on STM32U545) with the following architectural components:

- **Sensor Acquisition Layer** — four concurrent async tasks, one per measurement channel:
  - *Aroma Task* — reads BME680 over I²C every 500 ms (VOC resistance + humidity)
  - *Firmness Task* — orchestrates the VL53L0X + HX711 interaction (baseline distance → force ramp → final distance)
  - *Density Task* — combines HX711 weight reading with VL53L0X height estimation
  - *Conductivity Task* — samples the internal ADC (14-bit) on the electrode divider
- **Fusion & Scoring Layer** — a coordinator task that collects readings from the four acquisition tasks via Embassy channels, normalizes each raw value to a 0–100 score based on calibrated per-fruit-type thresholds, and computes the weighted Overall Quality Score.
- **Presentation Layer** — three parallel output tasks:
  - *Display Task* — renders scores on the SSD1306 OLED using embedded-graphics
  - *Indicator Task* — drives the RGB LED via PWM (green/yellow/red mapping) and the buzzer at measurement completion
  - *Telemetry Task* — streams a JSON line per measurement on UART2 (exposed as ST-Link Virtual COM Port) to the host PC
- **Host-side Plotting** — a Python script (pyserial + matplotlib) running on the PC parses the JSON stream and plots all four scores + the overall score in real time.

Data flow: Sensors → Acquisition Tasks → (Embassy Channel) → Fusion Task → (Embassy Channel) → Display / Indicator / Telemetry Tasks.

*(Graphical block diagram to be added.)*
### Architecture Diagram

<!-- TODO: Add diagrams.net diagram exported as SVG -->
![Architecture Diagram](images/architecture.svg)


## Log
### Week 5 - 11 May

### Week 12 - 18 May
### Week 19 - 25 May

## Hardware
The core of VERIT-SCAN is a **NUCLEO-U545RE-Q** board, built around the STM32U545RE (ARM Cortex-M33, 160 MHz, 512 KB flash, 272 KB SRAM) with an integrated ST-Link V3 debugger. The board exposes four I²C buses, two UARTs, a 12-bit ADC, and a 14-bit ADC — the latter is used for the DIY conductivity channel, where two stainless-steel electrodes in contact with the fruit flesh form a voltage divider with a 10 kΩ precision reference resistor.

Three digital sensors share a single I²C bus: **BME680** (VOC + humidity + temperature + pressure), **VL53L0X** (time-of-flight laser distance, used both for firmness-via-deformation and for height/volume estimation), and an **SSD1306 0.96" OLED** for local display. An **HX711** 24-bit ADC interfaces the 1 kg load cell, which is used both for weighing and for monitoring the controlled force applied during the non-destructive firmness test. User feedback comes from a common-cathode **RGB LED** driven by three PWM channels and a passive **buzzer** driven by a timer PWM. Communication with the host PC uses **USART2**, exposed transparently as a Virtual COM Port through the on-board ST-Link, so no external USB-to-UART adapter is needed.

### Schematics
*(KiCAD schematic to be added as SVG.)*

### Bill of Materials
| Device | Usage | Price |
|--------|-------|-------|
| NUCLEO-U545RE-Q | Main MCU board (STM32U545RE, Cortex-M33 @ 160 MHz, ST-Link V3 onboard) | already owned |
| [BME680 I²C Module](https://sigmanortec.ro/modul-senzor-bme680-i2c-33v) | Aroma measurement (VOC + humidity + temperature + pressure) | 82.62 RON
| [VL53L0X ToF Module](https://sigmanortec.ro/Modul-VL53L0X-timp-de-zbor-p126182383) | Laser distance sensor — baseline + deformation measurement, height/volume estimation | 17.04 RON |
| [Load Cell 1 kg](https://sigmanortec.ro/Senzor-cantar-1Kg-p136259733) | Weight + controlled-force measurement during firmness test | 12.80 RON
| [HX711 24-bit ADC](https://sigmanortec.ro/modul-citire-greutate-hx711-24ad-2-canale-3-5v) | Load cell amplifier and ADC | 4.57 RON
| [SSD1306 OLED 0.96" I²C](https://sigmanortec.ro/Display-OLED-0-96-I2C-IIC-Albastru-p135055705) | Local display of per-channel and overall scores | 16.96 RON |
| [RGB LED 5 mm common cathode](https://sigmanortec.ro/LED-RGB-5mm-4-pini-Catod-comun-p136284849) | Quality indicator (green / yellow / red via PWM) | 1.51 RON × 3 |
| [Passive buzzer 5 V](https://sigmanortec.ro/module/ambjolisearch/jolisearch?s=Buzzer+pasiv+5v%09) | Audible signal at measurement completion | 1.45 RON |
| 2× Stainless steel screws M3/M4 | DIY conductivity electrodes | ~2 RON |
| Resistor 10 kΩ 1% metal film | Voltage divider reference for conductivity ADC channel | 3.20 RON / 20 pcs |
| Resistor 220 Ω 1% metal film | Current limiting for RGB LED | 3.20 RON / 20 pcs |
| Breadboard 400 points | Prototyping | 6.62 RON × 2 |
| Dupont wires 30 cm M-M and M-F | Interconnects | ~16 RON |

## Software
| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for STM32 (feature `stm32u545re`) | Core hardware abstraction: GPIO, I²C, ADC, UART, PWM, async timers |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor for `no_std` environments | Concurrent sensor acquisition and output tasks |
| [embassy-time](https://github.com/embassy-rs/embassy) | Async time primitives | Timing for sampling periods and force ramps |
| [embassy-sync](https://github.com/embassy-rs/embassy) | Sync primitives (channels, mutexes) | Inter-task communication between acquisition and fusion layers |
| [ssd1306](https://github.com/rust-embedded-community/ssd1306) | Driver for SSD1306 OLED displays | OLED rendering over I²C |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics primitives | Drawing score bars, text, and icons on the OLED |
| [bme680](https://crates.io/crates/bme680) | BME680 sensor driver | Reading VOC resistance + humidity |
| [vl53l0x](https://crates.io/crates/vl53l0x) | VL53L0X ToF sensor driver | Distance / deformation / height measurement |
| [loadcell](https://crates.io/crates/loadcell) | HX711 driver for load cells | Reading force and weight from the 1 kg load cell |
| [defmt](https://github.com/knurling-rs/defmt) + [defmt-rtt](https://github.com/knurling-rs/defmt) | Efficient logging framework | Debug output over RTT during development |
| [probe-rs](https://probe.rs/) | Embedded flashing and debugging | Flashing firmware to NUCLEO-U545RE-Q via ST-Link V3 |
| [heapless](https://crates.io/crates/heapless) | Fixed-capacity collections for `no_std` | Score buffers and telemetry line buffering |
| [serde](https://crates.io/crates/serde) + [serde-json-core](https://crates.io/crates/serde-json-core) | `no_std` JSON serialization | Formatting telemetry lines sent over UART |
| pyserial + matplotlib (host-side Python) | Host-side serial reader and live plotter | Parsing JSON telemetry from UART and plotting the 4 scores + overall in real time |

## Links
1. [NUCLEO-U545RE-Q Product Page](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)
2. [STM32U545RE Datasheet](https://www.st.com/resource/en/datasheet/stm32u545re.pdf)
3. [STM32U545 Reference Manual (RM0456)](https://www.st.com/resource/en/reference_manual/rm0456-stm32u575585-armbased-32bit-mcus-stmicroelectronics.pdf)
4. [BME680 Datasheet — Bosch Sensortec](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bme680-ds001.pdf)
5. [VL53L0X Datasheet — STMicroelectronics](https://www.st.com/resource/en/datasheet/vl53l0x.pdf)
6. [HX711 Datasheet — AVIA Semiconductor](https://cdn.sparkfun.com/datasheets/Sensors/ForceFlex/hx711_english.pdf)
7. [SSD1306 Datasheet](https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf)
8. [The Embassy Book](https://embassy.dev/book/)
9. [Rust Workshop UPB — Embassy on STM32U545](https://rust.ipworkshop.ro/docs/embassy/)
10. [embassy-stm32 on crates.io](https://crates.io/crates/embassy-stm32)
11. [Non-destructive fruit firmness measurement methods — review](https://www.sciencedirect.com/science/article/abs/pii/S0925521410002802)