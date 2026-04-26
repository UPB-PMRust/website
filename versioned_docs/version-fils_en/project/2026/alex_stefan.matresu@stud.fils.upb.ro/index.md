\# PS4-Controlled Self-Parking Car

A ESP32-powered RC car controlled via PS4 controller with ultrasonic-based autonomous parking, written in Rust.



:::info



\*\*Author:\*\* Matresu Binjung Alex Stefan 





:::



\## Description



This project is a remote-controlled car built around the ESP32 microcontroller and programmed entirely in Rust using the Embassy async framework. The car is driven wirelessly using a PS4 DualShock controller connected via Bluetooth. Additionally, the car features an autonomous parking mode: using ultrasonic distance sensors mounted on all four sides, it can detect a suitable parking space and execute a parallel or perpendicular parking maneuver automatically.



\## Motivation



Embedded Rust is a rapidly growing field that combines the safety guarantees of Rust with low-level hardware control. This project was chosen to explore real-time Bluetooth communication, motor control, and sensor fusion on a resource-constrained device — all while leveraging Rust's async capabilities through Embassy. Self-parking adds a challenging algorithmic layer that mirrors real automotive systems.



\## Architecture



The system is composed of the following main components:



\* \*\*PS4 Controller (Bluetooth HID)\*\* — sends joystick and button events wirelessly to the ESP32.

\* \*\*ESP32 (Main MCU)\*\* — receives BT input, runs the control loop, reads sensors, and drives motors.

\* \*\*Motor Driver Module (L298N)\*\* — receives PWM signals from ESP32 and powers the DC motors.

\* \*\*DC Motors (x4)\*\* — drive the wheels for forward, backward, and turning motion.

\* \*\*Ultrasonic Sensors (x4)\*\* — mounted front, rear, left, and right; measure distances for parking logic.

\* \*\*Parking Logic Module (Software)\*\* — Embassy async task that reads sensor data and computes parking maneuvers.

\* \*\*Power Module\*\* — LiPo battery + voltage regulator supplying 5V to ESP32 and 6-12V to motors.



The PS4 controller connects over Bluetooth Classic to the ESP32. The ESP32 runs two concurrent Embassy tasks: one handling BT input and motor PWM output, and one polling the ultrasonic sensors. When parking mode is activated (via a controller button), the sensor task takes over control of the motors.



\## Hardware



The car uses an ESP32 as its brain, four DC gear motors for movement, an L298N dual H-bridge motor driver, four HC-SR04 ultrasonic sensors for distance measurement, and a LiPo battery pack for power. A PS4 DualShock 4 controller communicates with the ESP32 over Bluetooth Classic.



\## Schematics



Place your KiCAD schematic here in SVG format.



\## Bill of Materials



| Device | Usage | Price |

|--------|-------|-------|

| \[ESP32 DevKit v1](https://www.espressif.com/en/products/socs/esp32) | Main microcontroller — Bluetooth + motor control | 35 RON |

| \[L298N Motor Driver](https://www.st.com/resource/en/datasheet/l298.pdf) | Drives the 4 DC motors via PWM signals | 12 RON |

| DC Gear Motors x4 | Wheel drive — forward, backward, turning | 40 RON |

| \[HC-SR04 Ultrasonic Sensors x4](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf) | Distance sensing for parking on all 4 sides | 24 RON |

| PS4 DualShock 4 Controller | Wireless Bluetooth HID input device | \~200 RON |

| LiPo Battery 7.4V 2200mAh | Main power source for motors and MCU | 45 RON |

| LM7805 Voltage Regulator | Regulates 7.4V down to 5V for ESP32 | 3 RON |

| 4WD Robot Car Chassis Kit | Chassis, wheels, and motor mounts | 35 RON |

| Jumper Wires + Breadboard | Prototyping and wiring connections | 10 RON |



\## Software



| Library | Description | Usage |

|---------|-------------|-------|

| \[embassy-rs](https://embassy.dev/) | Async embedded framework for Rust | Main async runtime for all tasks |

| \[esp-hal](https://github.com/esp-rs/esp-hal) | ESP32 hardware abstraction layer | GPIO, PWM, timers, Bluetooth |

| \[bt-hid](https://github.com/esp-rs/esp-idf-hal) | Bluetooth HID host for Rust | Reading PS4 controller input |

| \[hc-sr04](https://crates.io/crates/hc-sr04) | Ultrasonic sensor driver | Distance measurement for parking |

| \[defmt](https://github.com/knurling-rs/defmt) | Lightweight logging for embedded | Debug output over RTT |



\## Links



1\. \[Embassy-rs Async Embedded Rust Framework](https://embassy.dev/)

2\. \[ESP32 Rust HAL - esp-hal](https://github.com/esp-rs/esp-hal)

3\. \[HC-SR04 Ultrasonic Sensor Datasheet](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf)

4\. \[PS4 Bluetooth HID Protocol Reference](https://www.psdevwiki.com/ps4/DS4-BT)

5\. \[L298N Motor Driver Datasheet](https://www.st.com/resource/en/datasheet/l298.pdf)

