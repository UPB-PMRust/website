# Wi-Fi Enabled Motion-Activated Security Alert System

A smart motion detection system powered by Raspberry Pi Pico W and Rust.

:::info

**Author**: Bilge Büşra Arık  
**GitHub Project Link**: **GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-bilgearikk

:::

---

## Description

This project implements a Wi-Fi-enabled motion sensor system that detects movement using a PIR sensor and sends real-time alerts via HTTP. The system is designed using Raspberry Pi Pico W and written in Rust. It aims to provide a lightweight, low-cost, and portable security device that can be used in homes, offices, or academic settings.

Features include:
- PIR-based motion detection
- Wi-Fi connectivity using the `esp-wifi` crate
- HTTP-based notifications using `ureq` or `IFTTT`
- Optional LED or buzzer for local feedback
- Compact and battery-friendly design

---

## Motivation

This project was inspired by a need for simple, cost-effective IoT-based security solutions that can be built by students using modern embedded technologies. The goal is to gain hands-on experience with embedded Rust programming, microcontroller networking, and real-time system design.

---

## Architecture

The architecture includes:
- **PIR Sensor**: Detects motion via infrared radiation changes.
- **Raspberry Pi Pico W**: Processes the sensor input and handles Wi-Fi.
- **Rust firmware**: Handles networking, event logic, and notifications.
- **Cloud integration**: Sends HTTP alerts to a webhook or service like IFTTT.

### System Flow:
1. System powers on, connects to Wi-Fi.
2. PIR sensor detects motion.
3. Wi-Fi module triggers an HTTP POST request.
4. Alert is sent to predefined server or mobile notification.

---

## Block Scheme

![Motion Alarm Diagram](/img/motion_alarm_diagram.webp)
![Hardware Schematic](/img/connection_diagram.svg)

---

### 📹 Demo Video

<video controls width="480">
  <source src="./demo.webm" type="video/webm">
  Your browser does not support the video tag.
</video>

## Log

### Week 5 – 11 May
- Designed schematic
- Implemented basic motion detection in Rust

### Week 12 – 18 May
- Added Wi-Fi communication using `esp-wifi`
- Integrated HTTP request with motion event

### Week 19 – 25 May
- Added visual and audio feedback
- Finalized testing and documentation

---

## Hardware

1. **Raspberry Pi Pico W** – Microcontroller with built-in Wi-Fi  
2. **PIR Motion Sensor** – For infrared-based motion detection  
3. **LED and Buzzer** – For real-time user feedback  
4. **Breadboard and Wires** – For prototyping  
5. **5V Power Supply / Battery Pack**

### Schematics

_To be added after wiring is finalized._

---

## Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| Raspberry Pi Pico W | Main MCU | ~35 RON |
| PIR Motion Sensor | Movement detection | ~15 RON |
| LED + Buzzer | Local feedback | ~5 RON |
| Breadboard + Cables | Prototyping | ~20 RON |
| Power Supply | Energy source | ~20 RON |

**Total**: ~95 RON

---

## Software

| Library | Description | Usage |
|--------|-------------|--------|
| `esp-wifi` | Wi-Fi driver | Connects Pico W to local network |
| `ureq` | HTTP client | Sends alerts to webhook |
| `embedded-hal` | HAL Traits | Interfaces with GPIO |
| `defmt` + `defmt-rtt` | Debugging | Lightweight logging |
| `heapless` | No-alloc structures | Buffers and queues |

---

## Links

1. [esp-wifi GitHub](https://github.com/esp-rs/esp-wifi)
2. [IFTTT Webhooks](https://ifttt.com/maker_webhooks)
3. [PIR sensor Arduino tutorial](https://randomnerdtutorials.com/arduino-pir-motion-sensor/)

