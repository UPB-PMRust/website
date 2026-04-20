# Wireless Game Controller

A custom wireless game controller inspired by Xbox/PlayStation designs, built on STM32 Nucleo-U545RE-Q and ESP32-C3, communicating via Bluetooth.

:::info

**Author**: Țopa Mihai-Sebastian \
**Github Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-topa-mihai-sebastian

:::

---

## Description

This project is a **wireless game controller** designed to replicate the experience of a commercial gamepad (Xbox / PlayStation style). The device communicates with a host computer via **Bluetooth Low Energy (BLE) 5**, presenting itself as a standard **HID (Human Interface Device)** — no drivers needed on any modern OS.

The controller features:
- **2 analog joysticks** (left/right), each with a clickable button (L3/R3)
- **2 analog triggers** (L2/R2) implemented via a dual-channel slide potentiometer
- **4 face buttons** (A, B, X, Y)
- **D-Pad** (4 directional buttons)
- **Shoulder buttons** (L1, R1)
- **System buttons** (Start, Select, Home)
- **3× AA battery power supply** (4.5V → 3.3V via LDO)

The system is split between two microcontrollers:
- The **STM32 Nucleo-U545RE-Q** handles all input reading (ADC for joysticks/triggers, GPIO for buttons) and runs the main game logic in **Rust** using the **Embassy** async framework.
- The **ESP32-C3 SuperMini** acts exclusively as a **Bluetooth LE 5 co-processor**, receiving controller state from the STM32 via UART and advertising it to the host as a HID gamepad.

---

## Motivation

As an avid gamer, I've always been curious about what's inside a game controller. This project is an opportunity to understand the full stack of an embedded HID device — from analog signal acquisition to wireless protocol implementation — while building something genuinely useful and fun. The challenge of splitting responsibilities between two microcontrollers (one for logic, one for wireless) also mirrors real-world embedded system design patterns.

---

## Architecture

The system is organized into three main layers:

**Input Layer** — The player's physical interactions (thumbstick movement, button presses, trigger pulls) are captured by analog and digital peripherals connected to the STM32.

**Processing Layer** — The STM32 reads all inputs, applies dead-zone filtering and scaling, packages the state into a structured report, and sends it over UART to the ESP32-C3.

**Communication Layer** — The ESP32-C3 receives the HID report and re-transmits it to the host computer via Bluetooth LE 5, presenting itself as a standard gamepad recognized natively by any modern operating system.

```
┌─────────────────────────────────────────────────────────────┐
│                        INPUT LAYER                          │
│  [Joystick L] [Joystick R] [Triggers L2/R2] [Buttons x18]  │
└───────────────────────┬─────────────────────────────────────┘
                        │ ADC / GPIO
                        ▼
┌─────────────────────────────────────────────────────────────┐
│         PROCESSING LAYER (STM32 Nucleo-U545RE-Q)            │
│  - Reads ADC channels (4 joystick axes + 2 triggers)        │
│  - Reads 18 GPIO pins (buttons + D-Pad)                     │
│  - Applies dead-zone filtering                              │
│  - Packs HID report struct (8 bytes)                        │
│  - Sends report over UART TX                                │
└───────────────────────┬─────────────────────────────────────┘
                        │ UART (115200 baud)
                        ▼
┌─────────────────────────────────────────────────────────────┐
│           COMMUNICATION LAYER (ESP32-C3 SuperMini)          │
│  - Receives HID report over UART RX                         │
│  - Advertises as BLE 5 HID Gamepad (HOGP profile)           │
│  - Sends notifications to host on state change              │
│  - Pairs once, auto-reconnects on power-on (bonding NVS)    │
└───────────────────────┬─────────────────────────────────────┘
                        │ Bluetooth LE 5 (BLE HID / HOGP)
                        ▼
                  [ Host: Laptop / PC ]
```

---

## Log

