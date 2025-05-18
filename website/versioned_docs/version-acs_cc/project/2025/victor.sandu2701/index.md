# AeroGuard 180

A smart gas-detection and evacuation system with a 180 ° pivoting fan

**Author**: Victor Sandu

**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-Victor270104

## Description

AeroGuard 180 continuously monitors the concentration of smoke and automatically evacuates contaminated air through a blower fan that can be rotated up to 180 °. The system sends Wi‑Fi alerts and provides local visual (LED) and audible (buzzer) feedback.

- Dual MQ‑2 gas sensors detect smoke, LPG, and CO in real‑time.

- An SG90 micro‑servo sweeps a 3 – 6 V DC blower fan up to 180 °, directing the airflow toward the gas source.

- Raspberry Pi Pico 2 W runs async Rust firmware, processes sensor data, and publishes MQTT/HTTP alerts.

- Status LEDs (green = safe, red = alert) and a passive buzzer offer immediate on‑site feedback.

## Motivation

**This project**  lets me dive deeper into embedded development with Rust while addressing a real‑world safety issue: preventing smoke or fires in homes and small labs by automating ventilation and issuing rapid notifications.

## Architecture

MQ‑2 gas sensors – supply analog voltage proportional to smoke/gas concentration. The Pico’s ADC samples these signals continuously for threshold analysis.

SG90 micro‑servo – receives PWM commands from the Pico and rotates the blower fan up to 180 °, so contaminated air is pushed away from the danger zone.

L298N dual H‑bridge – drives the 3 – 6 V DC blower fan under Pico control (start/stop and optional speed modulation).

Status LEDs & passive buzzer – connected to GPIO pins; green LED indicates safe air, red LED and buzzer indicate dangerous concentrations and trigger ventilation.

**LED indicators**:

- **Green LED**: Air concentration is safe.
- **Red LED**: Air concentration is dangerous.

Power source – a single USB‑C cable from the laptop delivers 5 V. The Pico distributes logic power, while the L298N shares the same 5 V rail for the fan. No external batteries are required.

---

## Log


### Week   5 - 11 May
<!--
- Finalized project idea and listed all components.
- Ordered hardware and prepared the schematic layout.
-->
### Week   12 - 18 May
<!--
- Breadboard test: analog readout from MQ‑2 and PPM threshold calibration.
- Built first servo + blower prototype; verified 180 ° rotation.
-->
### Week  19 - 25 May
<!--
- Integrated Rust firmware using embassy‑rp (async executor).
- Implemented Wi‑Fi MQTT notifications and OTA update stub.
-->
---

## Hardware


1. **Raspberry Pi Pico 2 W**:
   - **Purpose**: Central control unit.
   - **Function**: Runs Rust code to control sensors, fan, and LEDs.

2. **MQ‑2 Gas Sensor Module**:
   - **Purpose**: Detect flammable gases, CO, and smoke.
   - **Function**: Provide analog voltage proportional to gas concentration—read by the Pico ADC to trigger ventilation and alarms.

3. **Micro‑servo SG90 180 °**:
   - **Purpose**:  Mechanically orient the blower fan.
   - **Function**: Receives PWM from the Pico to sweep up to 180 °, directing the airflow toward the contamination source.

4. **DC Blower Fan 3 V – 6 V**:
   - **Purpose**: Evacuate contaminated air from the monitored space.
   - **Function**: Powered through an H‑bridge; speed and on/off state controlled by the Pico.

5. **L298N Dual H‑Bridge Driver**:
   - **Purpose**: Safely switch and drive the DC blower motor (with one spare channel for future actuators).
   - **Function**: Translates Pico logic‑level signals into motor currents up to 2 A.

6. **LEDs (Red, Green)**:
   - **Purpose**: Visual indicator.
   - **Function**: Green = safe, Red = alert; driven by Pico GPIO lines via current‑limiting resistors.

7. **Passive Buzzer 3 V – 3.3 V**:
   - **Purpose**: Audible warning when gas thresholds are exceeded.
   - **Function**: Generates a tone when toggled by a Pico GPIO pin.

---

### Hardware Overview

- The **Pico 2 W** reads air concentration.
- Based on air conditions, it controls the **ventilator** and the **motors**.
- LEDs give real-time feedback on air concentration.

---

### Schematics

![System Diagram](diagram.svg)

---

## Bill of Materials


| Device | Usage | Price |
|--------|-------|-------|
| [Raspberry Pi Pico 2W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=Raspberry+Pi+Pico+2W&results=26)                                                       | Main microcontroller                 | ~40 RON |
| [Senzor Module Gas MQ-2](https://www.optimusdigital.ro/ro/senzori-de-gaze/107-modul-senzor-gas-mq-2.html?search_query=Senzor+gaz&results=34)                                                                    | Air concentration sensing            | ~11 RON |
| [Micro Servomotor SG90 180°](https://www.optimusdigital.ro/ro/motoare-servomotoare/2261-micro-servo-motor-sg90-180.html?search_query=SG90&results=11)                                                           | Pivot for fan                        | ~12 RON |
| [Driver module Dual L298N](https://www.optimusdigital.ro/ro/drivere-de-motoare-cu-perii/145-driver-de-motoare-dual-l298n.html?search_query=L298N&results=4)                                                     | Driver for motor                     | ~11 RON |
| [Motor DC 3V-6V](https://ardushop.ro/ro/electronica/752-motor-dc-3v-6v-cu-reductor-1-48-6427854009609.html?gad_campaignid=22058879462)                                                                          | Motor                                | ~11 RON |
| [3V Passive Buzzer](https://www.optimusdigital.ro/ro/audio-buzzere/12247-buzzer-pasiv-de-33v-sau-3v.html?search_query=Buzzer+Pasiv+de+3.3V+sau+3V&results=1)                                                    | Buzzer                               | ~1 RON |

---

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy/tree/main/embassy-rp) | RP2040 async embedded support | Core hardware abstraction |
| [embedded-hal](https://crates.io/crates/embedded-hal) | Hardware abstraction layer | GPIO, ADC, PWM |

---

## Links

1. [Rust Embedded Book](https://docs.rust-embedded.org/book/)
2. [Embassy Rust Project](https://github.com/embassy-rs/embassy)
3. [Raspberry Pi Pico SDK Documentation](https://datasheets.raspberrypi.com/pico/raspberry-pi-pico-c-sdk.pdf)
