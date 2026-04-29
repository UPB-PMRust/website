# Radar Wars — Embedded Battleship over Wi-Fi
 A two-player Battleship game running on a pair of Raspberry Pi Pico 2 W that talk to each other directly over Wi-Fi.

:::info
**Author**: Barbu Andrei \
**Group**: 1221EA \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-ZaBoss667
:::

## Description

 Two identical hand-held consoles, each built around a Raspberry Pi Pico 2 W with a 2.8" TFT screen, an analog joystick, and a buzzer. One console acts as its own Wi-Fi access point, the other joins it, and they exchange shots over a TCP socket. Players place their ships on a grid at the start, then take turns aiming the cursor with the joystick and firing, with sound effects and a live radar view on the display. The whole thing runs on bare-metal Rust with the embassy async framework — no operating system underneath.
 
## Motivation

 I would say my motivation mainly came from loving to play games, so when I looked through last year projects, I knew I had to do one myself. And after looking for a bit, I came to the conclusion that no one tried to do this classic game (hopefully for no reason in particular). So here I am now trying to make it happen.
 
## Architecture

 ```
                                                                    +-------------------+
                                                                   | USB Power Bank B  |
                                                                   | (5V via USB)      |
                                                                   +---------+---------+
                                                                             |VBUS
                                                                             v
    +-------------------+                                          +---------+----------+
    | USB Power Bank A  |                                          |  Raspberry Pi      |
    | (5V via USB)      |                                          |  Pico 2 W  (B)     |
    +---------+---------+                                          |  (Central Ctrl)    |
              |VBUS                                                |                    |
              v                                                    |                    |
    +---------+----------+                                         |                    |
    |  Raspberry Pi      |    Wi-Fi (CYW43439, SoftAP <-> STA)     |                    |
    |  Pico 2 W  (A)     |<========================================|                    |
    |  (Central Ctrl)    |           SSID: RadarWars               |                    |
    |                    |           TCP port 1234                 |                    |
    |                    |========================================>|                    |
    +--+-------+-------+-+                                         +--+-------+-------+-+
       |       |       |                                              |       |       |
   SPI |   ADC |  GPIO |  PWM                                     SPI |   ADC |  GPIO |  PWM
 (SCK, |  (x,y)|  (SW) | (audio)                                (SCK, |  (x,y)|  (SW) | (audio)
 MOSI, |       |       |                                       MOSI, |       |       |
 CS,   |       |       |                                       CS,   |       |       |
 DC,   |       |       |                                       DC,   |       |       |
 RST)  |       |       |                                       RST)  |       |       |
       v       v       v         v                                   v       v       v       v
   +---+---+ +-+-----+-++---+ +--+----+                          +---+---+ +-+----+-++---+ +-+----+
   |ILI9341| |Analog | |EC | |Passive|                           |ILI9341| |Analog| |EC | |Passv |
   | 2.8"  | |Joy-   | |  -| |Piezo  |                           | 2.8"  | |Joy-  | |  -| |Piezo |
   | TFT   | |stick  | |   | |Buzzer |                           | TFT   | |stick | |   | |Buzzer|
   +-------+ +-------+ +---+ +-------+                           +-------+ +------+ +---+ +------+

                                 ^                                       ^
                                 |                 SWD + UART            |
                                 +------------+-------------+------------+
                                              |             |
                                         (one at a time,    |
                                          moved between     |
                                          consoles during   |
                                          development)      |
                                              v             v
                                       +------+-------------+------+
                                       |  Raspberry Pi Pico 2      |
                                       |  (debugprobe firmware)    |
                                       |  — bench-only, not on     |
                                       |    the finished console   |
                                       +-------------+-------------+
                                                     |USB
                                                     v
                                              +------+-------+
                                              |  Dev PC      |
                                              | (probe-rs +  |
                                              |  cargo run)  |
                                              +--------------+
```											  
											  
											  
											  
											  
##HARDWARE CONNECTIONS:**								  
											  
											  
```											  
	+--------------------+          +-------------------------------+
    |  USB Power Bank    |   VBUS   |  Raspberry Pi Pico 2 W        |
    |  (5V, USB-C)       +--------->|  VBUS (pin 40) / GND          |
    +--------------------+          +-------------------------------+


    +--------------------+          +-------------------------------+
    |  ILI9341 2.8" TFT  |  VCC  -->|  VBUS  (5V, pin 40)           |
    |  (red PCB)         |  LED  -->|  3V3   (pin 36)               |
    |  (240x320, SPI)    |  GND  -->|  GND                          |
    |                    |  SCK  -->|  GP2   (SPI0 SCK, pin 4)      |
    |                    |  SDI  -->|  GP3   (SPI0 MOSI, pin 5)     |
    |                    |  CS   -->|  GP5   (pin 7)                |
    |                    |  DC   -->|  GP6   (pin 9)                |
    |                    |  RESET-->|  GP7   (pin 10)               |
    |  T_*   (touch)     |  —       |  not connected (unused)       |
    |  SDO   (MISO)      |  —       |  not connected (display-only) |
    +--------------------+          +-------------------------------+


    +--------------------+          +-------------------------------+
    |  Analog Joystick   |  +5V  -->|  3V3   (pin 36)               |
    |  Module            |  GND  -->|  GND                          |
    |                    |  VRx  -->|  GP26  (ADC0, pin 31)         |
    |                    |  VRy  -->|  GP27  (ADC1, pin 32)         |
    |                    |  SW   -->|  GP22  (GPIO w/ pull-up, p29) |
    +--------------------+          +-------------------------------+


    +--------------------+          +-------------------------------+
    |  Passive Piezo     |   +   -->|  GP15  (PWM slice 7B, pin 20) |
    |  Buzzer            |          |        (via 100 Ohm series R) |
    |                    |   -   -->|  GND                          |
    +--------------------+          +-------------------------------+


    +--------------------+
    |  CYW43439 Wi-Fi    |          internal to Pico 2 W — handled
    |  (onboard)         |          on GP23/24/25/29 by the SDK,
    |                    |          no external wiring required.
    +--------------------+


    +------------------------------+
    |  Pico 2 W USB Port           |
    |  (to power bank only —       |
    |   not used as data link,     |
    |   Wi-Fi carries the game)    |
    +------------------------------+
```
## Log

