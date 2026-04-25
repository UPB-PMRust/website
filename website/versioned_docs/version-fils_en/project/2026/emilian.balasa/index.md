\---

id: index

title: Dual-Interface Hybrid Telemetry Dashboard

sidebar\_label: Telemetry Dashboard

\---



\# Dual-Interface Hybrid Telemetry Dashboard



\*\*Student:\*\* Balasa Emilian-Valentin

\*\*Group:\*\* 1221ED

\*\*Lab Assistant:\*\* Eva Cosma



\## 1. Description of the Functionality



The goal of this project is to build an interactive, physical desktop dashboard that monitors PC hardware telemetry in real-time. Instead of relying on traditional on-screen software overlays, this device brings the data to the physical world using a custom 3D-printed enclosure, analog gauges, and a digital display.



The system communicates with the host PC via two simultaneous channels: a wireless Bluetooth connection (HC-05 via UART) for receiving telemetry data, and a wired USB connection configured as a Human Interface Device (HID).



On the front panel, two SG90 servo motors act as analog needles constantly displaying the CPU and GPU temperatures. Between them, an SSD1306 OLED screen provides detailed digital readouts. By rotating the KY-040 encoder, the user can switch the OLED menu to view CPU/GPU load percentages or Network speeds.



The dashboard also features an active hardware alarm with multi-sensory feedback. If the CPU or GPU temperature exceeds a hardcoded threshold (e.g., 85°C), the STM32 triggers a piezo buzzer, activates a coin vibration motor, and shifts the WS2812B RGB LED lighting to a flashing red state, providing immediate acoustic, haptic, and visual warnings. Additionally, pressing the encoder button utilizes the USB HID connection to send multimedia keyboard commands (like Mute or Volume Up/Down) directly to the PC.



\## 2. Motivation for Choosing the Project



I chose this project because I wanted to build something I would actually use every day on my desk, rather than just a laboratory prototype. Monitoring thermals is important for PC performance, but software overlays can be annoying and take up valuable screen space while working or gaming. A dedicated physical dashboard solves this problem elegantly.



From a technical perspective, I wanted a project that forces me to dive deep into Embedded Rust and the `embassy-rs` framework. Managing two separate communication interfaces (UART and USB), driving PWM servos, reading an I2C display, controlling Neopixel LEDs, and handling external interrupts for the encoder at the same time is the perfect scenario to learn and apply asynchronous programming and safe task synchronization in Rust.



\## 3. Architecture \& Component Interconnection



The project revolves around the \*\*STM32 NUCLEO-U545RE-Q\*\* microcontroller acting as the central state machine.



\* \*\*Inputs:\*\* \* The HC-05 module receives serial data strings (e.g., "C:75 G:80") from a Python script running on the PC.

&#x20; \* The Rotary Encoder sends EXTI signals to navigate the menu and trigger HID commands.

\* \*\*Processing:\*\* Using `embassy-executor`, independent tasks run concurrently (e.g., reading Bluetooth data, updating the UI). Data is passed safely between these tasks using `embassy-sync` Channels without blocking the execution.

\* \*\*Outputs:\*\* \* The STM32 calculates PWM duty cycles to move the servo needles.

&#x20; \* It formats and sends text to the OLED via I2C.

&#x20; \* It triggers GPIO pins for the buzzer and the transistor controlling the vibration motor.

&#x20; \* It updates the WS2812B RGB LEDs using a dedicated PWM/Timer stream.

&#x20; \* It sends standard USB HID keyboard reports over the USB CDC line.



!\[Project System Architecture](./architecture.png)



\## 4. Hardware Design



\### 4.1. Hardware Description

The logic runs on 3.3V and 5V. Most components (Bluetooth, OLED, Encoder, Buzzer, LEDs) interface directly with the STM32 pins. However, the 3V coin vibration motor draws too much current for a standard GPIO pin. To fix this, I designed a simple driving circuit: the GPIO pin switches an NPN transistor (2N2222) via a 1k Ohm base resistor, allowing external current to power the motor. A 1N4007 flyback diode is placed parallel to the motor to protect the microcontroller from voltage spikes when the motor stops.



\### 4.2. Bill of Materials (Hardware)

\* 1x STM32 NUCLEO-U545RE-Q

\* 1x HC-05 Bluetooth Module

\* 1x SSD1306 OLED Display (0.96", I2C)

\* 2x SG90 Micro Servo Motors

\* 1x KY-040 Rotary Encoder (with push-button)

\* 1x Active Piezo Buzzer

\* 1x 3V Coin Vibration Motor

\* 1x WS2812B RGB LED Module (Neopixel)

\* 1x NPN Transistor (2N2222), 1x 1k Resistor, 1x 1N4007 Diode

\* Custom 3D-printed enclosure, Breadboard, Dupont wires



\## 5. Software Design



The firmware is developed in `#!\[no\_std]` Rust.

\* \*\*`embassy-stm32`\*\*: The main HAL for configuring UART, USB, PWM, I2C, and GPIO.

\* \*\*`embassy-usb`\*\*: To implement the USB HID multimedia keyboard class.

\* \*\*`ssd1306` \& `embedded-graphics`\*\*: For formatting and pushing pixels to the OLED.

\* \*\*`rotary-encoder-hal`\*\*: For hardware-agnostic, non-blocking quadrature decoding.

\* \*\*`heapless`\*\*: For buffering and parsing the incoming Bluetooth telemetry strings safely without dynamic memory allocation.



\## 6. Weekly Log



\* \*\*Week 9:\*\* Researched requirements, finalized the hardware component list, placed the order, and configured the GitHub repository documentation.

\* \*\*Week 10:\*\* (To be updated - Breadboard testing of individual components)

\* \*\*Week 11:\*\* (To be updated - Hardware Milestone presentation)

\* \*\*Week 12:\*\* (To be updated - 3D Enclosure design and printing)

\* \*\*Week 13:\*\* (To be updated - Software Milestone presentation)

\* \*\*Week 14:\*\* (To be updated - PM Fair final demo)

