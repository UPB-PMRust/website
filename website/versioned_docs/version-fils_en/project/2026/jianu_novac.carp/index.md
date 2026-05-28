# RustInjector

RC car that can jump on desks and inject USBs into laptops.

**Author**: Carp Jianu-Novac-Mihail
**GitHub Project Link**:  [https://github.com/UPB-PMRust-Students/fils-project-2026-novacLearnsCyber](https://github.com/UPB-PMRust-Students/fils-project-2026-novacLearnsCyber)

## Description

An RC car controlled via Wi-Fi from a mobile phone, designed to jump approximately 75cm (average desk height) and land safely to perform a physical USB injection attack on a target laptop. The design may resemble a monster truck toy to "camouflage" better.

## Motivation

Initially, I wanted to build a cybersecurity-oriented project, because I am quite passionate about the subject, but I also wanted it to be flashy. My first ideas were a bizarre hardware token or a Wi-Fi IDS monitor, but they weren't exciting enough. Then I remembered the Jumper from Watch Dogs 2 and decided to build something inspired by it.

## Architecture

Add here the schematics with the architecture of your project. Make sure to include:

* what the main components are (architecture components, not hardware components)
* how they connect to each other

## Log

### Week 20 - 25 April

* Ordered 99% of all the components online. I need to find a place that sells industrial springs strong enough to jump a 1 kg RC car into the air.
* Brainstormed ideas for the jump mechanism.
* A few rough sketches on paper for now.

### Week 5 - 11 May
- learned fusion 360 , finished 90% percent of the 3d render, only the injection mechanism and the camera cage are unfinished.
- printed the base layer of the car, dropped it on the ground, it broke , 
- decide to print it again and do some modification to add some resistance. 
- started writing the code for the control of the motors 
- soldered my picos

* Learned Fusion 360, finished 90% of the 3D render; only the injection mechanism and the camera cage are unfinished.
* Printed the base layer of the car, dropped it on the ground, and it broke.
* Decided to print it again and do some modifications to add some resistance.
* Started writing the code to control the motors.
* Soldered my Picos.

### Week 12 - 18 May

* Switched to Pico 2 W.
* Did a lot of 3D modeling; I wasted too much time on this part.
* Tested the powerful motor that will compress the spring, and it works.
* Got hardware materials like metal rods and knock-off Wagos for the power system.

### Week 19 - 25 May

* Designed the power system.
* Tested and implemented full RC control of the motors and servos using the web server.
* 3D printed the base and second level of the car.
* Finally finished all the 3D modeling.
* Random side-quest: broke the JGA25-370 motor (the spring motor), but repaired it.
* Tested a cardboard box prototype.
* Achieved remote control of the car.
* 3D printed the camera case.
* Designed the injector mechanism.

### Week 26 - 30 May

* Final stretch.
* Finished 3D modeling the jumping mechanism; it is ready to be printed.
* Broke a pole on the first level of the car; I don't care, it still works.
* Fried my main driver and step-up module; I can't replace them now, it's too late.
* Implemented the MPU-6050 sensor.
* On the last day before the PM-fair, I achieved the performance of short-circuiting 4 components: the step-up (which was crucial, now bye-bye motor), the L298N driver (bye-bye motor control), the VL53L0X (I found another, but we will find out if I can get my hands on it until tomorrow, it's 11:47 as I am logging this), and a small servo that didn't work, but a friend helped me and gave me another.

Conclusion of the project: I spent way too much time 3D designing the components and chose a way too complex theme from a mechanical point of view. The whole project is barely working from a technical point of view. If I were to start over, I would choose an easier idea. I learned a lot of new stuff, at least.

## Hardware

Simple RC car, uses a simple hardware setup, keeping in mind the shocks it will suffer from jumping.
Control: Raspberry Pi Pico 2 W, while the ESP32-CAM provides live video.
Jumping mech: High-torque JGA25 motor compresses a steel spring, released by the MG996R servo latch.
Drive: 4x TT motors with custom 3D-printed TPU wheels for traction and shock absorption.
Power: Gens Ace LiPo Battery regulated by an XL4015 5A Buck converter to keep the servos and logic rails stable during the jumps.
Sensing: VL53L0X measuring altitude, and MPU6050 monitors stability and landing orientation.
Injector: MG90S servo deploys the USB.

### Schematics

### Bill of Materials

| Device | Usage | Price |
| --- | --- | --- |
| Raspberry Pi Pico 2 W | The microcontroller | 35 RON |
| Dual Motor Driver L9110S | Driver for 4WD Drive Motors | 7.98 RON |
| SERVO MG996R (13kg) | Injector Latch | 17.68 RON |
| ESP32-CAM + OV2640 | Video Streaming, Web Control Interface | 70.51 RON |
| Motor JGA25-370 20RPM | Jumping Mechanism Motor (High Torque) | 107.00 RON |
| 4x TT Motors + Wheels | 4WD Locomotion System | 31.86 RON |
| SERVO MG90S Metal | USB Injection Arm Actuator | 27.43 RON |
| XL4015 Step-Down 5A | 5V Power Regulation for Servos & Logic | 14.13 RON |
| L298N Motor Driver | High-Current Driver for Jump Motor | 15.00 RON |
| MPU6050 IMU | Gyroscope & Accelerometer | 14.68 RON |
| VL53L0X ToF Sensor | Laser Distance Measurement | 25.11 RON |
| Gens Ace LiPo Battery | Main Power Source | 120 RON |
| IMAX B3 Charger | LiPo Battery Balancing Charger | 40.00 RON |
| Heavy-Duty Spring | Energy Storage for 1.20m Jump | 20.00 RON |
| PCB Prototype Kit | Custom Circuit Boards for Power Hub | 28.31 RON |

## Software

| Crate | Description | Usage |
| --- | --- | --- |
| **embassy-rp** | HAL (Hardware Abstraction Layer) for RP2350 (Pico 2W) | Controls physical pins: GPIO for the MGR98 motor, PWM for servos and the L298N motor driver, and the I2C interface for the sensors. |
| **embassy-executor** | Async task executor designed for microcontrollers | Runs multiple tasks concurrently in the background, like managing the Wi-Fi chip and running the web server. |
| **embassy-time** | Async timers and delays | Allows the code to pause (e.g., interval between sensor readings) without blocking the entire processor. |
| **embassy-sync** | Async synchronization primitives (Mutex, Channels) | Helps tasks communicate safely and share data without causing race conditions or conflicts. |
| **embassy-futures** | Utility functions for working with Futures | Combines async actions, such as waiting for web requests and reading sensors at the same time. |
| **embassy-embedded-hal** | Async wrapper for embedded-hal traits | Provides a standard interface for async hardware drivers (used under the hood by sensors and network drivers). |
| **embassy-net** | Async TCP/IP networking stack | Handles network communication, gets an IP address via DHCP, and runs the HTTP server for the web interface. |
| **cyw43** | Driver for the CYW43439 Wi-Fi chip | Manages the wireless connection, connects to the access point, and handles Wi-Fi data packets. |
| **cyw43-pio** | PIO-based interface for the cyw43 driver | Uses the Pico's programmable I/O (PIO) blocks to talk to the Wi-Fi chip at high speeds. |
| **mipidsi** | MIPI DSI / SPI display driver | Included in the project to control and initialize the color screen if physical display output is needed. |
| **display-interface** | Generic interface abstraction for displays | Provides a common protocol layer for display drivers. |
| **display-interface-spi** | SPI transport layer for displays | Sends graphics commands and pixel data to the display over the SPI bus. |
| **embedded-graphics** | 2D graphics library for embedded systems | Used to draw shapes, text, and images on the screen. |
| **heapless** | Fixed-size data structures (String, Vec) allocated on the stack | Lets you format dynamic strings (like the sensor data inside the HTML page) without needing a heap allocator. |
| **static_cell** | Safe runtime initialization of static variables | Safely initializes global statics needed for the Wi-Fi state and network stack resources. |
| **defmt** | Highly efficient logging framework | Prints debug messages to the console with minimal memory footprint and overhead compared to `println!`. |
| **defmt-rtt** | RTT (Real-Time Transfer) transport for defmt | Channels the logging output from the Pico directly to your PC console via the debugger. |
| **panic-probe** | Panic handler for debugger probes | If the firmware crashes, it catches the panic and prints the exact source file and line number to the console. |
| **cortex-m** | Low-level ARM Cortex-M hardware access | Provides register access and low-level CPU control (like sleep states and assembly delays). |
| **cortex-m-rt** | Startup runtime for ARM Cortex-M | Sets up the interrupt vector table, initializes RAM at boot, and jumps to the main function. |

## Links

1. [Jumper in game](https://www.youtube.com/watch?v=xSfsmZ3VV_s)
2. [Jumping mechanism inspiration](https://www.youtube.com/watch?v=RuMLHJW1teM)