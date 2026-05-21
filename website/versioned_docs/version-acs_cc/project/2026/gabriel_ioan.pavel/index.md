# Transmițător și traducător de cod Morse
Transmițător care emite un caracter în cod Morse și un receptor ce îl primește
și îl decodifică pentru a-l afișa pe un ecran.

:::info 

Author: Gabriel-Ioan PAVEL \
GitHub Project Link: https://github.com/UPB-PMRust-Students/acs-project-2026-gabrielioanpavel

:::

<!-- do not delete the \ after your name -->

## Description

Ideea proiectului este comunicarea unidirecțională prin cod Morse, implementată la nivel bare-metal.
Arhitectura este separată în două noduri complet izolate fizic, comunicarea realizându-se exclusiv prin
intermediul frecvenței de 433MHz folosind modularea OOK (On-Off Keying).

1. Nod emițător - Raspberry Pi Pico: Gestionează interfața cu utilizatorul. Microcontrollerul folosește
un pin ADC pentru a citi tensiunea unui potențiometru, mapând valoarea citită pe un index corespunzător
unei litere din alfabetul englez. La declanșarea unei întreruperi externe (apăsarea lungă a butonului), litera
selectată este codificată în semnale Morse (puncte și linii) și transmisă în eter prin pinul de date
al modulului RF.
2. Nod receptor - STM32: Gestionează captarea, filtrarea și afișarea datelor. Deoarece receptoarele RF
de 433MHz generează zgomot alb în absența unui semnal, nodul receptor folosește detecția fronturilor
și filtre software bazate pe praguri de timp pentru a izola semnalul util. Odată ce o secvență Morse
validă este identificată, aceasta este decodificată și trimisă via SPI către un ecran LCD de 1.44''
pentru afișare.

Logica este realizată cu ajutorul frameworkului `embassy` pentru a facilita execuția de cod
asincron, permițând procesarea non-blocantă a semnalelor radio și actualizarea ecranului
fără a recurge la un sistem de operare în timp real complex.

## Motivation

Am ales acest proiect din interesul pentru transmiterea semnalelor prin unde radio
folosind sisteme integrate, vizând înțelegerea la nivel fizic a comunicațiilor wireless
nesecurizate și neprotocolate. Din pasiunea pentru protocoalele de comunicații, am
luat decizia de a nu folosi module cu protocoale integrate (precum cele Bluetooth),
implementând astfel manual logica de timing, sincronizare și filtrare necesară.

## Architecture 

1. Nod emițător
- Input: Potențiometru conectat la un pin ADC. Se folosește de un filtru trece-jos
pentru a reduce zgomotul electric, facilitând maparea valorilor la cele 26 de litere
ale alfabetului englez. Un task asincron ascultă și actualizează constant caracterul într-o variabilă.
- Activare: Un task asincron așteaptă apăsarea lungă a unui buton pentru a transmite litera
selectată. Apăsarea scurtă a acestuia activează buzzer-ul care va reprezenta prin sunet
codul Morse al literei selectate.
- Transmisie: Odată ce butonul este apăsat lung, microcontrollerul citește litera și generează
timingurile high/low pentru a transmite "punctele" și "liniile", pentru a fi transmise
de modulul RF, care are o antenă legată. În timpul transmisiei, un LED verde stă aprins.

2. Nod receptor
- Recepție: Modulul receptor RF cu o antenă captează semnalul. Se folosește un
divizor de tensiune pentru a coborî tensiunea de 5V de la modul la 3.3V pentru ca semnalul
să fie primit de microcontroller printr-un pin GPIO.
- Decodificare: Un task asincron bazat pe EXTI monitorizează fronturile de semnal pe PA0.
Un algoritm de discriminare a duratei impulsurilor clasifică perioadele de high/low în "puncte"
(100ms), "linii" (300ms) sau zgomot/pauze, adăugând secvențele valide într-un buffer de decodificare.
- Afișare: Odată ce un mesaj este decodificat, un task asincron trimite prin interfața SPI caracterul
către ecranul de 1.44\'\'.

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

