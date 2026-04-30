# Project Name
Lumentra
:::info 

**Author**: Sandu Tudor Nicolas \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-TudorSN

:::

<!-- do not delete the \ after your name -->

## Description

Lumentra is a modern, hybrid USB-MIDI controller designed for music producers and live performers. Built on the Raspberry Pi Pico 2 W, it acts as a native "Plug-and-Play" USB-MIDI class device. It features physical arcade buttons for tactile performance and a single Time-of-Flight (ToF) laser sensor for touchless expression. Furthermore, the project utilizes the microcontroller’s built-in hardware True Random Number Generator (TRNG) to introduce subtle, natural "humanization" to drum beats by randomizing Note Velocity and Panning directly at the bare-metal hardware level.

## Motivation

As a music producer, I wanted a project that combines my passion for creating beats with the technical skills learned in the Microprocessor Architecture course. Standard MIDI controllers often lack expressive continuous control. Lumentra solves this by using an incredibly accurate laser distance sensor to create a theremin-like gesture effect channel. Furthermore, to overcome a common software issue where MIDI drum loops sound too sterile or "robotic," I engineered Lumentra to utilize the Pico 2 W's integrated hardware TRNG. By seeding beat logic with this internal noise, I can add natural human variations to performance dynamics directly in the embedded system.

## Architecture 

Main Components & Interconnections:
- Core Controller (Raspberry Pi Pico 2 W): Acts as the central hub, running asynchronous Rust (Embassy) to poll all inputs and handle native USB communication.
- Tactile Performance Array: Mechanical arcade buttons connected via standard GPIO pins, used to send MIDI Note On/Off messages.
- Gesture Sensor: A single VL53L0X Time-of-Flight (ToF) laser sensor connected to the I2C Bus. The distance read is mapped to MIDI   Control Change (CC) messages.
- Internal Humanizer: The RP2350’s integrated TRNG peripheral is utilized to provide high-quality random data for non-deterministic MIDI velocity and panning.
- Visual Interface: An ST7789 TFT display module connected to the high-speed SPI Bus to provide real-time visual feedback on active MIDI CC values and selected effects.
- Host PC Connection: The microcontroller's Native USB peripheral is configured as a native USB-MIDI Class Device, sending data directly to the DAW (e.g. FL Studio).

+-------------------+             +-------------------+             +-----------------------+
|  Gesture Sensor   |====I2C======|                   |===USB=======|   Host Device (PC)    |
|  (VL53L0X ToF)    |             |                   |             |   (DAW/FL Studio)     |
+-------------------+             |                   |             +-----------------------+
                                  |                   |
+-------------------+             | Logic Controller  |             +-----------------------+
|  Tactile Input    |===GPIO======|   (Pico 2 W)      |====SPI======|   Visual Interface    |
| (Arcade Buttons)  |             |                   |             | (ST7789 TFT Display)  |
+-------------------+             |                   |             +-----------------------+
                                  |                   |
                                  |                   |             +-----------------------+
                                  |                   |===GPIO======|    Audio Feedback     |
                                  +-------------------+             |   (Buzzer - Extra)    |
                                                                    +-----------------------+

## Log

<!-- write your progress here every week -->

### Week 1 - 9

- Decided on the final idea for the project and chose the Raspberry Pi Pico 2 W as the main board because of its native USB support and built-in random number generator.
- Researched exactly what parts were needed and ordered the ToF laser sensor, the SPI display, and the arcade buttons.
- Tested the individual parts separately on a breadboard to make sure they turn on and work correctly before putting everything together.
- Set up the Rust programming tools on my computer and successfully connected to the Pico 2 W to make sure I can upload code to it.

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The project is built around the Raspberry Pi Pico 2 W (featuring the dual-core ARM Cortex-M33 RP2350), selected for its excellent asynchronous Embassy Rust support and native USB capabilities. Gesture inputs are captured using a single, millimeter-accurate VL53L0X ToF laser sensor on the I2C bus. Performance input is handled by mechanical arcade buttons. Visual feedback is delivered via a 1.3" TFT LCD display driven by an ST7789 controller communicating over the SPI bus. To ensure a professional and durable musical instrument, the entire circuit will be permanently soldered onto a prototyped perfboard.

### Schematics



### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico 2 W](https://pip-assets.raspberrypi.com/categories/1088-raspberry-pi-pico-2-w/documents/RP-008304-DS-2-pico-2-w-datasheet.pdf?disposition=inline) | Main microcontroller running the native USB stack, I2C, and SPI logic. | [40 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=raspberry+pi+pico+2+w&results=24) |
| [VL53L0X ToF Sensor](https://www.st.com/resource/en/datasheet/vl53l0x.pdf) | Laser sensor to track hand distance for effect control. | [20 RON](https://sigmanortec.ro/Modul-VL53L0X-timp-de-zbor-p126182383?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72PYOVYYJVcWF-co_NlzFjur9&gclid=Cj0KCQjw77bPBhC_ARIsAGAjjV-JC2lOul-OrYUwsBMv0o_3_mwhgKs6WroU0MGVofJ-gFerOj4XN-AaAguKEALw_wcB) |
| [Display TFT ST7789](https://www.lcdwiki.com/1.3inch_IPS_Module)| Provides real-time visual feedback of MIDI values | [41 RON](https://electronix.ro/produs/display-tft-1-3-inch-st7789-negru/) |
| [Arcade Buttons (4 pack)]() | Tactile mechanical input for sending Note On/Off messages. | [40 RON](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1851-buton-arcade-iluminat-24mm-verde.html) |
| [Resistors 10kΩ]() | Pull-down for buttons | [5 RON](https://www.optimusdigital.ro/en/resistors/1088-025w-10k-resistor.html?search_query=resistor+10k&results=17) |
| [Perfboard] | Final assembly | [8 RON](https://www.optimusdigital.ro/en/protoboards/231-test-wiring-150-x-90-mm.html) |
| [Wires] | Connections | [20 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/882-set-fire-mama-mama-40p-30-cm.html) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-executor](https://github.com/embassy-rs/embassy/tree/main/embassy-executor) | Async Runtime | Core non-blocking peripheral management. |
| [embassy-rp](https://docs.embassy.dev/embassy-rp/0.9.0/rp2040/index.html) | Hardware Abstraction | RP2350 specific GPIO, I2C, TRNG, and SPI configuration. |
| [embassy-usb](https://docs.embassy.dev/embassy-usb/0.6.0/default/index.html) | USB Device Stack | Configuring the Pico's native hardware USB peripheral to communicate with the PC. |
| [vl53l0x](https://github.com/almindor/st7789) | I2C driver for ToF sensor | Reading millimeter distance values from the laser sensor. |
| [usbd-midi](https://crates.io/crates/usbd-midi) | USB MIDI Class | Formats raw button and sensor data into standard USB-MIDI packets. |
| [mipidsi](https://docs.rs/mipidsi/latest/mipidsi/) | ST7789 / SPI Driver | Handles the low-level initialization and SPI communication for the TFT screen. |
| [embedded-graphics](https://docs.rs/embedded-graphics/latest/embedded_graphics/) | 2D graphics library | Used for drawing shapes, text, and visual interface elements on the display. |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://www.instructables.com/AirMIDI-Beyond-Buttons-Knobs/)

...
