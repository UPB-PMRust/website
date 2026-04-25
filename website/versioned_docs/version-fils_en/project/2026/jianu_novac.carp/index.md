# RustInjector

RC car that can jump on desks and inject usbs into laptops.


**Author**: Carp Jianu-Novac-Mihail
**GitHub Project Link**:  [https://github.com/UPB-PMRust-Students/fils-project-2026-novacLearnsCyber](https://github.com/UPB-PMRust-Students/fils-project-2026-novacLearnsCyber)

## Description

An RC car controlled via Wi-Fi from a mobile phone, designed to jump approximately 75cm (average desk height) and land safely to perform a physical USB injection attack on a target laptop.The design may resemble a monster truck toy to "camouflage" better. 


## Motivation

Initially, I wanted to build a cybersecurity-oriented project,because i am quiete pasionate of the subject, but I also wanted it to be flashy. My first ideas were a bizarre hardware token or a Wi-Fi IDS monitor, but they weren't exciting enough. Then I remembered the Jumper from Watch Dogs 2 and decided to build something inspired by it.
## Architecture

Add here the schematics with the architecture of your project. Make sure to include:

- what are the main components (architecture components, not hardware components)
- how they connect with each other

## Log

### Week 20 - 25 April
- ordered online 99% of all the components .I need to find a place that sells industrial springs strong enough to jump a 1 kg RC car into air 
- brainstormed ideas for the jump mechanism 
- a few roughs sketches on paper for now 
### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

SImple rc car, uses a simple hardware setup, keeping in mind the shocks  it will suffer from jumping.
Control : Raspberry PI Pico W , while the ESP32-CAM provides live video 
Jumping mech : high torque JGA25 motor compresses a steel spring , released by the MG996R servo latch.
Drive : 4x TT motors with custom 3D-printed tpu wheels for traction and shock absorption.
Power: Gens Ace LiPo Battery regulated by an XL4015 5A Buck converter to keep the servos and logic rails stable during the jumps.
Sensing: VL54L0X measuring altitude and MPU6059 monitors stability and landing orientation 
Inject:MG90S servo deploys the usb.



### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

| Device                                                                                                   | Usage                                   | Price                                                                                         |
| -------------------------------------------------------------------------------------------------------- | --------------------------------------- | --------------------------------------------------------------------------------------------- |
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller                     | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| Dual Motor Driver L9110S                                                                                 | Driver for 4WD Drive Motors             | 7.98 RON                                                                                      |
| SERVO MG996R (13kg)                                                                                      | Jumping Latch                           | 17.68 RON                                                                                     |
| ESP32-CAM + OV2640                                                                                       | Video Streaming , Web Control Interface | 70.51 RON                                                                                     |
| Motor JGA25-370 20RPM                                                                                    | Jumping Mechanism Motor (High Torque)   | 107.00 RON                                                                                    |
| 4x TT Motors + Wheels                                                                                    | 4WD Locomotion System                   | 31.86 RON                                                                                     |
| SERVO MG90S Metal                                                                                        | USB Injection Arm Actuator              | 27.43 RON                                                                                     |
| XL4015 Step-Down 5A                                                                                      | 5V Power Regulation for Servos & Logic  | 14.13 RON                                                                                     |
| L298N Motor Driver                                                                                       | High-Current Driver for Jump Motor      | 15.00 RON                                                                                     |
| MPU6050 IMU                                                                                              | Gyroscope & Accelerometer               | 14.68 RON                                                                                     |
| VL53L0X ToF Sensor                                                                                       | Laser Distance Measurement              | 25.11 RON                                                                                     |
| Gens Ace LiPo Battery                                                                                    | Main Power Source                       |                                                                                               |
| IMAX B3 Charger                                                                                          | LiPo Battery Balancing Charger          | 40.00 RON                                                                                     |
| Heavy-Duty Spring                                                                                        | Energy Storage for 1.20m Jump           | 20.00 RON                                                                                     |
| PCB Prototype Kit                                                                                        | Custom Circuit Boards for Power Hub     | 28.31 RON                                                                                     |



## Software

| Library    | Description                       | Usage                                                                       |
| ---------- | --------------------------------- | --------------------------------------------------------------------------- |
| cyw43      | Driver for the Pico W WiFi chip   | Used to handle the wireless connection and the web control interface.       |
| embassy-rs | async framework for Rust embedded | The main framework for task management                                      |
| mpu6050    | MPU6050 IMU driver                | Used to read gyroscope and accelerometer data for landing stability.        |
| serde      | Data serialization                | Used to parse commands sent from the phone to ESP32-CAM via JSON or binary. |

## Links

1. [Jumper in game](https://www.youtube.com/watch?v=xSfsmZ3VV_s)
2. [Jumping mechanism inspiration](https://www.youtube.com/watch?v=RuMLHJW1teM) ...
