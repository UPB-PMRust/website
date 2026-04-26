---
layout: section
description: WiFi-Controlled RGB LED System using Raspberry Pi Pico W and Embassy Rust
---

# WiFi-Controlled RGB LED System

:::info

**Author:** Yosri BELHEDI  
**GitHub Project Link:** https://github.com/UPB-PMRust-Students/yosri.belhedi

:::

## Description

A Raspberry Pi Pico W-based system that hosts a local HTTP server, allowing users to control an RGB LED from any browser on the same network — no app, no cloud, just the chip itself serving a web interface. The user can set any color, choose lighting modes (static, breathe, rainbow, auto-dim, party strobe), and adjust brightness through a simple web page.

The system uses the **Embassy** async Rust framework with the `picoserve` crate for the HTTP server, and two async tasks communicating through a shared `Mutex<LedState>`: one task handles incoming HTTP requests and updates the state, while the other drives the PWM outputs for the RGB LED.

## Motivation

I wanted to explore what embedded Rust and WiFi on a microcontroller actually feels like end-to-end — from hardware PWM all the way up to serving HTTP responses. A WiFi-controlled RGB LED is a perfect scope: it's small enough to finish, but touches real async concurrency, ADC reading, PWM control, and TCP networking in one project. It also results in something genuinely usable and visually satisfying.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  Raspberry Pi Pico W                │
│                                                     │
│  ┌─────────────┐        ┌──────────────────────┐   │
│  │  WiFi Task  │◄──────►│  Shared LedState     │   │
│  │ (picoserve) │ Mutex  │  (color, mode,       │   │
│  │ HTTP Server │        │   brightness)        │   │
│  └─────────────┘        └──────────┬───────────┘   │
│                                    │               │
│  ┌─────────────┐                   │               │
│  │  ADC Task   │──────────────────►│               │
│  │ (photores.) │  auto-dim update  │               │
│  └─────────────┘        ┌──────────▼───────────┐   │
│                          │   LED Task           │   │
│                          │ PWM R / G / B pins   │   │
│                          └──────────────────────┘   │
└─────────────────────────────────────────────────────┘
         │
         │ WiFi (local network)
         ▼
  ┌─────────────────────┐
  │  Browser (phone /   │
  │  laptop) — web UI   │
  │  color picker,      │
  │  mode buttons,      │
  │  brightness slider  │
  └─────────────────────┘
```

**Key components:**

- **WiFi Task** — connects to the home AP at boot, obtains an IP address, and runs the `picoserve` HTTP server. Handles `POST /color`, `POST /mode`, `POST /brightness` REST endpoints. Writes into the shared `Mutex<LedState>`.
- **LED Task** — reads `LedState` every ~16 ms and updates PWM duty cycles on the three RGB pins. Implements all animation modes internally (breathe, rainbow HSV cycle, strobe).
- **ADC Task** (optional) — reads the photoresistor every second and writes an ambient-light value into `LedState` for the auto-dim mode.
- **Shared State** — a `embassy_sync::Mutex<LedState>` wrapping color (R,G,B), active mode, and brightness. No message queues needed; the LED task polls frequently enough.


## Hardware

| # | Component | Quantity | Purpose |
|---|-----------|----------|---------|
| 1 | Raspberry Pi Pico W | 1 | Main microcontroller + WiFi |
| 2 | Common-cathode RGB LED | 1 | Output indicator |
| 3 | 100 Ω resistor | 3 | Current limiting for R/G/B channels |
| 4 | Photoresistor (LDR) | 1 | Ambient light sensing (auto-dim) |
| 5 | 10 kΩ resistor | 1 | Pull-down for photoresistor divider |
| 6 | Tactile push button | 1 | Optional physical mode switch |
| 7 | 10 kΩ resistor | 1 | Pull-down for button |
| 8 | Breadboard | 1 | Prototyping |
| 9 | Jumper wires | ~15 | Connections |

**Wiring summary:**

- RGB LED: R → GP6, G → GP7, B → GP8 (all PWM-capable pins), common cathode → GND
- Photoresistor: one leg to 3.3 V, other leg to GP26 (ADC0) and through 10 kΩ to GND
- Button (optional): GP15 with 10 kΩ pull-down to GND

## Software

The project is written entirely in **Rust** using the **Embassy** async embedded framework.

```
src/
├── main.rs          — spawns all Embassy async tasks
├── wifi.rs          — connects to AP, runs picoserve HTTP server
├── led.rs           — PWM driver + animation engine (breathe, rainbow, strobe)
├── state.rs         — Mutex<LedState> shared between tasks
├── http_handler.rs  — parses REST requests, updates LedState
└── adc.rs           — photoresistor reading (auto-dim mode)
```

**Key crates:**

| Crate | Role |
|-------|------|
| `embassy-rp` | RP2040 HAL — PWM, ADC, GPIO |
| `embassy-executor` | Async task executor |
| `embassy-sync` | `Mutex` for shared state |
| `embassy-net` | Async TCP/IP networking |
| `cyw43` | CYW43439 WiFi driver (built into Pico W) |
| `picoserve` | Lightweight async HTTP server for embedded |
| `embassy-time` | Timers and delays |

**Lighting modes:**

| Mode | Behavior |
|------|----------|
| Static | Fixed RGB color set via color picker |
| Breathe | Sinusoidal fade in/out at adjustable speed |
| Rainbow | Cycles through HSV hue wheel (~360 steps) |
| Auto-dim | Reads LDR; dims in brightly lit environments |
| Party strobe | Fast random color flashes |

## Links

1. [Embassy Rust framework](https://embassy.dev)
2. [picoserve crate](https://crates.io/crates/picoserve)
3. [Raspberry Pi Pico W datasheet](https://datasheets.raspberrypi.com/picow/pico-w-datasheet.pdf)
4. [Embassy RP2040 PWM example](https://github.com/embassy-rs/embassy/tree/main/examples/rp)
5. [GitHub Repository](https://github.com/UPB-PMRust-Students/yosri.belhedi)
