# Ali coffee
Coffee vending machine with drinks selection options, based on ESP-32-C6 chip.

:::info

**Author:** Artem Lebedev (group 1221EB, FILS) \
**GitHub Project Link:** [Ali coffee](https://github.com/UPB-PMRust-Students/fils-project-2026-artemlebedev)

:::

## Description

This project is a coffee vending machine capable of creating a variety of coffee drinks using a gravity- and time-based dispenser of hot water and instant coffee. To implement this idea, we'll use a solenoid valve for the water tank, a servo motor with a flap for the coffee container, drink selection buttons, and a host of additional features: an LCD screen displaying the brewing process and water level, a buzzer to notify when the coffee is ready, and a float sensor to indicate when the water level is low. At the core of all these devices will be an ESP32-C6 chip; the enclosure will be a wooden T-shaped structure.

## Motivation

My desire to build a coffee machine, rather than any other device, is easy to explain – I'm a big fan of this aromatic drink, and if I'm going to do anything, it's going to be something I personally wouldn't shy away from using. Of course, my design is a far cry from a professional coffee machine, but at the same time, I think it will provide worthy competition to the machines dotted around campus.

## Architecture

The system is organized into three functional layers:

**Input Layer**
- Three tactile push buttons select the drink mode: Espresso, Double Espresso, or Americano.
- A float switch liquid level sensor monitors the water tank and signals when it is empty.

**Control Layer**
- The ESP32-C6 SuperMini runs the main async task loop via Embassy.
- On button press, a mode-specific timing profile is selected, and serving coffee and water for the correct duration starts.
- The water sensor is polled continuously; if the tank is empty, dispensing is blocked and the display shows a warning.

**Output Layer**
- A 12V solenoid valve (normally closed) controls water flow from the tank to the cup, driven by an IRF540N MOSFET with a 100Ω gate resistor.
- A servo motor (SG90) opens and closes a mechanical gate on the coffee powder container.
- An SSD1306 OLED display (0.96", I2C) shows brewing status and warnings.

All components share a common ground. The 12V rail powers the solenoid valve and is supplied by an external 12V 2A DC adapter. The ESP32-C6 is powered via USB.

## Diagram
![Ali_coffee_schematics](ali_coffee_schematics.webp)

## Log

### Week 5 - 11 May 

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The system is built around the ESP32-C6 SuperMini microcontroller, which manages all inputs and outputs over GPIO, PWM (LEDC), and I2C. Two food-grade plastic containers (1–2L) serve as the water and coffee powder tanks, each mounted above the dispensing point to allow gravity-fed flow.

Water flow is controlled by a 12V normally-closed solenoid valve. When the ESP32-C6 pulls the MOSFET gate high, the valve opens and hot water flows through a silicone tube into the cup. The MOSFET (IRF540N) is connected with a 100Ω resistor on the gate to limit switching transients.

Coffee powder is dispensed through a mechanical gate controlled by the SG90 servo. At rest, the gate is closed. When activated, the servo rotates 90° to open the gate, powder falls by gravity, and the servo returns to close it after the timed duration.

### Pin Connections

| ESP32-C6 GPIO | Function | Connected To |
|---|---|---|
| GPIO2 | Button – Espresso | Tactile button → GND |
| GPIO3 | Button – Double Espresso | Tactile button → GND |
| GPIO4 | Button – Americano | Tactile button → GND |
| GPIO5 | Solenoid valve control | IRF540N Gate (via 100Ω) |
| GPIO6 | I2C SCL | SSD1306 SCL |
| GPIO7 | I2C SDA | SSD1306 SDA |
| GPIO8 | Servo PWM (LEDC) | SG90 signal wire |
| GPIO9 | Water level sensor | Float switch → GND |
| 3.3V | Power | SSD1306 VCC, SG90 VCC, Float switch |
| GND | Common ground | All GND rails |

### Schematics


## Bill of Materials

| Device | Usage | Price |
|---|---|---|
| ESP32-C6 | Main microcontroller | 50 RON |
| Solenoid Valve 12V N/C | Water flow control | 25 RON |
| IRF540N MOSFET | Solenoid valve driver | 2.5 RON |
| Resistor 100Ω | Gate resistor for MOSFET | 0.5 RON |
| Servo SG90 | Coffee powder gate control | 13.5 RON |
| OLED SSD1306 0.96" I2C | Status display | 10 RON |
| Float Switch Liquid Level Sensor | Water tank monitoring | 25 RON |
| Food-grade plastic container 1–2L (×2) | Water and coffee powder tanks | 30 RON |
| Silicone tube (thermoresistant) | Water delivery from tank to cup | 10 RON |
| Hose barb fitting | Tank outlet connector | 5 RON |
| Tactile push buttons (×3) | Mode selection | 5 RON |
| Breadboard 830 points | Prototyping platform | 10 RON |
| Dupont jumper wires (M-M, M-F set) | Component connections | 10 RON |
| DC barrel jack 5.5mm | Power supply connector | 2.5 RON |
| 12V 2A DC power adapter | Powers solenoid valve | 20 RON |
| **TOTAL** | | **220 RON** |

***

## Software

| Library | Description | Usage |
|---|---|---|
| `esp-hal` | Hardware Abstraction Layer for ESP32-C6 | GPIO, LEDC PWM, I2C peripheral control |
| `esp-hal-embassy` | Embassy runtime adapter for ESP32 | Enables async task execution on ESP32-C6 |
| `embassy-executor` | Async task executor | Runs concurrent tasks: buttons, dispensing, display |
| `embassy-time` | Async time utilities | `Timer::after()` for precise dispensing durations |
| `embedded-hal` | Hardware abstraction traits | Standard interface for GPIO and PWM |
| `embedded-hal-async` | Async hardware abstraction traits | `wait_for_falling_edge()` for button and sensor input |
| `ssd1306` | SSD1306 OLED display driver | Renders status messages on the 0.96" display |
| `embedded-graphics` | 2D graphics primitives | Text rendering on the OLED display |
| `esp-backtrace` | Panic handler with serial output | Debugging — prints panic traces over UART |
| `esp-println` | `println!` macro over UART | Logging timing and sensor values during development |
