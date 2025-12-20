# RustyDucky

USB HID Attack Device with WiFi Exfiltration

:::info

**Author**: Duica Vlad \
**GitHub Project Link**: https://github.com/DuicaVlad/rustyducky

:::

## Description

RustyDucky is a USB-based penetration testing tool built with Rust on the Raspberry Pi Pico 2 W. The device emulates a USB keyboard to automatically execute commands on target systems and exfiltrates captured data via WiFi.

## Why RustyDucky?

This project combines cybersecurity, embedded systems, and Rust programming. It applies course knowledge including GPIO control, asynchronous programming, USB protocols, and wireless communication.
### Hardware Design

The Raspberry Pi Pico 2 W integrates:
- RP2350A dual-core Cortex-M33 (150MHz)
- 520KB SRAM, 4MB Flash
- USB HID support
- CYW43439 WiFi (2.4GHz)

## Software

### Technologies

- Rust (embassy-rp framework)
- usbd-hid (USB keyboard)
- embassy-net (WiFi stack)
### Architecture

Attack workflow:
1. Device powers on via USB
2. Emulates USB keyboard
3. Executes payload commands
4. Exfiltrates data via WiFi
