# Morse Coder
Converts typed text into Morse code signals (LEDs, buzzer, LCD).

:::info 

**Author**: Andronache Madalina-Georgiana \
**GitHub Project Link**: [Project Repository](https://github.com/UPB-PMRust-Students/proiect-madalinaandronache.git)

:::

## Description

This project enables the user to input text using a 4x4 matrix keypad, which is then converted into Morse code and transmitted through:

+ **Sound signals** using a buzzer.
+ **Light signals** using LEDs.
+ **Visual display** on an LCD screen, where dots and dashes represent Morse code.

Ultimately, the aim is to integrate all components inside a compact system.

## Motivation

This project was chosen because it combines both hardware and software aspects in a manageable way. Developing a text-to-Morse converter offers an opportunity to better understand:
- GPIO management
- Embedded Rust programming
- Hardware-software interactions

It allows practicing embedded programming without requiring complex electronics knowledge.

## Architecture 

The following diagram shows the system architecture:

![System Architecture Diagram](./diagram.svg)

The **Raspberry Pi Pico 2W** acts as the brain of the system, coordinating all interactions between input and output components.

- The **4x4 matrix keypad** provides the user input.
- The **Pico** processes the text and converts it into Morse code.
- The **LEDs** and **buzzer** output the corresponding Morse signals.
- The **LCD screen** displays the Morse code in real time using dots (.) and dashes (_).

## Log

### Week 28 - 4 May

Initially, I encountered problems connecting the Raspberry Pi Pico 2W to the debugger.  

**Problem**: Using jumper wires for the SWD connection resulted in an **unstable link** between the Pico and the debugger, causing random disconnections and flashing failures due to loose connections.

**Solution**: I soldered stable male pins onto the Pico board and also reinforced the jumper cable connections by using more secure, stable headers, preventing movement and improving overall connection stability during debugging.

After stabilizing the programming connection, I focused on testing each hardware component individually to understand their behavior and determine the necessary Rust crates for the project.  
This phase involved experimenting with:

- Raspberry Pi Pico 2W setup,
- 4x4 matrix keypad integration,
- LEDs and buzzer control,
- LCD 1602 screen functionality.

While working with the LCD 1602, I initially encountered display issues.  

**Problem**: The display showed no visible characters, and I suspected a software communication error (wrong crate usage or I2C misconfiguration).

**Solution**: After careful troubleshooting, I discovered that the issue was actually **hardware-related**: the **contrast potentiometer** on the back of the LCD module needed adjustment.  

Once the potentiometer was properly tuned, the LCD correctly displayed characters.

During LED testing, another issue occurred.  

**Problem**: I accidentally burned two LEDs. The initially purchased red LEDs were rated for **2.1V–2.4V forward voltage**, but I mistakenly powered them directly without using an appropriate resistor, which led to their failure.

**Solution**: This helped me realize the importance of properly calculating and setting up voltage/current protection for LEDs.

In parallel with the hardware experiments, I also started writing the initial documentation for the project. I structured the project description, architecture, hardware and software components sections, and started outlining the weekly activity logs.


### Week 5 - 11 May

I again encountered issues with the connection between the Raspberry Pi Pico 2W and the debugger.  

**Problem**: The thin jumper wires used initially were unreliable and caused occasional flashing failures or disconnections during programming — this was mainly due to **transporting the project between home and university**, which led to unstable mechanical connections.

**Solution**: I replaced them with **thicker, high-quality cables**, which provided much better physical contact and stability. This resolved all connection issues.

In parallel, I finalized the **pin mapping** between the Pico board and each component (keypad, LCD, LEDs, and buzzer), ensuring proper physical layout and consistency for both coding and schematic design.

### Week 12 - 18 May

I wrote a minimal Rust test program to verify that all hardware components were correctly connected and functional.

While working with LEDs, I re-evaluated the need for current-limiting resistors.

Problem: Although I had already burned two LEDs earlier, I realized that even if the GPIO pins seem to work, long-term use without protection may damage the Pico board.

Solution: I add resistors in series with each LED, and update the schematic accordingly.

I also worked in KiCad, continuing the design of the schematic. I included headers for all external modules (keypad, LCD, buzzer, LEDs), and clearly labeled all connections between components and the microcontroller. I ensured that each symbol had the correct electrical type (Input/Output/Power) and verified the pin orientation for modules like the keypad and LCD.

In addition, I had to go shopping for a suitable enclosure to house the final version of the project.

### Week 19 - 25 May

I finalized the design of the wooden enclosure that will house the entire hardware system. The enclosure was tailored to fit all components securely, ensuring proper access to the keypad, LCD screen, and power interface, while also providing structural stability for transport and demonstration.

On the software side, I completed the final version of the code. In addition to the initially planned features, I added several enhancements for user interaction and reliability. These include improved input handling, refined LCD messaging, and more robust Morse signal timing.

Finally, I prepared all materials and rehearsed the demonstration for the upcoming PM Fair, ensuring that both hardware and software are ready for public presentation.

## Hardware

| Component | Purpose | Function |
|:----------|:--------|:---------|
| **Raspberry Pi Pico 2W** | Main controller of the system | Reads input text, processes it, and controls outputs to buzzer, LEDs, and LCD |
| **4x4 Matrix Keypad** | Provides text input | Acts as the input device for entering characters |
| **Active Buzzer** | Outputs Morse code through sound | Emits short and long beeps representing dots and dashes |
| **LEDs** | Visual representation of Morse code signals | - When a dot (.) is detected, only **one LED** lights up (the middle one).<br/>- When a dash (_) is detected, **all three LEDs** light up simultaneously. |
| **LCD Display** | Displays the Morse code translation | Shows real-time dot and dash output |
| **Breadboard + Jumper Wires** | Temporary prototyping connections | Connects components to the Raspberry Pi Pico during development |

### Schematics

![Schematics](schematics.svg)

### Photos

![Picture 3](pm3.webp)
![Picture 1](pm1.webp)
![Picture 2](pm2.webp)

### Final design

![Final 2](final2.webp)
![Final 1](final1.webp)

### Bill of Materials

| Device | Usage | Price (RON) | Quantity |
|--------|-------|-------------|----------|
| [Breadboard HQ (830 points)](https://www.ardumarket.ro/ro/product/mb102-830-puncte-fara-lipire-breadboard?gad_source=1&gad_campaignid=22143406947&gbraid=0AAAAA-sic2S12h5YpZvxubae_KFo2O8sn&gclid=CjwKCAjw56DBBhAkEiwAaFsG-jhLz8nRj2WqLYTwF9vHXVysTlAxFiA3vGKmZ9Itg0CHqfYx6FJk5xoC_NMQAvD_BwE) | Prototyping, connecting components | 9,98 | 1 |
| [Breadboard Jumper Wires Set](https://www.optimusdigital.ro/en/wires-with-connectors/12-breadboard-jumper-wire-set.html?search_query=Breadboard+Jumper+Wires+Set&results=22) | Wiring between breadboard and Pico | 7,99 | 1 |
| [5mm Red LED with Diffused Lens](https://www.optimusdigital.ro/en/leds/29-5-mm-red-led-with-difused-lens.html?search_query=5+mm+Red+LED+with+Diffused+Lens&results=9) | Visual Morse code signaling | 0,39 | 7 |
| [400p HQ Breadboard](https://www.optimusdigital.ro/en/breadboards/44-400p-hq-breadboard.html?search_query=400p+HQ+Breadboard&results=6) | Additional prototyping area | 4,56 | 1 |
| [4x4 Matrix Keypad](https://www.optimusdigital.ro/en/touch-sensors/470-4x4-matrix-keyboard-with-female-pin-connector.html?search_query=4x4+Matrix+Keyboard+with+Female+Pin+Connector&results=1) | Input device for typing text | 6,99 | 1 |
| [1602 LCD with I2C Interface and Blue Backlight](https://www.optimusdigital.ro/en/lcds/2894-1602-lcd-with-i2c-interface-and-blue-backlight.html?search_query=1602+LCD+with+I2C+Interface+and+Blue+Backlight&results=2) | Display Morse translation | 16,34 | 1 |
| [PCB Mounted Active Buzzer Module](hhttps://www.optimusdigital.ro/en/buzzers/12513-pcb-mounted-active-buzzer-module.html?search_query=PCB+Mounted+Active+Buzzer+Module&results=1) | Alternate sound module for Morse | 4,98 | 1 |
| [Raspberry Pi Pico 2W](http://optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html?search_query=Raspberry+Pi+Pico+2W&results=36) | Main microcontroller | 39,66 | 2 |
| [150Ω Resistor](https://www.tme.eu/en/details/cf1_4w-150r/tht-resistors/sr-passives/) | Current limiting for LEDs | Already had them | 3 |
| **Total** |  |  **131,11** |  |

## Software

![Keypad Diagram](keypad.webp)

### Keypad Functionality

| Button Label | Description |
|--------------|-------------|
| `HELLO`      | Displays and plays the full Morse code for the word **HELLO**. |
| `S.O.S.`     | Displays and plays the full Morse code for **S.O.S** (`... --- ...`). |
| `DEMO`       | Plays the Morse code of a randomly selected letter to be guessed by the user. |
| `TEST ALL`   | Sends the entire message typed before this key was pressed, showing and playing each character's Morse code. |
| `FUN FACTS`  | Displays a fun fact on the LCD from a predefined rotating list. |
| `MODE`       | Switches between **Text mode** (multitap A–Z) and **Numeric mode** (0–9 input). |
| `0–9`        | Inputs digits in Numeric mode, or letters via multitap logic in Text mode. |

### Software Flow

![Software Flow Diagram](software_flow.webp)

The software operates in a continuous loop, monitoring keypad input. When a key is pressed, the program first checks whether it is a special key (such as HELLO, S.O.S., FUN FACTS, DEMO, or TEST ALL). If so, it executes the corresponding function: displaying or transmitting predefined Morse code sequences, showing a fun fact, playing a Morse quiz, or sending the entire message typed so far. If the key is not a special command, the program proceeds to check the current input mode—Text or Numeric. In Text mode, multitap logic is used to determine the intended character, while in Numeric mode digits are added directly. After a one-second pause without further taps, the current character is confirmed, added to a message buffer, and its Morse code is displayed and played. The system then returns to listening for the next key input.

### Software Dependencies

| Library | Description | Usage in your code |
|:--------|:------------|:-------------------|
| [embassy-rp](https://github.com/embassy-rs/embassy) | HAL for Raspberry Pi Pico W | Used for I2C interface, peripheral initialization |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Asynchronous task runtime | Used for `#[embassy_executor::main]` and async tasks |
| [embassy-time](https://github.com/embassy-rs/embassy) | Asynchronous timers and delays | Used for `Timer::after()` non-blocking delays |
| [lcd1602_driver](https://crates.io/crates/lcd1602_driver) | Driver for controlling LCD1602 over I2C | Used for initializing and writing text to the LCD |
| [defmt](https://github.com/knurling-rs/defmt) | Lightweight embedded logging | Used for debug prints (`defmt::info!`) |
| [defmt-rtt](https://github.com/knurling-rs/defmt) | RTT transport for `defmt` | Sends logs to the host |
| [panic-probe](https://github.com/knurling-rs/defmt) | Panic handler for embedded targets | Handles panics and sends diagnostic info |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Traits for I2C, GPIO and delays | Used indirectly via `embassy-rp` and `lcd1602_driver` |
| [heapless](https://crates.io/crates/heapless) | Fixed-size data structures for no_std | Used for buffer storage (messages, Morse code) |
| [rand](https://crates.io/crates/rand) + `small_rng` | Random number generation | Used for quiz feature (random letter) |

## Links

1. [4x4 Matrix Keypad Integration with Raspberry Pi Pico](https://www.instructables.com/Raspberry-Pi-Pico-4x4-Matrix-Keypad-and-1602-LCD-I/)
2. [1602 LCD with I2C Interface with Raspberry Pi Pico](https://www.youtube.com/watch?v=bXLgxEcT1QU&t=2s&ab_channel=NerdCave)
3. [Rust Programming Guide for Raspberry Pi Pico](https://www.alexdwilson.dev/how-to-program-raspberry-pi-pico-with-rust)

...
