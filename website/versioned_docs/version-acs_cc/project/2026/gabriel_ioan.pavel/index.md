# Transmițător și traducător de cod Morse
Transmițător care emite un caracter în cod Morse, un nod bridge WiFi/UART și un receptor
ce îl decodifică și îl afișează pe un ecran.

:::info 

Author: Gabriel-Ioan PAVEL \
GitHub Project Link: https://github.com/UPB-PMRust-Students/acs-project-2026-gabrielioanpavel

:::

<!-- do not delete the \ after your name -->

## Description

Ideea proiectului este comunicarea unidirecțională prin cod Morse, implementată la nivel bare-metal.
Arhitectura cuprinde trei noduri interconectate prin WiFi UDP și UART.

1. Nod emițător - Raspberry Pi Pico 2W: Gestionează interfața cu utilizatorul. Microcontrollerul creează
un punct de acces WiFi și folosește un pin ADC pentru a citi tensiunea unui potențiometru, mapând valoarea
citită pe un index corespunzător uneia din cele 26 de litere ale alfabetului englez sau semnalului special
CLEAR. La apăsarea scurtă a butonului, buzzerul redă codul Morse al literei selectate. La apăsarea lungă,
litera este transmisă nodului bridge printr-un protocol ARQ (Automatic Repeat reQuest) stop-and-wait, cu
retrimitere automată în cazul pierderii pachetelor.

2. Nod bridge - Heltec Wireless Stick V3: Asigură puntea de comunicație dintre transmițător și receptor.
Se conectează la rețeaua WiFi creată de Pico 2W, participă la protocolul ARQ (echo al datelor, confirmare
finală) și, la finalizarea cu succes a schimbului, trimite codul Morse prin UART nodului receptor.

3. Nod receptor - STM32 Nucleo-U545RE-Q: Primește secvențele Morse prin UART pe PA3, le decodifică în
litere ale alfabetului englez și le afișează pe un ecran LCD de 1.44''. Pe prima linie a ecranului afișează
permanent ultimul cod Morse primit brut (ex. `.-`), iar de la linia a doua în jos acumulează literele
decodificate cu reîmpachetare automată. La primirea semnalului special CLEAR, șterge complet afișajul.

Logica este realizată cu ajutorul frameworkului `embassy` pentru a facilita execuția de cod
asincron, permițând procesarea non-blocantă a comunicațiilor și actualizarea ecranului
fără a recurge la un sistem de operare în timp real complex.

## Motivation

Am ales acest proiect din interesul pentru comunicațiile wireless implementate la nivel
bare-metal, vizând înțelegerea practică a stivelor de comunicații fără abstracțiile unui
sistem de operare. Am optat pentru WiFi UDP în locul TCP, implementând manual un protocol
de confirmare ARQ (Automatic Repeat reQuest) stop-and-wait cu timeout și retrimitere, cu
scopul de a înțelege mecanismele de bază ale fiabilității în transmisiile de date fără a
recurge la soluții de protocol gata implementate.

## Architecture 

1. Nod emițător (Raspberry Pi Pico 2W)
- Input: Potențiometru conectat la un pin ADC. Se folosește un filtru trece-jos pentru a reduce
zgomotul electric, facilitând maparea valorilor pe 27 de poziții: cele 26 de litere ale
alfabetului englez și un slot special CLEAR (ultima poziție).
- Activare: Un task asincron diferențiază apăsarea scurtă (redare Morse local prin buzzer) de
cea lungă (transmisie WiFi). La apăsarea lungă, microcontrollerul intră în secvența ARQ și
blochează orice alt input pe durata ei.
- Transmisie ARQ: Nodul trimite codul Morse prin UDP, așteaptă echo-ul de confirmare de la nodul
bridge (cu timeout de 5s și retrimitere automată). La primirea echo-ului corect, trimite confirmarea
finală „OK", așteaptă „OK" înapoi de la bridge. La succes, LED-ul verde confirmă transmisia și
intrarea este deblocată.

2. Nod bridge (Heltec Wireless Stick V3)
- Recepție WiFi: Nodul rulează în modul STA, conectat la rețeaua creată de Pico 2W (IP static
169.254.1.2), și ascultă pe portul UDP 5000.
- Protocol ARQ: Implementează o mașină de stări cu două stări (WAITING_DATA / WAITING_OK). La
primirea datelor, trimite echo; la primirea confirmării „OK" de la Pico, trimite codul Morse
prin UART și confirmă înapoi cu „OK". Dacă primește „OK" în starea WAITING_DATA (caz de pierdere
a confirmării), răspunde cu „OK" fără a mai trimite pe UART.
- Transmisie UART: Trimite secvența Morse pe GPIO4, conectat prin cablu la PA3 al STM32.

