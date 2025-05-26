# Sistem de Detectare Obiecte
A one line project description

:::info 

**Author**: Popescu Darius-Stefan \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-darius-afk

:::

## Description

Proiectul presupune constructia unui sistem de securitate care detecteaza miscarea si care afiseaza un mesaj cu informatii despre aceasta.
Voi folosi un senzor ultrasonic pe care il voi lipi pe un motoras care se va invarti 360 de grade si va detecta miscarea. Cand se observa un obiect, voi afisa un mesaj cu cateva informatii despre acesta pe un display.

## Motivation

Sunt pasionat de securitate. Am vrut sa fac ceva din acest domeniu, dar ma pasioneaza si conceptul de aparat cu ultra-sunete.

## Architecture 

![Schematic Screenshot](./sspmpozaaranjata.webp)
Voi folosi un servo-motor, breadboard, Ultrasonic Sensor HC -SR04, Display LCD I2C, Fire de legatura.

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May
- Am comandat piesele necesare si am inceput dezvoltarea hardware.
- Am creat schema electrica in KiCad.

### Week 12 - 18 May
- Am testat fiecare piesa.
- Am dezvoltat partea hardware.

### Week 19 - 25 May
- Am inceput partea de dezvoltare software.
- Am gasit solutii pentru problemele de alimentare.

## Hardware

Aparatura hardware folosita este una interactiva, oarecum asemanatoare cu ce am folosit la laborator. Am ales aceste piese pentru ca au un raport pret/calitate mare (dupa cum spun review-urile de pe optimus digital)
![poza1](img1.webp)
![poza2](img2.webp)

### Schematics
![Schematic Screenshot](schematic.svg)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Buzzer Pasiv de 5 V](https://www.optimusdigital.ro/ro/audio-buzzere/634-buzzer-pasiv-de-5-v.html) | The buzzer | [2 RON](https://www.optimusdigital.ro/ro/audio-buzzere/634-buzzer-pasiv-de-5-v.html) |
| [Micro Servomotor FS90MR cu Rotatie Continua si Reductor din Metal](https://www.optimusdigital.ro/ro/motoare-micro-motoare-cu-reductor/3168-micro-servomotor-fs90mr-cu-rotatie-360-si-reductor-din-metal.html?search_query=micro+servomotor&results=31) | The Servomotor | [53 RON](https://www.optimusdigital.ro/ro/motoare-micro-motoare-cu-reductor/3168-micro-servomotor-fs90mr-cu-rotatie-360-si-reductor-din-metal.html?search_query=micro+servomotor&results=31) |
| [	Breadboard (750 Puncte)](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/13245-breadboard-750-puncte.html) | The Breadboard | [9 RON](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/13245-breadboard-750-puncte.html) |
| [	LCD Hat 1602 pentru Raspberry Pi](https://www.optimusdigital.ro/ro/lcd-uri/1158-lcd-hat-1602-pentru-raspberry-pi.html) | The LCD Hat | [2 RON](https://www.optimusdigital.ro/ro/lcd-uri/1158-lcd-hat-1602-pentru-raspberry-pi.html) |
| [Condensator Electrolitic 220 uF, 50 V](https://www.optimusdigital.ro/ro/componente-electronice-condensatoare/7850-condensator-electrolitic-220-uf-50-v.html) | The Condensator | [1 RON](https://www.optimusdigital.ro/ro/componente-electronice-condensatoare/7850-condensator-electrolitic-220-uf-50-v.html) |
| [Modul Buzzer Pasiv](https://www.optimusdigital.ro/ro/componente-electronice/12598-modul-buzzer-pasiv.html) | The buzzer | [2 RON](https://www.optimusdigital.ro/ro/componente-electronice/12598-modul-buzzer-pasiv.html) |
| [Senzor ultrasonic HC-SR04](https://www.optimusdigital.ro/ro/senzori-senzori-ultrasonici/9-senzor-ultrasonic-hc-sr04-.html?search_query=Ultrasonic+Sensor+HC+-SR04&results=5) | The Sensor | [7 RON](https://www.optimusdigital.ro/ro/senzori-senzori-ultrasonici/9-senzor-ultrasonic-hc-sr04-.html?search_query=Ultrasonic+Sensor+HC+-SR04&results=5) |



## Software

| Library | Description | Usage |
|---------|-----------|-----------|
| [embassy_rp](https://github.com/embassy-rs/embassy) | Librărie pentru gestionarea perifericelor RP2040 | Inițializarea și utilizarea perifericelor Raspberry Pi Pico |
| [embassy_executor](https://github.com/embassy-rs/embassy) | Executor pentru gestionarea task-urilor asincrone | Gestionarea task-urilor asincrone în aplicația ta |
| [embassy_time](https://github.com/embassy-rs/embassy) | Librărie pentru gestionarea timpului | Folosită pentru temporizări și întârzieri în diverse funcții |
| [hd44780_driver](https://github.com/DisplayCore/hd44780-driver) | Driver pentru afișaj LCD HD44780 | Inițializarea și manipularea ecranului LCD I2C |
| [fixed](https://github.com/japaric/fixed) | Tipuri de numere fixe | Utilizată pentru calcule precise cu numere fixe în configurațiile PWM |
| [heapless](https://github.com/droogmic/heapless) | Containere de date cu dimensiuni statice | Folosit pentru manipularea stringurilor pentru afișaj LCD |
| [defmt](https://github.com/knurling-rs/defmt) | Sistem de logging și debugging | Folosit pentru output-ul de mesaje de debug și info |


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://arduinointro.com/articles/projects/using-the-hc-sr04-ultrasonic-sensor-to-detect-objects)
2. [link](https://forum.arduino.cc/t/ultrasonic-sensor-hc-sr04-tracking-and-following-object/1296914)
3. [link](https://forum.arduino.cc/t/how-to-sound-the-buzzer-when-ultrasonic-sensor-detect-the-object-please-help/577631)
...
