# Smart Door Lock
A biometric-based smart door lock system utilizing facial recognition technology to enhance home security by ensuring access control through used identification.

:::info 

**Author**: Gugulea Maria-Alexandra \
**GitHub Project Link**: https://github.com/alexandra-gugulea

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

### Week 19 - 25 May

## Hardware

Detail in a few words the hardware used.

### Schematics

Place your KiCAD schematics here.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |


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
