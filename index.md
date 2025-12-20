# RustyDucky

USB HID Attack Device with WiFi Exfiltration

:::info

**Author**: Duica Vlad \
**GitHub Project Link**: https://github.com/DuicaVlad/rustyducky

:::

## Description

RustyDucky is a USB-based penetration testing tool built with Rust on the Raspberry Pi Pico 2 W. The device emulates a USB keyboard to automatically execute commands on target systems and exfiltrates captured data via WiFi.

### Motivation

This project combines my interests in cybersecurity, embedded systems, and Rust programming. It applies course knowledge including GPIO control, asynchronous programming, USB protocols, and wireless communication in a practical security research context.

## Architecture

### Block Diagram
```
┌─────────────────────────────────┐
│  Raspberry Pi Pico 2 W (RP2350) │
│  ┌───────────────────────────┐  │
│  │ USB HID Keyboard Emulator │◄─┼── Target Computer
│  └───────────────────────────┘  │
│  ┌───────────────────────────┐  │
│  │ Payload Execution Engine  │  │
│  └───────────────────────────┘  │
│  ┌───────────────────────────┐  │
│  │ WiFi Module (CYW43439)    │──┼── Data Exfiltration
│  └───────────────────────────┘  │
└─────────────────────────────────┘
```

### Components

- **USB HID Module**: Implements USB keyboard protocol using usbd-hid crate
- **Payload Engine**: Interprets DuckyScript-style commands
- **WiFi Stack**: Manages wireless connectivity using embassy-net
- **Data Exfiltration**: HTTP client for transmitting captured data

## Log

| Week | Date Range | Work Done |
|------|------------|-----------|
| Week 1 | 17-23 Nov | Project research and documentation setup |
| Week 2 | 24-30 Nov | Hardware selection, ordered Raspberry Pi Pico 2 W |
| Week 3 | 1-7 Dec | Research on USB Rubber Ducky concepts and attack vectors |
| Week 4 | 8-14 Dec | Hardware arrived, studied component specifications and project architecture |
| Week 5 | 15-21 Dec | Development environment setup, initial code structure |
| Week 6 | 22-28 Dec | Component assembly, GitHub repository setup, serial communication testing |

## Hardware

### Schematics

The Raspberry Pi Pico 2 W integrates all required components on a single development board. No external components needed.
```
Raspberry Pi Pico 2 W
├── RP2350A (Dual Cortex-M33, 150MHz)
├── USB Port (Micro USB)
├── CYW43439 WiFi Module
└── 4MB Flash Storage
```

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| Raspberry Pi Pico 2 W | Main microcontroller | 37 lei |
| Micro USB Cable | Programming & deployment | 7 lei |
| **Total** | | **43 lei** |

### Photos

*Photos will be added during hardware assembly phase.*

## Software

### Technologies

- **Rust** (embassy-rp framework)
- **usbd-hid** (USB keyboard emulation)
- **embassy-net** (WiFi networking stack)
- **HTTP client** (data exfiltration)

### Software Architecture
```
main.rs          - Entry point and executor
usb_hid.rs       - USB keyboard implementation
payload.rs       - Script interpreter
wifi.rs          - Network connectivity
exfiltrate.rs    - Data transmission
```

### Workflow

1. Device powers on when plugged into USB port
2. OS enumerates device as USB keyboard
3. Payload engine executes pre-programmed commands
4. WiFi module connects to network
5. Captured data transmitted to remote server

## Results

*Results will be updated as development progresses.*

## Conclusions

RustyDucky demonstrates practical application of embedded Rust for security research. This project is strictly for educational purposes and authorized penetration testing only.

## Links

1. [BadUSB Attack Explanation](https://www.youtube.com/watch?v=8obUvNkZdwc)
2. [Rust Debugging Techniques](https://users.rust-lang.org/t/rubber-ducking-generics-and-traits/106750)
3. [rubber_duck Crate](https://lib.rs/crates/rubber_duck)
4. [USB Rubber Ducky Payloads](https://github.com/hak5/usbrubberducky-payloads)
5. [Raspberry Pi Pico Documentation](https://www.raspberrypi.com/documentation/microcontrollers/)
6. [embassy-rs Book](https://embassy.dev/book/)
