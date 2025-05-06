# **SafeDelivery**

A smart package security system to monitor and verify package integrity during transport

:::info

**Author**: Sebastian Stefan Bahrin  

**GitHub Project Link**: [\[link_to_github\]](https://github.com/UPB-PMRust-Students/proiect-SebiBahrin)

:::

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
| [Raspberry Pi Pico 2W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=raspberry+pi+pico&results=26) | The main microcontroller that manages all sensors and logs events | 40 RON |
| [Light Sensor (LDR)](https://www.bitmi.ro/electronica/modul-senzor-cu-fotorezistor-ldr-compatibil-arduino-10394.html) | Detects if the package has been opened (light exposure) | 5 RON |
| [Vibration Sensor (SW-420)](https://www.bitmi.ro/electronica/senzor-vibratie-sw-420-11516.html) | Detects shocks, movements, or violent handling of the package | 5 RON |
| [Reed Switch + Magnets](https://www.bitmi.ro/senzor-magnetic-digital-reed-magnetron-10460.html?gad_campaignid=22504586579) | Detects if the package lid has been opened (magnetic seal broken) | 20 RON |
| [IMU Sensor (MPU6500)](https://www.bitmi.ro/module-electronice/modul-accelerometru-giroscop-pe-6-axe-i2c-mpu-6500-gy-6500-11213.html) | Monitors orientation, detects abnormal tilting or positioning | 13 RON |
|  [LCD de 1.44''](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/2167-lcd-de-144-pentru-stc-stm32-i-arduino.html?gad_campaignid=19615979487) | Displays package status (intact/tampered) and stores all recorded events for later verification with the SD card | 40 RON |
| [GPS Module](https://www.optimusdigital.ro/ro/gps/8239-modul-gps-neo-7m.html?search_query=modul+gps&results=34) | Provides real-time location tracking of the package, allowing you to monitor its journey and verify if it has deviated from the intended route | 60 RON |
| [Battery](https://www.optimusdigital.ro/ro/suporturi-de-baterii/4057-suport-pentru-baterie-de-12v-23a.html?search_query=baterie&results=311)  | Provides autonomous power during transport | 20 RON |
| Case / Box | Simulates the sealed package in which sensors are mounted | Found at home |

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

1. [I2C](https://pmrust.pages.upb.ro/docs/acs_cc/lab/06)
2. [WIFI](https://pmrust.pages.upb.ro/docs/acs_cc/lab/07)