### Week 4 idea thinking
I mainly tried to find a good idea for the project.
Researched past projects.
Saw games were pretty popular, and my idea came.

### Week 5 ordering parts
Tried to think what hardware do I need.
Placed the orders on the sites.

### Week 6 unboxing of the parts
All the stuff came and I mostly just made sure its all in good condition.
Ordered more stuff I realised i would need.

### Week 7 soldering and testing
Went to get the stuff soldered (pins on the Raspberrys).
Managed to get the screen to light up, and confirm all the pieces are working and usable for the project.

### Week 8 further testing
Started to research harder exact crates that I will need.
Attempted to make a functional display of a grid, important core functionality of the game.
Tested the joystick movement and interactions, to assure its able to move the target cursor accurately when it will be fully coded.

## Hardware

The project centres on a Raspberry Pi Pico 2 W as the microcontroller on each console — two identical hand-helds, one per player. A 2.8" ILI9341 SPI TFT display renders the game grid, radar sweep, and HUD. An analog joystick module with an integrated push-button provides all navigation and firing input via the onboard ADC and a GPIO. A passive piezo buzzer generates hit, miss and victory tones through a PWM output. The two consoles talk to each other over the Pico 2 W's onboard CYW43439 Wi-Fi chip — one console acts as a SoftAP, the other joins it, and gameplay is synchronised over a raw TCP socket. Each console is powered from a USB power bank for fully untethered play. Everything is wired on 400-point breadboards using male-male and male-female jumper wires, a 100 Ω series resistor in front of the buzzer, and 2.54 mm pin headers soldered onto the bare Pico boards.

