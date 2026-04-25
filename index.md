---
title: Smartwatch IoT cu monitorizare de sănătate
---

# Smartwatch IoT cu monitorizare de sănătate

**Student:** Mihai Alexandru Vătafu
**Link Repository:** [https://github.com/UPB-PMRust-Students/fils-project-2026-Alex22-ai](https://github.com/UPB-PMRust-Students/fils-project-2026-Alex22-ai)

## 1. Descriere Proiect
Proiectul vizează realizarea unui smartwatch de tip IoT bazat pe microcontrolerul RP2040 (Raspberry Pi Pico W), utilizând limbajul Rust și framework-ul Embassy. 

## 2. Hardware Design
* **Microcontroler:** Raspberry Pi Pico W
* **Display:** Ecran circular IPS 1.28 inch (Driver GC9A01)
* **Baterie:** Li-Po 3.7V 250mAh
* **Incarcare:** Modul TP4056 USB-C
* **Backup RTC:** Baterii Duracell CR2032
* **Componente:** Set rezistente si butoane SMD

## 3. Software Design
Codul este bazat pe framework-ul asincron **Embassy**, gestionând eficient perifericele SPI și I2C.

## 4. Progres și Implementare
- [x] Achiziție componente hardware (Baterie, Display, Rezistențe)
- [x] Configurare mediu de dezvoltare (WSL2/Rust)
- [ ] Implementare driver afișaj circular
