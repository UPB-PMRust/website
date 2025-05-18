# Laser Alarm System
Check out for intruders. Or your noisy kids. Or both. 

:::info 

**Author:** Ștefan Adrian Brai \
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/proiect-StefanBrai

:::

## Description

This project implements a security system where a continuous laser beam is emitted toward a photo-sensor. If the beam is interrupted, the system sounds an alarm and pushes a notification to the user’s phone via the Pico W’s Wi-Fi module. A manual button allows arming and disarming the system. The most recent intrusions will be stored in a database with their timestamps

## Motivation

The goal of the project is to develop an affordable and easy-to-implement security solution using low-cost components and the Raspberry Pi Pico W platform, which provides integrated Wi‑Fi connectivity for real-time notifications.

## Architecture

The system is divided into four logical modules:

1. **Laser Module** (emitter + sensor)  
2. **Control Unit** (Raspberry Pi Pico W firmware)  
3. **User Interface** (buzzer + push-button)  
4. **Network Service** (cloud notification endpoint)

```text
+----------------+          +----------------+
|   Laser        |          | Notification   |
| emitter+sensor |          | Service (Wi-Fi)|
+----------------+          +----------------+
        ^                       ^
        v                       |
+----------------+             |
| Raspberry Pi   |-------------+
| Pico W         |
+----------------+
  |            ^
  v            |
+--------+  +--------+
| Buzzer |  | Button |
+--------+  +--------+
```
![Diagram](PM.svg)

## Hardware

We use off-the-shelf components mounted on a breadboard and powered by a 5 V supply. The main utilities are the button, the passive buzzer which will be configured to emit a custom tune, the laser which will just be controlled by the button and the photoresistor which checks the laser output.

## Bill of Materials
| Device                    | Usage                                 | Price (RON) |
| ------------------------- | ------------------------------------- | ----------: |
| [Raspberry Pi Pico W](https://3dstar.ro/raspberry-pi-pico-wh)       | Microcontroller with integrated Wi-Fi |          35 |
| [5 V Laser Diode](https://www.emag.ro/modul-dioda-laser-650nm-5v-12x35-mm-5904162804269/pd/DVM5KLMBM/)           | Laser beam emitter                    |          14 |
| [LDR / Photoresistor](https://www.optimusdigital.ro/)          | Laser-beam interruption sensor        |           1.90 |
| [Passive Buzzer](https://www.optimusdigital.ro/en/buzzers/634-5v-passive-buzzer.html?search_query=buzzer&results=87)            | Audible alarm output                  |           1 |
| [Push-Button](https://www.optimusdigital.ro/en/search?controller=search&orderby=position&orderway=desc&search_query=Buton&submit_search=)               | Manual arm/disarm control             |           2 |
| [Breadboard & Jumper Wires](https://www.optimusdigital.ro/en/kits/2222-breadboard-kit-hq-830-p.html?search_query=Breadboard&results=362) | Prototyping                           |          22 |
| 5 V /3.3 V Power Supply          | System power                          |          0(came with breadboard) |

## Software

| Library         | Description                            | Usage                                    |
| --------------- | -------------------------------------- | ---------------------------------------- |
| [`rp2040-hal`](https://github.com/rp-rs/rp-hal)    | RP2040 hardware abstraction layer      | GPIO, timer and peripheral configuration |
| [`embedded-hal`](https://github.com/rust-embedded/embedded-hal)  | Common embedded-Rust traits            | Unified driver interfaces                |
| [`cyw43`](https://github.com/embassy-rs/embassy/tree/main/embassy-net)         | Wi-Fi driver for CYW43 module          | Network stack and connectivity           |
| [`defmt`](https://github.com/knurling-rs/defmt)         | Efficient logging framework            | Capture debug logs over USB/UART         |
| [`cortex-m-rtic`](https://github.com/rtic-rs/rtic) | Real-time interrupt-driven concurrency | Task scheduling and timing               |


## Links

-  https://how2electronics.com/laser-light-security-system-with-raspberry-pi-pico-ldr/
(found this after i got the idea)
- https://www.instructables.com/Controlling-a-5V-Laser-Diode-With-Raspberry-Pi-Pic/
