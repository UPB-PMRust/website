# EcoLink
A LoRa-powered smart-home ecosystem using an STM32 hub and ESP32 nodes to optimize energy consumption through automated climate and appliance control.

:::info 

**Author**: Nastase Denis-Cristian \
**GitHub Project Link**: [link_to_github](https://github.com/Denixzertyux/website)

:::

<!-- do not delete the \ after your name -->

## Description

A smart-home solution designed to efficiently manage the electrical consumption of home appliances based on personal needs and preferences. The target temperature of different rooms or areas in the house is set through a mobile or desktop app that communicates with the microcontrollers in the system.

## Motivation

I chose this project to show the potential of embedded software development in improving daily life through practical, energy-efficient automation.

## Architecture 

The system is built around a central hub and distributed room nodes:
- **STM32 central hub** coordinates logic, receives node data, and manages system decisions.
- **ESP32 edge nodes** collect room-level sensor data and control actuators locally.
- **LoRa communication** links all nodes to the hub over long distances with low power consumption.
- **Control app** (mobile/desktop) sets user preferences such as room temperature targets.

![EcoLink Architecture Diagram](./diagram_denis_nastase.svg)

## Log

<!-- write your progress here every week -->

### Week 16 - 22 March

Decided on the project idea and started initial research.

### Week 23 - 29 March

Submitted the project idea together with the planned components.

### Week 30 March - 5 April

Ordered the components and waited for delivery.

### Week 6 - 12 April

Checked component integrity, performed debugging, and finalized the architecture before implementation.

### Week 13 - 19 April

Finished setting up the distributed nodes, each with sensors, actuators, and one ESP32 controller.

## Hardware

The hardware setup includes one high-performance central controller and multiple distributed edge nodes connected through long-range wireless communication.
- **Main Hub (STM32 NUCLEO-U545RE-Q)**: ultra-low-power MCU for coordination and high-level control logic.
- **Edge Nodes (ESP32 DFR1139)**: room-level controllers for sensing and local actuation.
- **Environmental Sensors**: temperature and humidity inputs for climate automation.
- **Energy Monitoring Sensors**: voltage/current tracking for real-time consumption analysis.
- **Actuators (relays)**: physical switching of connected appliances.
- **LoRa modules**: low-power, long-distance communication between hub and nodes.

### Schematics

The electric scheme will be added later.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 NUCLEO-U545RE-Q](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D&utm_id=6470900573&utm_source=google&utm_medium=cpc&utm_marketing_tactic=emeacorp&gad_source=1&gad_campaignid=6470900573&gbraid=0AAAAADn_wf1Bn3QAN7vnR3zNOR5YbzCJB&gclid=CjwKCAjw46HPBhAMEiwASZpLRGsr429DKmxJjS_l8ra5qBHsnehbK18Lb_vNyvXPiFPbeGzHYWjdFRoCBhgQAvD_BwE) | Central hub microcontroller | [107 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D&utm_id=6470900573&utm_source=google&utm_medium=cpc&utm_marketing_tactic=emeacorp&gad_source=1&gad_campaignid=6470900573&gbraid=0AAAAADn_wf1Bn3QAN7vnR3zNOR5YbzCJB&gclid=CjwKCAjw46HPBhAMEiwASZpLRGsr429DKmxJjS_l8ra5qBHsnehbK18Lb_vNyvXPiFPbeGzHYWjdFRoCBhgQAvD_BwE) |
| [ESP32 DFR1139](https://eu.mouser.com/ProductDetail/DFRobot/DFR1139?qs=i8QVZAFTkqRgZpplqWnRrw%3D%3D&countryCode=RO&currencyCode=RON) | Distributed edge nodes (x2) | [122 RON total](https://eu.mouser.com/ProductDetail/DFRobot/DFR1139?qs=i8QVZAFTkqRgZpplqWnRrw%3D%3D&countryCode=RO&currencyCode=RON) |
| [PT100 Inline Temperature Sensor](https://kitsguru.com/products/pt100-sensor?variant=40708859625653&country=AE&currency=INR&utm_medium=product_sync&utm_source=google&utm_content=sag_organic&utm_campaign=sag_organic&srsltid=AfmBOort3m8Ic_yBN-rb1ZuttwlmXilMAbk-tQ_HpoI_8SO7L2qR8AGTg9c) | Temperature sensing | [8 RON](https://kitsguru.com/products/pt100-sensor?variant=40708859625653&country=AE&currency=INR&utm_medium=product_sync&utm_source=google&utm_content=sag_organic&utm_campaign=sag_organic&srsltid=AfmBOort3m8Ic_yBN-rb1ZuttwlmXilMAbk-tQ_HpoI_8SO7L2qR8AGTg9c) |
| [DHT11 + ESP-01S Board](https://www.optimusdigital.ro/en/humidity-sensors/12427-dht11-temperature-and-humidity-sensor-board-and-esp-01s-module.html) | Temperature and humidity sensing (x2) | [50 RON total](https://www.optimusdigital.ro/en/humidity-sensors/12427-dht11-temperature-and-humidity-sensor-board-and-esp-01s-module.html) |
| [INA219 Bidirectional Sensor](https://sigmanortec.ro/Senzor-monitor-curent-tensiune-bidirectional-INA219-p136254418?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72NIvD2P3bXy1bp7aBZwpHAVS&gclid=CjwKCAjw46HPBhAMEiwASZpLRD5eg0euZCOZ6fTrcJlhfAAX4i9VFgCm1FebKdoir9TV8KFL7u0H8hoC880QAvD_BwE) | Voltage and current monitoring (x2) | [24 RON total](https://sigmanortec.ro/Senzor-monitor-curent-tensiune-bidirectional-INA219-p136254418?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72NIvD2P3bXy1bp7aBZwpHAVS&gclid=CjwKCAjw46HPBhAMEiwASZpLRD5eg0euZCOZ6fTrcJlhfAAX4i9VFgCm1FebKdoir9TV8KFL7u0H8hoC880QAvD_BwE) |
| [5V Relay Module, High-Level Trigger](https://sigmanortec.ro/Modul-releu-5V-comanda-High-Level-p158739003?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72NIvD2P3bXy1bp7aBZwpHAVS&gclid=CjwKCAjw46HPBhAMEiwASZpLRN7nF-9xdcnAXjq_FZPMrYYyvzVZSELsMlqzCOLJ4tThqZKXVTA6FhoCoHQQAvD_BwE) | Appliance switching (x2) | [14 RON total](https://sigmanortec.ro/Modul-releu-5V-comanda-High-Level-p158739003?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72NIvD2P3bXy1bp7aBZwpHAVS&gclid=CjwKCAjw46HPBhAMEiwASZpLRN7nF-9xdcnAXjq_FZPMrYYyvzVZSELsMlqzCOLJ4tThqZKXVTA6FhoCoHQQAvD_BwE) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Hardware abstraction layer | Async support for STM32 Nucleo-U545RE peripherals |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async executor | Concurrent task management (LoRa polling, processing) |
| [lorawan-device](https://github.com/lora-rs/lorawan-device) | LoRaWAN protocol stack | Long-range communication handling |
| [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) | ESP-IDF HAL | Hardware access on ESP32 nodes |
| [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc) | ESP-IDF services | Wi-Fi and system services for nodes |
| [dht-sensor](https://github.com/dragonvga/dht-sensor) | DHT11/22 sensor driver | Temperature and humidity reading |
| [heapless](https://github.com/japaric/heapless) | Static memory collections | Heap-free data structures |
| [nb](https://github.com/rust-embedded/nb) | Non-blocking abstractions | Polling-style peripheral interactions |
| [embedded-ads1115](https://github.com/robamler/embedded-ads1115) | ADC driver | Precise voltage/current sensor reading |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [STM32 Standard Products](https://ro.mouser.com/new/stmicroelectronics/stm-standard-products/)
2. [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/index.html)
3. [Smart Home Projects Overview](https://www.seeedstudio.com/blog/2025/11/27/smart-home-projects-using-arduino-esp32-and-raspberry-pi/?srsltid=AfmBOoplbzdKWr6qQFDP8Rs3iJafEBAw7lafzCyet0LKzCDORFlZIZCW)
4. [Smart Home Reference Repository](https://github.com/LamFra/Smart-Home)
