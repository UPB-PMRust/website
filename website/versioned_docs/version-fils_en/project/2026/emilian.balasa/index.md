# Dual-Interface Hybrid Telemetry Dashboard

An interactive, physical desktop dashboard that monitors PC hardware telemetry in real-time.



:::info



**Author:** Balasa Emilian-Valentin \

**GitHub Project Link:** https://github.com/UPB-PMRust-Students/fils-project-2026-EmilianBalasa



:::



## Description



The goal of this project is to build an interactive, physical desktop dashboard that monitors PC hardware telemetry in real-time. Instead of relying on traditional on-screen software overlays, this device brings the data to the physical world using a custom 3D-printed enclosure, analog gauges, and a digital display.



The system communicates with the host PC via two simultaneous channels: a wireless Bluetooth connection (HC-05 via UART) for receiving telemetry data, and a wired USB connection configured as a Human Interface Device (HID).



On the front panel, two SG90 servo motors act as analog needles constantly displaying the CPU and GPU temperatures. Between them, an SSD1306 OLED screen provides detailed digital readouts. By rotating the KY-040 encoder, the user can switch the OLED menu to view CPU/GPU load percentages or Network speeds.



The dashboard also features an active hardware alarm with multi-sensory feedback. If the CPU or GPU temperature exceeds a hardcoded threshold (e.g., 85°C), the STM32 triggers a piezo buzzer, activates a coin vibration motor, and shifts the WS2812B RGB LED lighting to a flashing red state. Additionally, pressing the encoder button utilizes the USB HID connection to send multimedia keyboard commands (like Mute or Volume Up/Down) directly to the PC.



## Motivation



I chose this project because I wanted to build something I would actually use every day on my desk, rather than just a laboratory prototype. Monitoring thermals is important for PC performance, but software overlays can be annoying and take up valuable screen space while working or gaming. A dedicated physical dashboard solves this problem elegantly.



From a technical perspective, I wanted a project that forces me to dive deep into Embedded Rust and the `embassy-rs` framework. Managing two separate communication interfaces (UART and USB), driving PWM servos, reading an I2C display, controlling Neopixel LEDs, and handling external interrupts for the encoder at the same time is the perfect scenario to learn and apply asynchronous programming and safe task synchronization in Rust.



## Architecture



The project revolves around the **STM32 NUCLEO-U545RE-Q** microcontroller acting as the central state machine.

* **Inputs:** The HC-05 module receives serial data strings from a Python script on the PC. The Rotary Encoder sends EXTI signals to navigate the menu.
* **Processing:** Using *embassy-executor*, independent tasks run concurrently. Data is passed safely between these tasks using *embassy-sync* Channels.
* **Outputs:** The STM32 calculates PWM duty cycles to move the servo needles, formats text to the OLED via I2C, triggers GPIO pins for the buzzer and vibration motor, updates the RGB LEDs, and sends standard USB HID keyboard reports.



!\[Project System Architecture](./architecture.svg)



## Log



### Week 5

Defined the project's core idea and established the general system direction: a physical telemetry dashboard. Researched communication protocols between the PC and STM32 (UART via Bluetooth and USB HID) and drafted the initial architecture documentation. Placed the order for the STM32 development board.



### Week 6-7

Finalized the list of remaining hardware components (OLED, servo motors, encoder, HC-06 module) and ordered them. While waiting for delivery, I studied the Embassy framework documentation for Rust and created the first theoretical pinout and connection diagrams.



### Week 8-9

Since the hardware components are still in transit, I focused on planning and documentation. I configured the GitHub repository, finalized the documentation for Milestone 1, and began working on the 3D enclosure design to ensure it is ready for printing once the physical parts arrive for final measurements.



## Hardware



The logic runs on 3.3V and 5V. Most components interface directly with the STM32 pins. However, the 3V coin vibration motor draws too much current for a standard GPIO pin. To fix this, I designed a simple driving circuit: the GPIO pin switches an NPN transistor (2N2222) via a 1k Ohm base resistor, allowing external current to power the motor. A 1N4007 flyback diode is placed parallel to the motor to protect the microcontroller.



## Schematics

Place your KiCAD or similar schematics here in SVG format.



## Bill of Materials

|Device|Usage|Price|
|-|-|-|
|[STM32 NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)|Main controller|125 RON|
|[HC-06 Bluetooth Module](https://components101.com/wireless/hc-06-bluetooth-module-pinout-datasheet)|Wireless telemetry reception via UART|20.85 RON|
|[SSD1306 OLED Display (0.96")](https://www.adafruit.com/product/3296)|Digital data readout via I2C|16.30 RON|
|[2x SG90 Micro Servo Motors](https://components101.com/motors/servo-motor-basics-pinout-datasheet)|Analog temperature gauges via PWM|23.61 RON|
|[KY-040 Rotary Encoder](https://components101.com/sensors/ky-040-rotary-encoder-module)|Menu navigation and HID button|14.68 RON|
|[Active Piezo Buzzer](https://components101.com/misc/buzzer-pinout-working-datasheet)|Acoustic alarm system|10.49 RON|
|[3V Coin Vibration Motor](https://www.adafruit.com/product/1201)|Haptic alarm system|18.51 RON|
|[WS2812B RGB LED Stick](https://www.adafruit.com/product/1426)|Visual alarm system|14.18 RON|
|[Breadboard (Large)](https://www.optimusdigital.ro/)|Prototyping base|19.52 RON|
|[Dupont Cables (M-M \& M-F)](https://www.optimusdigital.ro/)|Interconnections|38.33 RON|
|[2N2222, 1k Resistor, 1N4007](https://www.optimusdigital.ro/)|Motor driving circuit components|36.55 RON|



## Software



|Library|Description|Usage|
|-|-|-|
|[embassy-stm32](https://github.com/embassy-rs/embassy)|STM32 Hardware Abstraction Layer|Configuring UART, USB, PWM, I2C, and GPIO|
|[embassy-executor](https://github.com/embassy-rs/embassy)|Async task executor|Schedules concurrent firmware tasks|
|[embassy-usb](https://github.com/embassy-rs/embassy)|USB device stack|Implementing the USB HID multimedia keyboard class|
|[ssd1306](https://crates.io/crates/ssd1306)|Display driver|Formatting and pushing pixels to the OLED|
|[embedded-graphics](https://github.com/embedded-graphics/embedded-graphics)|2D drawing library|Rendering text and UI overlays on the OLED|
|[rotary-encoder-hal](https://crates.io/crates/rotary-encoder-hal)|Encoder driver|Non-blocking quadrature decoding|
|[heapless](https://crates.io/crates/heapless)|Data structures|Buffering and parsing Bluetooth strings without dynamic memory allocation|





## Links



1. [Embassy-rs Documentation](https://embassy.dev/)
2. [Rust on Embedded Devices](https://docs.rust-embedded.org/)

