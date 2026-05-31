# Thermo-Tracking Smart Fan

## Description

This project implements an autonomous, closed-loop environmental control system using the STM32F103C8T6 (Blue Pill) microcontroller. It functions as a smart desk fan that actively tracks the user's position and adjusts its cooling power based on ambient temperature. Using an array of three PIR sensors, the system detects movement and drives a servo motor to orient the fan towards the user. Simultaneously, a DHT22 sensor monitors the room temperature, increasing the DC motor's fan speed via PWM as the heat rises. The system also features an OLED display to show live telemetry (temperature and fan speed) and an active buzzer that sounds an alarm if the temperature exceeds a critical threshold. The firmware is written entirely in Embedded Rust, ensuring memory safety and reliable hardware control.

## Motivation

Standard desk fans are inefficient, blowing air in fixed directions at constant speeds regardless of where the user is or how hot the room actually is. This project aims to create a highly practical, responsive desk accessory while demonstrating mastery over multiple embedded systems concepts. By combining digital inputs (motion sensing), digital communication protocols (I2C, 1-Wire), and analog-like actuation (PWM motor control), this project bridges the gap between environmental sensing and physical actuation. Utilizing Embedded Rust ensures that the device operates securely and consistently without runtime crashes, making it a robust hardware appliance rather than a simple prototype.

## Architecture

**STM32F103C8T6 (Blue Pill)**
* **Role:** Acts as the "brain" of the system — it controls all components: evaluates spatial logic from the PIR sensors, maps temperature data from the DHT22 to fan speeds, updates the OLED display, drives the servo and DC motor via PWM, and triggers the buzzer.
* **Connections:**
  * I2C to OLED Display
  * GPIO Inputs to HC-SR501 PIR Sensors
  * GPIO Input to DHT22 Sensor
  * PWM to SG90 Servo
  * PWM to L9110S Motor Driver
  * GPIO Output to Active Buzzer

**SSD1306 OLED Display (0.96")**
* **Interface:** I2C
* **Role:** Displays live telemetry including ambient temperature, current fan speed percentage, and tracking status.
* **Connections:**
  * SDA to STM32 SDA
  * SCL to STM32 SCL
  * VCC to 3.3V
  * GND to GND

**HC-SR501 PIR Sensors (x3)**
* **Interface:** GPIO
* **Role:** Detects infrared motion to divide the desk area into "Left", "Center", and "Right" zones for spatial tracking.
* **Connections:**
  * OUT signals to STM32 GPIO Inputs
  * VCC to 5V
  * GND to GND

**DHT22 Temperature Sensor**
* **Interface:** 1-Wire / GPIO
* **Role:** Continuously reads the ambient room temperature to dictate the required fan speed.
* **Connections:**
  * DATA to STM32 GPIO Input
  * VCC to 3.3V
  * GND to GND

**SG90 Servo Motor**
* **Interface:** PWM
* **Role:** Pans the fan assembly left and right to physically aim the airflow at the user.
* **Connections:**
  * Signal to STM32 PWM Output
  * VCC to 5V
  * GND to Shared GND

**DC Motor & L9110S Driver**
* **Interface:** PWM
* **Role:** The driver isolates the high-current DC fan motor from the microcontroller, allowing safe variable speed control.
* **Connections:**
  * IN-A (Signal) to STM32 PWM Output
  * VCC to External 5V Battery/Supply
  * GND to Shared GND

**Active Piezo Buzzer**
* **Interface:** GPIO
* **Role:** Emits a continuous warning tone if the room temperature exceeds a safe maximum threshold (e.g., 30°C).
* **Connections:**
  * Signal to STM32 GPIO Output
  * GND to GND

## Log

**Week 7 - 14 April**
Project selection, requirements gathering, and initial GitHub repository setup. Finalized the system architecture, selected the STM32F103C8T6 as the primary microcontroller, and mapped out the required peripheral interfaces. 

**Week 8 - 21 April**
*(Planned)* Acquire all hardware components. Begin initial breadboard testing by setting up the basic GPIO inputs for the three PIR sensors and configuring the digital output for the active buzzer.

**Week 9 - 28 April**
*(Planned)* Implement hardware timers in Rust to generate precise PWM signals. Test the physical actuation of the SG90 Servo motor for panning, and the L9110S driver for variable DC fan speed control.

**Week 10 - 5 May**
*(Planned)* Integrate the communication protocols. Implement I2C to initialize and draw text to the SSD1306 OLED display, and write the polling logic for the DHT22 temperature sensor.

**Week 11 - 12 May**
*(Planned)* Combine all subsystems into the main control loop. Finalize the spatial tracking logic and thermal regulation thresholds. Begin creating the official hardware schematic in KiCad.

**Week 12 - 19 May**
*(Planned)* Final debugging of the system. Assemble the hardware onto a permanent mount/chassis. Finalize all project documentation and prepare for the live PM Fair demonstration.

## Hardware

* **STM32F103C8T6 (Blue Pill)** → Acts as the central controller, evaluating sensor data and driving actuators.
* **HC-SR501 PIR Sensors (x3)** → Detects user position to guide the fan's orientation.
* **DHT22 Temperature Sensor** → Measures room temperature to dynamically adjust fan speed.
* **SG90 Servo Motor** → Physically rotates the fan to aim at the user.
* **DC Motor & Fan Blade** → Provides the actual cooling airflow.
* **L9110S Motor Driver** → Safely powers the DC motor and allows for PWM speed control.
* **SSD1306 OLED Display (I2C)** → Serves as a dashboard to display current temperature and fan status.
* **Active Piezo Buzzer** → Provides an auditory alarm if temperatures get too high.

## Schematics

*(KiCad schematic images here later)*
## Bill of Materials

| Device | Usage | Price |
| :--- | :--- | :--- |
| STM32F103C8T6 | The main microcontroller board | 20 LEI |
| ST-Link V2 | Used as a debug probe to flash code | 20 LEI |
| HC-SR501 PIR (x3) | Senses user movement for tracking | 30 LEI |
| DHT22 Sensor | Reads ambient temperature | 15 LEI |
| SG90 Servo Motor | Pans the fan left and right | 12 LEI |
| 5V DC Motor + Fan | Provides cooling airflow | 10 LEI |
| L9110S Driver | Controls the DC motor speed safely | 10 LEI |
| SSD1306 OLED | Displays telemetry and status via I2C | 15 LEI |
| Active Buzzer | Emits warning tone for high heat | 3 LEI |
| Breadboard & Wires | Prototyping and connecting components | 35 LEI |

## Software

| Library | Description | Usage |
| :--- | :--- | :--- |
| `cortex-m-rt` | ARM Cortex-M runtime | Handles the startup code and minimal runtime for the STM32 |
| `stm32f1xx-hal` | Hardware abstraction layer for STM32F1 | Interfaces with peripherals like PWM (Motors), I2C (OLED), and GPIO (Sensors) |
| `ssd1306` | Driver for SSD1306 OLEDs | Initializes and communicates with the display screen over I2C |
| `embedded-graphics` | 2D graphics library | Used to draw text, numbers, and UI elements on the OLED screen |
| `dht-sensor` | Driver for DHT sensors | Reads and parses temperature and humidity data from the DHT22 |
| `panic-halt` | Panic handler | Ensures the microcontroller stops safely if the code encounters a fatal error |

## Links

[Source Code Repository](https://github.com/UPB-PMRust-Students/fils-project-2026-Marwan-010)