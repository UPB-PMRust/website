# Hardware password vault
A secure password manager that stores credentials on-chip and acts as a physical USB keyboard to inject them.

:::info
**Author**: Draghici Paul Adrian \
**Group:** 1221EA \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-UserT101
:::

## Description

My project aims to build a hardware-based password manager using the STM32U545RE microcontroller. The device securely stores user credentials in its internal non-volatile memory. Through a physical rotary encoder and an OLED interface, the user can unlock the vault using a Master PIN and select a specific account. Then, the STM32 will decrypt the associated password and will act as a standard Human Interface Device (HID) keyboard, injecting the keystrokes directly into the host computer.

## Motivation

The goal is to design a highly secure credential vault that mitigates software-based keylogging and clipboard snooping. I wanted to build something with a strict security architecture. By utilizing the STM32's Internal Flash Memory rather than external storage (like an SD card or EEPROM), the device leverages Hardware Readout Protection (RDP) to physically lock the silicon against external debugging and data extraction. This prevents physical theft and I2C/SPI bus sniffing.

## Architecture

The system architecture centers around the STM32U545RE, which coordinates user input, visual feedback, cryptography, and USB communication. 

The user interacts with the device via the KY-040 Rotary Encoder, scrolling through a text-based menu displayed on the SSD1306 OLED screen (connected via the I2C bus). Because the STM32's built-in USB port is strictly linked to the ST-LINK programmer, a dedicated USB Type-C breakout board is going to be used. Once the user selects an account and inputs their Master PIN, the STM32 derives a 256-bit AES key, decrypts the payload stored in the dedicated Internal Flash sector, and formats it as HID reports sent to the PC over the Type-C connection. Resistors are used on the breadboard to ensure I2C signal integrity.

#### Component connections
```

[INTERFACE & OUTPUT SECTION]            [CONTROL & STORAGE SECTION]

+-------------------------+             +----------------------------+
|         HOST PC         |             |          HOST PC           |
|  (Receives Keystrokes)  |             |   (Power & Flash/Debug)    |
+-----------+-------------+             +-------------+--------------+
            ^                                         ^
      [USB Data Cable]                              [USB]
            |                                         |
            v                                         v
+-----------+-------------+             +----------------------------+
|  USB Type-C Breakout    |             |    NUCLEO STM32U545RE-Q    |
|  (Custom HID Keyboard)  |             |   (Main Microcontroller)   |
+-----------+-------------+             +---+--------+--------+------+
            |                               |        |        |
            | [D+ and D- Data Pins]         |      [I2C]    [GPIO]
            |                               |      (Bus)  (Interrupts)
            +------------------------------>|        |        |
                                            |        v        v
                                            |   +-------+ +----------+
                                            |   |SSD1306| |  KY-040  |
                                            |   | OLED  | | Encoder  |
                                            |   +-------+ +----------+
                                            |        ^         ^
                                            |        |         |
                                            |   [3.3V Logic Power]
                                            |  (From Nucleo pins)
                                            |        |         |
                                            +--------+---------+
                                            |
                                            v
                                 +----------+----------+
[Common Ground] <----------------|   INTERNAL FLASH    |
                                 | (Encrypted Storage) |
                                 +---------------------+
```
## Log

### Week 3-4
I have defined the hardware architecture and ordered all the necessary components. Finalized the decision to use internal Flash memory rather than an external SD card to maximize hardware-level security and prevent bus-sniffing.

### Week 5-7
I studied the technical documentation and datasheets for the SSD1306 OLED (I2C) and the KY-040 rotary encoder to prepare the necessary firmware communication protocols and hardware timer configurations. I also researched the STM32's USB 2.0 Full Speed capabilities and finalized the full bill of materials before ordering all necessary hardware components and prototyping materials.

### Week 8-9
I began the hardware assembly phase by soldering the necessary header pins to the USB Type-C breakout board. I set up the initial breadboard prototype, establishing the first communication links between the OLED and the STM32 via I2C (using resistors for signal integrity), while testing the initial USB HID enumeration to ensure the host PC correctly recognized the microcontroller as a standard keyboard.

# Hardware
The central part of the system is the STM32U545RE microcontroller, utilizing its internal Flash memory for secure data storage. 

The user interface consists of the KY-040 Rotary Encoder (for navigation and selection via its  push-button) and a 0.96" SSD1306 OLED Display for visual feedback. A USB Type-C Breakout Board is required to physically bridge the STM32's data pins to the host PC, bypassing the onboard ST-LINK. Standard prototyping materials (breadboard, Dupont wires, and resistors) are used to combat "floating pins" and stabilize the I2C communication.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The main microcontroller acting as the vault brain | 120 RON |
| [KY-040 Rotary Encoder](https://sigmanortec.ro/Encoder-rotativ-KY-040-p126258216) | User input interface (navigation and tactile button) | 6.50 RON |
| [0.96" SSD1306 OLED (I2C)](https://sigmanortec.ro/Display-OLED-0-96-I2C-IIC-Albastru-p135055705) | Display interface for the text-based menu | 12.99 RON |
| [USB Type-C Breakout (Data)](https://sigmanortec.ro/modul-type-c-la-pini-de-panou-tensiune-si-date-usb-31) | Host connectivity to establish a direct HID connection | 4.99 RON |
| [Breadboard 400 points](https://sigmanortec.ro/Breadboard-400-puncte-p125633800) | Prototyping hub and resistor mounting | 5.00 RON |
| [40 Dupont Wires Female-Female](https://sigmanortec.ro/40-fire-Dupont-10cm-Mama-Mama-p129872525) | Direct board-to-module connections | 4.50 RON |
| [40 Dupont Wires Male-Male](https://sigmanortec.ro/40-fire-Dupont-10cm-Tata-Tata-p129872516) | Breadboard connections | 4.50 RON |
| [4.7kΩ Resistors (20x)](https://sigmanortec.ro/Rezistor-p126025265) | Hardware pull-ups for I2C and signal integrity | 3.20 RON |

## Schematics

KiCAD schematic will be placed here.

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async embedded framework | Initializing communication and connecting peripherals |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async/await executor | Spawning the main hardware tasks |
| [embassy-usb](https://github.com/embassy-rs/embassy) | USB protocol stack | Establishing the USB 2.0 Full Speed connection |
| [usbd-hid](https://github.com/rust-embedded-community/usb-device) | USB HID class definitions | Transmitting decrypted keystrokes to the host PC |
| [ssd1306](https://github.com/jamwaffles/ssd1306) | Display driver | Initializing the I2C OLED screen |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Drawing the text-based menu UI |
| [aes-gcm](https://github.com/RustCrypto/AEADs) | Cryptography library | Encrypting and decrypting password payloads |
| [pbkdf2](https://github.com/RustCrypto/password-hashes) | Key derivation function | Deriving a secure 256-bit AES key from the user's PIN |
| [embedded-storage](https://github.com/rust-embedded/embedded-hal) | Storage abstraction | Managing non-volatile memory in the STM32's Internal Flash |

## Links

1. [STM32U5 Series Documentation](https://www.st.com/en/microcontrollers-microprocessors/stm32u5-series.html)
2. [USB Human Interface Device (HID) Class](https://en.wikipedia.org/wiki/USB_human_interface_device_class)
3. [Embassy Framework](https://embassy.dev/)