### Week 1
- Defined the architecture and component list.
- Researched BLE HID profile (HOGP) and HID report descriptors for gamepads.
- Set up Embassy project for STM32U545 and esp-idf project for ESP32-C3.

### Week 2
- Wired joysticks and verified ADC readings on all 4 axes (Morpho connectors CN7/CN10).
- Implemented dead-zone filtering and 12-bit → 8-bit axis scaling in Rust.
- Wired all tactile buttons with internal pull-ups; verified GPIO interrupt debouncing.

### Week 3
- Implemented UART communication between STM32 (TX) and ESP32-C3 (RX).
- Implemented BLE HID GATT server on ESP32-C3 using `esp-idf-svc`.
- Tested pairing with laptop — controller recognized as a gamepad with no extra drivers.

### Week 4
- Wired L2/R2 dual-channel slide potentiometer and validated analog trigger response.
- Wired 3× AA battery holder and LDO regulator; validated power path (4.5V → 3.3V).
- Full integration test: all inputs reflected correctly on host OS gamepad tester.
- Tuned dead-zones and trigger curves; finalized HID report descriptor.

---

## Hardware

### Main Components

#### 1. STM32 Nucleo-U545RE-Q
Main microcontroller board. Runs the Rust/Embassy firmware, reads all analog and digital inputs, drives UART communication with the ESP32-C3.

- 6 ADC channels: Joystick L (X, Y), Joystick R (X, Y), Trigger L, Trigger R
- 18 GPIO inputs: A, B, X, Y, L1, R1, Start, Select, Home, D-Pad (4), L3, R3, + 2 spare
- 1 UART TX to ESP32-C3 SuperMini
- Pins accessed via ST Morpho connectors CN7 + CN10 (≈42 pini liberi, nevoie de 26)

#### 2. ESP32-C3 SuperMini
Bluetooth LE 5 co-processor. Receives HID reports from the STM32 via UART and advertises them to the host as a standard gamepad — natively recognized by Windows, Linux, and macOS without any additional drivers.

- BLE 5 HID over GATT Profile (HOGP), Appearance = Gamepad (0x03C4)
- Firmware: Rust + `esp-idf-svc` + `esp-idf-hal`
- Bonding stored in NVS flash — auto-reconnects on power-on after first pairing

#### 3. Analog Joysticks — PS2 Breakout Module (×2)
Standard XY joystick modules with integrated push-button (L3/R3). Each module: VCC, GND, VRx, VRy, SW.

- VRx/VRy → 2 ADC channels on STM32
- SW → GPIO with internal pull-up

#### 4. Tactile Buttons — 12×12mm with round cap (×20)
A, B, X, Y, L1, R1, Start, Select, Home, D-Pad ×4, L3, R3, + 2 spare.

- Wired between GPIO and GND, internal pull-ups enabled in software
- 5 ms software debounce filter

#### 5. Analog Triggers — Dual-Channel Slide Potentiometer B10K (×1)
One dual-channel module covers both L2 and R2.

- Wiper pins → 2 ADC channels on STM32
- 0–3.3V range mapped to 0–255 axis value per trigger

#### 6. Power Supply — 3× AA Battery + AMS1117-3.3V LDO

3× AA alkaline (4.5V) feeds into the AMS1117-3.3 LDO regulator → stable 3.3V rail for STM32 and ESP32-C3.

- No charging circuit needed — simpler and safer for a breadboard prototype
- Runtime: ~8–10 hours with alkaline AA
- Batteries available at any supermarket or electronics store
- Upgrade path: swap battery holder for Li-Po 3.7V + TP4056 module — rest of circuit unchanged

**Power path:**
```
3× AA (4.5V) → AMS1117-3.3 LDO → 3.3V Rail → STM32 Nucleo + ESP32-C3 SuperMini
```

**UART connection (3.3V logic, no level shifter needed):**
```
STM32 TX (Morpho CN10) → ESP32-C3 RX
STM32 GND (Morpho CN7) → ESP32-C3 GND
3.3V  (Morpho CN7)     → ESP32-C3 3V3
```

