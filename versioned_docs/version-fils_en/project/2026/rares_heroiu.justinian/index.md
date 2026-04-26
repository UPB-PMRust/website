# W.A.S.P. - Wide Area Swarm Protection

## Description

W.A.S.P. (short for Wide Area Swarm Protection) is a drone swarm system designed to function as an anti-personnel and anti-air defense platform. The full system architecture consists of *slave drones* coordinated by a *queen* drone. Due to cost constraints, this project focuses on building the slave drones and emulating the queen's command signals from a stationary controller.

Each slave drone is a small quadcopter built around an ESP32-S3 (Arduino Nano ESP32).  The drones use a UWB module for relative positioning between swarm members, an IMU with onboard sensor fusion for stabilization, and coreless brushed motors for propulsion (since I need to produce PCBs and 3-5 drones, I need to keep the costs down somehow; the full price goes up to 8000 lei; for damage control I need to use coreless although I do realize that BLDC is superior in everyuthing but price; around 15 lei for a coreless compared to 60-70 lei for a BLDC). The internal communication bus is SPI, while inter-drone and queen-to-slave communication uses UWB and Wi-Fi. (I think I will shamelessly be using ESP-NOW if it proves reliable enough in Rust)

## Motivation

I chose this project because it brings together almost every topic covered during the semester into a single, demanding system: real-time embedded control, async task scheduling, multiple SPI peripherals sharing a bus, sensor fusion data pipelines, wireless communication, power management, and custom PCB design. Drones are unforgiving as a platform; a stabilization loop that misses its deadline by a few milliseconds simply falls out of the sky  which makes them an honest test of whether the firmware actually works.

The swarm aspect adds a second layer of difficulty on top of single-drone flight: relative positioning, role coordination, and resilience to individual drone loss. Even with only the slaves built and the queen faked, getting two or more units to hold formation using UWB ranging would be a meaningful result.

## Architecture

The system is split into three logical layers per drone:

1. **Sensing & Communication layer** - IMU (BNO085), UWB module (DWM3000), and Wi-Fi (ESP32-S3 internal). All sensor peripherals share a single SPI bus with chip-select multiplexing.
2. **Control layer** - Cascaded PID controllers (outer loop: position/attitude target, inner loop: rate stabilization) running as Embassy async tasks. Receives setpoints from the queen-emulator over Wi-Fi and relative-position deltas from UWB.
3. **Actuation & Power layer** - Four AO3400A N-MOSFETs driving coreless motors via PWM, fed from a TPS630802 buck-boost converter. The MCP1826S LDO provides a clean rail for the analog/sensor side. A TP4056 module handles LiPo charging.

# AI SLOP GENERATED SCHEMATIC INCOMING

```
                ┌──────────────────────────────────────────┐
                │            Queen (emulated)              │
                │      Wi-Fi broadcast of setpoints        │
                └──────────────────┬───────────────────────┘
                                   │ Wi-Fi
                ┌──────────────────▼───────────────────────┐
                │           ESP32-S3 (Nano ESP32)          │
                │  ┌────────────────────────────────────┐  │
                │  │  Embassy async executor            │  │
                │  │  ┌──────────┐  ┌────────────────┐  │  │
                │  │  │ Comms    │  │ Cascaded PID   │  │  │
                │  │  │ task     │─▶│ (outer+inner)  │  │  │
                │  │  └──────────┘  └───────┬────────┘  │  │
                │  │  ┌──────────┐          │           │  │
                │  │  │ Sensor   │──────────┘           │  │
                │  │  │ fusion   │   setpoints/state    │  │
                │  │  │ task     │                      │  │
                │  │  └────┬─────┘                      │  │
                │  └───────┼──────────────────────────────┘ │
                │          │ SPI bus                        │
                └──────────┼────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   ┌────▼────┐       ┌─────▼─────┐      ┌─────▼─────┐
   │ BNO085  │       │  DWM3000  │      │  Motor    │
   │  IMU    │       │   UWB     │      │  drivers  │
   │ (M0 fus)│       │ (ranging) │      │ (4×AO3400)│
   └─────────┘       └───────────┘      └─────┬─────┘
                                              │ PWM
                                       ┌──────▼──────┐
                                       │ 4× coreless │
                                       │   motors    │
                                       └─────────────┘

   Power: LiPo ──▶ TP4056 (charge) ──▶ TPS630802 (buck-boost)
                                  └──▶ MCP1826S LDO ──▶ sensors
```

Inter-drone coordination happens at the UWB layer: each slave continuously ranges against its neighbors, builds a local relative-position estimate, and feeds that into the outer PID loop as a formation-keeping correction term.

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The drone is built around the **Arduino Nano ESP32**, chosen because it embeds the ESP32-S3 module on a small carrier board - this lets me keep the custom PCB simpler by avoiding the RF design work needed for a bare ESP32-S3 chip, while still getting Wi-Fi and the Xtensa LX7 cores.

