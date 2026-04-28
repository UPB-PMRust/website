# SmartCash – Intelligent Cash Register System

A smart cash register with product scanning, real-time alerts, and business monitoring features.

---

## ℹ️ Info

- **Author:** David Adrian-Mario  
- **Group:** 331 CC  
- **Project Type:** Embedded System + Retail Automation  
- **GitHub Project Link:** (you can add later)

---

## 📖 Description

SmartCash is an intelligent cash register system designed to modernize small retail operations by integrating product scanning, real-time alerts, and smart monitoring features.

The system runs on a microcontroller platform and connects to a barcode scanner, display interface, and alert modules. It allows fast product identification, automatic price calculation, and detection of unusual events such as suspicious transactions, missing products, or system errors.

The device features a display interface for the cashier, a buzzer and LED alert system for notifications, and optional connectivity to a computer for data logging and analytics.

SmartCash is designed to improve efficiency, reduce human error, and provide better control over retail operations.

---

## 🎯 Motivation

As someone familiar with how markets and small shops operate, I noticed that many systems are outdated and rely heavily on manual work. This can lead to mistakes, slow service, and lack of control over sales and stock.

The goal of this project is to create a smarter and more automated cash register that:
- speeds up transactions  
- reduces errors  
- provides alerts for important situations  
- improves overall shop management  

Additionally, the project helps explore embedded systems, hardware integration, and real-world applications of programming.

---

## 🏗️ Architecture

### Main Controller
A microcontroller (STM32 / Arduino / Raspberry Pi Pico) handles:
- product processing  
- scanning logic  
- alert system  
- communication between modules  

---

### Scanning Subsystem
- Barcode scanner (USB / UART)  
- Reads product codes  
- Sends data to microcontroller  
- Matches products from database  

---

### Display Subsystem
- LCD / TFT screen  
- Shows product, price, and total  

---

### Alert Subsystem
- Buzzer (audio alerts)  
- LEDs (visual alerts)  

Used for:
- invalid scan  
- suspicious activity  
- system errors  

---

## 📅 Log

### Week 5
Defined project idea and received approval. Designed the overall system architecture and selected components.

### Week 7
Started hardware setup. Tested barcode scanner and basic communication with the microcontroller. Began experimenting with display output.

### Week 8
Worked on documentation structure and project planning. Improved hardware connections and testing.

### Week 9
Implemented core functionality: product scanning, basic processing, and alert system logic.

### Week 10+
Planned improvements: UI enhancement, data logging, and optimization of system performance.

---

## 🔌 Hardware

- Microcontroller (STM32 / Arduino / Raspberry Pi Pico)  
- Barcode Scanner  
- Display (LCD / TFT)  
- Buzzer  
- LEDs  
- Buttons  
- Wires & Breadboard  

---

## 🚀 Future Improvements

- Inventory management system  
- Mobile app integration  
- Cloud synchronization  
- Receipt printing  
- Advanced fraud detection  
