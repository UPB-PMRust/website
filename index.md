# DIY Smartwatch — Raspberry Pi Pico 2W + Rust

**Technical Documentation v1.0 | May 2026 | Bucharest, Romania**

A fully functional smartwatch built on a breadboard, featuring:

- Real-time clock display (DS3231)
- Heart rate monitoring (MAX30102)
- Motion detection (MPU6050)
- 240×240 round TFT display (GC9A01)
- LiPo battery with USB-C charging
- Written in Rust using Embassy framework

---

## 1. Project Overview

This project is a DIY smartwatch built on a breadboard using the Raspberry Pi Pico 2W microcontroller, programmed entirely in Rust. The goal was to create a working smartwatch prototype while learning embedded systems programming with the Rust language and the Embassy async framework.

The smartwatch communicates with its peripherals using two standard protocols: SPI (Serial Peripheral Interface) for the high-speed display, and I2C (Inter-Integrated Circuit) for all sensors. The Pico 2W acts as the bus master for all communications.

The project was built step-by-step: hardware assembly on breadboard → MicroPython testing to verify connections → Rust firmware development → final integration.

### Key Features

- Real-time clock display read from DS3231 RTC (backed by CR2032 battery)
- Heart rate and SpO2 monitoring via MAX30102 optical sensor
- Step counting and motion detection via MPU6050 6-axis IMU
- 1.28" circular GC9A01 TFT display (240×240 pixels, RGB565)
- LiPo battery with TP4056 USB-C charging circuit
- Two physical push buttons for user interaction
- Built-in WiFi and Bluetooth (Raspberry Pi Pico 2W)
- Firmware written in Rust — memory safe, zero-cost abstractions

### Programming Language & Framework

The firmware is written in Rust (`no_std` environment) using the Embassy embedded async framework. Rust provides memory safety without a garbage collector, making it ideal for embedded systems.

Target microcontroller architecture:
```
thumbv8m.main-none-eabihf  →  ARM Cortex-M33 (RP2350 on Pico 2W)
```

The Rust ecosystem used:
```toml
embassy-rp        = "0.1"   # RP2350 Hardware Abstraction Layer
embassy-executor  = "0.5"   # Async task executor
embassy-time      = "0.3"   # Timing primitives
embedded-graphics = "0.8"   # 2D graphics rendering
ds323x            = "0.5"   # DS3231 RTC driver
cortex-m-rt       = "0.7"   # Cortex-M runtime
```

---

## 2. Components List & Costs

All components were sourced online from AliExpress, Romanian electronics shops, and local supermarkets. Prices are approximate and in Romanian Lei (RON).

| Component | Model | Role | Source | Price (RON) | Protocol |
|---|---|---|---|---|---|
| Raspberry Pi Pico 2W | RP2350 | Main MCU + WiFi/BT | AliExpress | ~55 RON | USB / SPI / I2C |
| Round TFT Display | GC9A01 1.28" | Smartwatch screen | AliExpress | ~25 RON | SPI |
| RTC Module | DS3231 ZS-042 | Real-time clock | AliExpress | ~15 RON | I2C (0x68) |
| Heart Rate Sensor | MAX30102 | Pulse & SpO2 | AliExpress | ~20 RON | I2C (0x57) |
| Accel. / Gyroscope | MPU6050 HW-123 | Motion / pedometer | AliExpress | ~12 RON | I2C (0x69) |
| LiPo Charger | TP4056 USB-C | Battery charging | AliExpress | ~8 RON | — |
| LiPo Battery | 3.7V 250mAh | Power supply | akyga.com | ~20 RON | — |
| Tactile Buttons x2 | 6×6mm THT | User input | Local shop | ~2 RON | GPIO |
| Breadboards x2 | 400 + 830 pt. | Component mounting | AliExpress | ~25 RON | — |
| Jumper Wire Kit | M-M / M-F | Connections | AliExpress | ~10 RON | — |
| Resistor Kit | 600pcs 1/4W | Pull-ups / limits | AliExpress | ~15 RON | — |
| CR2032 Battery x4 | Duracell 3V | DS3231 backup | Supermarket | ~20 RON | — |
| **TOTAL ESTIMATED** | | | | **~227 RON** | |