3. Nod receptor (STM32 Nucleo-U545RE-Q)
- Recepție UART: Ascultă pe PA3 (USART2) datele trimise de nodul bridge.
- Decodificare: Un tabel invers față de cel de pe Pico mapează secvențele Morse la literele
corespunzătoare. La primirea „CLEAR", afișajul și bufferul sunt șterse.
- Afișare: Linia 1 a ecranului afișează permanent ultimul cod Morse primit brut. De la linia 2
în jos, caracterele decodificate sunt adăugate unui buffer cu reîmpachetare automată pe linii
(FONT_6X10, 21 caractere/linie) și trimise ecranului de 1.44'' prin interfața SPI.

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

În urma unor teste am conclus că modulul RF de recepție este defect. Având
acasă un Heltec Wireless Stick V3, am decis înlocuirea modulelor RF cu WiFi.
Schimbări:

- (/) Înlocuit RF cu WiFi.
- (+) Adăugat Heltec Wireless Stick V3 pe schema de recepție.
- (/) Modificat documentație.
- (-) Eliminat componentele neutilizate din scheme și de pe plăcile de testare.
- (+) Scris firmware pentru Pico.
- (+) Scris firmware pentru Heltec.

Altele:

- Testat transmisie de pachete prin UDP între Pico și Heltec, cu confirmare
hardware (LED și buzzer).

În urma unor teste pentru a verifica protocolul, am observat că ecranul Heltecului
este defect. Din acest motiv, am făcut următoarele modificări:

- (/) Mutat logică ecran din Heltec in STM.
- (-) Eliminat logică ecran din firmware Heltec.

### Week 19 - 25 May

## Hardware

Proiectul folosește trei microcontrollere/module de procesare, un ecran SPI,
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
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/products/raspberry-pi-pico-2/) | Microcontroller emițător, punct de acces WiFi | ~52 RON |
| [Heltec Wireless Stick V3](https://heltec.org/project/wireless-stick-v3/) | Nod bridge WiFi → UART | ~70 RON |
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Microcontroller receptor | ~125 RON |
| [1.44'' SPI LCD](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/2167-lcd-de-144-pentru-stc-stm32-i-arduino.html) | Ecran pentru output | 43 RON |
| Potențiometru rotativ | Selectare de litere | ~10 RON |
| [2x Placă de testare 70x90](https://www.optimusdigital.ro/ro/prototipare-cablaje-de-test/232-cablaj-de-test.html) | Plăci pentru componentele externe ale celor două noduri | 2x 3 RON |
| Condensatoare 100nF | Filtre trece-jos | ~0.2 RON / buc. |
| Rezistențe 220Ω, 1kΩ | Rezistență LED + bază tranzistor buzzer | ~0.15 RON / buc. |
| [Tranzistor 2N2222](https://www.optimusdigital.ro/en/transistors/935-transistor-npn-2n2222-to-92.html) | Tranzistor NPN pentru a comanda buzzerul de la un pin GPIO | 0.17 RON |
| [Buzzer](https://www.optimusdigital.ro/en/buzzers/12247-3-v-or-33v-passive-buzzer.html) | Feedback selectare caracter | 1 RON |
| [LED Verde](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/697-led-verde-de-3-mm-cu-lentile-difuze.html) | Feedback confirmare transmisie ARQ | 0.39 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy) | Framework async bare-metal pentru RP2040 | ADC, GPIO, PIO pentru Pico 2W |
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Framework async bare-metal pentru STM32 | UART, SPI, GPIO pentru STM32 |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Executor async pentru sisteme embedded | Rularea task-urilor asincrone pe toate nodurile |
| [embassy-time](https://github.com/embassy-rs/embassy) | Ceas async pentru sisteme embedded | Timeout-uri ARQ și temporizări non-blocante |
| [embassy-net](https://github.com/embassy-rs/embassy) | Stack de rețea async pentru sisteme embedded | UDP și gestionare stivă IP pe Pico 2W și Heltec |
| [cyw43 / cyw43-pio](https://github.com/embassy-rs/embassy) | Driver WiFi pentru cipul CYW43439 (Pico 2W) | Inițializare chip WiFi, mod AP |
| [esp-hal](https://github.com/esp-rs/esp-hal) | HAL bare-metal pentru ESP32 | Periferice hardware pentru Heltec: UART, timer |
| [esp-radio](https://github.com/esp-rs/esp-radio) | Driver WiFi pentru ESP32 | Conectare WiFi în mod STA pe Heltec |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | Bibliotecă de grafică 2D pentru sisteme embedded | Randarea textului pe LCD |
| [mipidsi](https://crates.io/crates/mipidsi) | Driver generic pentru display-uri MIPI DSI/SPI | Inițializarea și controlul ecranului ST7735S de 1.44'' |
| [embedded-hal-bus](https://github.com/rust-embedded/embedded-hal) | Utilitar pentru partajarea bus-urilor hardware | Împachetarea bus-ului SPI cu pinul CS pentru driver-ul mipidsi |
| [static_cell](https://github.com/embassy-rs/static-cell) | Alocare statică sigură în Rust | Alocarea resurselor de rețea și display fără heap și fără `static mut` |