Raspberry Pi Pico 2 W (Central Controller): runs the async Rust firmware (embassy-rp). Drives the SPI display, samples the joystick axes over the ADC, generates PWM tones on the buzzer pin, and owns the Wi-Fi radio + TCP socket that links it to the other console.

ILI9341 2.8" SPI TFT Display: shows the fleet-placement grid, radar view of enemy waters, HUD, and end-of-game stats.
Connection: SPI0 (SCK, MOSI) plus CS, DC and RESET on three GPIOs.

Analog Joystick Module: moves the cursor for ship placement and targeting; the built-in Z-axis push-button drops ships, fires shots, and confirms menu selections (long-press rotates ships during placement).
Connection: VRx and VRy into two ADC-capable GPIOs, SW into a pull-up GPIO.

Passive Piezo Buzzer: plays short jingles on every game event (fire click, miss thud, hit stab, sunk fanfare, victory/defeat songs).
Connection: PWM output on one GPIO through a 100 Ω series resistor to the buzzer's positive pin.

Onboard CYW43439 Wi-Fi Chip: hosts or joins a private SoftAP and carries the game protocol between the two consoles.
Connection: internal to the Pico 2 W, driven over the PIO-SPI bus on four dedicated pins — no external wiring.

USB Power Bank (per console): supplies 5 V into the Pico's VBUS via the USB-C port, powering the MCU, the display (on 5 V), and the 3V3 rail feeding the joystick.

### Schematics
To be added

## Bill of Materials

