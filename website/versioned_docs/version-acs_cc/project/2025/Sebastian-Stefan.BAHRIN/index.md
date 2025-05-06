# **SafeDelivery**

A smart package security system to monitor and verify package integrity during transport

**Author**: Sebastian Stefan Bahrin  
**GitHub Project Link**: [\[link_to_github\]](https://github.com/SebiBahrin/SafeDelivery)

## **Description**

SafeDelivery is an intelligent system for securing package transport, built with Raspberry Pi Pico 2W. The device monitors the physical conditions of a package during transport and detects opening attempts, rough handling, or abnormal positioning. The system logs all these events to a microSD card, allowing recipients to verify if the package was properly handled throughout delivery.

## **Motivation**

The inspiration for this project came from my personal experiences with damaged deliveries and the frustrating uncertainty of not knowing whether valuable items were mishandled before they reached me. By creating an affordable, reliable monitoring system, SafeDelivery empowers both senders and recipients with irrefutable evidence of a package's treatment during transit.

Beyond individual consumers, this technology has significant implications for industries shipping sensitive, fragile, or high-value items such as medical supplies, electronics, or artwork. SafeDelivery represents not just a technical solution but a step toward greater accountability in global logistics chains.

## **Architecture**

### Main Components:

1. **Sensor Array** - Multiple sensors that monitor different aspects of package integrity:
   - Light sensor (LDR) for detecting package opening
   - Vibration sensor (SW-420) for detecting rough handling
   - Reed switch with magnets for detecting seal breakage
   - IMU sensor (MPU6050) for detecting orientation and position changes

2. **Processing Unit** - Raspberry Pi Pico 2W processes all sensor inputs and makes decisions

3. **Storage System** - microSD card with SPI adapter for logging all events with timestamps

4. **Notification System**:
   - OLED display showing package status
   - LED indicator (green = intact, red = tampered)
   - Active buzzer for audible alerts

5. **Power Management** - Battery system for autonomous operation

### Connection Diagram:
## System Diagram
TBA

## **Log**

### **Week 5 - 11 May**
TBA

### **Week 12 - 18 May**
TBA

### **Week 19 - 25 May**
TBA

## **Hardware**

The project uses the Raspberry Pi Pico 2W as its central microcontroller, which offers Wi-Fi connectivity, low power consumption, and sufficient GPIO pins for all sensors. The sensor array includes an LDR for light detection, SW-420 for vibration sensing, reed switches with magnets for tamper detection, and an MPU6050 IMU for orientation monitoring. Visual feedback is provided through an OLED display and status LEDs, while a buzzer provides audible alerts. All events are logged to a microSD card via SPI interface.

## **Schematics**
TBA

## **Bill of Materials**

| **Device** | **Usage** | **Price** |
|------------|-----------|-----------|
| Raspberry Pi Pico 2W | The main microcontroller that manages all sensors and logs events | - RON |
| Light Sensor (LDR) | Detects if the package has been opened (light exposure) | - RON |
| Vibration Sensor (SW-420) | Detects shocks, movements, or violent handling of the package | - RON |
| Reed Switch + Magnets | Detects if the package lid has been opened (magnetic seal broken) | - RON |
| IMU Sensor (MPU6050) | Monitors orientation, detects abnormal tilting or positioning | - RON |
| microSD + SPI Adapter | Stores all recorded events for later verification | - RON |
| OLED 0.96" | Displays package status (intact/tampered) | - RON |
| Active Buzzer | Emits sound alert if suspicious event is detected | - RON |
| LED + Resistor | Visual display of package status (green = OK, red = tampered) | - RON |
| Battery | Provides autonomous power during transport | - |
| Breadboard / PCB | Organized mounting of components (prototype) | - RON |
| Jumper Wires | Connection between sensors and microcontroller | - RON |
| Case / Box | Simulates the sealed package in which sensors are mounted | - RON |

## **Software**

| **Library** | **Description** | **Usage** |
|-------------|-----------------|-----------|
| [embassy](https://embassy.dev/) | Asynchronous embedded framework for Rust | Core framework for running async tasks on the Raspberry Pi Pico 2W |
| [embassy-rp](https://embassy.dev/) | Embassy HAL for RP2040 | Hardware abstraction layer for the Raspberry Pi Pico 2W |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware Abstraction Layer (HAL) traits | Provides unified interfaces for hardware drivers |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the OLED display |
| [ssd1306](https://github.com/jamwaffles/ssd1306) | Rust driver for SSD1306 OLED displays | Controls the 0.96" OLED display |
| [mpu6050](https://github.com/juliangaal/mpu6050) | Rust driver for MPU6050 | Interfacing with the IMU sensor |
| [shared-bus](https://github.com/Rahix/shared-bus) | Bus sharing for embedded Rust | Sharing I2C/SPI buses between multiple devices |
| [embedded-sdmmc](https://github.com/rust-embedded-community/embedded-sdmmc-rs) | SD/MMC card access | Managing log files on the microSD card |
| [defmt](https://github.com/knurling-rs/defmt) | Logging framework for embedded devices | Debugging and development logging |
| [chrono](https://github.com/chronotope/chrono) | Date and time library | Creating timestamps for event logging |

## **Links**`

1. [Link1](https://github.com/SebiBahrin/SafeDelivery)
2. [Link2](https://github.com/SebiBahrin/SafeDelivery)
3. [Link3](https://github.com/SebiBahrin/SafeDelivery)
4. [Link4](https://github.com/SebiBahrin/SafeDelivery)
5. [Link5](https://github.com/SebiBahrin/SafeDelivery)
6. [Link6](https://github.com/SebiBahrin/SafeDelivery)