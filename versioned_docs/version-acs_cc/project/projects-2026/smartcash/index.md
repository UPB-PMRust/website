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

The following diagram shows the main connections between the components of the SmartCash system.

```mermaid
flowchart TD
    MCU[Microcontroller<br/>STM32 / Raspberry Pi Pico / Arduino]

    SCAN[Barcode Scanner<br/>USB / UART]
    DISPLAY[LCD / TFT Display<br/>SPI / I2C]
    BUZZER[Passive Buzzer<br/>PWM]
    LEDS[Status LEDs<br/>GPIO]
    BUTTONS[Control Buttons<br/>GPIO]
    PC[PC / Laptop<br/>Serial Monitor / Database]

    SCAN -->|Product code| MCU
    MCU -->|Product name, price, total| DISPLAY
    MCU -->|Alert signal| BUZZER
    MCU -->|Visual status| LEDS
    BUTTONS -->|Confirm / Reset / Alert stop| MCU
    MCU <-->|Transaction logs / product database| PC


---

## Bill of Materials

| Device | Usage | Price |
|------|------|------|
| Microcontroller | Main processing unit | ~50 RON |
| Barcode Scanner | Product scanning | ~70 RON |
| Display | User interface | ~80 RON |
| Buzzer | Audio alerts | ~10 RON |
| LEDs | Visual alerts | ~10 RON |

---

## Software

| Library | Description | Usage |
|--------|------------|------|
| embedded-hal | Hardware abstraction | Communication with peripherals |
| display driver | Display control | UI rendering |
| serial library | UART communication | Barcode input |

---

## Links

