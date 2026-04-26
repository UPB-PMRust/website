# Thermo-Tracking Smart Fan

## 1. Description of the Functionality
The **Thermo-Tracking Smart Fan** is an autonomous, closed-loop environmental control system. Unlike a standard desk fan that blows air in a fixed direction at a fixed speed, this system actively tracks the user's position in real-time and dynamically adjusts its cooling power based on the ambient room temperature. 

It features two primary modes of operation:
* **Spatial Tracking:** Using an array of three Passive Infrared (PIR) sensors, the system detects user movement and drives a servo motor to orient the fan directly toward the user.
* **Thermal Regulation:** A DHT22 sensor continuously monitors the environment. As the temperature rises, the microcontroller uses Pulse Width Modulation (PWM) to increase the DC motor's fan speed. 

Additionally, the system acts as a desktop telemetry dashboard, featuring an I2C OLED display to show live temperature and fan speed data, along with a piezoelectric buzzer that sounds an alarm if the temperature exceeds a critical threshold.

## 2. Motivation
The motivation behind this project is to create a highly practical desk accessory while demonstrating mastery over multiple embedded systems concepts. I wanted to build a project that bridges the gap between digital inputs (motion sensing), digital communication protocols (I2C and 1-Wire), and analog-like actuation (PWM motor control). Writing the firmware in Embedded Rust ensures memory safety and high reliability, making this not just a toy, but a robust hardware appliance.

## 3. Architecture
The system architecture revolves around a centralized microcontroller (STM32) acting as the brain, gathering data from environmental sensors, and sending control signals to mechanical actuators and the user interface.

* **Inputs:** PIR motion sensors (Digital GPIO), DHT22 Temperature Sensor (1-Wire).
* **Processing:** STM32F103C8T6 evaluating spatial logic and thermal mapping.
* **Outputs:** SG90 Servo (PWM), DC Motor via L9110S Driver (PWM), OLED Display (I2C), Active Buzzer (Digital GPIO).

*(image here later)*
## 4. Hardware Design
### Description of Hardware Used
* **STM32F103C8T6 (Blue Pill):** An ARM Cortex-M3 microcontroller that provides the necessary GPIO pins, hardware timers for PWM, and I2C peripherals.
* **HC-SR501 PIR Sensors:** Infrared sensors used to divide the desk area into "Left", "Center", and "Right" zones.
* **DHT22 Sensor:** A highly accurate digital sensor for reading ambient room temperature and humidity.
* **SG90 Servo:** A 180-degree micro servo used to pan the fan assembly.
* **DC Motor & L9110S Driver:** The driver safely isolates the high-current DC motor from the STM32's logic pins while allowing speed control via PWM.
* **SSD1306 OLED Display (0.96"):** A small screen used to display the system's current telemetry.
* **Piezo Buzzer:** Provides auditory alerts for thermal warnings.

*(KiCad schematic and device photos here later)*
## 5. Software Design
The software is written in **Embedded Rust** using a `no_std` environment. It utilizes the `stm32f1xx-hal` (Hardware Abstraction Layer) to safely interact with the chip's registers.

The logic operates in a continuous control loop:
1. **Sensor Polling:** The system checks the state of the three PIR sensors and updates the target angle for the servo motor.
2. **Thermal Mapping:** The DHT22 is polled. The temperature float value is mapped to a PWM duty cycle range (e.g., 24°C = 50% duty, 30°C = 100% duty).
3. **UI Update:** The `embedded-graphics` library buffers the new temperature and speed data and flushes it to the OLED via I2C.
4. **Safety Check:** If the temperature exceeds 30°C, the GPIO pin attached to the buzzer is pulled HIGH to trigger the alarm.

*(software functional diagram here later)*
## 6. Materials (BOM)

### Hardware
| Component | Quantity | 
| :--- | :--- | 
| STM32F103C8T6 (Blue Pill) | 1 | 
| ST-Link V2 Debugger | 1 |
| HC-SR501 PIR Sensor | 3 | 
| DHT22 Temperature Sensor | 1 |
| SG90 Micro Servo | 1 | 
| 5V DC Motor + Fan Blade | 1 |
| L9110S Motor Driver | 1 |
| 0.96" I2C OLED Display | 1 | 
| Active Piezo Buzzer | 1 |
| Breadboard & Wires | 1 | 


### Software (Rust Crates)
* `cortex-m-rt`: ARM Cortex-M runtime.
* `stm32f1xx-hal`: Hardware abstraction layer.
* `dht-sensor`: DHT22 protocol driver.
* `ssd1306`: OLED display driver.
* `embedded-graphics`: 2D drawing library for the display.
* `panic-halt`: Panic handler.

## 7. Weekly Log
| Week | Task Description | Status |
| :--- | :--- | :--- |
| **Week 7** | Project selection, requirements gathering, and initial GitHub repository setup. | Completed |
| **Week 8** | Hardware acquisition and initial breadboard testing of basic GPIO (Buzzer, PIR sensors). | Pending |
| **Week 9** | Implementation of PWM control for the Servo and DC Motor. | Pending |
| **Week 10** | Integration of I2C OLED display and DHT22 sensor logic. | Pending |
| **Week 11** | System assembly, final code integration, and KiCad schematic generation. | Pending |
| **Week 12** | Final debugging and documentation review. | Pending |
| **Week 13/14** | PM Fair presentation and live hardware demo prep. | Pending |