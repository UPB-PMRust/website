# Smart Door Lock
A biometric-based smart door lock system utilizing facial recognition technology to enhance home security by ensuring access control through used identification.

:::info 

**Author**: Gugulea Maria-Alexandra \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-alexandra-gugulea

:::

## Description

This project presents the design and implemetation of a smart home security system incorporating a facial recognition-based smart door lock, developed using the Raspberry Pi Pico W Microcontroller. The system integrates multiple sensors and modules to ensure robust access control and tamper detection.

A PIR motion sensor continously monitors for movement near the entrace and, upon detection, activates an ESP32-CAM module, which captures an image of the approaching individual. Facial recognition is then performed to determine user authorization. If a match is found, a servo motor is engaged to unlock the door. In addition, an SW-420 vibration sensor is embedded in the door handle to detect unusual vibrations or tampering attempts. Together, these components provide a multi-layered security system that combines biometric authentication, motion detection, and physical disturbance monitoring to enhance residential safety in an efficient and cost-effective manner.

## Motivation

In an age where smart technologies are becoming increasingly integrated into daily life, ensuring home security through intelligent systems is more important than ever. This project was motivated by the need to develop a more secure, responsive, and user-friendly home entry system that leverages facial recognition and tamper detection.

By combining biometric access control with motion and vibration sensing, the system aims to create a multi-layered defense mechanism against intrusion.

## Architecture 

### 1. Raspberry Pi Pico W

The Raspberry Pi Pico W functions as the central controller of the smart home security system. It coordinates all operations by interfacing with peripheral modules through GPIO and UART communication.

Upon receiving input from the PIR motion sensor, it sends a command to the ESP32-CAM to capture an image for facial recognition. Based on the response from the ESP32-CAM, the Pico W activates the servo motor to unlock the door if access is authorized.

### 2. SW-420 Vibration Sensor

The SW-420 vibration sensor is used to detect physical disturbances , such as shaking or tampering attemptsat the door. It is installed on or near the door handle and generates a digital signal when vibrations exceed a set threshold.

This signal is read by the Raspberry Pi Pico W, which then processes the event as a potential security breach. IN response, the system can activate the buzzer to alert nearby individuals. This component serves as a tamper-detection layer in the multi-tiered security framework.

### 3. PIR Motion Sensor

The Passive Infrared (PIR) motion sensor continously monitors the area near the enterance for movement. It detects changes in infrared radiation, typically caused by human presence.

When motion is detected, it sends a digital signal to the Raspberry Pi Pico W, prompting it to initiate facial recogntion via the ESP32-CAM. This mechanism ensures that the camera is only activated when necessary, conserving power and computational resources while increasing system efficiency.

### 4. ESP32-CAM

The ESP32-CAM is a compact camera module with onboard image processing capabilities. Upon receiving a trigger signal from the Raspberry Pi Pico W, it captures an image of the individual near the enterance. 

Facial recognition is performed locally on the device, comparing the captured face against stored profiles. If a match is found, a success response is sent back to the Pico W via UART. The ESP32-CAM plays a key role in enabling biometric access control, forming the core of the system's intelligent decision-making.

### 5. SG90 Servo Motor

The SG90 servo motor is a miniature actuator responsible for operating the physical door locking mechanism. Controlled by a PWM signal from the Raspberry Pi Pico W, the servo motor rotates to unlock the door when an authorized face is recognized. After a short delay, it returns to its original position to re-engage the lock.

The servo's role is critical for translating electronic authentication into mechanical action, thereby completing the access control process. 

### 6. Buzzer

The buzzer is used for generating audible alerts in the system. It is connected to the Raspberry Pi Pico W and activated in response to specific events such as tampering (from the vibration sensor) or unauthorized access attempts. It can also be used to signal successful operations such as confirmed facial match. The buzzer enhances system feedback and serves as a deterrent to unauthorized individuals.

## Log

### Week 14 - 18 April

I began the project by conducting research on all the components required for the build. This included reviewing datasheets, compatibility, and use cases. After finalizing the list of components, I proceeded to order them online.

### Week 20 - 25 April

Once the first batch of components arrived, I started soldering or attaching pins to the sensors and the other electronic parts. I also began prototyping by placing some of the components onto the breadboards to test their fit and layout.

### Week 5 - 11 May

This week focused on documentation and planning. I wrote the necessary technical documentation outlining the project's motivation and architecture.

### Week 12 - 18 May

This week focused entirely creating a strong basis for the project, by ensuring that all the hardware is placed correctly and writing the first lines of code. Additionaly, I completed the hardware milestone, describing all the components used, as well as their purpose in the course of my project, whilst also adding pictures of everything I've achieved so far. 

### Week 19 - 25 May

## Hardware

The purpose of the hardware milestone is to assemble and validate all essential electronic components required for the smart door lock system. This ensures that each hardware component is properly connected, tested for functionality, and integrated with the Raspberry Pi Pico before advancing to software integration and logic development.

