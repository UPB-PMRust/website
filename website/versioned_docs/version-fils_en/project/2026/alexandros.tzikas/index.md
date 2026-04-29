# RustParking

An embedded project that simulates a real-life parking, using a barrier to allow cars to enter and exit a parking lot

:::info

*Author:* Tzikas Alexandros \

*GitHub Project Link:* https://github.com/UPB-PMRust-Students/fils-project-2026-AlexTzikas

:::

## Description
This project is designed to be a model of a real-life parking; it uses two ultrasonic sensor to detect cars approaching a barrier (operated by a SG90 servomotor), both entering and exiting the parking, opening the barrier for exiting cars and opening the barrier for entering cars if they are authorized to enter, witch is ascertained by an ESP32-CAM. If the parking is full, the barrier will not open for any cars attempting to enter. The computations and processing are all done by the microcontroller (STM32-NUCLEO-U545RE-Q).

## Motivation
I chose this project because it is a real-life application witch can help me learn rust better, as it help me understand subjects such as real-time sensor integration; it was also suggested to me by a coleague as beeing a reliable project idea.

## Architecture

![System Architecture](./maarchbun.svg)

Input: 

*HC-SRO4* ultrasonic sensor will receive signals to see the ditance of a car from the sensor and will send the signals through a GPIO pin

*ESP32-CAM* will detect authorized vehicles trying to enter and send a signal when one is seen

Processing:

*STM32-NUCLEO-U545RE-Q* will do computations and signal processing and send data to the laptop via USB.

Output:

*SG90 servomotor* will receive signals to open/close a barrier when a car enters/exits

*LED* will be used to see that the system works properly

## Log

### Week 13 April - 19 April

I got the components (except the ESP32-CAM) and set up the start of my project (memory.x,cargo.toml,Config.toml,build.rs) and also created the GitHub page for the project

### Week 20 April - 26 April

I got the rest of the components (the ESP32-CAM, some wires and a battery holder) and started assembling the project, starting with the servomotor; I wired the servomotor and wrote some code to test it, worked mostly fine, but each time would stop working after a while; suspected it was because of insuficient power so I decided to get some batteries for power. Afterwards wired the first sensor and started testing; had errors, as sensor would get stuck on high - did a lot of debugging to narrow down the problem (this log unfinished)

## Hardware

picture coming soon

*STM32-NUCLEO-U545RE-Q* will do computatiosn, signals processing and transmit data to the laptop

*HC-SRO4 ultrasonic sensor* – a sensor witch receives and transmis signals to measure distance

*SG90 servo motor* will open/close the barrier

*ESP32-CAM* will scan incoming vehicles and check for authorization

*LED* will provide visual feedback during testing

## Schematics

coming soon

## Bill of Materials

| Device                                                                                                           | Usage                                     | Price                     
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------- | ------------------------------------
| [STM32-NUCLEO-U545RE-Q] | microcontroller | borrowed from politehnica
| [HC-SRO4 ultrasonic sensor x 2](https://www.emag.ro/set-2-senzori-distanta-ultrasonic-digital-3-3-5v-45x20x15mm-multicolor-9344435370736/pd/DCPK2H3BM/?ref=history-shopping_484549223_232871_1)          | measuring distance from barrier  | 36 Lei
| [SG90 servo motor x 4](https://www.emag.ro/set-servomotor-sg90-unghi-de-lucru-180-grade-4-bucati-3874783591898/pd/DLHDYTYBM/?ref=history-shopping_484549223_157633_1)   | moving the barrier       | 48 Lei
| [ESP32-CAM](https://www.emag.ro/microcontroler-esp32-cam-cu-ov2640-wi-fi-si-camera-bluetooth-5v-5904162804207/pd/DSTDNLMBM/?ref=history-shopping_485848746_38837_1) |   checks vehicles for authorization   | 67 Lei
| [Breadboard kit](https://www.emag.ro/kit-electronica-pentru-incepatori-cu-modul-esp8266-si-placa-d1-compatibil-cu-arduino-ideal-pentru-proiecte-diy-be000116/pd/D15YQF3BM/?ref=history-shopping_484549223_206277_1)         | Connecting components   | 124 Lei
| [female-male wires](https://www.emag.ro/10-x-fire-dupont-mama-tata-20cm-ai306-s459/pd/DZJ66JBBM/?ref=history-shopping_485848746_38837_2)   | connecting components   | 1 Leu
| [female-female wires](https://www.emag.ro/10-x-fire-dupont-mama-mama-10cm-ai310-s450/pd/DWF66JBBM/?ref=history-shopping_485848746_38837_3)   | connectign components   | 2 Lei
| [4xAA battery holder](https://www.emag.ro/suport-pentru-baterii-4xaa-r6-tensiune-6v-din-plastic-abs-fire-conexiune-10cm-32011303/pd/DMHWLB2BM/?ref=history-shopping_485848746_7099_1)   |  holding batteries for power  | 8 lei

## Software

| Library  | Description  | Usage   |
| ------------------------------------------------------|--------|---------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | async HAL for STM32 microcontrollers  | controls PWM(servo, sensor), UART (ESP32), GPIO  |
| [embassy-time](https://github.com/embassy-rs/embassy) |  timing and delay for embassy projects | used for delays, timing measurements, ultrasonic echo timing and servo contorl intervals |
| [embassy-executor](https://github.com/embassy-rs/embassy) | async task executor  | runs synchronous tasks, currently servomotor control and readinf of ultrasonic sensor  |
| [embassy-embedded-hal](https://github.com/rust-embedded/embedded-hal) | compatibility between embassy and embedded-hal | embedded hardware abstraction suport
| [embassy-sync](https://github.com/embassy-rs/embassy) | syncronization for async embedded programs | for sharing data safely between async tasks  |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | hardware abstraction for embedded rust | PWM traits like enablind PWM and setting duty cycle |
| [defmt](https://github.com/knurling-rs/defmt) | logging for embedded rust | print debugging information like sensor state, distances from sensor, system status |
| [defmt-rrt](https://github.com/knurling-rs/defmt) | rrt backend for defmt | send debug logs from STM to laptop |
| [panic-probe](https://github.com/knurlings-rs/probe-run) | panic handler | used to report problems during debuggging
| [cortex-m](https://github.com/rust-embedded/cortex-m) | low level support for microcontroller | acces to STM32 features
| [cortex-m-rt](https://github.com/rust-embedded/cortex-m) | runtime support for cortex-m microcontrollers | provides runtime support  |


## Links

coming soon
