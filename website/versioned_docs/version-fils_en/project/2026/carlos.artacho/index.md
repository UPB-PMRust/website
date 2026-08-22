# SkyRust | Custom Wi-Fi Quadcopter Flight Controller

A from-scratch flight controller for an auto-stabilizing quadcopter, built on the Raspberry Pi Pico 2 W and piloted entirely over a custom Wi-Fi UDP protocol from a PC.

:::info

**Author**: Artacho Carlos
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-carlibar50

:::

## Description

SkyRust is a custom flight controller designed and built from the ground up. Instead of dropping in an off-the-shelf board like a Betaflight F4 or a CC3D, every layer of this project is hand-engineered: the circuit lives on an FR4 perfboard with point-to-point soldering, the firmware is written in `no_std` Rust, and the radio link is replaced entirely by a custom Wi-Fi UDP protocol talking to a PC client.

The headline features are:

* **Wi-Fi UDP Control:** No commercial RC transmitter. Throttle, pitch, roll, and yaw are streamed from a PC client over Wi-Fi using a tiny custom UDP protocol, leveraging the Pico 2 W's onboard CYW43439 chip.
* **Asynchronous PID Auto-Stabilization:** A strict `Ticker`-based PID loop runs on a fixed millisecond schedule, fusing IMU data to keep the airframe level even when the operator stops sending input.
* **Sensor Fusion:** Raw gyroscope and accelerometer data from an MPU6050 (or BMI270) is filtered in software to produce clean pitch/roll/yaw estimates.
* **Hard Software Failsafe:** If a valid UDP packet does not arrive within a 200 ms window, the motor mixer is forced to zero throttle. No packets, no spinning props.
* **Manual Hardware Assembly:** The whole circuit is point-to-point soldered on an FR4 perfboard, with a star-grounding topology and physical noise isolation between the buck converter and the IMU.

The closest commercial analogues are flight controllers running Betaflight or iNav firmware. Those are mature, optimized, and incredibly battle-tested. SkyRust does not aim to compete with them on features. It aims to demonstrate that the *whole stack*, from copper to control loop, can be rebuilt by hand in a memory-safe language, and still actually fly.

## Motivation

This project is also the perfect excuse to push embedded Rust into a hard real-time domain. A quadcopter is unforgiving. If the control loop misses a deadline, the aircraft falls out of the sky. If the network task blocks the CPU, the failsafe never triggers. Building this on top of `embassy` forces me to take async scheduling, priority, and timing constraints seriously, which is exactly the kind of thing this course is about.

## Architecture

The system is split into a flying half (the quadcopter itself) and a ground half (the PC client). They are connected over Wi-Fi.

The Pico 2 W on the airframe acts as a UDP server, an IMU reader, a PID controller, and a PWM motor mixer, all concurrently, thanks to `embassy`'s async executor. The PC client is a small Rust program that reads keyboard or gamepad input and streams control packets at a fixed rate.

* **Core Processing & Networking Layer:** Raspberry Pi Pico 2 W (RP2350 + CYW43439). Hosts the UDP server, runs all flight logic, and drives four PWM channels for the ESCs.
* **Sensing Layer:** MPU6050 over I2C (or BMI270 over SPI as a faster alternative). Provides raw acceleration and angular velocity at hundreds of Hz.
* **Actuation Layer:** Four 30 A BLHeli ESCs driving four 2212 920KV brushless motors. The MCU speaks to them via standard servo-style PWM.
* **Power Layer:** A 3S 11.1 V LiPo feeds the ESCs directly. An MP1584EN buck module steps the pack voltage down to a clean, filtered 5 V rail for the logic side.
* **Control Link:** A PC running a Rust UDP client sends 4-axis control packets to the Pico 2 W over the local Wi-Fi network.

![Architecture Diagram](./diagram.svg)

### Component Interconnection

**Wi-Fi Link (PC ↔ CYW43439 ↔ RP2350):** The CYW43439 is internal to the Pico 2 W. The RP2350 talks to it over the dedicated SPI/PIO interface that the `cyw43-pio` driver expects. From the PC's perspective, the drone is just a UDP endpoint on the LAN.

