# Precision Parking Simulator
A smart parking assistant that detects vehicles, controls access via a physical barrier, provides visual feedback through RGB leds, and plays contextual audio cues.

:::info

**Author**: Netoiu Oana-Mihaela \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-oanaNetoiu

:::
## Description

The Automated Parking Barrier System is built around an STM32 microcontroller that orchestrates multiple peripherals to simulate a smart parking gate. An HC-SR04 ultrasonic sensor monitors the distance of an approaching vehicle. The distance at which the vehicle is and at which it must be are displayed on a 0.96" OLED screen. Based on the distance, the system controls an SG90 servo motor to raise or lower a physical barrier. Visual alerts are provided via an RGB LED, while a DFPlayer Mini MP3 module plays specific audio files (e.g., successful parking, collision warning) through a 1W 8-Ohm speaker, if the 30 second timer runs out.

## Motivation

I chose this project as a fun way to implement the core concepts learned during the labs, such as asynchronous task management with the Embassy framework, peripheral interfacing via I2C and UART, and hardware control through PWM and GPIO. Although my project is a simple simulation, it serves as a crucial exercise in peripheral orchestration and concurrency. I intended to better understand how sensors, actuators, and communication modules interact within a safety-first Rust environment in order to enhance the complexity of my future projects.

## Architecture

The STM32 interfaces with the components using the following protocols:

- **I2C Bus**: Drives the 0.96" OLED Display for rendering text and basic UI elements.
- **UART**: Communicates with the DFPlayer Mini to trigger specific audio tracks from the microSD card.
- **PWM**: Controls the angle of the SG90 servo motor to simulate the physical barrier movement.
- **GPIO**: 
  - HC-SR04 ultrasonic sensor (Trigger pin as output, Echo pin as input).
  - RGB LED pins for visual state indication.
  - Push buttons for manual override or interaction.

**Component Connections:**

| Component | Interface | STM32 Pins |
|-----------|-----------|------------|
| 0.96" OLED Display | I2C | SDA / SCL (TBD) |
| DFPlayer Mini | UART | TX / RX (TBD) |
| SG90 Servo Motor | PWM | PWM Output (TBD) |
| HC-SR04 Sensor | GPIO | Trig / Echo (TBD) |
| RGB LED | GPIO / PWM | R, G, B pins (TBD) |
| Push Buttons | GPIO | Digital inputs (TBD) |

## Log

### Week 1-4

I started doing research on potential project ideas and what software and hardware I would need for them. At last, I chose the final project idea and started focusing on it.

### Week 5-8

Ordered all the hardware components from multiple suppliers. Set up the Rust development environment and initialized the project using the Embassy framework. Prepared the microSD card by formatting it to FAT32 and organizing the audio cues for the DFPlayer Mini. Designed the visual layout for the OLED display and the state machine logic for the 30-second timer while awaiting delivery.

### Week 9

Received all the hardware components, including the STM32 Nucleo board. Unboxed and organized the components for assembly. Verified the pinout compatibility between the Nucleo-U545RE-Q and the peripheral modules. Started the initial hardware assembly on the breadboards and began the first connectivity tests for the power rails and basic GPIO.

## Hardware

