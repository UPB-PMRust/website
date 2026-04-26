# Smart Pantry Bin (Inventory & Precision Measurement System)

An Intelligent Storage Container for dry ingredients featuring dynamic weighing, custom quality monitoring, and Bluetooth connectivity.

:::info 

**Author**: Banu Alexandru Cristian \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-banul27

:::

## Description

The Smart Pantry Bin is an active kitchen tool designed to manage and preserve dry ingredients like coffee, sugar, or flour. Unlike a passive container, this system uses a load cell to function as a real-time kitchen scale, detecting exact amounts removed, and continuously monitors inventory levels. It also tracks environmental humidity to ensure ingredient quality, sending all data and alerts to a mobile application via Bluetooth.

## Motivation

I chose to build the **Smart Pantry Bin** because food waste and inefficient kitchen organization are common household problems. Often, we forget about basic ingredients (like flour, sugar, or rice) until we run out or they expire.

By integrating a weighing system and environmental sensors, this project aim to:
- *Automate Inventory:* Monitor the exact amount of remaining stock without manual checks.
- *Ensure Food Safety:* Track humidity and temperature levels to prevent spoilage.
- *Smart Integration:* Provide real-time data via Bluetooth, turning a simple storage container into an intelligent kitchen assistant.

## Architecture 

The system consists of four main functional blocks:

- *Precision Sensing Engine*: Interfaces with the HX711 24-bit ADC to process raw signals from the load cell into stable weight measurements, calculating deltas for "Kitchen Scale Mode".

- *Environmental Monitor*: Reads data from the AHT21 sensor via I2C and compares current humidity against product-specific thresholds.

- *Communication Hub*: Serializes weight, humidity, and alert status into data packets sent via UART to the HC-05 Bluetooth module for mobile synchronization.

- *Local Interface*: Manages the 0.96" OLED display using the ssd1306 crate to provide immediate visual feedback without needing a phone.

![System Architecture Diagram](./architecture.svg)

## Log

### Week 3-4

Researched various project ideas focused on smart home automation. After deciding on a Smart Pantry Bin, I began researching the precision requirements for load cells and the low-power capabilities of the STM32U5 series. I finalized the list of components, ensuring compatibility between the 3.3V logic of the Nucleo board and the sensors (HX711, AHT21, HC-05).

### Week 6      

The components arrived. I started individual hardware validation by testing the STM32U575RE-Q Nucleo board and ensuring the Rust toolchain (probe-rs) correctly identifies the target. I performed a basic continuity test on the breadboard and verified the 0.96" OLED display using a simple I2C scanner script to confirm the address.

### Week 8-9

Started the core software implementation. I initialized the Embassy async runtime on the STM32U5 and set up the I2C and UART peripherals. I began implementing the driver logic for the HX711 to convert raw differential signals into weight data. Additionally, I designed the initial hardware layout on the breadboard, focusing on minimizing signal noise for the load cell to ensure accurate measurements.

## Hardware

The project is based on the STM32U575RE-Q (Nucleo-64), an ultra-low-power ARM Cortex-M33 microcontroller. It uses a 5kg Load Cell with an HX711 amplifier for precision weighing, and an AHT21 sensor for humidity/temperature monitoring. Connectivity is handled by an HC-05 Bluetooth module. The system is powered by a 6xAA battery holder via the DC Jack for portability.

### Schematics

TODO

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| STM32U575RE-Q (Nucleo-64) | Main Microcontroller | Borrowed (University) |
| [HX711 Weight Sensor Module](https://ardushop.ro/ro/electronica/2323-modul-citire-senzor-greutate-hx711-6427854004611.html) | Load cell amplifier / ADC | 7.00 RON |
| [AHT21 Humidity & Temp Sensor](https://ardushop.ro/ro/groundstudio/1598-modul-senzor-umiditate-si-temperatura-aht21-groundstudio-6427854000439.html) | Environmental monitoring | 7.00 RON |
| [5kg Load Cell](https://ardushop.ro/ro/electronica/2418-senzor-greutate.html) | Weight sensing | 10.00 RON |
| [HC-05 Bluetooth Module](https://ardushop.ro/ro/comunicatie/1582-modul-bluetooth-hc-05-transciever-serial-6427854023520.html) | Wireless data transmission | 30.00 RON |
| [0.96" OLED I2C Display](https://ardushop.ro/ro/display-uri-si-led-uri/818-display-oled-096-i2c-iic-albastru-6427854010636.html) | Local UI feedback | 22.00 RON |
| [6xAA Battery Holder](https://ardushop.ro/ro/electronica/564-suport-baterii-6xaa-mufa-6427854006677.html) | Portable power supply | 6.00 RON |
| [Breadboard + Jumper Wires](https://ardushop.ro/ro/electronica/2297-breadboard-830-puncte-mb-102-6427854012265.html) | Prototyping | 25.00 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| stm32u5xx-hal | Hardware Abstraction Layer | GPIO, I2C, UART and Power management |
| hx711 | 24-bit ADC Driver | Reading weight data from the load cell |
| aht20 | AHT20/21 Sensor Driver | Reading humidity and temperature data |
| ssd1306 | OLED Display Driver | Rendering text and graphics to the I2C display |
| embedded-graphics | 2D graphics library | Drawing icons and status bars on the OLED |
| nb | Non-blocking I/O | Managing asynchronous UART for Bluetooth |

## Links

1. [STM32U5 Series Reference Manual](https://www.st.com/resource/en/reference_manual/rm0456-stm32u575585-armbased-32bit-mcus-stmicroelectronics.pdf)
2. [Rust Embedded Book](https://docs.rust-embedded.org/book/)
3. [HX711 Arduino/Embedded Tutorial & Calibration Logic](https://learn.sparkfun.com/tutorials/load-cell-amplifier-hx711-breakout-hookup-guide)
4. [Tutorial: Building a Digital Scale with Load Cells](https://www.youtube.com/watch?v=nGUpzwEa4vg)
5. [STM32U5 Ultra-Low Power Introduction](https://www.st.com/en/microcontrollers-microprocessors/stm32u5-series.html#design-resources)