- Am finalizat ideea arhitecturii și funcționalității celor două noduri.
- Am actualizat schema nodului de transmisie.
  - (+) LED, rezistență de 220Ohm.
  - (+) Buzzer, tranzistor NPN 2N2222.
  - (/) Potențiometrul și condensatorul aferent sunt acum conectate la
  `AGND`, nu la `GND`.
  - (/) Înlocuit antena de 17.3cm cu YAGEO S432.
- Am actualizat schema nodului de recepție
  - (/) Înlocuit antena de 17.3cm cu YAGEO S432.
- Am actualizat documentația.
- Am lipit toate componentele pe plăcile de testare.

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Proiectul folosește două microcontrollere, un ecran SPI, o pereche de module RF,
un buzzer, un LED, condensatori și rezistori.

### Schematics

![nod-tx](./images/nod1.svg)
![nod-rx](./images/nod2.svg)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Microcontroller pentru emitere | [32 RON](https://ardushop.ro/ro/raspberry-pi/513-raspberry-pi-pico-6427854006004.html) |
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Microcontroller pentru recepție | ~125 RON |
| [Pereche emițător-receptor RF 433MHz](https://www.optimusdigital.ro/ro/ism-433-mhz/252-pereche-emitator-si-receptor-rf-433-mhz.html) | Pereche pentru transmisie prin radio | [9 RON](https://www.emag.ro/emitator-si-receptor-rf-433-mhz-radiofrecventa-ai196/pd/DXV1WGMBM/?ref=sponsored_products_search_f_b_1_1&recid=recads_1_5eeb774327d821d1507dab10e3a073b16b434e24323b157c1df591be8cee1ca6_1777052118&aid=624c1c25-3c92-11f1-801c-06eaf0d4245d&oid=58895050&scenario_ID=1) |
| [1.44'' SPI LCD](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/2167-lcd-de-144-pentru-stc-stm32-i-arduino.html) | Ecran pentru output | 43 RON |
| Potențiometru rotativ | Selectare de litere | ~10 RON |
| [2x Placă de testare 70x90](https://www.optimusdigital.ro/ro/prototipare-cablaje-de-test/232-cablaj-de-test.html) | Plăci pentru cele două noduri | 2x 3 RON |
| [ANT1204F002R0433A](https://www.digikey.ro/en/products/detail/pulse-electronics/ANT1204F002R0433A/3927193) | Antene | ~3 RON |
| Condensatoare 100nF | Filtre trece-jos | ~0.2 RON / buc. |
| Rezistențe 220, 1k, 2k | Divizor de tensiune + rezistență pentru anumite componente | ~ 0.15 RON / buc. |
| [Tranzistor 2N2222](https://www.optimusdigital.ro/en/transistors/935-transistor-npn-2n2222-to-92.html) | Tranzistor NPN pentru a comanda buzzerul de la un pin GPIO | 0.17 RON |
| [Buzzer](https://www.optimusdigital.ro/en/buzzers/12247-3-v-or-33v-passive-buzzer.html) | Feedback selectare caracter | 1 RON |
| [LED Verde](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/697-led-verde-de-3-mm-cu-lentile-difuze.html) | Feedback transmisie | 0.39 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy) | Framework async bare-metal pentru RP2040 | Abstractizare hardware pentru Pico: ADC, GPIO, PWM, timere |
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Framework async bare-metal pentru STM32 | Abstractizare hardware pentru STM32: EXTI, SPI, GPIO, timere |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Executor async pentru sisteme embedded | Rularea task-urilor asincrone pe ambele noduri |
| [embassy-time](https://github.com/embassy-rs/embassy) | Ceas async pentru sisteme embedded | Măsurarea duratei impulsurilor RF și temporizări non-blocante |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | Bibliotecă de grafică 2D pentru sisteme embedded | Randarea textului pe ecranul LCD |
| [mipidsi](https://crates.io/crates/mipidsi) | Driver generic pentru display-uri MIPI DSI/SPI | Inițializarea și controlul ecranului ST7735S de 1.44'' |
| [embedded-hal-bus](https://github.com/rust-embedded/embedded-hal) | Utilitar pentru partajarea bus-urilor hardware | Împachetarea bus-ului SPI cu pinul CS pentru driver-ul mipidsi |
| [static_cell](https://github.com/embassy-rs/static-cell) | Alocare statică sigură în Rust | Alocarea buffer-ului de display fără heap și fără `static mut` |
