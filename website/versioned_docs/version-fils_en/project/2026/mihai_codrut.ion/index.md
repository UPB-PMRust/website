---
sidebar_position: 1
slug: /project/2026/mihai_codrut.ion
---

# Desk Pet - Robot Interactiv de Birou

**Student:** Ion Mihai-Codruț (1221EA)

:::tip

Acesta este un proiect realizat în cadrul cursului de Microprocesoare pentru anul universitar 2025-2026.

:::

## Descriere

Robotul "Desk Pet" este un companion interactiv de birou bazat pe Raspberry Pi Pico (RP2040) și programat în Rust. Proiectul implementează o Mașină de Stări Finite (FSM) pentru a gestiona diferite comportamente ale robotului în funcție de interacțiunile utilizatorului și mediul înconjurător.

### Caracteristici principale:
- **Interacțiune tactilă**: Folosește senzor TTP223 pentru detectarea "mângâierilor"
- **Sistem de hrănire**: Module RFID MFRC522 pentru identificarea "mâncării"
- **Mișcare autonomă**: Șasiu 2WD cu evitarea obstacolelor prin HC-SR04
- **Feedback vizual/audio**: LCD I2C 1602 și buzzer pentru comunicarea stării robotului
- **Control remote**: Modul Bluetooth HC-05 pentru telecomandă

---

## Motivație

### De ce am ales acest proiect?

Am ales să construiesc acest robot deoarece îmbină perfect:
1. **Aspectul educativ**: Învățarea limbajului Rust într-un context embedded
2. **Design interactiv**: Crearea unui obiect cu "personalitate" prin FSM
3. **Provocare tehnică**: Integrarea mai multor senzori și actuatori pe RP2040
4. **Utilitate practică**: Un companion de birou care "învață" preferințele utilizatorului

### Relevanța pentru cursul de Microprocesoare

Proiectul demonstrează:
- Programare asincronă cu Embassy framework
- Gestionarea multiplelor periferice (I2C, SPI, UART, PWM, GPIO)
- Implementarea pattern-ului FSM în Rust
- Optimizarea consumului de memorie și putere de procesare

---

## Arhitectură

### Diagrama bloc a sistemului
┌─────────────────────────────────────────────────────┐
│              Raspberry Pi Pico (RP2040)              │
│                                                      │
│  ┌────────────────────────────────────────────┐    │
│  │         Embassy Async Runtime              │    │
│  │  ┌──────────────────────────────────┐      │    │
│  │  │    Finite State Machine (FSM)    │      │    │
│  │  │  • IDLE                          │      │    │
│  │  │  • FERICIT (Happy)               │      │    │
│  │  │  • HRĂNIT (Fed)                  │      │    │
│  │  │  • TRIST (Sad)                   │      │    │
│  │  │  • TELECOMANDĂ (Remote Control)  │      │    │
│  │  └──────────────────────────────────┘      │    │
│  └────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────┘
│           │           │           │
┌────┴────┐ ┌───┴───┐  ┌───┴────┐ ┌───┴────┐
│ Senzori │ │Display│  │ Motoare│ │Bluetooth│
└─────────┘ └───────┘  └────────┘ └────────┘

### Stări FSM și tranziții

| Stare | Condiții de Intrare | Comportament | Tranziție Următoare |
|-------|---------------------|--------------|----------------------|
| **IDLE** | Power-on / Timeout | LED slow blink, display "Zzz..." | Touch → FERICIT<br/>RFID → HRĂNIT<br/>No interaction (30s) → TRIST |
| **FERICIT** | Touch detectat | LED verde rapid, afișare ":)", mișcare aleatorie | Timeout (15s) → IDLE<br/>RFID → HRĂNIT |
| **HRĂNIT** | RFID tag citit | Buzzer melody, afișare "Yum!", animație LED | Timeout (10s) → IDLE |
| **TRIST** | Nicio interacțiune prelungită | LED roșu, afișare ":(", sunet trist | Touch → FERICIT<br/>RFID → HRĂNIT |
| **TELECOMANDĂ** | Comandă Bluetooth primită | Control manual motoare | Comandă STOP → IDLE |

---

## Design Hardware

### Componente Utilizate

