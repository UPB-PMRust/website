# Altitude-Hold Drone

A WiFi-controlled quadrotor drone that autonomously maintains altitude when no operator input is received.

:::info

**Author**: David-Cristian Balan  
**GitHub Project Link**: [https://github.com/davidbalan05/website](https://github.com/davidbalan05/website)

:::

## Description

The project is a quadrotor drone designed for indoor flight, controlled from an Android phone over WiFi. It reads orientation data from an IMU and altitude data from an ultrasonic sensor, fuses the IMU readings through a Kalman filter, and uses PID controllers to stabilize attitude and hold altitude. When the operator releases the throttle, the drone locks the last commanded altitude and stays in place. The flight controller firmware and the Android control app are written in Rust.

## Motivation

I chose this project because it combines real-time embedded control, sensor fusion, and wireless communication into one system where every component matters. Most flight controllers are written in C/C++ — building one in Rust pushes me to understand every layer of a real cyber-physical system rather than treating any of them as a black box.

## Architecture

The system is organized into six layers, with data flowing top-down from the user's command to the spinning motors and power flowing bottom-up from the battery to every component:

* **Remote control** — an Android phone runs a Rust UDP client that sends target setpoints (throttle, pitch, roll, yaw) over WiFi.
* **Sensors and input** — MPU-6050 reads gyroscope and accelerometer data over I2C at 200 Hz, HC-SR04P reads altitude over GPIO at 50 Hz, and the CYW43439 WiFi chip on the Pico 2W receives setpoints over a UDP socket.
* **Flight controller** (running on the Pico 2W in Rust) — a Kalman filter fuses the IMU readings into stable pitch and roll estimates, a setpoint manager holds the current targets (and locks altitude when the throttle is released), and four independent PID controllers (pitch, roll, yaw, altitude) compute the corrections needed to drive the filtered state toward the setpoints.
* **Motor mixing and drivers** — the motor mixer combines the four PID outputs into individual speed commands for each of the four motors; each command is then sent as a PWM signal to a 30A BLHeli ESC, which converts it into a 3-phase drive signal.
* **Propulsion** — four A2212 1000KV brushless motors with 1045 propellers, arranged on the F450 frame in an X configuration with alternating CW and CCW rotation directions to cancel out the torque around the vertical axis.
* **Power** — a 3S LiPo battery (11.1V, 2200mAh, 30C) connects through an XT60 plug to the PDB integrated in the F450 frame, which distributes 11.1V to the four ESCs. One of the ESC's BEC outputs supplies regulated 5V back to the Pico 2W and the sensors.

![Architecture Diagram](architecture.svg)

## Log

### Week 20 - 26 April

Defined the project scope and architecture, researched existing Pi Pico-based drone projects for reference, and ordered all the hardware components.

### Week 27 April - 3 May

### Week 4 - 10 May

### Week 11 - 17 May

### Week 18 - 24 May

## Hardware

The drone is built from the following components:

* **Raspberry Pi Pico 2W** — the flight controller; runs the Rust firmware (Kalman filter, four PID loops, motor mixer) and hosts the WiFi access point that receives commands from the phone.
* **MPU-6050 (GY-521)** — 6-axis IMU containing a 3-axis gyroscope and a 3-axis accelerometer; provides raw orientation data over I2C at 200 Hz, used by the Kalman filter to estimate pitch and roll.
* **HC-SR04P** — ultrasonic distance sensor (3.3V variant, safe for the Pico's GPIO); measures the distance to the ground at 50 Hz, used as input for the altitude PID.
* **F450 frame with integrated PDB and landing gear** — the airframe that holds everything together; the integrated power distribution board distributes 11.1V from the battery to all four ESCs (rated up to 100A), and the landing gear protects the electronics during landings.
* **4× A2212 1000KV brushless motors** — provide the lift; rated for 2-3S LiPo and up to 12A each, paired with 1045 propellers for stable hover on a 3S battery.
* **4× 1045 propellers (2 CW + 2 CCW)** — convert motor rotation into thrust; the alternating rotation directions cancel out torque around the vertical axis so the drone doesn't spin uncontrollably.
* **4× 30A BLHeli ESCs (with 5V/2A BEC)** — convert the PWM signal from the Pico into the 3-phase drive signal for the brushless motors; one of the four BEC outputs is used to supply regulated 5V to the Pico and the sensors, the other three are disconnected to avoid backfeeding between regulators.
* **3S LiPo battery (11.1V, 2200mAh, 30C, XT60 connector)** — the main power source; the 30C rating provides up to 66A continuous current, well above the 4 × 12A peak draw of the motors.
* **SkyRC iMAX B6AC V2 charger** — used off-board to balance-charge the LiPo battery between flights; supports 1-6S LiPo at up to 6A.

### Schematics

### Bill of Materials

| Device | Usage | Price |
| --- | --- | --- |
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/products/raspberry-pi-pico-2/) | Flight controller with WiFi | 40 RON |
| [F450 Frame Kit + Landing Gear](https://www.aliexpress.com/) | Quadrotor airframe with integrated PDB | 127 RON |
| [A2212 1000KV Motor + 30A ESC + 1045 Propeller Set (×4)](https://www.aliexpress.com/) | Propulsion system | 250 RON |
| [3S 2200mAh 30C LiPo Battery (XT60)](https://www.emag.ro/) | Main power source | 120 RON |
| [SkyRC iMAX B6AC V2 Charger](https://www.skyrc.com/) | LiPo balance charger | 213 RON |
| [MPU-6050 (GY-521)](https://www.optimusdigital.ro/) | Gyroscope + accelerometer (I2C) | 32 RON |
| [HC-SR04P](https://www.optimusdigital.ro/) | Ultrasonic distance sensor (3.3V) | 32 RON |

## Software

| Library | Description | Usage |
| --- | --- | --- |
| [embassy-rp](https://github.com/embassy-rs/embassy) | HAL for RP2040/RP2350 | Abstracts Pico 2W peripherals (PWM, I2C, GPIO) |
| [embassy-net](https://github.com/embassy-rs/embassy) | TCP/UDP network stack | Receives control packets from the Android app |
| [cyw43](https://github.com/embassy-rs/embassy) | Driver for the CYW43439 WiFi chip | Enables WiFi access point and UDP socket |
| [mpu6050](https://crates.io/crates/mpu6050) | MPU-6050 driver | Reads raw accelerometer and gyroscope data |
| [kalman-filter](https://crates.io/crates/kalman_filter) | Kalman filter implementation | Fuses IMU readings into stable pitch/roll estimates |
| [pid](https://crates.io/crates/pid) | Generic PID controller | Implements the four control loops |

## Links

1. [Woobu Autonomous Drone (ellenrapps)](https://github.com/ellenrapps/Woobu-Autonomous-Drone)
2. [Scout Flight Controller (Tim Hanewich)](https://timhanewich.medium.com/taking-flight-with-the-raspberry-pi-pico-micropython-diy-quadcopter-drone-61ed4f7ee746)
3. [Building a Raspberry Pi Pico 2 drone from scratch (Raspberry Pi blog)](https://www.raspberrypi.com/news/building-a-raspberry-pi-pico-2-powered-drone-from-scratch/)
4. [PiWings — Pico flight controller controlled from a phone](https://www.hackster.io/news/ravi-butani-shows-off-piwings-a-custom-raspberry-pi-pico-flight-controller-for-educational-drones-cba8420e0189)
5. [Pico Drone — can we build a drone using a Raspberry Pi Pico?](https://www.youtube.com/watch?v=Us0bSUX9UkQ)
6. [Building a Quadcopter — Parts (pythonprogramming.net)](https://www.youtube.com/watch?v=aZltBzPUPKI)
7. [Autonomous drone with payload release (uC9hVyqGvDE)](https://www.youtube.com/watch?v=uC9hVyqGvDE)
8. [Raspberry Pi PICO Drone — Hardware (Robu.in)](https://www.youtube.com/watch?v=bYRzRxEIJiU)
9. [How a Kalman filter works, in pictures (Bzarg)](https://www.bzarg.com/p/how-a-kalman-filter-works-in-pictures/)
10. [Embassy framework documentation](https://embassy.dev/)