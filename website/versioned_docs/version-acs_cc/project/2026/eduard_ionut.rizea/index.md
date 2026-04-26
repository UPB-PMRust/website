# Quadcopter
A radio controled drone capable of pith roll and yaw.

:::info 

**Author**: Rizea Eduard-Ionut \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-Edy1298

:::

<!-- do not delete the \ after your name -->

## Description

A quadcopter made using a NUCLEO board that acts as a flight controller. Based on the inputs it receives from the radio reciever and gyroscope the Nucleo board processes them and sends coresponding signals to each of the 4 ESC's in order to keep the drone stable. 
Everything will be built on a 3D printed body and powered by a Lipo 3s battery.

## Motivation

I believe that it would be a fun challenging experience and I find drone building quite interesting.

## Architecture 

The system starts with the receiver, which sends pilot commands to the Nucleo  flight controller. At the same time, the gyroscope provides live motion data.

The Nucleo board processes both the pilot inputs and sensor data. It compares the desired state with the actual state and calculates the necessary corrections to maintain stability or execute movement.

These corrections are sent as signals to the four ESCs, which regulate the speed of each motor. By adjusting motor speeds independently, the system controls lift and enables movement in all directions.

The 3S LiPo battery supplies power to the entire system.

![Architecture Diagram](images/drone.svg)

## Log

<!-- write your progress here every week -->
### Week 20 - 26 April
Got approval and researched the components. 
Ordered all of the components.

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The project uses the Nucleo board as a flight controller. It receives data from the radio receiver and gyroscope. And after processing the data, it sends signals to each of the ESCs that are connected to the motors. 

### Schematics

<!-- Place your KiCAD or similar schematics here in SVG format. -->

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/) | Main Controller | Lab Provided |
| [4 x A2212 1400kV Motors](https://www.drot.ro/platforma-arduino/1631-motor-outrunner-a2212-10t-fara-perii-1400-kv.html) | Motors | 36 RON |
| [4 X 30A ESC](https://sigmanortec.ro/Controller-Motor-ESC-30A-p139673260) | ESC | 47 RON | 
| [Lip Tattu 1550 mAh](https://www.lerato.ro/acumulator-lipo-tattu-cu-mufa-xt60-1550-mah-galben.html) | Battery | 120RON |
| [MPU6050](https://www.optimusdigital.ro/ro/senzori-senzori-inertiali/13611-modul-accelerometru-i-giroscop-cu-3-axe-mpu6050-cu-pini-lipiti.html?search_query=MPU6050&results=7) | Gyroscope | 15 RON |
| [Receiver FlySky](https://www.emag.ro/receptor-rc-flysky-fs-ia6b-ia6b-2-4g-6ch-ppm-pentru-transmitator-rc-fs-i6x-fs-i6s-i8-lrz20250306-4905/pd/DMN64Y3BM/) | Radio Receiver | 159 RON


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Hardware Abstraction Layer | Handling I2C, SPI, and PWM peripherals |    

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->
[Video Inspiration](https://m.youtube.com/watch?v=fQhsgUEnV2w)
