# Plant watering alarm
When your plant is thirsty it can't tell you that, but I can help with it.

:::info

**Author**: Pacuraru Mihai-Dorin \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-dmihai03

:::

## Description

The "Plant Watering Alarm" is a smart monitoring system that detects when your plants need water and alerts you before they become dehydrated. The system uses a soil moisture sensor inserted into the plant's soil to continuously measure moisture levels. When the soil becomes too dry (falling below a configurable threshold), the system triggers an audio and visual alarm to notify you that your plant needs attention.

The core components include a Raspberry Pi Pico 2 W microcontroller, soil moisture sensors, a buzzer for audible alerts and a RGB LED that shows the watering level:
- green (normal)
- yellow (prepare for watering)
- red (urgent watering)


## Motivation

I had a plant that i've forgot to water and died.

## Architecture 

![Plant Watering Alarm System Architecture](architecture.svg)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

Write documentation.

### Week 12 - 18 May

Assemble hardware components and write demo source code.

### Week 19 - 25 May

## Hardware

- 2x Raspberry Pi Pico 2W
    - one for flashing the program to the other
    - one for controlling input from senzor and sending output signals to buzzer and LED
- buzzer
    - alert when plant is thirsty
- RGB LED
    - feedback of watering level

<<<<<<< HEAD
<!-- [![](https://markdown-videos-api.jorgenkh.no/youtube/EuqZ8Y21hg4)](https://youtu.be/EuqZ8Y21hg4) -->

<iframe width="auto" height="512" src="https://www.youtube.com/embed/EuqZ8Y21hg4?si=xL99fmc-H0W0BoAg" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

![](demo.webp)

### Schematic
![](schematic.svg)
=======
### Schematic
![Plant Watering Alarm System Schematic](schematic.jpg)
>>>>>>> 670e828 (Add schematic)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [2x Raspberry Pi Pico 2](https://www.tme.eu/ro/details/sc1632/raspberry-pi-sisteme-incorporate/raspberry-pi/raspberry-pi-pico-2-with-header/) | The microcontroller | 70 RON |
| [DC DC Voltage Converter](https://www.emag.ro/tranzistor-2n2222am-npn-se2312151536/pd/DKHH5TYBM/) | The transistor for buzzer | 3 RON |
| [Buzzer](https://www.emag.ro/buzzer-activ-12v-compatibil-arduino-raspberry-oky0151-oky0151-1/pd/D2KJNNMBM/?ref=embedding_similar_model_1_1&provider=rec&recid=rec_102_2799c2c87a26676847d5aa996bad6a8432c1cbf753de8447ade35beed1e6cfe5_1746002555&scenario_ID=102) | The buzzer | 2 RON |
| [Transistor](https://www.emag.ro/tranzistor-2n2222am-npn-se2312151536/pd/DKHH5TYBM/) | The transistor for buzzer | 3 RON |
| [RGB LED](https://sigmanortec.ro/led-rgb-10mm-catod-comun?SubmitCurrency=1&id_currency=2) | The LED for feedback | 2 RON |
| [Rezistors](https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/10928-plusivo-kit-250-buc-rezistoare.html?search_query=rezistor&results=110) | The resistors kit for LED | 15 RON |
| [Soil Humidity Sensor](https://sigmanortec.ro/Senzor-umiditate-sol-higrometru-p125814620?SubmitCurrency=1&id_currency=2&gQT=1) | The sensor | 4 RON |
| [Breadboard](https://www.emag.ro/breadboard-400-puncte-ai059-s69/pd/DRJ66JBBM/?ref=sponsored_products_search_r_1_1&recid=recads_1_3f8bcd3d0d1fce50ce875c811f44c8b37341887cf1698acd0193b6f39e905ded_1746374198&aid=d0eeaed2-9c23-11ef-9ec5-023c6ef0a87b_eyJicGQiOjAuMTN9-64d0f1bc2be845562de29773259d5d2f4c3408ca51e59b1cb3a0201b8d7c9467&oid=50658364&scenario_ID=1#reviews-section) | The Breadboard | 17 RON |
| [Wires](https://www.emag.ro/10-x-fire-dupont-tata-tata-20cm-cl55/pd/DV8M9WBBM/?ref=history-shopping_422378681_12161_1) | The Wires | 10 RON |
| Total | 126 |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/almindor/st7789) | Async embedded peripherals | Peripherals access |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async embedded tasks | Tasks manager |
| [embassy-time](https://docs.rs/embassy-time/latest/embassy_time/) | Async embedded time | Delay time manager |
| [defmt](https://docs.rs/defmt/latest/defmt/) | Logging framework | Manage logs |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Async dev lab](https://pmrust.pages.upb.ro/docs/acs_cc/lab/04)
2. [Serial Peripheral Interface](https://pmrust.pages.upb.ro/docs/acs_cc/lab/05)
3. [Inter-Integrated Circuit](https://pmrust.pages.upb.ro/docs/acs_cc/lab/06)
4. [Plants Care Guide](https://plnts.com/en/care/doctor/temperature-and-humidity)