### Raspberry Pi Pico
The Raspberry Pi Pico is a compact and cost-effective microcontroller board powered by the RP2040 chip. It features a dual-core ARM Cortex-M0+ processor running at up to 133 MHz, 264 KB of SRAM, and 2 MB of onboard Flash memory. It supports multiple I/O protocols including GPIO, SPI, I2C, UART, PWM, and ADC, making it versatile for embedded applications. In this project, the Pico acts as the central controller, coordinating input from sensors and controlling outputs like the servo motor.

### ESP32-CAM
The ESP32-CAM is a compact module that combines the powerful ESP32 microcontroller with a camera interface, onboard Wi-Fi, and an SD card slot. It's especially useful for adding facial recognition or motion-triggered image capture capabilities to your system, acting as an intelligent visual sensor for additional security or logging.

### HC-SR505 PIR Motion Sensor
The HC-SR505 is a mini passive infrared (PIR) motion sensor designated for small-scale embedded systems. It detects infrared radiation emitted by moving objects like humans and triggers a digital high signal when motion is detected. In this project, it is used to sense when someone approaches the door, triggering the rest of the system to evaluate further input from other sensors components.

### SW-420 Vibration Sensor
The SW-420 is a vibration switch module that detects physical vibrations or shocks. It outputs a digital signal when the threshold vibration is detected, typically used in anti-theft or tamper-detection systems. In the smart door lock project, the SW-420 provides an extra layer of security by sensing forced entry attempts or unusual tampering at the door, prompting the system to sound an alert or take preventative actions.

### SG90 Servo Motor
The SG90 is a micro servo motor commonly used in DIY electronics and robotics projects. It operates on a pulse width modulation (PWM) signal to rotate its output shaft between 0 and 180 degrees. In the context of the smart door lock, the servo motor physically controls the locking mechanism, opening or closing the latch when the system grants or denies access based on sensor input or other logic conditions.

### Buzzer
The buzzer is a simple audio output device that emits a sound when powered. It can be driven by a GPIO pin to produce beeps or alarms, alering users to specific events such as motion detection, tampering, or successful lock/unlock actions. In this project, it serves as an auditory feedback tool to indicate system states or intrusions, enhancing the interactivity and security of the setup.

### Schematics

![KiCad Schematics](schematics.svg)
![1st Image](set_up1good.webp)
![2nd Image](set_up2good.webp)
![3rd Image](set_up3good.webp)

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller that acts central controller for the smart lock system. It receives input from sensors (motion, vibration), processes logic, and controls actuators like the servo and buzzer. | [47 RON](https://ardushop.ro/ro/raspberry-pi/2453-raspberry-pi-pico-2-5056561803951.html?gad_source=1&gad_campaignid=22058879462&gclid=EAIaIQobChMIzOvO_M2qjQMVWgcGAB17YBkyEAQYASABEgKKw_D_BwE) |
| [ESP32-CAM](https://ardushop.ro/ro/comunicatie/1374-modul-esp32-cu-camera-6427854020031.html?gad_source=1&gad_campaignid=22058879462&gclid=EAIaIQobChMI94yh0c6qjQMV2WVBAh1GTTwuEAQYASABEgIOA_D_BwE) | Planned for future enhancement to provide visual verification at the door | [65 RON](https://ardushop.ro/ro/comunicatie/1374-modul-esp32-cu-camera-6427854020031.html?gad_source=1&gad_campaignid=22058879462&gclid=EAIaIQobChMI94yh0c6qjQMV2WVBAh1GTTwuEAQYASABEgIOA_D_BwE) |
| [HC-SR505 PIR Motion Sensor](https://static.rapidonline.com/pdf/78-4110_v1.pdf) | Detects motion near the door. When someone approaches, it triggers the Pico to prepare for possible unlock logic. | [10 RON](https://ardushop.ro/ro/module/508-modul-mini-senzor-pir-hc-sr505-6427854005922.html?gad_source=1&gad_campaignid=22058879462&gclid=EAIaIQobChMIo7rDwNCqjQMV7QYGAB3XIi1fEAQYBCABEgIyafD_BwE) |
| [SW-420 Vibration Sensor](https://media.digikey.com/pdf/Data%20Sheets/Seeed%20Technology/Grove_Vibration_Sensor_SW-420_Web.pdf) | Detects strong vibrations or knocks on the door | [6 RON](https://www.bitmi.ro/senzor-vibratie-sw-420-11516.html?gad_source=1&gad_campaignid=22005142538&gclid=EAIaIQobChMI4Ie_4uuqjQMVgz4GAB3aFg6TEAQYASABEgKrUPD_BwE) |
| [SG90](http://www.ee.ic.ac.uk/pcheung/teaching/DE1_EE/stores/sg90_datasheet.pdf) | Physically locks or unlocks a latch or bolt by rotating to a defined angle. | [10 RON](https://www.bitmi.ro/servomotor-sg90-180-grade-9g-10496.html?gad_source=1&gad_campaignid=22005721655&gclid=EAIaIQobChMIqISq6eyqjQMVvzsGAB1aXgsNEAQYASABEgIpwPD_BwE) | 


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...
