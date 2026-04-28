# TermoTrack 9000
Sistem de monitorizare termica cu interpolare si urmarire automata prin laser



**Author**: Radulescu Robert-Stefan 331CC \
**GitHub Project Link**: [UPB-PMRust-Students/acs-project-2026-robsterad](https://github.com/UPB-PMRust-Students/acs-project-2026-robsterad)



## Description

TermoTrack 9000 este un sistem avansat de imagistica termica bazat pe microcontrolerul STM32. Dispozitivul captureaza date de temperatura in infrarosu folosind un senzor matriceal MLX90640 (32x24 pixeli) si le proceseaza in timp real pentru a genera o imagine fluida pe un ecran TFT color.

Sistemul implementeaza un algoritm de **interpolare biliniara** pentru a transforma matricea bruta de 768 de puncte intr-o harta termica de inalta rezolutie (320x240). Utilizatorul poate interactiona cu sistemul prin intermediul unui **encoder rotativ** pentru a ajusta dinamic plaja de temperaturi monitorizata (scaling), permitand optimizarea imaginii atat pentru circuite electronice (diferente mici de temperatura), cat si pentru detectia prezentei umane.

O caracteristica distinctiva a proiectului este **subsistemul de urmarire automata (tracking)**. Microcontrolerul identifica in timp real punctul cu temperatura maxima si actioneaza un mecanism pan-tilt cu **doua servomotoare** pentru a indrepta un **indicator laser** exact catre sursa de caldura detectata. Suplimentar, un buton dedicat permite realizarea unui "screenshot" prin trimiterea matricei de date catre un PC via **UART**.

## Motivation

Am ales acest proiect deoarece combina procesarea complexa de semnal (DSP) cu controlul actuatorilor mecanici. Termoviziunea are aplicatii practice vaste, de la diagnoza circuitelor electronice pana la sisteme de securitate, iar implementarea pe o arhitectura ARM Cortex-M ofera oportunitatea de a utiliza periferice avansate precum DMA, Timere hardware si FPU.

## Architecture

Sistemul este organizat in jurul unitatii centrale de procesare (STM32) care coordoneaza urmatoarele blocuri:
- **Unitatea de achizitie**: Senzorul MLX90640 conectat prin I2C Fast Mode.
- **Unitatea de procesare**: Algoritmi de interpolare si cautare a extremelor locale (Max Tracking).
- **Unitatea de afisare**: Display TFT ILI9341 controlat prin SPI cu suport DMA.
- **Unitatea de actionare**: Doua servomotoare controlate prin PWM pentru orientarea laserului.
- **Interfata utilizator**: Encoder rotativ (Timer Hardware) si buton de captura (EXTI).

## Log

### Week 20 - 26 April

Am redactat documentatia initiala a proiectului si am definit structura arhitecturala.

## Hardware

Sistemul utilizeaza microcontrolerul **STM32** pentru a gestiona protocoalele de comunicatie si calculul matematic. Senzorul termic comunica prin **I2C**, in timp ce ecranul utilizeaza **SPI** de mare viteza. Servomotoarele sunt controlate prin semnale **PWM**, iar laserul este actionat digital prin **GPIO**.

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| STM32 (Nucleo-F401RE / BlackPill) | Microcontroler principal | TBD |
| [MLX90640](https://www.melexis.com/en/product/MLX90640/Far-Infrared-Thermal-Sensor-Array) | Senzor termic IR (32x24 pixeli) | TBD |
| [ILI9341 TFT Display](https://www.displayfuture.com/Display/Datasheet/Controller/ILI9341.pdf) | Afisare harta termica | TBD |
| Encoder Rotativ KY-040 | Ajustare praguri temperatura | TBD |
| 2x SG90 Servomotors | Mecanism Pan-Tilt pentru laser | TBD |
| Modul Laser 5V/3.3V | Indicator punct fierbinte | TBD |
| Buton Push-button | Captura screenshot (UART) | TBD |
| Breadboard & Jumper Wires | Conectica hardware | TBD |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| `embassy-rs` | Framework async pentru microcontrolere | Runtime si gestionarea concurenta a task-urilor (achizitie, afisare, tracking) |
| `embassy-stm32` | Suport Embassy pentru STM32 | Acces hardware low-level (I2C, SPI, PWM, UART, EXTI) cu suport DMA |
| `embassy-time` | Functionalitati de timp | Timere si delay-uri non-blocante |
| `embedded-hal` | Abstractizari hardware | Interfete standard pentru periferice (I2C, SPI) |
| `embedded-graphics` | Biblioteca grafica `no_std` | Randarea imaginii termice, a paletei de culori si a UI-ului pe ecranul TFT |
| `ili9341` | Driver ecran TFT | Controlul hardware al display-ului |
| `mlx9064x` | Driver senzor termic | Citirea datelor si a EEPROM-ului din senzorul MLX90640 |
| `defmt` | Logging embedded | Debugging si afisare log-uri structurate via probe-rs |
| `heapless` | Structuri de date fara alocare dinamica | Buffere statice pentru stocarea matricei termice (768 elemente) |
| `micromath` (sau `libm`) | Operatii matematice `no_std` | Calcule matematice rapide pentru interpolarea biliniara si maparea unghiurilor pentru servomotoare |

## Links

1. [Embassy-rs Documentation](https://embassy.dev/)
2. [MLX90640 Datasheet](https://www.melexis.com/-/media/files/documents/datasheets/mlx90640-datasheet-melexis.pdf)
3. [Embedded Graphics Documentation](https://docs.rs/embedded-graphics/latest/embedded_graphics/)
4. [Rust Embedded Book](https://docs.rust-embedded.org/book/)