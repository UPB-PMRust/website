# RustKey
A hardware TOTP authenticator device with encrypted secret storage, USB-HID output, and an OLED display.

:::info

**Author**: Mardale Andrei-Bogdan \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-bmardale

:::

## Description

RustKey is a hardware TOTP authenticator built on the STM32U545 microcontroller, implemented in async Rust using the Embassy framework. It manages multiple accounts, stores their encrypted secrets on an external EEPROM using XChaCha20-Poly1305, and displays generated codes on an OLED screen. 
A dedicated button triggers USB HID keyboard emulation, typing the code directly into the host PC. 
A companion CLI tool handles account provisioning and time synchronisation over UART.

## Motivation

The inspiration for this project came from a hardware security key (a YubiKey) sitting on my desk. Having used it regularly for two-factor authentication, I wanted to replicate its functionality as an embedded project.
Initially I considered implementing the WebAuthn/FIDO2 component, but that didn't require any external peripherals. I then decided to implement a TOTP authenticator device, which would require multiple external components.
An additional goal is to use the device during the development. This allows for better testing of correctness and user experience, while also showing bugs and edge cases that might not be obvious during normal testing.

## Architecture

Main components:
- STM32 NUCLEO-U545RE-Q development board, with a STM32U545 microcontroller will act as the central unit of the device. It's used for generating TOTP secrets via hardware-accelerated cryptographic functions (SHA1, HMAC), for driving the peripherals (I2C), UART, USB-HID. Its internal true random number generator (TRNG) is used for XCHaCha20-Poly1305 nonce. The internal flash is used for storing the master key.
- TOTP engine generates the one-time passwords using the selected account secret and the current time. The project will support only SHA1-based, 6-digit TOTP codes, as these are the most widely used in practice.
- Security layer encrypts TOTP secrets before storage and decrypts them only when needed for code generation. The encrypted records are stored externally, while the master key remains in internal flash. Secrets are never stored in plaintext in EEPROM and are never exposed through the host interface.
- Account store uses an external AT24C256 EEPROM over I2C to persist account data. Each record contains the account label, the XChaCha20-Poly1305-encrypted secret, and the corresponding nonce.
- Time service uses the DS3231 RTC module over I2C to keep accurate time across power cycles. Precise timekeeping is essential for correct TOTP generation.

User interface: 
- Display UI shows the currently selected account label, the generated TOTP code, and the remaining validity time for the current time window.
- User input layer is implemented using capacitive touch inputs, supporting short and long presses for navigation and interaction.

Host communication layer:
- UART: provisioning is performed through a Rust-based CLI. The CLI can send the current Unix epoch, check for possible time desynchronization, and add new accounts by sending an account label and a Base32-encoded secret.
- USB-HID: sends the currently generated TOTP code to a host computer when triggered by user input.

### Diagram

![Diagram](diagram.svg)

All I2C components share the same bus.

## Log

### Week 1 - 3
- Got the initial idea. 
- Researched the TOTP algorithm, its hardware implementation requirements and potential hardware components for the project.

### Week 5
- Received the borrowed STM32 development board. 
- Ordered the DS3231 RTC and the SSD1306 screen. First screen was dead on arrival, ordered two more and wired them up successfully. 
- Got working codes for RFC 6238 test data.

### Week 7
- After using an AT24C256 EEPROM at the laboratory, I decided to order one for the project as well.
- Read about and tested hardware True Random Number Generation (TRNG).
- Decided on the device powering strategy. Ordered 2 x 18650 rechargeable batteries, a dual battery holder, and an LM2596 5V step-down converter to regulate the ~7.4V battery output to the 5V required by the board's E5V pin.

### Week 8
- Received and tested the EEPROM.
- Successfully replaced the external crates in TOTP generation (SHA1 and HMAC) with hardware accelerated cryptographic functions, provided by `embassy-stm32::hash`.
- Attempted to use hardware-accelerated AES for encryption, but the STM32U545's AES peripheral is not exposed by the `embassy-stm32` library for this chip variant.
- Tested `xchacha20poly1305`, encrypted and then decrypted a string and verified that I got the same output.

## Hardware

The device is composed of the following physical components:
- STM32 NUCLEO-U545RE-Q (STM32U545 MCU): serves as the main processing unit. It interfaces with all peripherals via I2C and GPIO, and provides UART and USB connectivity.
- SSD1306 (SSD1306 OLED Display (128x64, I2C): used to display the current account label, TOTP code, and remaining validity time.
- DS3231 RTC (I2C): provides accurate timekeeping required for TOTP generation and maintains time across power cycles using a CR2032 backup battery.
- AT24C256 EEPROM (I2C): external non-volatile memory used to store account data.
- TTP223 Capacitive Touch Sensors (x2): provide user input through touch interaction, supporting short and long presses.

Communication Interfaces: **UART** for provisioning and debugging; **USB** for USB-HID communication with a host computer.

The device is powered by two series-connected 18650 lithium-ion cells (~7.4V combined), regulated down to 5V by an LM2596 step-down converter. The 5V output feeds the board's E5V pin, which bypasses the onboard voltage regulator.

All peripheral components communicate with the microcontroller primarily over the shared I2C bus.

### Schematics

KiCAD schematics will be added here as soon as they're done.

### Bill of Materials

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
5. [RFC 6238: Time-Based One-Time Password Algorithm](https://www.rfc-editor.org/rfc/rfc6238)
6. [RFC 4226: An HMAC-Based One-Time Password Algorithm](https://www.rfc-editor.org/rfc/rfc4226)