For sensing, the **BNO085** IMU is used because it has an onboard ARM Cortex-M0 that runs the sensor fusion algorithm internally and exposes already-filtered orientation data - this offloads a significant chunk of work from the main MCU and shortens the control loop. The IMU is driven by an external crystal oscillator, since the datasheet explicitly notes that running it without one produces unreliable data.

The **DWM3000 UWB module** handles ranging between drones. UWB was chosen over alternatives like RSSI-based localization because of its much higher accuracy (decimeter-level vs. several meters), which is what you need for tight formation flying.

Power is handled by a **TP4056** linear charger (with overcharge/overdischarge protection ICs if budget allows), a **TPS630802** buck-boost converter to maintain a stable rail across the LiPo's discharge curve, and an **MCP1826S** LDO to provide clean power to the analog sensor side.

Propulsion uses **0820 or 1020 coreless brushed motors** (depending on availability) driven by **AO3400A N-channel MOSFETs**. Brushed coreless motors were chosen over BLDC because they don't require ESCs, which dramatically simplifies the schematic and BOM for a micro-form-factor drone.

The chassis is **3D printed**, designed as simple clamps that snap onto the motors. SMD LEDs provide status feedback, and THT LEDs are reserved for the (optional) detonation indication circuitry.

All internal communication between the MCU and peripherals runs over **SPI**.

### Schematics

*KiCad schematics will be added here in SVG format once the PCB design is complete.*

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| [Arduino Nano ESP32](https://store.arduino.cc/products/nano-esp32) | Main MCU (ESP32-S3, Xtensa LX7) | ~120 RON |
| [BNO085 IMU](https://www.adafruit.com/product/4754) | 9-DoF IMU with onboard sensor fusion | ~140 RON |
| [DWM3000 UWB module](https://www.qorvo.com/products/p/DWM3000) | Inter-drone relative positioning | ~90 RON |
| [TPS630802 buck-boost](https://www.ti.com/product/TPS63080) | Maintains stable rail across LiPo discharge | ~15 RON |
| [MCP1826S LDO](https://www.microchip.com/en-us/product/MCP1826S) | Clean rail for sensor analog supply | ~5 RON |
| [TP4056 charging module](https://www.optimusdigital.ro/en/battery-chargers/12-modul-de-incarcare-tp4056.html) | LiPo charging (with optional protection ICs) | ~5 RON |
| AO3400A MOSFET ×4 | Driver transistors for coreless motors | ~2 RON total |
| 0820/1020 coreless motors ×4 | Propulsion | ~40 RON total |
| Crystal oscillator (32.768 kHz) | External clock source for BNO085 | ~3 RON |
| LiPo battery (1S, 300–500 mAh) | Power source | ~25 RON |
| Custom 4-layer PCB | Carrier for all electronics | ~50 RON |
| 3D printed chassis | Mechanical frame and motor clamps | ~5 RON (filament) |
| SMD + THT LEDs, passives | Status indication, support components | ~10 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [esp-hal](https://github.com/esp-rs/esp-hal) | Hardware abstraction layer for ESP32 family | Peripheral access (SPI, GPIO, PWM, timers) on the ESP32-S3 |
| [esp-wifi](https://github.com/esp-rs/esp-hal/tree/main/esp-wifi) | Wi-Fi driver for esp-hal | Receiving setpoint commands from the (emulated) queen |
| [esp-backtrace](https://github.com/esp-rs/esp-backtrace) | Panic handler with backtraces | Debugging during firmware development |
| [embassy](https://github.com/embassy-rs/embassy) | Async executor and HAL for embedded Rust | Concurrent task scheduling for sensor reads, control loop, and comms |
| [bno080](https://crates.io/crates/bno080) | Driver for the BNO080/BNO085 IMU | Reading fused orientation data over SPI |
| [dw3000-ng](https://crates.io/crates/dw3000-ng) | Driver for the DW3000 UWB chip | Two-way ranging with neighbor drones |
| [micromath](https://crates.io/crates/micromath) | `no_std` math primitives (trig, vectors, quaternions) | Attitude math inside the control loop without pulling in `libm` |
| Cascaded PID crate (TBD) | Cascaded PID controller | Outer attitude loop + inner rate loop for stabilization |

# IMPORTANT

THe swarming will be done using simple Boid algorithms; the NATO-cleared version of the project (protected and patented, presented at the EUDIS hackathon, currently in development and fast-tracked for production and use in war theaters soon) uses more advanced swarming algorithms which react in ms time to new information about threats;

I WILL NOT BE SIMULATING THREATS; the result will be a small drone show that will last between 10-30 seconds (which aligns well with the max battery of the drone, lasting somewhere around 5 minutes).

I WILL INCLUDE LICENSING later but I do not agree with the use of any of the materials that I push in any projects without my direct signed agreement and any attempt to do so will be promptly followed by legal action against the person using it.

## Links
