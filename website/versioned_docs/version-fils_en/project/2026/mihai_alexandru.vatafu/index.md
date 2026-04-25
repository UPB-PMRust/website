# Smartwatch IoT cu monitorizare de sănătate în Rust

**Student:** Mihai Alexandru Vătafu  
**Curs:** Proiectarea Microprocesoarelor în Rust  
**Repository Proiect:** [https://classroom.github.com/a/sVB0pKHF](https://classroom.github.com/a/sVB0pKHF)

## 1. Descriere Proiect
Acest proiect vizează realizarea unui smartwatch de tip IoT, construit de la zero, folosind limbajul Rust. Dispozitivul utilizează un ecran circular IPS pentru interfața grafică și senzori pentru monitorizarea parametrilor vitali. Obiectivul este demonstrarea capabilităților asincrone ale framework-ului **Embassy** în gestionarea eficientă a energiei și a perifericelor.

## 2. Hardware Design
Proiectul integrează componente de joasă putere pentru a asigura o autonomie ridicată.
![Smartwatch Components](./images2/C:\Users\Dell\Downloads\duracell-ezgif.com-png-to-webp-converter.jpg)
![Smartwatch Components](./images2/C:\Users\Dell\Downloads\vvvv-ezgif.com-png-to-webp-converter.jpg)
![Smartwatch Components](./images2/C:\Users\Dell\Downloads\ddddd-ezgif.com-png-to-webp-converter.jpg)
![Smartwatch Components](./images2/C:\Users\Dell\Downloads\ccccc-ezgif.com-png-to-webp-converter.jpg)
![Smartwatch Components](./images2/C:\Users\Dell\Downloads\bbb-ezgif.com-png-to-webp-converter.jpg)
![Smartwatch Components](./images2/C:\Users\Dell\Downloads\aaaaaaa-ezgif.com-png-to-webp-converter.jpg)
### Componente Principale:
* **Microcontroler:** Raspberry Pi Pico W (RP2040).
* **Display:** Ecran circular IPS 1.28 inch (Driver GC9A01, SPI).
* **Timp Real (RTC):** Modul DS3231 pentru precizie temporală.
* **Energie:** Baterie Li-Po 250mAh cu încărcător TP4056 USB-C.
* **Stabilizare:** Regulator LDO HT7333-A (3.3V).
* **Interacțiune:** Butoane SMD cu rezistențe de pull-down (10kΩ).

## 3. Software Design
Codul este bazat pe **Embassy**, framework-ul asincron pentru Rust embedded.

### Caracteristici Software:
* **Multitasking:** Gestionarea ecranului și a butoanelor prin `tasks` asincrone.
* **Power Management:** Utilizarea modurilor de sleep pentru conservarea bateriei.
* **Grafică:** Randare watchface circular folosind `embedded-graphics`.

## 4. Progres și Implementare
- [x] Achiziție componente și curele silicon 20mm.
- [x] Configurare mediu de dezvoltare (Ubuntu/Rust).
- [ ] Implementare driver afișaj.
- [ ] Asamblare mecanică.