| Componentă | Model/Specificație | Cantitate | Scop |
|------------|-------------------|-----------|------|
| Microcontroler | Raspberry Pi Pico (RP2040) | 1 | Procesare centrală |
| Șasiu Robot | 2WD Car Chassis Kit | 1 | Platformă mobilă |
| Driver Motoare | L298N Dual H-Bridge | 1 | Controlul motoarelor DC |
| Display | LCD 1602 I2C (Albastru) | 1 | Afișare stare/mesaje |
| Senzor RFID | MFRC522 (13.56 MHz) | 1 | Identificare "hrană" |
| Senzor Touch | TTP223 Capacitiv | 1 | Detectare "mângâiere" |
| Senzor Distanță | HC-SR04 Ultrasonic | 1 | Evitare obstacole |
| Modul Bluetooth | HC-05 UART | 1 | Control remote |
| Buzzer | Buzzer Activ 5V | 1 | Feedback audio |
| LED-uri | LED-uri roșu/verde/galben 5mm | 3 | Indicare stare |
| Alimentare | Suport baterii 4xAA (6V) | 1 | Sursă motoare |
| Regulator Tensiune | AMS1117-3.3V | 1 | Alimentare Pico |
| Rezistoare | 220Ω, 1kΩ, 10kΩ | - | Limitare curent LED, pull-up |
| Breadboard | 400 puncte | 1 | Prototipare |
| Cabluri Jumper | M-M, M-F | 40 | Interconectare |

### Schema de Conexiuni (Pinout)
Raspberry Pi Pico (RP2040)
┌─────────────────────────┐
│                         │
│  GP0  ────── Motor1 ENA (PWM) [L298N]
│  GP1  ────── Motor1 IN1 [L298N]
│  GP2  ────── Motor1 IN2 [L298N]
│  GP3  ────── Motor2 ENB (PWM) [L298N]
│  GP4  ────── Motor2 IN3 [L298N]
│  GP5  ────── Motor2 IN4 [L298N]
│                         │
│  GP6  ────── HC-SR04 Trigger
│  GP7  ────── HC-SR04 Echo
│                         │
│  GP8  ────── UART1 TX (Bluetooth HC-05 RX)
│  GP9  ────── UART1 RX (Bluetooth HC-05 TX)
│                         │
│  GP10 ────── TTP223 OUT (Touch)
│                         │
│  GP12 ────── SPI1 MISO [MFRC522 MISO]
│  GP13 ────── SPI1 CS   [MFRC522 SDA]
│  GP14 ────── SPI1 SCK  [MFRC522 SCK]
│  GP15 ────── SPI1 MOSI [MFRC522 MOSI]
│  GP16 ────── MFRC522 RST
│                         │
│  GP18 ────── I2C1 SDA  [LCD 1602]
│  GP19 ────── I2C1 SCL  [LCD 1602]
│                         │
│  GP20 ────── Buzzer (PWM)
│  GP21 ────── LED RGB (WS2812) sau LED simplu
│                         │
│  3V3  ────── Alimentare logică (senzori)
│  VSYS ────── Alimentare de la regulator 3.3V
│  GND  ────── Masă comună
│                         │
└─────────────────────────┘
Notă: L298N se alimentează separat de la baterii 6V
### Considerații Alimentare

- **Logică (3.3V)**: Pico, senzori RFID/Touch/Ultrasonic, LCD
- **Motoare (6V)**: Alimentare separată prin L298N din baterii 4xAA
- **Izolare**: Condensatori de decuplare 100nF pe liniile VCC ale senzorilor
- **Protecție**: Diode flyback pe motoare pentru spike-uri de tensiune

### Schema Fizică (Layout)
[HC-SR04]       [LCD Display]
    │                │
┌───┴────────────────┴────┐
│   Breadboard (frontal)   │
│  [RFID Reader]           │
│  [TTP223 Touch]          │
└─────────────┬────────────┘
              │
     ┌────────┴────────┐
     │  Raspberry Pi   │
     │      Pico       │
     └────────┬────────┘
              │
     ┌────────┴────────┐
     │   L298N Driver  │
     └────┬──────┬─────┘
          │      │
     [Motor1] [Motor2]
          │      │
     ┌────┴──────┴─────┐
     │   2WD Chassis    │
     │   [Baterii 4xAA] │
     └──────────────────┘
---

## Design Software

### Arhitectura Rust Embassy