**IMU Link (RP2350 ↔ MPU6050):** Standard I2C bus (SDA/SCL) with pull-ups. The IMU is physically mounted at the geometric center of the perfboard so its readings are not contaminated by rotational offsets.

**Motor Link (RP2350 ↔ ESCs):** Four PWM channels generate standard 50 Hz ESC pulses. The signal lines are routed away from the buck converter to keep switching noise off the gates.

**Power Link:** The LiPo's main leads go to the four ESCs through a power distribution layer. A separate tap feeds the MP1584EN, which in turn feeds the Pico 2 W's VSYS pin and the IMU.

## Development Log

### Weeks 1–4
Spent these weeks getting comfortable with embedded Rust and the `embassy` async model. Read through the Embassy book, played with the basic Pico 2 W examples (blinky, UART, PWM), and got `probe-rs` flashing reliably over the debug header. Started sketching what a flight controller's task graph would look like in async Rust.

### Week 5
Locked in the project idea and bought time on the 3D printer for prop guards. Researched IMUs and decided to start with the MPU6050 because the I2C interface is dead simple and there are existing Rust drivers I can lean on or replace. Confirmed that the Pico 2 W has enough PWM channels and headroom for four ESCs plus the IMU bus plus Wi-Fi.

### Week 6
Began the first round of hardware planning. Drew up a rough block diagram, listed every signal that needed to cross the perfboard, and counted GPIOs against the Pico 2 W's pinout. Ordered the F450 frame, the four 2212 motors, the ESCs, and the MP1584EN buck modules. The 3S LiPo and charger were already in my parts bin from a previous build.

### Week 7
**Project Theme Milestone.** Submitted the proposal and got it approved. Started prototyping the Wi-Fi UDP server on a bare Pico 2 W, with no peripherals attached yet, just to validate that I could maintain low-latency packet reception under `embassy-net`.

### Week 8
Wrote the first cut of the control packet format. Each frame is a small fixed-size payload carrying four `i16`s (throttle, pitch, roll, yaw) plus a sequence number and a tiny checksum. Built a matching PC-side client in Rust that reads keyboard input. End-to-end, the round trip was well under the 200 ms failsafe window, which is the bar I care about.

### Week 9
**Documentation Milestone.** Wrote this document. Bench-tested the IMU on a separate breadboard and got clean gyro readings via I2C. Started sketching the PID structure and the failsafe state machine, but did not commit code for them yet.

### Weeks 10–11
**Hardware Milestone window.** Plan: assemble the perfboard with star-grounding, mount the MCU, IMU, and buck module, and wire the four ESC signal lines. Smoke-test on bench power before ever touching the LiPo. Photos of the finished perfboard will live in this section.

### Weeks 12–13
**Software Milestone window.** Plan: bring up the full task graph (UDP server, IMU reader, PID loop, motor mixer, failsafe monitor), tune PID gains on a tethered test rig, and validate the failsafe by physically pulling the Wi-Fi router's plug mid-flight on the bench.

### Week 14
**PM Fair.** Live demo of the airframe responding to PC inputs with auto-leveling enabled.

## Hardware

The hardware is intentionally minimal on the logic side and intentionally robust on the power side. The Pico 2 W carries the entire flight control workload; the rest of the board exists to feed it clean power and to fan signals out to the ESCs.

A few non-obvious design decisions worth calling out:

The **IMU is centered** on the perfboard, not tucked into a corner. Mounting it off-axis would inject artificial rotational components into the gyro readings whenever the airframe yaws, which would have to be corrected in software. Putting it at the centroid is free and removes a whole class of bugs.

The **buck converter is at the far edge** of the board, as physically separated from the IMU and the MCU as possible. Switching regulators are noisy by nature, and the IMU is the most sensitive analog component on the board. Keeping them apart matters.

The **grounding topology is star-style.** Every ground-bearing component runs its return through a single physical point on the board rather than daisy-chaining. This avoids ground loops that would otherwise let high-current motor returns bleed into the I2C reference voltage.

*Schematic (KiCad) and final perfboard photos to be added during the Hardware Milestone.*

### Bill of Materials

