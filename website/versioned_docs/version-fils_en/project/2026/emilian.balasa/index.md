\---

id: index

title: Dual-Interface Hybrid Telemetry Dashboard

sidebar\_label: Telemetry Dashboard

\---



\# Dual-Interface Hybrid Telemetry Dashboard

An interactive, physical desktop dashboard that monitors PC hardware telemetry in real-time.



:::info



\*\*Author\*\*: Balasa Emilian-Valentin \\

\*\*GitHub Project Link\*\*: #



:::



\## Description



The goal of this project is to build an interactive, physical desktop dashboard that monitors PC hardware telemetry in real-time. Instead of relying on traditional on-screen software overlays, this device brings the data to the physical world using a custom 3D-printed enclosure, analog gauges, and a digital display.



The system communicates with the host PC via two simultaneous channels: a wireless Bluetooth connection (HC-05 via UART) for receiving telemetry data, and a wired USB connection configured as a Human Interface Device (HID).



On the front panel, two SG90 servo motors act as analog needles constantly displaying the CPU and GPU temperatures. Between them, an SSD1306 OLED screen provides detailed digital readouts. By rotating the KY-040 encoder, the user can switch the OLED menu to view CPU/GPU load percentages or Network speeds.



The dashboard also features an active hardware alarm with multi-sensory feedback. If the CPU or GPU temperature exceeds a hardcoded threshold (e.g., 85°C), the STM32 triggers a piezo buzzer, activates a coin vibration motor, and shifts the WS2812B RGB LED lighting to a flashing red state. Additionally, pressing the encoder button utilizes the USB HID connection to send multimedia keyboard commands (like Mute or Volume Up/Down) directly to the PC.



\## Motivation



I chose this project because I wanted to build something I would actually use every day on my desk, rather than just a laboratory prototype. Monitoring thermals is important for PC performance, but software overlays can be annoying and take up valuable screen space while working or gaming. A dedicated physical dashboard solves this problem elegantly.



From a technical perspective, I wanted a project that forces me to dive deep into Embedded Rust and the `embassy-rs` framework. Managing two separate communication interfaces (UART and USB), driving PWM servos, reading an I2C display, controlling Neopixel LEDs, and handling external interrupts for the encoder at the same time is the perfect scenario to learn and apply asynchronous programming and safe task synchronization in Rust.



\## Architecture



The project revolves around the \*\*STM32 NUCLEO-U545RE-Q\*\* microcontroller acting as the central state machine.

\* \*\*Inputs:\*\* The HC-05 module receives serial data strings from a Python script on the PC. The Rotary Encoder sends EXTI signals to navigate the menu.

\* \*\*Processing:\*\* Using `embassy-executor`, independent tasks run concurrently. Data is passed safely between these tasks using `embassy-sync` Channels.

\* \*\*Outputs:\*\* The STM32 calculates PWM duty cycles to move the servo needles, formats text to the OLED via I2C, triggers GPIO pins for the buzzer and vibration motor, updates the RGB LEDs, and sends standard USB HID keyboard reports.



!\[Project System Architecture](./architecture.png)



\## Log



\### Week 9

Researched requirements, finalized the hardware component list, placed the order, and configured the GitHub repository documentation.



\### Week 10

Breadboard testing of individual components (OLED, Servos, Bluetooth).



\### Week 11

Hardware Milestone presentation and final circuit assembly.



\### Week 12

3D Enclosure design and printing.



\### Week 13

Software Milestone presentation.



\### Week 14

PM Fair final demo.



\## Hardware



The logic runs on 3.3V and 5V. Most components interface directly with the STM32 pins. However, the 3V coin vibration motor draws too much current for a standard GPIO pin. To fix this, I designed a simple driving circuit: the GPIO pin switches an NPN transistor (2N2222) via a 1k Ohm base resistor, allowing external current to power the motor. A 1N4007 flyback diode is placed parallel to the motor to protect the microcontroller.



\### Schematics



\*(Initial architecture block diagram. Full KiCAD schematic to follow after breadboard prototyping).\*

!\[Project System Architecture](./architecture.svg)



\### Bill of Materials



| Device | Usage | Price |

|--------|--------|-------|

| \[STM32 NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The central microcontroller | N/A |

| HC-05 Bluetooth Module | Wireless telemetry reception via UART | \~30 RON |

| SSD1306 OLED Display (0.96") | Digital data readout via I2C | \~15 RON |

| 2x SG90 Micro Servo Motors | Analog temperature gauges via PWM | \~20 RON |

| KY-040 Rotary Encoder | Menu navigation and HID button | \~10 RON |

| Active Piezo Buzzer | Acoustic alarm system | \~5 RON |

| 3V Coin Vibration Motor | Haptic alarm system | \~5 RON |

| WS2812B RGB LED Module | Visual alarm system | \~10 RON |

| 2N2222, 1k Resistor, 1N4007 | Motor driving circuit | \~5 RON |



\## Software



The firmware is developed in `#!\\\[no\\\_std]` Rust using the asynchronous Embassy framework.



| Library | Description | Usage |

|---------|-------------|-------|

| \[embassy-stm32](https://github.com/embassy-rs/embassy) | STM32 Hardware Abstraction Layer | Configuring UART, USB, PWM, I2C, and GPIO |

| \[embassy-usb](https://github.com/embassy-rs/embassy) | USB device stack | Implementing the USB HID multimedia keyboard class |

| \[ssd1306](https://crates.io/crates/ssd1306) | Display driver | Formatting and pushing pixels to the OLED |

| \[rotary-encoder-hal](https://crates.io/crates/rotary-encoder-hal) | Encoder driver | Non-blocking quadrature decoding |

| \[heapless](https://crates.io/crates/heapless) | Data structures | Buffering and parsing Bluetooth strings without dynamic memory allocation |



\## Links



1\. \[Embassy-rs Documentation](https://embassy.dev/)

2\. \[Rust on Embedded Devices](https://docs.rust-embedded.org/)