```rust
// Structura principală a proiectului
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::{Duration, Timer};

// Stările FSM
#[derive(Debug, Clone, Copy, PartialEq)]
enum RobotState {
    Idle,
    Fericit,
    Hranit,
    Trist,
    Telecomanda,
}

// Task-uri asincrone pentru fiecare modul
#[embassy_executor::task]
async fn touch_sensor_task(/* parametri */) { /* ... */ }

#[embassy_executor::task]
async fn rfid_reader_task(/* parametri */) { /* ... */ }

#[embassy_executor::task]
async fn ultrasonic_task(/* parametri */) { /* ... */ }

#[embassy_executor::task]
async fn motor_control_task(/* parametri */) { /* ... */ }

#[embassy_executor::task]
async fn display_task(/* parametri */) { /* ... */ }

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Inițializare periferice
    // Spawn task-uri
    // Loop principal FSM
}
```

### Biblioteci Rust (Cargo.toml)

```toml
[dependencies]
embassy-rp = { version = "0.2", features = ["time-driver"] }
embassy-executor = { version = "0.6", features = ["arch-cortex-m", "executor-thread"] }
embassy-time = "0.3"
embedded-hal = "1.0"
embedded-hal-async = "1.0"
mfrc522 = "0.6"
hd44780-driver = "0.4"
static_cell = "2.0"
panic-probe = { version = "0.3", features = ["print-defmt"] }
defmt = "0.3"
defmt-rtt = "0.4"
```

### Logica FSM - Pseudocod
LOOP principal:
match stare_curenta {
IDLE:
IF touch_detectat() THEN
stare_curenta = FERICIT
porneste_timer(15s)
ELSE IF rfid_detectat() THEN
stare_curenta = HRANIT
porneste_timer(10s)
ELSE IF timer_idle > 30s THEN
stare_curenta = TRIST

FERICIT:
        afiseaza_lcd(":)")
        led_verde_blink_rapid()
        IF timer_expirat() THEN
            stare_curenta = IDLE
    
    HRANIT:
        canta_melodie()
        afiseaza_lcd("Yum!")
        IF timer_expirat() THEN
            stare_curenta = IDLE
    
    TRIST:
        afiseaza_lcd(":(")
        led_rosu_on()
        IF touch_detectat() OR rfid_detectat() THEN
            stare_curenta = FERICIT
    
    TELECOMANDA:
        controleaza_motoare(comanda_bluetooth)
        IF comanda == STOP THEN
            stare_curenta = IDLE
}

// Task-uri de fundal
verifica_obstacole_async()
actualizeaza_display_async()
### Gestionarea Memoriei

- Folosire `static_cell` pentru resurse partajate între task-uri
- Evitarea alocărilor dinamice (no `alloc`)
- Buffer-e fixe pentru UART/SPI/I2C
- Stack sizing conform `memory.x`

---

## Jurnal de Progres

### Săptămâna 1 (31 Mar - 6 Apr 2026)
- [x] Definirea specificației proiectului
- [x] Achiziționarea componentelor hardware
- [x] Configurare mediu dezvoltare Rust (probe-rs, flip-link)
- [ ] Testare blink LED pe Pico

### Săptămâna 2 (7 - 13 Apr 2026)
- [ ] Implementare driver I2C pentru LCD 1602
- [ ] Test senzor TTP223 (GPIO interrupt)
- [ ] Schematic hardware în Fritzing/KiCad

### Săptămâna 3 (14 - 20 Apr 2026)
- [ ] Implementare SPI pentru MFRC522
- [ ] Testare citire tag-uri RFID
- [ ] Integrare modul Bluetooth HC-05 (UART)

### Săptămâna 4 (21 - 27 Apr 2026)
- [ ] Control motoare prin L298N (PWM)
- [ ] Testare HC-SR04 (distanță ultrasonică)
- [ ] Asamblare mecanică șasiu

### Săptămâna 5 (28 Apr - 4 Mai 2026)
- [ ] Implementare FSM în Rust
- [ ] Integrare toate modulele hardware
- [ ] Debugging și optimizări

### Săptămâna 6 (5 - 11 Mai 2026)
- [ ] Testare finală sistem complet
- [ ] Documentație finalizată
- [ ] Video demonstrație proiect

---

## BOM (Bill of Materials)

### Componente Principale

