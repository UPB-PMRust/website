# MIDI Controller

A physical MIDI controller with piano-layout keys, real-time chord detection, and BPM sensing over WiFi.

:::info

**Author**: Racu Delia-Andreea \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-DeliaRacu

:::

## Description

A physical MIDI controller with 13 piano-layout buttons, two potentiometers, an OLED display, an RGB LED, and a microphone. It connects via ESP-01 to a browser dashboard over WebSocket, where sound is synthesised using the Web Audio API. Firmware is written in Rust with Embassy-RS on an STM32U545.
Two modes: single note (one button + octave selector) and chord (3+ buttons, auto-detects Major/Minor/Diminished/Augmented). The microphone detects rhythmic input for real-time BPM detection, shown on the OLED and in the browser; the RGB LED pulses in sync with the beat, its colour set by the active chord type.

## Motivation

Studying both IoT at Politehnica and music at the Conservatory, I wanted a portable device to sketch chord progressions and test melodies without a full piano -- no DAW, no drivers, no cables. The BPM detection lets me clap a rhythm and see the tempo lock in instantly, just like a real musician would. Technically, the project spans async Rust firmware, ADC signal processing, I2C, UART, and WebSocket streaming across the full IoT stack.

## Architecture

The system is built around five concurrent Embassy-RS tasks running on the STM32U545 microcontroller:

- **button_task** -- scans 13 GPIO pins every 10 ms, maintains pressed-key state, triggers chord detection
- **adc_task** -- reads both potentiometers, maps ADC values to volume (0-100) and octave (2-6)
- **mic_task** -- samples MAX4466 via ADC, detects amplitude peaks, computes rolling BPM, drives RGB LED via PWM
- **display_task** -- runs chord detection on state change, updates OLED over I2C
- **wifi_task** -- serialises state as JSON, sends to ESP-01 over UART using AT commands

The ESP-01 connects to the local WiFi network and exposes a WebSocket server. The browser dashboard connects to it and uses the Web Audio API to synthesise sound in real time.

![Architecture Diagram](architecture_diagram.drawio.svg)

## Log

### Week 5 - 11 May
- Decided on the final project idea: a physical MIDI controller with piano-layout keys, chord detection, and BPM sensing

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The main microcontroller is the **STM32U545** on a NUCLEO-U545RE-Q development board, chosen for its multiple ADC channels, hardware I2C, hardware UART, and native Embassy-RS support. WiFi connectivity is handled by an **ESP-01 (ESP8266)** module communicating via UART AT commands. Thirteen **tactile push buttons** are arranged in the physical layout of a piano octave, each with a 10 kOhm pull-down resistor and software debounce. Two **10 kOhm potentiometers** control master volume and octave selection via ADC. An **SSD1306 OLED display** (128x64 px, I2C) shows the note or chord name, octave, WiFi status, and current BPM. An **RGB LED** (5mm, common-cathode) provides visual feedback: colour encodes the chord type and brightness pulses with the detected BPM. The **MAX4466 microphone module** provides an analog output (0-3.3V) read by ADC for peak detection and BPM computation.

### Schematics


**Pin mapping:**

| Signal | MCU Pin | Notes |
| --- | --- | --- |
| Buttons 0-12 | PC0-PC12 | 10 kOhm pull-down, active HIGH |
| Volume pot | PA0 (ADC1_IN5) | 0-3.3V |
| Octave pot | PA1 (ADC1_IN6) | 0-3.3V |
| Microphone | PA2 (ADC1_IN7) | MAX4466 output |
| OLED SDA | PB7 (I2C1_SDA) | 4.7 kOhm pull-up |
| OLED SCL | PB6 (I2C1_SCL) | 4.7 kOhm pull-up |
| RGB LED R | PB0 (TIM3_CH3) | 220 Ohm series resistor |
| RGB LED G | PB1 (TIM3_CH4) | 220 Ohm series resistor |
| RGB LED B | PA6 (TIM3_CH1) | 220 Ohm series resistor |
| ESP-01 TX | PA9 (USART1_TX) | 3.3V logic |
| ESP-01 RX | PA10 (USART1_RX) | 3.3V logic |

### Bill of Materials

| Device | Usage | Price |
| --- | --- | --- |
| [NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main microcontroller (STM32U545) | ~120 RON |
| [ESP-01 (ESP8266) + adapter](https://www.optimusdigital.ro/en/wireless-esp8266/13244-esp-01-esp8266-wifi-module.html) | WiFi module, UART AT commands | ~15 RON |
| [Tactile push buttons x13](https://www.optimusdigital.ro/en/buttons-and-switches/1119-6x6x6-push-button.html) | Piano octave keys | ~5 RON |
| [Potentiometer 10 kOhm x2](https://www.optimusdigital.ro/en/potentiometers/901-10k-wh148-linear-potentiometer.html) | Volume + octave selection | ~5 RON |
| [OLED SSD1306 128x64 I2C](https://www.optimusdigital.ro/en/lcds/194-096-i2c-oled-display.html) | Displays note, chord, BPM | ~20 RON |
| [RGB LED 5mm common-cathode](https://www.optimusdigital.ro/en/leds/483-rgb-led-5mm-common-cathode.html) | Chord colour + BPM pulse | ~1 RON |
| [MAX4466 microphone module](https://www.optimusdigital.ro/en/microphones/1328-max4466-microphone-amplifier.html) | BPM detection via ADC | ~25 RON |
| Resistors (10 kOhm x13, 220 Ohm x3, 4.7 kOhm x2) | Pull-downs, current limiting, I2C pull-ups | ~3 RON |
| Breadboard + jumper wires | Prototyping | ~15 RON |

## Software

| Library | Description | Usage |
| --- | --- | --- |
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for STM32U545 | GPIO, ADC, I2C, UART, PWM drivers |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task scheduler | Runs all concurrent firmware tasks |
| [embassy-sync](https://github.com/embassy-rs/embassy) | Synchronisation primitives | Mutex and Signal for shared state between tasks |
| [embassy-time](https://github.com/embassy-rs/embassy) | Timestamp utilities | Inter-beat interval measurement for BPM detection |
| [ssd1306](https://github.com/jamwaffles/ssd1306) | OLED display driver over I2C | Renders note, chord name and BPM on the display |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Text layout and rendering on the OLED |
| [heapless](https://github.com/rust-embedded/heapless) | Stack-allocated collections | Ring buffer for BPM interval history |
| [serde-json-core](https://github.com/rust-embedded-community/serde-json-core) | JSON serialisation for no_std | Serialises musical events into WebSocket messages |

## Links

1. [Embassy-RS framework](https://embassy.dev)
2. [STM32U545 reference manual](https://www.st.com/en/microcontrollers-microprocessors/stm32u545re.html)
3. [Web Audio API (MDN)](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API)
4. [MAX4466 datasheet](https://www.analog.com/en/products/max4466.html)
5. [SSD1306 datasheet](https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf)