---

### Schematics

*(Add KiCad or Fritzing schematic here)*

---

### Bill of Materials

| Device | Usage | Preț |
|--------|-------|------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main microcontroller | ~110 RON |
| ESP32-C3 SuperMini — [sigmanortec.ro](https://sigmanortec.ro/en/esp32-c3-supermini-development-board-33v-wifi-bluetooth) / [emag.ro](https://www.emag.ro/microcontroller-esp32-c3-super-mini-wifi-bt-5-0-usb-c-1-2-29/pd/D5C34SYBM/) | Bluetooth LE 5 co-processor — HID gamepad | [~30 RON](https://sigmanortec.ro/en/esp32-c3-supermini-development-board-33v-wifi-bluetooth) |
| [PS2 Joystick Breakout ×2](https://www.optimusdigital.ro/en/touch-sensors/742-ps2-joystick-breakout.html) | Joystick stâng + drept cu L3/R3 | [~10 RON × 2](https://www.optimusdigital.ro/en/touch-sensors/742-ps2-joystick-breakout.html) |
| [Butoane tactile 12×12mm cu capac ×20](https://www.optimusdigital.ro/ro/207-butoane-i-comutatoare) | A, B, X, Y, D-Pad, L1, R1, Start, Select, Home | [~0.50 RON × 20](https://www.optimusdigital.ro/ro/207-butoane-i-comutatoare) |
| [Potențiometru Liniar Dual-Canal B10K 75mm](https://www.optimusdigital.ro/ro/componente-electronice-potentiometre/3914-poteniometru-liniar-cu-doua-canale-de-75-mm-b10k.html) | Triggere analogice L2/R2 (un singur modul) | [~15 RON](https://www.optimusdigital.ro/ro/componente-electronice-potentiometre/3914-poteniometru-liniar-cu-doua-canale-de-75-mm-b10k.html) |
| [Suport baterii 3× AA](https://www.emag.ro/suport-baterii-aa-pentru-3buc-cu-contacte-pentru-lipire-gni0055/pd/D62S5JYBM/) | Alimentare 4.5V | [~5 RON](https://www.emag.ro/suport-baterii-aa-pentru-3buc-cu-contacte-pentru-lipire-gni0055/pd/D62S5JYBM/) |
| Baterii AA alkaline ×3 | Putere efectivă | ~10 RON (orice magazin) |
| [Modul LDO AMS1117-3.3V](https://www.optimusdigital.ro/en/others/2219-mini-ams1117-33-33v-voltage-regulator-module.html) | Regulator 3.3V | [~5 RON](https://www.optimusdigital.ro/en/others/2219-mini-ams1117-33-33v-voltage-regulator-module.html) |
| [Breadboard 830p ×2](https://sigmanortec.ro/en/breadboard-830-points-mb-102) | Bază prototipare | [~11 RON × 2](https://sigmanortec.ro/en/breadboard-830-points-mb-102) |
| [Cabluri Dupont M-M 65 buc](https://www.optimusdigital.ro/en/wires-with-connectors/12-breadboard-jumper-wire-set-65-pcs.html) | Conexiuni breadboard | [~8 RON](https://www.optimusdigital.ro/en/wires-with-connectors/12-breadboard-jumper-wire-set-65-pcs.html) |
| [Cabluri Dupont M-F 40 buc](https://www.optimusdigital.ro/en/wires-with-connectors/214-colored-40p-254-mm-pitch-male-to-female-wire.html) | Conexiuni module joystick | [~6 RON](https://www.optimusdigital.ro/en/wires-with-connectors/214-colored-40p-254-mm-pitch-male-to-female-wire.html) |
| Condensatoare ceramice 100nF ×10 | Decuplare VCC / ADC | ~3 RON |
| Condensatoare electrolitice 10µF ×5 | Filtrare șină 3.3V | ~3 RON |
| Set rezistențe 10kΩ / 1kΩ | Pull-up / protecție UART | ~5 RON |
| | | **Total ~232 RON** |

---

## Software

### STM32 Firmware (Rust + Embassy)

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy/tree/main/embassy-stm32) | HAL for STM32U5 | ADC reads (joysticks, triggers), GPIO (buttons), UART TX |
| [embassy-executor](https://github.com/embassy-rs/embassy/tree/main/embassy-executor) | Async task executor | Runs ADC task, button polling task, and UART send task concurrently |
| [embassy-time](https://github.com/embassy-rs/embassy/tree/main/embassy-time) | Time management | Button debounce timers, polling intervals |
| [embassy-sync](https://github.com/embassy-rs/embassy/tree/main/embassy-sync) | Async primitives | Mutex/Channel for sharing controller state between tasks |
| [defmt](https://github.com/knurling-rs/defmt) | Debug logging | RTT-based logging over STLINK-V3EC probe |
| [panic-probe](https://github.com/knurling-rs/probe-run) | Error handling | Crash reporting via integrated probe |

**Key Implementation Details (STM32 side):**

- **ADC Scanning**: Embassy's `AdcChannel` scans all 6 analog inputs in sequence. Readings are 12-bit (0–4095), scaled to i8 (−128 to +127) for joysticks and u8 (0–255) for triggers.
- **Dead Zone Filtering**: ±8% dead zone applied to each joystick axis before packaging the report.
- **Button Debouncing**: Each GPIO polled every 5ms; state change registered only after 2 stable consecutive polls.
- **HID Report Struct**: Packed 8-byte struct sent over UART on every state change.

```rust
#[repr(C, packed)]
struct HidReport {
    lx: i8, ly: i8,        // Left joystick
    rx: i8, ry: i8,        // Right joystick
    lt: u8, rt: u8,        // Triggers
    buttons_lo: u8,        // Buttons 0–7
    buttons_hi: u8,        // Buttons 8–15 (D-Pad + extras)
}
```

### ESP32-C3 Firmware (Rust + esp-idf-svc)

| Library | Description | Usage |
|---------|-------------|-------|
| [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc) | ESP-IDF Rust wrapper | BLE GATT server, HID over GATT Profile (HOGP) |
| [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) | Hardware abstraction | UART RX from STM32 |
| [embedded-svc](https://github.com/esp-rs/embedded-svc) | Embedded service traits | Abstractions for BLE traits |

**Key Implementation Details (ESP32-C3 side):**

- **BLE HID Profile (HOGP)**: GATT server with HID Service (UUID 0x1812). Report Descriptor describes 6 axes + 18 buttons — recognized natively as a gamepad by any modern OS.
- **UART Reception**: Listens for 8-byte HID reports; sends BLE notification only on state change to reduce radio traffic.
- **Pairing & Bonding**: Keys stored in NVS flash — host pairs once, auto-reconnects afterwards.
- **Advertising**: Device advertises as `"RustPad"` with Appearance = Gamepad (0x03C4).

---

## Links

1. [Embassy Async Embedded Framework (Rust)](https://embassy.dev/)
2. [esp-rs — Rust on ESP32](https://github.com/esp-rs)
3. [BLE HID over GATT Profile Specification](https://www.bluetooth.com/specifications/specs/hid-over-gatt-profile-1-0/)
4. [USB HID Usage Tables (Gamepad)](https://usb.org/sites/default/files/hut1_21.pdf)
5. [STM32 Nucleo-U545RE-Q User Manual (UM3062)](https://www.st.com/resource/en/user_manual/um3062-stm32u3u5-nucleo64-board-mb1841-stmicroelectronics.pdf)
6. [ESP32-C3 SuperMini Pinout](https://www.espboards.dev/esp32/esp32-c3-super-mini/)
7. [KiCad EDA — Free PCB Design](https://www.kicad.org/)