| Nr. | Componentă | Specificație | Cantitate | Preț Unitar (RON) | Total (RON) | Link Furnizor |
|-----|-----------|--------------|-----------|-------------------|-------------|---------------|
| 1 | Raspberry Pi Pico | RP2040, 264KB RAM | 1 | 25 | 25 | [ardushop.ro](https://www.ardushop.ro) |
| 2 | 2WD Robot Chassis | Kit complet cu motoare | 1 | 35 | 35 | [robofun.ro](https://www.robofun.ro) |
| 3 | L298N Motor Driver | Dual H-Bridge | 1 | 12 | 12 | [optimusdigital.ro](https://www.optimusdigital.ro) |
| 4 | LCD 1602 I2C | Display 16x2 albastru | 1 | 18 | 18 | [ardushop.ro](https://www.ardushop.ro) |
| 5 | MFRC522 RFID | 13.56 MHz + 2 tag-uri | 1 | 15 | 15 | [optimusdigital.ro](https://www.optimusdigital.ro) |
| 6 | TTP223 Touch Sensor | Capacitiv digital | 1 | 5 | 5 | [sigmanortec.ro](https://www.sigmanortec.ro) |
| 7 | HC-SR04 Ultrasonic | Senzor distanță 2-400cm | 1 | 8 | 8 | [robofun.ro](https://www.robofun.ro) |
| 8 | HC-05 Bluetooth | Modul UART | 1 | 20 | 20 | [ardushop.ro](https://www.ardushop.ro) |
| 9 | Buzzer Activ 5V | 12mm | 1 | 3 | 3 | [optimusdigital.ro](https://www.optimusdigital.ro) |
| 10 | LED-uri | Roșu/Verde/Galben 5mm | 3 | 1 | 3 | [optimusdigital.ro](https://www.optimusdigital.ro) |
| 11 | Suport Baterii 4xAA | Cu switch ON/OFF | 1 | 4 | 4 | [sigmanortec.ro](https://www.sigmanortec.ro) |
| 12 | Regulator AMS1117-3.3V | SOT-223 | 1 | 2 | 2 | [tme.eu](https://www.tme.eu) |
| 13 | Breadboard 400 puncte | - | 1 | 6 | 6 | [optimusdigital.ro](https://www.optimusdigital.ro) |
| 14 | Cabluri Jumper M-M | 40 buc | 1 set | 8 | 8 | [optimusdigital.ro](https://www.optimusdigital.ro) |
| 15 | Cabluri Jumper M-F | 40 buc | 1 set | 8 | 8 | [optimusdigital.ro](https://www.optimusdigital.ro) |

### Componente Auxiliare

| Nr. | Componentă | Specificație | Cantitate | Preț Unitar (RON) | Total (RON) |
|-----|-----------|--------------|-----------|-------------------|-------------|
| 16 | Rezistoare 220Ω | 1/4W | 5 | 0.1 | 0.5 |
| 17 | Rezistoare 1kΩ | 1/4W | 5 | 0.1 | 0.5 |
| 18 | Rezistoare 10kΩ | 1/4W | 5 | 0.1 | 0.5 |
| 19 | Condensatori 100nF | Ceramic | 5 | 0.5 | 2.5 |
| 20 | Diode 1N4007 | Flyback motoare | 2 | 0.5 | 1 |
| 21 | Baterii AA | Alcaline | 4 | 3 | 12 |
| 22 | Cablu Micro-USB | Pentru programare Pico | 1 | 5 | 5 |

### **TOTAL COST: ~194 RON**

---

## Resurse și Referințe

### Documentație Oficială
- [Raspberry Pi Pico Datasheet](https://datasheets.raspberrypi.com/pico/pico-datasheet.pdf)
- [Embassy Framework Docs](https://embassy.dev/)
- [MFRC522 RFID Datasheet](https://www.nxp.com/docs/en/data-sheet/MFRC522.pdf)
- [L298N Motor Driver Guide](https://www.st.com/resource/en/datasheet/l298.pdf)

### Tutorial-uri Relevante
- [Rust on Raspberry Pi Pico](https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry)
- [Embassy RP2040 Examples](https://github.com/embassy-rs/embassy/tree/main/examples/rp)
- [FSM Design Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/)

### Cod Sursă
- Repository GitHub: `https://github.com/UPB-PMRust/proiect-mihai_codrut.ion` *(de completat)*

---

## Video Demonstrație

*(Link YouTube de adăugat după finalizare)*

---

## Concluzii

*(Secțiune de completat la finalul proiectului - lecții învățate, dificultăți întâmpinate, îmbunătățiri viitoare)*

---

**Ultima actualizare:** 15 Aprilie 2026  
**Status:** 🟡 În Dezvoltare