| Component | Quantity | Role | Approx. Price |
|---|---|---|---|
| Raspberry Pi Pico 2 W | 1 | Main MCU + Wi-Fi (UDP server, PID, motor mixer) | ~40 RON |
| MPU6050 (or BMI270) | 1 | IMU — accelerometer + gyroscope (I2C / SPI) | ~15 RON |
| MP1584EN Buck Module | 1 | Steps 11.1 V LiPo down to 5 V for logic | ~10 RON |
| F450 Quadcopter Frame | 1 | Airframe / chassis | ~90 RON |
| 2212 920KV Brushless Motor | 4 | Propulsion | ~200 RON (set of 4) |
| 30 A BLHeli ESC | 4 | PWM-driven motor controllers | ~180 RON (set of 4) |
| 3S 11.1 V LiPo Battery | 1 | Main power source | ~120 RON |
| Propellers (1045 or similar) | 4+ | Thrust + spares | ~30 RON |
| FR4 Perfboard | 1 | Structural base for point-to-point assembly | ~10 RON |
| AWG 30 Wire-Wrap Wire | — | Logic-signal routing under the board | ~15 RON |
| XT60 Connectors + Silicone Wire | — | High-current LiPo wiring | ~25 RON |
| Misc. Hardware | — | Headers, screws, standoffs, heatshrink | ~15 RON |
| PC / Laptop | 1 | Ground-station UDP client (already owned) | — |

## Software

The firmware is `no_std` Rust on top of `embassy`. The core idea is that every "thing the drone has to do" becomes its own async task, and `embassy`'s executor handles the scheduling. There is no RTOS, no manual interrupt juggling, and no busy-waiting.

The task graph looks roughly like this:

* **`udp_server_task`** — Owns the UDP socket. Decodes incoming control packets and drops them into a shared atomic state struct. Never blocks the executor.
* **`imu_reader_task`** — Polls the MPU6050 over I2C at high frequency and pushes filtered orientation estimates into the shared state.
* **`pid_control_loop`** — A `Ticker`-driven task running on a fixed schedule. Reads the latest setpoint and orientation, computes per-axis PID corrections, and writes the result into the mixer state.
* **`motor_mixer_task`** — Translates throttle + roll/pitch/yaw corrections into four PWM duty cycles for the ESCs.
* **`failsafe_monitor`** — Watches the timestamp of the last valid UDP packet. If the gap exceeds 200 ms, it forces the mixer output to zero throttle, regardless of what the PID loop wants.

| Crate / Framework | Role |
|---|---|
| [`embassy-rp`](https://docs.embassy.dev/embassy-rp/) | HAL for the RP2350. Configures GPIO, PWM, I2C, and PIO. |
| [`embassy-executor`](https://docs.embassy.dev/embassy-executor/) | Async runtime. Runs every task above concurrently on a single core. |
| [`embassy-net`](https://docs.embassy.dev/embassy-net/) | Async network stack. Provides the UDP socket the control protocol runs on. |
| [`cyw43`](https://docs.rs/cyw43/) / [`cyw43-pio`](https://docs.rs/cyw43-pio/) | Driver for the Pico 2 W's onboard Wi-Fi chip. |
| [`embassy-time`](https://docs.embassy.dev/embassy-time/) | `Ticker` and `Instant` primitives — the backbone of the PID loop and the failsafe timeout. |
| [`mpu6050`](https://crates.io/crates/mpu6050) | I2C driver for the IMU. Will be replaced with a custom thin driver if I need finer control over register access. |
| [`micromath`](https://crates.io/crates/micromath) | Fast `no_std` math primitives for the PID and sensor-fusion code. |
| [`serde`](https://serde.rs/) + [`postcard`](https://docs.rs/postcard/) | Used on the PC-side client and on the device for serializing the control packet format. |
| [`defmt`](https://defmt.ferrous-systems.com/) + [`probe-rs`](https://probe.rs/) | Logging and flashing toolchain over the debug header. |

## Links

1. [Embassy Framework Documentation](https://embassy.dev/book/)
2. [Raspberry Pi Pico 2 W Documentation](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html)
3. [The Rust on Embedded Devices Book](https://docs.rust-embedded.org/book/)
4. [MPU6050 Register Map (datasheet)](https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf)
5. [BLHeli ESC Protocol Reference](https://github.com/bitdump/BLHeli)
6. [`embassy-net` UDP example](https://github.com/embassy-rs/embassy/tree/main/examples)
