# Sound lights

An lighting system based on an addressable LED strip controlled by sound and IR remote, designed for dynamic ambient effects synchronized with music.

:::info 

**Author**: Izabela-Elena Apostu \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-iza03

:::

## Description

An interactive ambient lighting system based on a Raspberry Pi Pico 2W and a WS2812B LED strip. The lights dynamically respond to environmental sound, generating real-time visual effects. The user can switch between colors and modes using an infrared remote, enhancing the ambiance of any space through both automatic and manual control.



## Motivation

The project idea emerged from my passion for ambient lighting and music. I often enjoy working or relaxing with various lighting setups. This inspired me to build a system where an LED strip reacts to environmental sound, creating synchronized visual effects that follow the rhythm of the music. The user can also manually control the color of the lights using an infrared remote.

## Architecture

The system is composed of four core components that interact with each other in real-time:

- **Ambient Sound Sensor**: captures sound from the environment and sends an analog signal.
- **Infrared Receiver**: receives commands from the remote to switch color modes or power on/off.
- **Raspberry Pi Pico W**: central controller; reads input from the sound sensor and IR receiver, processes data, and controls the LED strip.
- **LED Strip (WS2812B)**: displays colors and patterns according to sound intensity and user commands.

The system is powered via a 5V external supply. The IR receiver and sound sensor provide input signals, while the LED strip acts as output.

![Architecture diagram](architecture.webp)



## Log


### Week 5 – 11 May

TO DO

### Week 12 – 18 May

TO DO

### Week 19 – 25 May

TO DO
## Hardware


1. **Raspberry Pi Pico W**
   - Main microcontroller running the logic of the system. It processes input from the sound sensor and the IR receiver, and controls the LED strip accordingly.

2. **Sound Sensor (MAX4466)**
   - Captures ambient sound and converts it into an analog signal. This signal is used to determine when to flicker the LEDs in sync with music or surrounding noise.

3. **IR Receiver + Remote**
   - Allows the user to interact with the system remotely. Sends control signals to change color or intensity of the LED strip.

4. **WS2812B Addressable LED Strip**
   - RGB LED strip that responds to both ambient sound and user input, creating dynamic lighting effects. Controlled via a single GPIO pin using PWM or digital protocol.

5. **5V 3A Power Supply**
   - Powers the Raspberry Pi Pico W and the LED strip, ensuring stable current for high-intensity lighting.

6. **Breadboard & Jumper Wires**
   - Used for prototyping the circuit without soldering. Allows quick and clean connection between components.

## Schematics

![Schematics](schematic.webp)


## Bill of Materials



| Device | Usage | Price |
|--------|-------|-------|
| [Raspberry Pi Pico W](https://www.optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html) | The microcontroller | [40 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html) |
| [MAX4466 Sound Sensor](https://www.emag.ro/microfon-cu-amplificare-cu-max4466-ajustabil-ai1082/pd/DFLMTFMBM/) | Captures environmental sound | [15 RON](https://www.emag.ro/microfon-cu-amplificare-cu-max4466-ajustabil-ai1082/pd/DFLMTFMBM/) |
| [WS2812B LED Strip](https://ardushop.ro/ro/electronica/900-1193-leduri-rgb-la-banda-neopixels-ws2812b-pretul-este-pentru-un-led.html) | RGB lighting effects | [2 RON/LED x20](https://ardushop.ro/ro/electronica/900-1193-leduri-rgb-la-banda-neopixels-ws2812b-pretul-este-pentru-un-led.html) |
| [IR Kit](https://ardushop.ro/ro/comunicatie/2358-kit-ir-telecomanda-receptor-cablu-6427854032461.html) | Remote control for LED modes | [10 RON](https://ardushop.ro/ro/comunicatie/2358-kit-ir-telecomanda-receptor-cablu-6427854032461.html) |
| [5V 3A Power Supply](https://www.emag.ro/sursa-de-alimentare-5v-3a-pa5-3/pd/DJYZ6TMBM/) | Powers Pico W and LEDs | [23 RON](https://www.emag.ro/sursa-de-alimentare-5v-3a-pa5-3/pd/DJYZ6TMBM/) |
| [Breadboard + Wires](https://www.optimusdigital.ro/ro/kituri/2222-kit-breadboard-hq-830-p.html) | Prototyping connections | [22 RON](https://www.optimusdigital.ro/ro/kituri/2222-kit-breadboard-hq-830-p.html) |

## Software



| Library           | Description                               | Usage                                             |
|------------------|-------------------------------------------|---------------------------------------------------|
| embassy-rp       | Embedded async runtime for Raspberry Pi   | Provides asynchronous execution on Pico W         |
| embassy-executor | Task scheduling for embedded systems      | Allows concurrent non-blocking task management    |
| smart-leds       | LED driver crate for WS2812B              | Used to control the RGB LED strip                 |
| defmt             | Lightweight logging framework             | Debugging and runtime logging                     |
| defmt-rtt         | RTT-based transport for `defmt` logs      | Enables real-time terminal output via USB         |
| pio              | Programmable I/O interface crate          | Drives precise WS2812B timings using PIO hardware |

## Links
