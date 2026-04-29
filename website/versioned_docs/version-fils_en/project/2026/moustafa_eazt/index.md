# Pan-Tilt Radar System 


:::info
**Author**: Moustafa Ezat \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-moustafaezat
:::

## Description
This project demonstrates a smart parking barrier system built using the STM32 NUCLEO-U545RE microcontroller. When a vehicle arrives at the entrance, an ultrasonic sensor detects it and opens the entry barrier while registering the vehicle as unpaid.

At the payment station, the driver must scan an NFC tag and insert a coin that is detected by an IR sensor to confirm payment. After this, the system updates the vehicle status.

At the exit barrier, the NFC tag is scanned again and the system verifies payment. If confirmed, the barrier opens with a green LED and a message is displayed on the LCD. Otherwise, the barrier remains closed, a red LED turns on, and a buzzer alerts the user.

## Motivation
The motivation of this project is to demonstrate how real-life parking control systems improve efficiency, security, and automation. It also provides hands-on experience with hardware integration and system logic used in smart city applications.

## Architecture
This system is built using the STM32 NUCLEO-U545RE microcontroller, which coordinates all functions:

1. Vehicle Detection Module: Detects incoming cars and opens the entry barrier, marking status as NOT PAID  
2. Payment Processing Module: Uses NFC and coin detection to confirm payment and update status to PAID  
3. Exit Verification Module: Verifies payment at exit using NFC  
4. User Notification Module: Displays messages and triggers LEDs and buzzer
---

## Log
## Week 1 - 3
- Got the initial idea. 
- Researched the TOTP algorithm, its hardware implementation requirements and potential hardware components for the project.

## Week 5
- Received the borrowed STM32 development board. 
- Ordered the DS3231 RTC and the SSD1306 screen. First screen was dead on arrival, ordered two more and wired them up successfully. 
- Got working codes for RFC 6238 test data.

## Week 7
- After using an AT24C256 EEPROM at the laboratory, I decided to order one for the project as well.
- Read about and tested hardware True Random Number Generation (TRNG).
- Decided on the device powering strategy. Ordered 2 x 18650 rechargeable batteries, a dual battery holder, and an LM2596 5V step-down converter to regulate the ~7.4V battery output to the 5V required by the board's E5V pin.