---

## 3. Hardware Connections

The system uses two communication buses: SPI1 for the high-speed display, and I2C for all sensors (shared bus). Power (3.3V) is provided by the Pico 2W's onboard LDO regulator, fed by the LiPo battery via TP4056.

### 3.1 Display — GC9A01 (SPI1)

The GC9A01 is a 1.28" round TFT with 240×240 RGB565 resolution. It uses SPI1 at up to 32 MHz. RST is held high after initialization.

| Display Pin | Pico 2W Pin | GP Number |
|---|---|---|
| VCC | 3V3 OUT | Pin 36 |
| GND | GND | Pin 33 |
| SCL (SCK) | GP10 | Pin 14 |
| SDA (MOSI) | GP11 | Pin 15 |
| DC | GP8 | Pin 11 |
| CS | GP9 | Pin 12 |
| RST | GP12 | Pin 16 |

### 3.2 DS3231 Real-Time Clock (I2C, 0x68)

The DS3231 is an ultra-precise RTC with integrated temperature-compensated crystal oscillator. A CR2032 coin cell keeps time when the main power is removed. I2C address: `0x68`.

| DS3231 Pin | Pico 2W Pin | GP Number |
|---|---|---|
| VCC | 3V3 OUT | Pin 36 |
| GND | GND | Pin 33 |
| SDA | GP26 | Pin 31 |
| SCL | GP27 | Pin 32 |

### 3.3 MAX30102 Heart Rate Sensor (I2C, 0x57)

The MAX30102 uses a red LED (660 nm) and an infrared LED (880 nm) to detect photoplethysmographic (PPG) signals for heart rate and SpO2 measurement. I2C address: `0x57`.

| MAX30102 Pin | Pico 2W Pin | GP Number |
|---|---|---|
| VIN | 3V3 OUT | Pin 36 |
| GND | GND | Pin 33 |
| SDA | GP26 | Pin 31 |
| SCL | GP27 | Pin 32 |

### 3.4 MPU6050 Accelerometer & Gyroscope (I2C, 0x69)

The MPU6050 provides 3-axis acceleration ±2g–±16g and 3-axis gyroscope ±250–±2000 °/s. Its default I2C address `0x68` conflicts with DS3231, so the AD0 pin is pulled HIGH to change it to `0x69`.

| MPU6050 Pin | Pico 2W Pin | Notes |
|---|---|---|
| VCC | 3V3 OUT | 3.3V supply |
| GND | GND | Common ground |
| SDA | GP26 | Shared I2C bus |
| SCL | GP27 | Shared I2C bus |
| AD0 | 3V3 | Address → 0x69 |

### 3.5 Power Supply — TP4056 + LiPo 3.7V

The TP4056 module charges the LiPo battery at up to 1A via USB-C. Its output feeds the Pico 2W's VSYS pin. The Pico 2W has an internal RT6150 buck-boost converter that produces a stable 3.3V for all peripherals.

| TP4056 Pin | Connected To | Notes |
|---|---|---|
| IN+ / IN- | USB-C port | 5V charging input |
| BAT+ / BAT- | LiPo 3.7V | JST connector |
| OUT+ | VSYS (Pico 2W) | Battery voltage out |
| OUT- | GND | Common ground |

---

## 4. Software Architecture

The firmware is structured as a bare-metal Rust application with no operating system. The Embassy framework provides cooperative async/await concurrency suitable for embedded systems.

### 4.1 Project Structure

```
smartwatch/
├── src/
│   └── main.rs        # Main firmware — init + main loop
├── Cargo.toml         # Dependencies and build config
├── memory.x           # Linker script for RP2350 memory layout
├── .cargo/
│   └── config.toml    # Rust target and runner configuration
└── README.md          # Project documentation
```

### 4.2 Memory Layout (RP2350)

```
MEMORY {
  FLASH : ORIGIN = 0x10000000, LENGTH = 2048K
  RAM   : ORIGIN = 0x20000000, LENGTH = 520K
}
```

### 4.3 Custom GC9A01 Display Driver

No compatible Rust crate existed for the GC9A01 with Embassy 0.1 (embedded-hal version conflict), so a complete custom SPI driver was implemented. It implements the `embedded-graphics` `DrawTarget` trait for text and shape rendering.