The core of the project is the STM32 Nucleo-U545RE-Q board. The visual interface is handled by a 0.96" I2C OLED display. Distance detection is measured by the HC-SR04 ultrasonic sensor, which dictates the state of the SG90 180-degree servo motor, controlling the barrier. Audio feedback is processed by a DFPlayer Mini MP3 module reading from a FAT32-formatted microSD card, outputting to a 1W 8-Ohm speaker. Visual feedback is provided by 3 common-cathode RGB LEDs, and manual inputs are handled by the 6x6x6 push button. The circuit is prototyped on 1 to 3 400-point breadboards using standard Dupont jumper wires.

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | [106.00 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D) |
| [HC-SR04 Ultrasonic Sensor](http://www.bitmi.ro/electronica/senzor-ultrasonic-hc-sr04-10406.html) | Distance measurement | [6.09 RON](http://www.bitmi.ro/electronica/senzor-ultrasonic-hc-sr04-10406.html) |
| [RGB LED (Common Cathode)](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/483-led-rgb-catod-comun.html?search_query=rgb&results=95) | Visual feedback | [2.97 RON](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/483-led-rgb-catod-comun.html?search_query=rgb&results=95) |
| [220Ω Resistor 0.25W](https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/1097-rezistor-025w-220.html?search_query=Rezistor+0.25W+220%CE%A9&results=3) | Current limiting for LEDs | [0.90 RON](https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/1097-rezistor-025w-220.html?search_query=Rezistor+0.25W+220%CE%A9&results=3) |
| [0.96" SSD1306 OLED Display (I2C)](http://www.bitmi.ro/componente-electronice/ecran-oled-0-96-cu-interfata-iic-i2c-10488.html) | Display output | [18.98 RON](http://www.bitmi.ro/componente-electronice/ecran-oled-0-96-cu-interfata-iic-i2c-10488.html) |
| [DFPlayer Mini MP3 Module](https://www.drot.ro/platforma-arduino/4857-modul-audio-cu-mp3-incorporat-dfplayer.html) | Audio playback | [12.42 RON](https://www.drot.ro/platforma-arduino/4857-modul-audio-cu-mp3-incorporat-dfplayer.html) |
| [MicroSD Card 2GB](https://www.emag.ro/card-de-memorie-microsd-siks-2gb-cu-adaptor-sd-cmm2g/pd/DW37NZMBM/) | Audio storage | [39.90 RON](https://www.emag.ro/card-de-memorie-microsd-siks-2gb-cu-adaptor-sd-cmm2g/pd/DW37NZMBM/) |
| [USB Card Reader](https://www.emag.ro/card-reader-spacer-46-in-1-usb-2-0-spcr-658/pd/DWBXN3BBM/) | Writing data to MicroSD | [12.10 RON](https://www.emag.ro/card-reader-spacer-46-in-1-usb-2-0-spcr-658/pd/DWBXN3BBM/) |
| [8Ω 1W Speaker](https://www.drot.ro/platforma-arduino/121355-difuzor-1w-8-ohm-20-x-30-mm.html) | Audio output | [6.32 RON](https://www.drot.ro/platforma-arduino/121355-difuzor-1w-8-ohm-20-x-30-mm.html) |
| [SG90 Micro Servo Motor](http://www.bitmi.ro/electronica/servomotor-sg90-180-grade-9g-10496.html) | Mechanical movement of barrier | [9.99 RON](http://www.bitmi.ro/electronica/servomotor-sg90-180-grade-9g-10496.html) |
| [Push Button](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1119-buton-6x6x6.html?search_query=buton&results=163) | User input | [0.36 RON](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1119-buton-6x6x6.html?search_query=buton&results=163) |
| [Male-Male Jumper Wires](http://www.bitmi.ro/componente-electronice/40-fire-dupont-tata-tata-30cm-10505.html) | Connections | [7.99 RON](http://www.bitmi.ro/componente-electronice/40-fire-dupont-tata-tata-30cm-10505.html) |
| [Male-Female Jumper Wires](http://www.bitmi.ro/electronica/40-fire-dupont-tata-mama-30cm-10504.html) | Connections | [7.99 RON](http://www.bitmi.ro/electronica/40-fire-dupont-tata-mama-30cm-10504.html) |
| [Breadboard (400 points)](http://www.bitmi.ro/electronica/breadboard-400-puncte-pentru-montaje-electronice-rapide-10633.html) | Prototyping | [20.97](http://www.bitmi.ro/electronica/breadboard-400-puncte-pentru-montaje-electronice-rapide-10633.html) |
| [Barrier]  | Mechanical structure | [have_not_bought_yet] |

## Software

| Library / Interface | Description | Usage |
|---------------------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Embassy HAL for STM32 | I2C (OLED), UART (DFPlayer), PWM (Servo), GPIO (HC-SR04, Buttons, LED), RNG |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor | Concurrently running the 30-second timer, sensor polling, and display updates |
| [embassy-time](https://github.com/embassy-rs/embassy) | Timers and delays | Handling the 30-second logic, ultrasonic timeouts, and button debouncing |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware Abstraction Layer traits | Standardized interfaces for the peripheral drivers |
| [ssd1306](https://github.com/jamwaffles/ssd1306) | OLED display driver | Initialization and framebuffer management for the 0.96" display |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Drawing text, shapes, and distance metrics on the OLED |
| Hardware RNG | Random Number Generator driver | Generating random values |
| UART / USART | Serial communication interface | Sending hexadecimal command packets to the DFPlayer Mini |
| TIM / PWM | Timer / Pulse Width Modulation | Generating specific duty cycles to control the SG90 servo motor angle |

## Links

1. [Embassy async framework](https://embassy.dev/)
2. [Embedded Rust 101 course labs](https://embedded-rust-101.wyliodrin.com/docs/fils_en/lab/01)
3. [Nucleo-U545RE-Q user manual](https://www.st.com/resource/en/user_manual/um3062-stm32u5-nucleo64-board-mb1841-stmicroelectronics.pdf)
4. [SSD1306 Crate Documentation](https://docs.rs/ssd1306/latest/ssd1306/)
5. [Embedded Graphics Library](https://docs.rs/embedded-graphics/latest/embedded_graphics/)
6. [DFPlayer Mini Technical Manual](https://wiki.dfrobot.com/DFPlayer_Mini_SKU_DFR0299)
7. [HC-SR04 Ultrasonic Sensor Datasheet](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf)
8. [SG90 Servo Motor Specifications](https://www.google.com/search?q=https://datasheetspdf.com/pdf-file/791970/TowerPro/SG90/1&authuser=1)
9. [Embassy STM32 RNG Documentation](https://www.google.com/search?q=https://docs.embassy.dev/embassy-stm32/git/stm32u545re/rng/index.html&authuser=1)