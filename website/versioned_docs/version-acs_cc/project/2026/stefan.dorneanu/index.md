# Datalogger GPS pentru Bicicleta

Un sistem embedded de inregistrare si monitorizare a datelor de deplasare pentru bicicleta, cu afisaj in timp real si detectie de evenimente.

:::info

**Author**: Dorneanu Stefan Cristian  \
**GitHub Project Link**: [link_to_github](https://github.com/stefandorneanu/website)

:::

<!-- do not delete the \ after your name -->

## Description

Sistemul achizitioneaza date de la un modul GPS (pozitie, viteza, altitudine), un accelerometru si giroscop (detectie franare brusca si viraj) si un senzor barometric (altitudine precisa). Datele sunt afisate in timp real pe un display si salvate persistent pe card SD in format CSV. Software-ul ruleaza mai multe task-uri async paralele: achizitie GPS cu decodificarea datelor GPS, detectie evenimente din accelerometru si giroscop cu filtrare pe magnitudinea vectorului de acceleratie, afisaj live si scriere pe card SD. Fuziunea datelor combina altitudinea GPS cu cea barometrica pentru precizie mai buna.

## Motivation

Am ales acest proiect deoarece combina intr-un mod echilibrat hardware si software, si produce un rezultat concret si testabil in lumea reala. Posibilitatea de a vizualiza traseul parcurs pe harta dupa o tura cu bicicleta, cu date despre viteza, altitudine si evenimente de franare, face proiectul relevant si demonstrabil.

## Architecture`

Sistemul este structurat in 4 componente principale care ruleaza in paralel:

- **Task GPS** — citeste datele de pozitie si viteza la 10Hz si calculeaza distanta parcursa
- **Task IMU** — citeste accelerometrul si giroscopul la 100Hz si detecteaza evenimentele de franare brusca sau viraj
- **Task Display** — actualizeaza ecranul OLED la 5Hz cu datele curente
- **Task SD** — scrie datele in fisierul CSV pe cardul SD o data pe secunda

Toate task-urile comunica prin mecanisme de tip Mutex si Channel pentru a evita conflictele de date.

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Sistemul foloseste placa NUCLEO-U545RE-Q ca microcontroler principal. Senzorul GPS furnizeaza date de pozitie si viteza prin UART. Accelerometrul si giroscopul (IMU) sunt conectate prin I2C si detecteaza miscarile bruste. Senzorul barometric, tot pe I2C, masoara presiunea atmosferica pentru calculul altitudinii. Un display OLED afiseaza datele in timp real, iar un modul de card SD salveaza toate datele in format CSV. LED-ul RGB si buzzerul semnalizeaza evenimentele detectate (franare brusca, viraj).

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Microcontroler principal | ~ 100 RON |
| [Modul GPS NEO-6M](https://www.optimusdigital.ro/en/gps/2137-gyneo6mv2-gps-module-with-miniature-antenna.html) | Pozitie, viteza, altitudine GPS | ~ 35 RON |
| [Accelerometru si Giroscop MPU-6050](https://www.optimusdigital.ro/en/inertial-sensors/96-mpu6050-accelerometer-and-gyroscope-module.html) | Detectie franare brusca si viraj | ~ 10 RON |
| [Senzor Barometric BMP280](https://www.optimusdigital.ro/en/pressure-sensors/1777-bmp280-barometric-pressure-sensor-module.html) | Temperatura si altitudine barometrica | ~ 8 RON |
| [Display OLED 128x64 I2C](https://www.optimusdigital.ro/en/lcds/2894-096-i2c-oled-module.html) | Afisaj date in timp real | ~ 15 RON |
| [Modul Card MicroSD](https://www.optimusdigital.ro/en/memories/1516-microsd-card-slot-module.html) | Logging date in format CSV | ~ 5 RON |
| Card MicroSD 8GB | Stocare fisiere CSV | ~ 15 RON |
| LED RGB | Indicator vizual evenimente | ~ 2 RON |
| Buzzer pasiv | Alerta sonora franare brusca | ~ 3 RON |
| Baterie LiPo 3.7V + modul TP4056 | Alimentare mobila si incarcare | ~ 20 RON |
| Fire, breadboard, rezistori | Conectare componente | ~ 10 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Framework async pentru embedded Rust | Task-uri paralele, drivere periferice |
| [embassy-time](https://github.com/embassy-rs/embassy) | Gestionare timp in Embassy | Timere si delays async |
| [embedded-sdmmc](https://github.com/rust-embedded-community/embedded-sdmmc-rs) | Sistem de fisiere FAT32 pentru SD | Scriere CSV pe card SD |
| [mpu6050](https://github.com/juliangaal/mpu6050) | Driver pentru accelerometru/giroscop | Citire date IMU prin I2C |
| [bmp280-ehal](https://github.com/uber-foo/bmp280) | Driver pentru senzorul barometric | Citire temperatura si presiune |
| [ssd1306](https://github.com/jamwaffles/ssd1306) | Driver display OLED | Afisaj date in timp real |
| [libm](https://github.com/rust-lang/libm) | Functii matematice pentru no_std | Formula Haversine, sqrt |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Embassy Rust Framework](https://embassy.dev)
2. [STM32U545 Reference Manual](https://www.st.com/resource/en/reference_manual/rm0456-stm32u5-series-armbased-32bit-mcus-stmicroelectronics.pdf)
3. [Proiecte PM ani anteriori](https://embedded-rust-101.wyliodrin.com/docs/acs_cc/project/2025)