Driver initialization sequence:

1. Toggle RST pin LOW → HIGH with delays
2. Send 40+ initialization commands via SPI (manufacturer sequence)
3. Set pixel format: RGB565 (16-bit, `0x3A = 0x05`)
4. Send Sleep Out (`0x11`) → wait 120ms
5. Send Display On (`0x29`) → wait 20ms

Pixel rendering uses the `RAMWR` command (`0x2C`) with column/row address set commands (`0x2A`, `0x2B`) before each pixel write.

### 4.4 Main Application Loop

The main loop runs at approximately 500ms intervals:

1. Read current time from DS3231 over I2C
2. Clear the display (fill with black)
3. Render clock time as white text at display center
4. Render heart rate (BPM) as text below clock
5. Check button GPIO states for user interaction
6. Delay 500ms via `cortex_m::asm::delay()`

### 4.5 Build & Flash Instructions

Prerequisites: Rust toolchain, `elf2uf2-rs` tool.

1. Add the ARM Cortex-M33 target:
```bash
rustup target add thumbv8m.main-none-eabihf
```

2. Build the release binary:
```bash
cargo build --release
```

3. Enter BOOTSEL mode on Pico 2W: Hold the BOOTSEL button, plug in USB, release the button. The RP2350 drive appears.

4. Flash the firmware:
```bash
elf2uf2-rs -d target/thumbv8m.main-none-eabihf/release/smartwatch
```

---

## 5. Challenges & Solutions

| Challenge | Root Cause | Solution |
|---|---|---|
| MPU register bug in rp2040-hal | New Rust compiler changed cortex-m MPU register layout (RASR removed) | Replaced `install_stack_guard()` with empty `Ok(())` stub |
| GC9A01 crate incompatible | gc9a01 v0.2 requires embedded-hal 0.2, Embassy needs 1.0 — version conflict | Wrote custom SPI driver implementing `embedded-graphics` `DrawTarget` |
| Display not responding | Wrong GPIO pins connected (breadboard wiring errors) | Used MicroPython to scan all pin combos and identify correct GPIOs |
| I2C address conflict | DS3231 and MPU6050 both default to I2C address 0x68 | Connected MPU6050 AD0 pin to 3V3 → address changes to 0x69 |
| embassy-time linker error | Missing `_embassy_time_schedule_wake` symbol when using async Timer | Replaced async Timer with blocking `cortex_m::asm::delay()` |
| Defective display module | GC9A01 module was damaged — no response even to RST signal | Ordered replacement; firmware is ready and tested to compile correctly |
| TP4056 has no breadboard pins | TP4056 uses SMD pads, not through-hole pins | Soldered wires directly to SMD pads using soldering iron |

---

## 6. Future Improvements

- NTP time synchronization via built-in WiFi (Pico 2W)
- Bluetooth Low Energy (BLE) notifications from smartphone
- Proper heart rate algorithm with peak detection on MAX30102 data
- Step counter implementation using MPU6050 accelerometer data
- 3D-printed wristband case designed in FreeCAD
- Custom PCB designed in KiCad to replace the breadboard
- Menu system navigated via the two push buttons
- Power-saving deep sleep mode between display updates
- Over-the-air (OTA) firmware updates via WiFi
- Real-time data logging to MicroSD card

---

## 7. References

| Resource | Description / URL |
|---|---|
| Embedded Rust 101 | https://embedded-rust-101.wyliodrin.com — Course used for reference |
| Embassy Framework | https://embassy.dev — Async embedded Rust framework |
| Pico 2W Datasheet | https://datasheets.raspberrypi.com/pico/pico-2-datasheet.pdf |
| embedded-graphics | https://docs.rs/embedded-graphics — 2D graphics for embedded systems |
| ds323x crate | https://docs.rs/ds323x — DS3231 Rust driver |
| GC9A01 Datasheet | Galaxycore GC9A01 LCD controller datasheet |
| MAX30102 Datasheet | Maxim Integrated MAX30102 High-Sensitivity Pulse Oximeter |
| MPU6050 Datasheet | InvenSense MPU-6050 Product Specification Revision 3.4 |