## Hardware
- STM32 NUCLEO-U545RE microcontroller  
- NFC Readers (2 units)  
- Ultrasonic Sensor  
- IR Sensor  
- Servo Motors (2x SG90)  
- LCD Display  
- LEDs (Red & Green)  
- Buzzer  
- Breadboard and jumper wires
---
## Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | N/A (borrowed) |
| [SSD1306 0.96" 128x64 OLED Screen](https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf) | Displays accounts and codes| [18.98 RON](https://www.emag.ro/afisaj-oled-0-96-i2c-iic-ssd1306-128x64px-3-5v-e498/pd/DX0LYDYBM/) |
| [DS3231 Real-time Clock Module](https://www.analog.com/media/en/technical-documentation/data-sheets/ds3231.pdf) | Keeps time for TOTP| [15.98 RON](https://www.optimusdigital.ro/ro/altele/12402-modul-cu-ceas-in-timp-real-ds3231.html) |
| CR2032 battery | For DS3231 | [6.24 RON](https://www.emag.ro/baterie-philips-lithium-cr2032-3v-2-buc-cr2032p2-01b/pd/DCVKTSBBM/) |
| 2 x 18650 rechargeable batteries | For powering the device | [38.96 RON](https://www.emag.ro/baterie-reincarcabila-18650-isr-li-ion-3-6v-2600mah-2-6a-cu-descarcare-maxima-7-65a-ted-electric-a0116012-01/pd/DYFNB73BM/) |
| 18650 dual battery holder | Holds the 18650 batteries | [3.99 RON](https://www.optimusdigital.ro/ro/suporturi-de-baterii/941-suport-de-baterii-2-x-18650.html) |
| [LM2596 Step Down - fixed 5V output](https://www.ti.com/lit/ds/symlink/lm2596.pdf) | Steps down battery voltage to 5V | [12.99 RON](https://www.optimusdigital.ro/ro/surse-coboratoare-de-5-v/13597-sursa-coboratoare-de-tensiune-lm2596-cu-iesire-fixa-de-5v.html) |
| [EEPROM Module AT24C256](https://ww1.microchip.com/downloads/en/DeviceDoc/doc0670.pdf) | Stores account data | [8.99 RON](https://www.optimusdigital.ro/ro/memorii/632-modul-eeprom-at24c256.html) |
| Breadboard kit (wires, LEDs, buttons, resistances) | Prototyping and connections | [49.97 RON](https://www.emag.ro/set-componente-electronice-sinbinta-253-buc-cu-cutie-de-depozitare-rezistenta-de-lunga-durata-usor-de-manevrat-perfect-de-transportat-compatibila-cu-arduino-si-raspberry-pi-182-88-44mm-metal-multicolo/pd/D6F7S5YBM/) |
| [2 x TTP223](https://files.seeedstudio.com/wiki/Grove-Touch_Sensor/res/TTP223.pdf) | For button inputs | [3.98 RON](https://www.optimusdigital.ro/en/touch-sensors/861-modul-cu-senzor-capacitiv-tp223.html) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy/tree/main/embassy-stm32) | HAL for STM32 microcontrollers | Used for I2C, UART, USB, hardware HMAC-SHA1 via HASH peripheral, TRNG  |
| [embassy-sync](https://github.com/embassy-rs/embassy/tree/main/embassy-sync)| Async-aware synchronisation primitives | Channels and signals for inter-task communication |
| [embassy-time](https://github.com/embassy-rs/embassy/tree/main/embassy-time) | Timekeeping and async delays | Timers for code expiry countdown and debouncing |
| [embassy-executor](https://github.com/embassy-rs/embassy/tree/main/embassy-executor) | Async task executor for embedded systems | Runs all concurrent Embassy tasks |
| [embassy-usb](https://github.com/embassy-rs/embassy/tree/main/embassy-usb) | Async USB device stack | USB device initialisation and HID transport |
| [usbd-hid](https://github.com/twitchyliquid64/usbd-hid) | USB HID descriptor and report types | HID reports for typing TOTP codes |
| [ssd1306](https://github.com/rust-embedded-community/ssd1306) | Display driver for SSD1306 | Used for the display, displays codes, accounts |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |
| [ds323x](https://github.com/eldruin/ds323x-rs) | DS3231 RTC Driver | Used for writing and reading the time |
| [eeprom24x](https://github.com/eldruin/eeprom24x-rs) | Driver for 24x series I2C EEPROMs | Reading and writing TOTP records to the AT24C256 |
| [chacha20poly1305](https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305) | XChaCha20-Poly1305 AEAD cipher | Encrypting and decrypting TOTP secrets |
| [heapless](https://github.com/rust-embedded/heapless) | Static, fixed-capacity data structures | Strings and vecs without a heap allocator |
| [defmt](https://github.com/knurling-rs/defmt) | Efficient logging framework for embedded targets | Structured debug logging over RTT |
| [defmt-rtt](https://github.com/knurling-rs/defmt/tree/main/firmware/defmt-rtt) | RTT transport backend for defmt | Transmits defmt log output via RTT to a debug probe |
| [panic-probe](https://github.com/knurling-rs/defmt/tree/main/firmware/panic-probe) | Panic handler using probe-rs | Forwards panic messages to the debug probe via defmt |
| [serialport](https://github.com/serialport/serialport-rs) | Cross-platform serial port library | UART communication between CLI and device |
| [clap](https://github.com/clap-rs/clap) | Command line argument parser | Used for CLI argument parsing, defaults |

## Links

1. [STM32U545RE Datasheet](https://www.st.com/resource/en/datasheet/stm32u545re.pdf)
2. [NUCLEO-U545RE-Q User Manual](https://www.st.com/resource/en/user_manual/um3062-stm32u3u5-nucleo64-boards-mb1841-stmicroelectronics.pdf)
3. [Embassy Book](https://embassy.dev/book/)
4. [AT24C256 Datasheet](https://ww1.microchip.com/downloads/en/DeviceDoc/doc0670.pdf)
