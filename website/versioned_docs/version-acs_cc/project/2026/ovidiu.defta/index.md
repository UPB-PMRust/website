# AeroGuard
Monitor Solar de Poluare Atmosferica cu GPS

:::info 

**Author**: Defta Ovidiu 344C2 \
**GitHub Project Link**: [UPB-PMRust-Students/acs-project-2026-odefta](https://github.com/UPB-PMRust-Students/acs-project-2026-odefta)

:::

## Description

Sistem portabil de monitorizare a calitatii aerului, alimentat solar. Achizitioneaza periodic date de la senzori de particule in suspensie (PM2.5/PM10), gaze (NOx, CO, VOC), temperatura si umiditate, corelate cu coordonate GPS si timestamp. Datele sunt stocate local pe card microSD in format JSON si expuse printr-un server web embedded via WiFi, prin endpoint-uri REST. O aplicatie mobila consuma API-ul si afiseaza grafice de poluare, pozitia pe harta si nivelul bateriei. Sistemul functioneaza autonom prin alimentare solara cu modul de incarcare si baterie Li-Ion.

## Motivation

Statiile de monitorizare a calitatii aerului sunt rare si costisitoare, iar datele publice au granularitate scazuta. AeroGuard propune o solutie portabila, autonoma energetic, care permite monitorizarea locala a poluarii si vizualizarea datelor in timp real printr-o aplicatie mobila.

## Architecture

<!-- TODO: Add architecture diagram -->

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Microcontrolerul **STM32** citeste date de la senzorii de calitate a aerului prin **I2C** (BME280, SGP40), **UART** (PMS5003, NEO-6M) si **ADC** (MICS-4514). Datele sunt stocate pe **card microSD** via **SPI**. Comunicatia WiFi se realizeaza printr-un modul **ESP** conectat prin UART.

Alimentarea se face printr-un **panou solar 5V/1W** conectat la un modul de incarcare **TP4056**, care incarca o **baterie Li-Ion 18650**. Nivelul bateriei este monitorizat prin ADC.

### Schematics

<!-- TODO: Place KiCAD schematics here in SVG format -->

### Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| STM32 (TBD) | Microcontroler principal | TBD |
| ESP module (TBD) | Comunicatie WiFi | TBD |
| [PMS5003](https://www.aqmd.gov/docs/default-source/aq-spec/resources-page/plantower-pms5003-manual_v2-3.pdf) | Senzor particule PM2.5/PM10 | TBD |
| [MICS-4514](https://www.sgxsensortech.com/content/uploads/2023/03/MiCS-4514-rev-16.pdf) | Senzor NOx/CO | TBD |
| [SGP40](https://sensirion.com/products/catalog/SGP40/) | Senzor VOC | TBD |
| [BME280](https://www.bosch-sensortec.com/products/environmental-sensors/humidity-sensors-bme280/) | Senzor temperatura/umiditate | TBD |
| [NEO-6M](https://www.u-blox.com/en/product/neo-6-series) | Modul GPS | TBD |
| Card microSD + cititor SPI | Stocare date | TBD |
| Panou solar 5V/1W | Alimentare | TBD |
| TP4056 | Modul incarcare | TBD |
| Baterie Li-Ion 18650 | Stocare energie | TBD |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| `embassy-rs` | Framework async pentru microcontrolere | Runtime si task-uri async |
| `embassy-stm32` | Suport Embassy pentru STM32 | Acces hardware low-level |
| `embassy-time` | Functionalitati de timp | Timere si delay-uri |
| `embassy-net` | Stack de retea | Comunicatie TCP/IP |
| `embedded-hal` | Abstractizari hardware | Interfete I2C, SPI, UART, ADC |
| `serde` / `serde_json_core` | Serializare/deserializare | Format JSON pentru date si API |
| `embedded-sdmmc` | Suport card microSD | Stocare fisiere FAT32 |
| `bme280` | Driver BME280 | Citire temperatura/umiditate |
| `nmea0183` | Parser date GPS | Parsare coordonate NMEA |
| `defmt` | Logging embedded | Debug si logging |
| `heapless` | Structuri de date fara alocare dinamica | Buffere si colectii no_std |

## Links

1. [Embassy-rs Documentation](https://embassy.dev/)
2. [PMS5003 Datasheet](https://www.aqmd.gov/docs/default-source/aq-spec/resources-page/plantower-pms5003-manual_v2-3.pdf)
3. [BME280 Datasheet](https://www.bosch-sensortec.com/products/environmental-sensors/humidity-sensors-bme280/)
4. [SGP40 Datasheet](https://sensirion.com/products/catalog/SGP40/)