| Device | Usage | Price |
|---|---|---|
| [Raspberry Pi Pico 2 W (×2)](https://www.tme.eu/ro/details/sc1633/raspberry-pi-sisteme-incorporate/raspberry-pi/raspberry-pi-pico-2-w/) | Main microcontrollers — one per player console. Run game logic, render graphics, and handle the Wi-Fi socket connection. | 60.98 RON |
| [Raspberry Pi Pico 2 (×1)](https://www.tme.eu/ro/details/sc1631/raspberry-pi-sisteme-incorporate/raspberry-pi/raspberry-pi-pico-2/) | Debug probe — flashed with debugprobe firmware to provide SWD + UART over USB for flashing and defmt logs. | 21.25 RON |
| [2.8" SPI TFT Display, ILI9341 (×2)](https://www.emag.ro/display-tactil-tft-lcd-2-8-inch-320x240-touchscreen-spi-driver-ili9341-arduino-rx961/pd/DSFJ88YBM/) | Game UI output — grid, HUD, menus. Partial-rendering via mousefood's framebuffer avoids flicker. | 182.32 RON |
| [Analog Joystick Module (×2)](https://www.optimusdigital.ro/en/touch-sensors/742-ps2-joystick-breakout.html?srsltid=AfmBOorYcm2h4umi7Qc37LbWZXiMuPDyAKH7ZBI5942Gw_CSUAbQH1Rq) | Grid navigation and "click-to-fire" via the built-in Z-axis button. | 10.70 RON |
| [3 V / 3.3 V Passive Buzzer (×2)](https://www.optimusdigital.ro/en/buzzers/12247-3-v-or-33v-passive-buzzer.html?srsltid=AfmBOoovHKr4ygKxlqucUWs6mDL-HmpC-fmKlpz41-RI8nHOhrIFRCEt) | PWM-driven sound effects (firing, hit, miss, game-over jingle). | 1.98 RON |
| [400-Point Breadboard (×2)](https://www.emag.ro/breadboard-400-puncte-ai059-s69/pd/DRJ66JBBM/) | Power rails and tie-points for mounting the Pico and wiring all peripherals. | 14.08 RON |
| [Male-Male Jumper Wire Set (40 pcs)](https://www.optimusdigital.ro/en/wires-with-connectors/890-set-fire-tata-tata-40p-30-cm.html?search_query=male+male+jumper+wire&results=54) | Inter-component wiring and the probe-to-target debug harness. | 7.98 RON |
| [20 cm 10-pin M-F Jumper Wires (×4)](https://www.optimusdigital.ro/en/wires-with-connectors/212-female-female-10p-20-cm-wire.html?search_query=male+to+female&results=427) | Connecting the through-hole pins on the display and joystick modules to the breadboard. | 19.8 RON |
| [40p 2.54 mm Male Pin Header Strips (×4)](https://www.optimusdigital.ro/en/pin-headers/462-colored-40p-254-mm-pitch-male-pin-header-blue.html?search_query=40p+2.54+mm+Male+Pin+Header+Strips&results=6)| Soldered onto the bare Pico boards so they can plug into the breadboards. | 3.96 RON |
| [110 pcs Resistor Assortment Set](https://www.optimusdigital.ro/en/resistors/13607-resistor-set-110-resistors.html?search_query=resistor&results=222) | General prototyping; the 100 Ω value is used in series with the GPIO driving the buzzer. | 7 RON |
| [Power bank (x2)](https://www.vexio.ro/baterii-externe-powerbank/extreme/2034617-power-bank-quark-2000mah-black/) | Powering up each Raspberry Pi Pico W | 40.00 RON |
| **Total** | | **370.05 RON** |



## Software

| Library | Description | Usage |
|---|---|---|
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async task executor for no-std embedded Rust | Runs input polling, networking, rendering, and audio concurrently on a single core. |
| [embassy-rp](https://crates.io/crates/embassy-rp) | HAL for RP2040 / RP2350 | Async access to SPI, ADC, PWM, GPIO, and PIO. |
| [embassy-time](https://crates.io/crates/embassy-time) | Async timers and Duration / Instant types | Input debouncing, redraw tick, buzzer note lengths. |
| [embassy-sync](https://crates.io/crates/embassy-sync) | Async channels, signals, and mutexes | Event plumbing between the input, network, and render tasks. |
| [embassy-futures](https://crates.io/crates/embassy-futures) | select / join combinators | Main loop waits on "joystick event OR network message OR redraw tick" simultaneously. |
| [embassy-net](https://crates.io/crates/embassy-net) | Async TCP/IP stack (smoltcp under the hood) | Single TCP socket between the two consoles. |
| [cyw43](https://crates.io/crates/cyw43) | Driver for the Infineon CYW43439 Wi-Fi chip | Brings up Wi-Fi in SoftAP mode on the server console and STA mode on the client. |
| [cyw43-pio](https://crates.io/crates/cyw43-pio) | PIO-backed SPI driver for the CYW43 bus | Uses a PIO state machine for the radio's proprietary bus, freeing the hardware SPI for the display. |
| [static_cell](https://crates.io/crates/static_cell) | Zero-cost initialization of 'static singletons | Required by embassy-net and cyw43 to hand them 'static state references. |
| [embedded-graphics](https://crates.io/crates/embedded-graphics) | 2D primitives (pixels, shapes, text) for embedded devices | The common drawing abstraction consumed by both mipidsi (as sink) and mousefood (as its rendering backend). |
| [mipidsi](https://crates.io/crates/mipidsi) | MIPI / SPI display driver | Owns the hardware SPI and drives the ILI9341 panel. Exposes an embedded_graphics::DrawTarget. |
| [mousefood*](https://crates.io/crates/mousefood) | embedded-graphics backend for Ratatui, with a partial-redraw framebuffer | Bridges Ratatui's cell buffer to the ILI9341 via embedded-graphics; its internal framebuffer enables flicker-free partial updates. |
| [ratatui*](https://crates.io/crates/ratatui) | Immediate-mode terminal UI framework | Builds every game screen — placement grid, targeting view, HUD, waiting and end-of-game screens — via layout constraints and widgets. |
| [heapless](https://crates.io/crates/heapless) | Fixed-capacity collections (Vec, String, Deque) for no-std | Fleet bitmap, shot history, and network buffers without needing an allocator. |

## Links
1. [Embassy GitHub Repository](https://github.com/embassy-rs/embassy)
2. [Embassy Documentation Book](https://embassy.dev/book/index.html)
3. [crates.io (Rust Package Registry)](https://crates.io/)                                                                                                                                                                                                                        