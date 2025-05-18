# Soda Dispenser
A proximity-triggered, multi-flavour soda dispenser

:::info

**Author:** Popescu Vladimir \
**GitHub Project Link:** [Github Project Repository](https://github.com/UPB-PMRust-Students/proiect-ErrorsEverywhere)

:::

## Description
The dispenser now offers **several drink flavours**.  
A front-panel button array lets the user choose the desired beverage, while the two ultrasonic sensors have been repurposed to measure **glass height** so the firmware can pour the correct volume for short or tall glasses. Once the button is pressed and a suitable glass is detected, the MCU actuates the corresponding pump.

## Motivation
Commercial soda fountains are bulky and expensive, while DIY builds often limit users to a single flavour and a fixed pour size. By adding a four‑button flavour selector and ultrasonic cup‑height sensing, this dispenser stays low‑cost and hands‑free yet delivers multiple beverages in portions that match short or tall glasses—making.

## Subsystems 
- **Button pad** (3 tactile switches) selects flavour.  
- **Ultrasonic sensors** measure distance to the glass rim to infer glass height.  
- **RP2350 MCU** implements a state-machine: wait for flavour select → validate glass height → run correct pump for a preset volume.  
- **Three pumps** deliver the chosen drink from separate reservoirs.

## Hardware
Current component lineup:
- [Raspberry Pi Pico 2 W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) 
- [Two HC-SR04 ultrasonic sensors](https://www.optimusdigital.ro/ro/senzori-senzori-ultrasonici/2328-senzor-ultrasonic-de-distana-hc-sr04-compatibil-33-v-i-5-v.html) 
- [Three tactile push-buttons (6×6×6 mm)](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1119-buton-6x6x6.html) 
- [Three submersible pumps (3–6 V DC)](https://sigmanortec.ro/Pompa-apa-submersibila-3-6VDC-verticala-p172447502) 
- [MP1584EN step-down module](https://www.optimusdigital.ro/en/adjustable-step-down-power-supplies/166-mp1584en-mini-dc-dc-step-down-module.html) 
- [Three NPN transistors 2N2222](https://www.optimusdigital.ro/en/transistors/935-transistor-npn-2n2222-to-92.html)  
- [Two 40-pin 2.54 mm male pin headers](https://www.optimusdigital.ro/ro/componente-electronice-headere-de-pini/462-header-de-pini-albastru-254-mm-40p.html)  
- [Breadboard 400 points](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/44-breadboard-400-points.html)  
- [Breadboard jumper-wire set](https://www.optimusdigital.ro/ro/fire-fire-mufate/12-set-de-cabluri-pentru-breadboard.html)  
- [Plusivo 250-piece resistor kit](https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/10928-plusivo-kit-250-buc-rezistoare.html)  
- [Micro-USB cable (1 m)](https://www.optimusdigital.ro/ro/cabluri-cabluri-usb/11939-cablu-negru-micro-usb-1-m.html)  
- [Battery holder for two × 18650 cells](https://www.optimusdigital.ro/ro/suporturi-de-baterii/941-suport-de-baterii-2-x-18650.html)  

## Bill of Materials

| Component | Quantity | Price per Unit (RON) | Total Price (RON) |
|-----------|----------|----------------------|-------------------|
| [Fire Colorate jumper (10 cm, 10 p)](https://www.optimusdigital.ro/ro/fire-fire-mufate/650-fire-colorate-mama-tata-10p.html) | 2 | 2.99 | 5.98 |
| [Raspberry Pi Pico 2 W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) | 2 | 39.66  | 79.32  |
| [Breadboard 400 points](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/44-breadboard-400-points.html) | 1 | 4.56 | 4.56 |
| [Breadboard jumper-wire set](https://www.optimusdigital.ro/ro/fire-fire-mufate/12-set-de-cabluri-pentru-breadboard.html) | 1 | 7.99 | 7.99 |
| [Plusivo resistor kit (250 pcs)](https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/10928-plusivo-kit-250-buc-rezistoare.html) | 1 | 14.99 | 14.99 |
| [Micro-USB cable (1 m)](https://www.optimusdigital.ro/ro/cabluri-cabluri-usb/11939-cablu-negru-micro-usb-1-m.html) | 2 | 3.99 | 7.98 |
| [Battery holder for two × 18650 cells](https://www.optimusdigital.ro/ro/suporturi-de-baterii/941-suport-de-baterii-2-x-18650.html) | 1 | 3.99 | 3.99 |
| [HC-SR04 ultrasonic sensor](https://www.optimusdigital.ro/ro/senzori-senzori-ultrasonici/2328-senzor-ultrasonic-de-distana-hc-sr04-compatibil-33-v-i-5-v.html) | 2 | 14.99 |29.98 |
| [Tactile push-button (6×6×6 mm)](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1119-buton-6x6x6.html) | 3 | 0,36 | 1.08 |
| [Submersible pump (3–6 V DC)](https://sigmanortec.ro/Pompa-apa-submersibila-3-6VDC-verticala-p172447502) | 3 | 7.15 | 21.45 |
| [NPN transistor 2N2222 (TO-92)](https://www.optimusdigital.ro/en/transistors/935-transistor-npn-2n2222-to-92.html) | 3 | 0.17 | 0.51 |
| [MP1584EN step-down module](https://www.optimusdigital.ro/en/adjustable-step-down-power-supplies/166-mp1584en-mini-dc-dc-step-down-module.html) | 1 | 6.49 | 6.49 |
| [40-pin 2.54 mm male pin header](https://www.optimusdigital.ro/ro/componente-electronice-headere-de-pini/462-header-de-pini-albastru-254-mm-40p.html) | 4 | 0.99 | 3.96 |

**Total Estimated Cost:** 188.28 RON
## Software

| Library / Crate              | Description                                    | Usage                                  |
|------------------------------|------------------------------------------------|----------------------------------------|
| [`embassy-time`, `embassy-executor`](https://github.com/embassy-rs/embassy) | Async runtime for Cortex-M                    | Non-blocking timing & tasks            |
| [`rp-pico` HAL](https://github.com/raspberrypi/rp2040-hal) | Pico W hardware abstraction                   | GPIO, PWM, UART                        |
| [`panic-halt`](https://github.com/rust-lang/panic-halt) | Halt on panic                                 | Error handling                         |
| [`defmt` + `probe-run`](https://github.com/knurling-rs/defmt) | Logging                                       | On-device debugging                    |
| [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer                    | Standardized interfaces for peripherals |
| [`defmt-rtt`](https://github.com/knurling-rs/defmt) | Real-Time Transfer for logging                | Low-latency logging via RTT            |
| [`nb`](https://docs.rs/nb/)     | Non-blocking operations                        | Handling non-blocking I/O              |
| [`usb-device`](https://github.com/mciantyre/usb-device) | USB device support for embedded systems       | USB functionality (e.g., communication or programming) |

## Logs
### Log — Week 1 (28 Apr – 4 May 2025)

| Date       | Activity |
|------------|----------|
| 28 Apr 2025 | Compiled and verified full component list. |
| 30 Apr 2025 | Updated design: added 4-button flavour selector; repurposed ultrasonic sensors for glass-height detection; revised architecture and documentation. |

### Log — Week 2 (5-11 May 2025)

| Date       | Activity |
|------------|----------|
| 5 May 2025 | Added `embedded-hal`, `defmt-rtt`, `nb`, and `usb-device` crates to the software stack for better hardware abstraction, logging, non-blocking operations, and USB support. And add pricing to my tables.|
