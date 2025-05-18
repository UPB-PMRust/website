# Music Lights
A Raspberry Pi Pico system that synchronizes LED lighting with music

:::info

**Author:** Antonio-Christian Carazeanu  \
**GitHub Project Link:** [link_to_github](https://github.com/UPB-PMRust-Students/proiect-antocarazeanu)

:::

## Description
Music Lights is a project that uses a Raspberry Pi Pico to control an LED strip based on live music input. A microphone captures ambient audio, and a signal processing algorithm detects the beat and dominant frequencies. The LEDs react in real-time by changing color and brightness, creating an immersive audio-visual experience.

## Motivation
The idea behind this project came from a desire to blend music with dynamic visual effects. I wanted to explore how embedded systems can process audio and produce synchronized lighting, making music more interactive and engaging.

## Architecture
![Arhitecture diagram](./arhitecture.svg)


The project is composed of the following main components:
- **Microcontroller**: Raspberry Pi Pico running real-time firmware using Embassy.
- **Audio Input**: MAX4466 Microphone module connected to the Pico's ADC, capturing audio signals.
- **Signal Processing**: Algorithm performing FFT using the `microfft` library to analyze frequencies.
- **LED Control**: LED strip controlled via the RP2040's **PIO unit** using `embassy-rp` and a compatible WS2812 PIO program (like `ws2812-pio` or similar). Requires a **Logic Level Shifter** (3.3V to 5V) for the data signal.
- **Output**: WS2812 LED strip visualizing music dynamically.

Connections:
- Microphone Analog Out → Pico ADC GPIO (e.g., GPIO26)
- Pico GPIO (PIO Output) → Level Shifter (Low Voltage Input)
- Level Shifter (High Voltage Output) → LED Strip Data In (via ~330Ω resistor)
- Buttons → Pico GPIO Inputs (with 10kΩ pull-up/pull-down resistors)
- Pico USB → Power (for Pico) and programming
- **Separate 5V Power Supply → LED Strip (+5V, GND)**
- **Separate 5V Power Supply → Level Shifter (HV Pin)**
- **Common Ground:** Crucial connection between Pico GND, Level Shifter GND, and the Separate 5V Power Supply GND.

## Log
- **Week 5 - 11 May**: Research and initial setup of Raspberry Pi Pico and LED strip. Completed the project description.
- **Week 12 - 18 May**:
- **Week 19 - 25 May**:   

## Hardware
- Raspberry Pi Pico W (x2 - one for development/debug)
- WS2812 LED Strip (3x 10cm segments = 18 LEDs)
- MAX4466 Microphone Module
- Logic Level Shifter Module
- Jumper wires (M-M, M-F) and Breadboard
- USB cables (for Pico power/programming)

## Schematics
*(KiCAD schematics to be added here)*

## Bill of Materials

| Item                                              | Quantity | Unit Price | Total Price | Notes                                   |
|---------------------------------------------------|----------|------------|-------------|-----------------------------------------|
| [Jumper Wires for Breadboard](https://www.optimusdigital.ro/ro/fire-fire-mufate/12-set-de-cabluri-pentru-breadboard.html?search_query=Set+Fire+pentru+Breadboard&results=37) | 1        | 7.99 RON   | 7.99 RON    |                                         |
| [Raspberry Pi Pico W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=raspberry+pi+pico+2&results=26) | 2        | 39.66 RON  | 79.32 RON   |                                         |
| [Breadboard HQ (830 tie points)](https://sigmanortec.ro/Breadboard-830-puncte-MB-102-p125923983) | 1        | 9.98 RON   | 9.98 RON    |                                         |
| [White Pin Header 2.54 mm (40p)](https://www.optimusdigital.ro/ro/cautare?controller=search&orderby=position&orderway=desc&search_query=header+pini+alb&submit_search=) | 2        | 0.99 RON   | 1.98 RON    |                                         |
| [Colored Jumper Wires Female-Male (40p, 15 cm)](https://www.optimusdigital.ro/ro/toate-produsele/877-set-fire-mama-tata-40p-15-cm.html) | 1        | 7.99 RON   | 7.99 RON    |                                         |
| [Electrolytic Capacitor 1000 µF, 35 V](https://www.optimusdigital.ro/ro/componente-electronice-condensatoare/3004-condensator-electrolitic-de-1000-uf-la-35-v.html?search_query=condensator+electrolitic&results=105) | 1        | 2.99 RON   | 2.99 RON    |                                         |
| [Electrolytic Capacitor 470 µF, 50 V](https://www.optimusdigital.ro/ro/componente-electronice-condensatoare/3008-condensator-electrolitic-de-470-uf-la-50-v.html?search_query=condensator+electrolitic&results=105) | 1        | 0.79 RON   | 0.79 RON    |                                         |
| USB Cables (for LEDs and Pico)                   | 2        | -          | -           | Powered from laptop                     |
| [Addressable LED Strip WS2812, 10 cm (60led/m)](https://sigmanortec.ro/Banda-LED-adresabila-RGB-WS2812-60led-m-IP67-10cm-p166125661) | 3        | 2.69 RON   | 8.07 RON    |                                         |
| [Button 12x12x7.3 mm](https://sigmanortec.ro/Buton-12x12x7-3-p160373654) | 2        | 1.10 RON   | 2.20 RON    | For switching LED modes                 |
| [Button Cap 12x12x7.3 mm, Blue](https://sigmanortec.ro/capac-buton-12x12x73-albastru) | 1        | 0.34 RON   | 0.34 RON    |                                         |
| [Button Cap 12x12x7.3 mm, Red](https://sigmanortec.ro/capac-buton-12x12x73-albastru) | 1        | 0.34 RON   | 0.34 RON    |                                         |
| [MAX4466 Microphone Module with Amplifier](https://sigmanortec.ro/modul-microfon-max4466-cu-amplificare-castig-reglabil-23-5vdc) | 1        | 7.18 RON   | 7.18 RON    |                                         |
| [4-Channel Logic Level Converter Module](https://sigmanortec.ro/Modul-Translator-nivel-logic-I2C-IIC-bidirectional-8-canale-3-3V-5V-TXS0108E-p136254032) | 1        | 4.00 RON   | 4.00 RON    |                                         |
| 330 Ω Resistor                                    | 2        | 0.12 RON   | 0.24 RON    | For buttons                             |
| 10k Ω Resistor                                    | 1        | 0.13 RON   | 0.13 RON    | For data line                           |

**Estimated Total:** ~ 133.84 RON (excluding USB cables)




## Software
This list details the recommended Rust crates for the "Music Lights" project using the Embassy async runtime on the Raspberry Pi Pico.

| Category          | Library                                      | crates.io Link                                       | Description & Usage in Project                                      |
| :---------------- | :------------------------------------------- | :--------------------------------------------------- | :------------------------------------------------------------------ |
| **Runtime Async** | `embassy-executor`                           | [link](https://crates.io/crates/embassy-executor)    | Core asynchronous task executor. Runs all concurrent operations.   |
|                   | `embassy-time`                               | [link](https://crates.io/crates/embassy-time)        | Async time primitives (Delay, Timer, Instant). Essential for timing. |
|                   | `embassy-sync`                               | [link](https://crates.io/crates/embassy-sync)        | Async synchronization (Mutex, Channel, Signal). For safe data sharing between tasks. |
| **HAL (Embassy)** | `embassy-rp`                                 | [link](https://crates.io/crates/embassy-rp)          | Async HAL for RP2040 peripherals (ADC, PIO, GPIO, DMA, etc.).     |
| **LED Control**   | `ws2812-pio`                                 | [link](https://crates.io/cratesx/ws2812-pio)        | **(Recommended)** PIO program & driver for WS2812 LEDs.             |
|                   | `pio`                                        | [link](https://crates.io/crates/pio)             | Base crate for defining PIO programs (dependency for `ws2812-pio`). |
| **Audio Proc.**   | `microfft`                                   | [link](https://crates.io/crates/microfft)        | `no_std` Fast Fourier Transform implementation for frequency analysis. |
|                   | `libm`                                       | [link](https://crates.io/crates/libm)            | `no_std` math functions (sqrt, powf, etc.) for FFT post-processing. |
| **Core Embedded** | `cortex-m`                                   | [link](https://crates.io/crates/cortex-m)        | Low-level access to ARM Cortex-M core APIs.                       |
|                   | `cortex-m-rt`                                | [link](https://crates.io/crates/cortex-m-rt)     | Minimal runtime for Cortex-M (entry point, exceptions).           |
|                   | `panic-probe`  *or* `panic-halt`         | [link](https://crates.io/crates/panic-probe)  [link](https://crates.io/crates/panic-halt) | Panic handler implementation. `panic-probe` works well with `defmt`. |
| **Logging**       | `defmt`                                      | [link](https://crates.io/crates/defmt)           | **(Recommended)** Efficient embedded logging framework.             |
|                   | `defmt-rtt`                                  | [link](https://crates.io/crates/defmt-rtt)       | **(Recommended)** RTT transport for `defmt` (requires debug probe). |
| **HAL Utilities** | `embedded-hal`  `embedded-hal-async`    | [link](https://crates.io/crates/embedded-hal)  [link](https://crates.io/crates/embedded-hal-async) | Standard HAL traits used for interoperability (often dependencies). |
|                   | `embedded-io`  `embedded-io-async`      | [link](https://crates.io/crates/embedded-io)  [link](https://crates.io/crates/embedded-io-async) | Standard IO traits (often dependencies).                          |

**Notes:**

1.  `ws2812-pio` or a similar PIO-based implementation is recommended for controlling WS2812 LEDs with `embassy-rp` on the RP2040, leveraging its Programmable I/O capabilities. An alternative like `ws2812-spi` might require adaptation for async use.
2.  Using `defmt` also typically involves adding `defmt-rtt` for transport and a suitable panic handler like `panic-probe`. Basic runtime crates like `cortex-m` and `cortex-m-rt` are also fundamental dependencies.

## Links
- [Lab1](https://pmrust.pages.upb.ro/docs/acs_cc/lab/01)
- [Lab2](https://pmrust.pages.upb.ro/docs/acs_cc/lab/02)
- [Lab3](https://pmrust.pages.upb.ro/docs/acs_cc/lab/03)
- [Lab4](https://pmrust.pages.upb.ro/docs/acs_cc/lab/04)
- [Lab5](https://pmrust.pages.upb.ro/docs/acs_cc/lab/05)
- [Lab6](https://pmrust.pages.upb.ro/docs/acs_cc/lab/06)
- [Lab7](https://pmrust.pages.upb.ro/docs/acs_cc/lab/07)
