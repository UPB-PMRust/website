# Smart Irrigation System

:::info

**Author**: Niculae Filip Ioan  
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/project-NFI-279](https://github.com/UPB-PMRust-Students/project-NFI-279)

:::

## Description

This project presents a small-scale smart irrigation system capable of automating or manually controlling water delivery to a cultivated area. Managed through a Windows desktop GUI developed using ImGui and DirectX 11, the system communicates via UART to display real-time environmental and system data. 

Users can switch between AUTO mode, where irrigation is triggered automatically based on soil humidity, and MANUAL mode, which allows full control over the water pump and specific water quantities to be dispensed.

Additional features include the ability to monitor parameters such as external temperature, atmospheric pressure, water tank temperature, and current water volume. The system also supports tank configuration, enabling users to input its shape and dimensions so that accurate calculations can be made based on sensor readings. This project demonstrates how digital interfaces and real-time sensor integration can support precision irrigation, even at a reduced scale.

## Motivation

I’ve always been fascinated by how technology can make life easier, and agriculture is a perfect area to apply that. This project allows me to combine my passion for tech with my interest in sustainable practices. I wanted to create a system that can help people automate the irrigation process and make it more efficient, especially for small-scale or hobbyist growers.

## Architecture

The following diagram presents an overview of the physical interconnections in the system:

![System schematic](./schematic.webp)

### NUCLEO-F411RE

- **Role**: Central controller that reads sensor data, manages irrigation logic, triggers alerts, and communicates with the desktop app.
- **Interfaces Used**:
  - I2C: BME280
  - ADC: Soil Moisture Sensor, NTC Thermistor
  - Digital GPIO: HC-SR04, Relay Module
  - PWM: Passive Buzzer
  - UART: Communication with ImGui App

---

### BME280 Sensor

- **Interface**: I2C  
- **Role**: Measures atmospheric temperature and pressure  
- **Pins Used**:
  - SCL: PB6  
  - SDA: PB7  
- **Connection**: Connected directly to NUCLEO via I2C1

---

### HC-SR04 Ultrasonic Sensor

- **Interface**: Digital GPIO  
- **Role**: Measures distance to estimate water level in tank  
- **Pins Used**:
  - Trig: PA8 (Output)  
  - Echo: PA9 (Input)  
- **Connection**: Trigger pin sends pulse, Echo pin receives return

---

### Soil Moisture Sensor

- **Interface**: ADC  
- **Role**: Detects soil humidity level  
- **Pins Used**:
  - Signal: PA0  
- **Connection**: Connected to ADC as voltage divider output

---

### NTC Thermistor

- **Interface**: ADC  
- **Role**: Measures temperature (soil or ambient)  
- **Pins Used**:
  - Signal: PA1  
- **Connection**: Voltage divider setup; analog output to ADC pin

---

### 4-Channel Relay Module

- **Interface**: Digital Output  
- **Role**: Switches power to the Mini Submersible Water Pump  
- **Connection**:
  - Controlled by NUCLEO through level shifter  
  - Relay IN pins connected to 5V level-shifted GPIO

---

### Mini Submersible Water Pump

- **Interface**: Relay-controlled  
- **Role**: Dispenses water to the plants  
- **Power**: Controlled via relay module from a 5V power source

---

### Passive Buzzer

- **Interface**: PWM / GPIO  
- **Role**: Beeps for alerts (dry soil, low water, etc.)

---

### UART Communication

- **Interface**: UART over USB  
- **Role**: Sends data to and receives commands from ImGui desktop app

---

### Level Translator

- **Role**: Converts 3.3V GPIO signals from NUCLEO to 5V for relay module compatibility  
- **Connection**: Between NUCLEO GPIO pins and Relay IN pins  
- **Power Source**: 5V external power

---

### 5V Power Source

- **Role**: Powers the following modules:
  - Relay module  
  - Mini Submersible Water Pump  
- **Important**: This power source is separate from the USB powering the NUCLEO board

---

## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 8 - 26 May

---

## Hardware

### Schematics

Place your KiCAD schematics here.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [NUCLEO F411RE](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-F411RE?qs=Zt3UNFD9mQjdEJg18RwZ2g%3D%3D) | The microcontroller | [65 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-F411RE?qs=Zt3UNFD9mQjdEJg18RwZ2g%3D%3D) |
| [HC-SR04 Ultrasonic Sensor](https://www.optimusdigital.ro/en/ultrasonic-sensors/2328-senzor-ultrasonic-de-distana-hc-sr04-compatibil-33-v-i-5-v.html) | Distance Measurement | [15 RON](https://www.optimusdigital.ro/en/ultrasonic-sensors/2328-senzor-ultrasonic-de-distana-hc-sr04-compatibil-33-v-i-5-v.html) |
| [BME280 Module](https://www.optimusdigital.ro/en/pressure-sensors/5649-bme280-barometric-pressure-sensor-module.html) | Barometric Pressure & Temperature | [20 RON](https://www.optimusdigital.ro/en/pressure-sensors/5649-bme280-barometric-pressure-sensor-module.html) |
| [4 Channel Level Translator](https://www.optimusdigital.ro/en/level-shifters/181-4-channnel-bidirectional-level-translator.html) | Voltage Level Translator | [5 RON](https://www.optimusdigital.ro/en/level-shifters/181-4-channnel-bidirectional-level-translator.html) |
| [4 Relay Module](https://www.optimusdigital.ro/en/relay-modules/478-blue-optoisolated-4-relay-module.html) | Controls devices | [14 RON](https://www.optimusdigital.ro/en/relay-modules/478-blue-optoisolated-4-relay-module.html) |
| [Mini Submersible Water Pump](https://www.optimusdigital.ro/en/others/4149-mini-submersible-water-pump.html) | Water Pump | [10 RON](https://www.optimusdigital.ro/en/others/4149-mini-submersible-water-pump.html) |
| [Ground Humidity Sensor Module](https://www.optimusdigital.ro/en/humidity-sensors/73-ground-humidity-sensor-module.html) | Detects soil humidity | [10 RON](https://www.optimusdigital.ro/en/humidity-sensors/73-ground-humidity-sensor-module.html) |

---

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction traits for embedded systems | Used for ADC, GPIO, and I2C traits |
| [embassy](https://github.com/embassy-rs/embassy) | Async framework for embedded systems | Used for async execution and peripheral control |
| [embassy-stm32](https://github.com/embassy-rs/embassy) | STM32 HAL for Embassy | Used to control STM32F411RE peripherals |
| [embassy-time](https://github.com/embassy-rs/embassy) | Async timing utilities | Used for delays and timers |
| [embassy-sync](https://github.com/embassy-rs/embassy) | Sync primitives for async tasks | Used for potential task synchronization |
| [bme280](https://github.com/VersBinarii/bme280-rs) | Driver for BME280 sensor | Used for reading temperature and pressure |
| [hcsr04_async](https://github.com/1-rafael-1/hcsr04_async) | HC-SR04 ultrasonic sensor driver | Used for measuring distance |
| [panic-probe](https://github.com/embedded-graphics/embedded-graphics) | Panic handler for embedded systems | Used for debugging panic messages |
| [defmt](https://github.com/knurling-rs/defmt) | Logging framework for embedded development | Used for efficient logging over RTT |
| [libm](https://github.com/rust-lang/compiler-builtins) | Math functions for no-std environments | Used for thermistor temperature calculation |

---

## Links

1. [Previous Implementation of My Project For InfoEducatie](https://community.infoeducatie.ro/t/smartlauncher-utilitar-bucuresti-lucrari-2022-nationala/5709)
2. [UART Communication with STM32](https://blog.theembeddedrustacean.com/stm32f4-embedded-rust-at-the-hal-uart-serial-communication)
3. [Automatic Plant Watering](https://www.youtube.com/watch?v=ojhrVsBs0nM)
4. [Nerdcave](https://nerdcave.xyz/)


