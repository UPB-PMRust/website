# SmartCash – Intelligent Cash Register System

A smart cash register with product scanning, real-time alerts, and business monitoring features.

---

## info

- **Author:** David Adrian-Mario  
- **Group:** 331 CC  
- **GitHub Project Link:** 

---

## Description

SmartCash is an intelligent cash register system designed to modernize small retail operations by integrating product scanning, real-time alerts, and smart monitoring features.

The system runs on a microcontroller platform and connects to a barcode scanner, display interface, and alert modules. It allows fast product identification, automatic price calculation, and detection of unusual events such as suspicious transactions, missing products, or system errors.

---

## Motivation

I chose this project because I am familiar with how markets and small shops operate, and I noticed that many systems are outdated and rely heavily on manual work. This leads to mistakes, slow service, and lack of control.

The goal is to build a smarter cash register that improves efficiency, reduces errors, and provides real-time alerts.

---

## Architecture

The system is composed of the following main components:

- **Main Controller** – handles product processing, scanning logic, and alerts  
- **Scanning Module** – reads barcode data and sends it to the controller  
- **Display Module** – shows product information and total price  
- **Alert Module** – buzzer and LEDs for notifications  

All components communicate through the microcontroller, which acts as the central processing unit.

---

## Log

### Week 5 – 11 May
Defined the project idea and overall architecture.

### Week 12 – 18 May
Started hardware setup and tested barcode scanner and display.

### Week 19 – 25 May
Implemented scanning logic and alert system.

---

## Hardware

The project uses:
- Microcontroller (STM32 / Arduino / Raspberry Pi Pico)  
- Barcode scanner  
- LCD / TFT display  
- Buzzer  
- LEDs  
- Buttons  
- Breadboard and wires  

---

## Schematics
          +--------------------+
          |  Barcode Scanner   |
          |    (USB / UART)    |
          +---------+----------+
                    |
                    v
          +--------------------+
          |                    |
          |  Microcontroller   |
          |  (STM32 / Pico)    |
          |                    |
          +----+----+----+-----+
               |    |    |
               |    |    |
               v    v    v

     +-----------+  +-----------+  +-----------+
     |  Display  |  |  Buzzer   |  |   LEDs    |
     | (LCD/TFT) |  |  (PWM)    |  |  (GPIO)   |
     +-----------+  +-----------+  +-----------+

               ^
               |
       +-------+-------+
       |    Buttons    |
       |    (GPIO)     |
       +---------------+

               ^
               |
       +---------------+
       |      PC       |
       | (Logs / DB)   |
       +---------------+

---

## Bill of Materials

| Device | Usage | Price |
|------|------|------|
| Microcontroller (STM32 / Pico / Arduino) | Main processing unit | ~50 RON |
| Barcode Scanner | Product scanning | ~70 RON |
| LCD / TFT Display | User interface | ~80 RON |
| Passive Buzzer | Audio alerts | ~10 RON |
| LEDs | Visual alerts | ~10 RON |
| Buttons | User interaction | ~5 RON |
| Jumper Wires (M-F, F-F) | Connections between components | ~20 RON |
| Breadboard | Prototyping connections | ~10 RON |
| USB Cable | Power and communication with PC | ~15 RON |

---

## Software

| Library | Description | Usage |
|--------|------------|------|
| embedded-hal | Hardware abstraction | Communication with peripherals |
| display driver | Display control | UI rendering |
| serial library | UART communication | Barcode input |

---

## Links

