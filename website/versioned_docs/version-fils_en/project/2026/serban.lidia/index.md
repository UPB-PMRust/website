# Autonomous Light-Following Flower Robot

:::info 

**Author**: Serban Lidia-Andreea \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-lidiaserban

:::

<!-- do not delete the \ after your name -->

## Description

The Autonomous Light-Following Flower Robot is a bio-inspired system that mimics plant phototropism by navigating toward the strongest light source using dual photoresistors and an STM32 Nucleo U545RE-Q. Built with the Embassy framework for asynchronous control, it balances light-seeking with active obstacle avoidance via ultrasonic sensors while monitoring environmental conditions through a DHT22 sensor and LCD. The robot utilizes a dual-rail power strategy to isolate high-torque motor noise from sensitive logic, ensuring stable, real-time performance in a standalone package.

## Motivation

My motivation for this project is to apply embedded systems engineering to a nature-inspired concept. I chose a light-following robot because it provides a practical way to learn how the STM32 Nucleo U545RE-Q handles real-time sensor data and motor control. The project requires me to use the Embassy framework to solve a specific technical challenge: prioritizing obstacle avoidance while concurrently tracking light sources. Additionally, implementing a dual-rail power system with a DC-DC converter allows me to explore power management and noise isolation between high-current motors and sensitive logic circuitry.

## Architecture 

```text
       ┌───────────────────────────┐
       │   PRIMARY POWER SOURCE    │
       │      (4x Batteries)       │
       └─────┬───────────────┬─────┘
             │               │
             │        ┌──────▼──────────────────────────┐
             │        │    DC-DC VOLTAGE CONVERTER      │
             │        │   (High-Current / Motor Rail)   │
             │        └──────────────┬──────────────────┘
             │                       │
             │                       ▼
┌────────────▼────────────┐  ┌──────────────────────────┐
│      STM32 U545RE-Q     │  │      L9110S DRIVER       │
│   (Logic / Brain Rail)  │  │   (Motor Control H-Bridge)│
└────────────┬────────────┘  └──────────────▲───────────┘
             │                              │
             ├──────────────────────────────┘
             │      (PWM Speed Control)
             │
     ┌───────┴───────┬───────────────┬───────────────┐
     │               │               │               │
┌────▼────┐    ┌─────▼─────┐   ┌─────▼─────┐   ┌─────▼─────┐
│  LIGHT  │    │ DISTANCE  │   │   DHT22   │   │ LCD 16x2  │
│ SENSORS │    │ (HC-SR04) │   │ (Temp/Hum)│   │ (Status)  │
└─────────┘    └───────────┘   └───────────┘   └───────────┘ 
```

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The system is based on the STM32 Nucleo U545RE-Q microcontroller, which manages data processing and control. Light detection is performed using photoresistors, while an ultrasonic sensor provides obstacle detection and a temperature–humidity sensor monitors environmental conditions.
Movement is achieved using DC motors controlled by an H-bridge driver with PWM signals. The structure includes wheels and a caster for stability. Power is supplied by a battery and regulated with a DC-DC converter. An LCD display is used to present system data.

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | [~130 RON](https://ro.mouser.com)| 
| [DC Motors](https://ardushop.ro/ro/electronica/752-motor-dc-3v-6v-cu-reductor-1-48-6427854009609.html)| The DC motors| [~15 RON](https://ardushop.ro/ro/)|
| [Ultrasonic sensor](https://ardushop.ro/ro/electronica/2289-modul-senzor-ultrasonic-detector-distanta-hc-sr04-6427854030726.html)| Distance sensors | [~11 RON](https://ardushop.ro/ro/)|
| [Photo rezistors](https://ardushop.ro/ro/componente-discrete/2209-fotorezistenta-90mw-6427854034038.html)| Light sensors | [~10 RON](https://ardushop.ro/ro/)|
| [Movement](https://ardushop.ro/ro/roboti/2150-roata-roboti-cauciuc-65mm-diametru-6427854033017.html/) | Wheels | [~11 RON](https://ardushop.ro/ro/)|
| [Support caster ball](https://ardushop.ro/ro/carcase-i-suporturi/1782-suport-cu-bila-ball-caster-6427854026989.html) | Caster | [~4 RON](https://ardushop.ro/ro/)|
| [Baterry case](https://ardushop.ro/ro/carcase-i-suporturi/948-suport-carcasa-baterii-18650-4buc-6427854012692.html)| Baterry holder | [~5 RON](https://ardushop.ro/ro/)|
| [Driver for motor](https://ardushop.ro/ro/groundstudio/368-modul-l9110s-groundstudio-6427854000385.html)| H-Driver for motors| [~8 RON](https://ardushop.ro/ro/)|
| [Module for humidity sensor](https://ardushop.ro/ro/groundstudio/1598-modul-senzor-umiditate-si-temperatura-aht21-groundstudio-6427854000439.html)| Humidity sensor module | [~7 RON](https://ardushop.ro/ro/)|
| [Humidity and temperature sensor](https://ardushop.ro/ro/electronica/2302-senzor-de-temperatura-si-umiditate-dht22-6427854031617.html)| Humidity and temperature sensor | [~18 RON](https://ardushop.ro/ro/)|
| [Adjustable DC voltage reducer module](https://ardushop.ro/ro/module/341-modul-coborator-tensiune-reglabil-lm2576hv-dc-dc-5-60v-la-125-26v-6427854003737.html)| DC-DC voltage converter module | [~28 RON](https://ardushop.ro/ro/)|
| [LCD](https://www.emag.ro/display-lcd-2-x-16-cu-convertor-i2c-80-x-35-mm-verde-albastru-negru-2-e-001/pd/DHRJ0LMBM/)| LCD Display | [~23RON](https://www.emag.ro)|
| | | Total ~= 275RON|


## Software

| Library | Description | Usage |
|---------|-------------|-------|
|[embassy-stm32](https://github.com/embassy-rs/embassy/blob/main/embassy-stm32/Cargo.toml)| Hardware Abstraction Layer (HAL) for STM32|Configures GPIO, ADC for light sensors, and PWM for motors|
|[embassy-executor](https://github.com/embassy-rs/embassy)|Async runtime for embedded systems| Manages concurrent tasks like obstacle detection and movement|
|[hd44780-driver](https://github.com/JohnDoneth/hd44780-driver)|LCD Controller library|Displays temperature, humidity, and status on the 16x2 LCD screen|
|[dht-sensor](https://crates.io/crates/dht-sensor)|DHT22 Driver|Specifically used to parse the digital signal from the DHT22 temperature and humidity sensor|
|[panic-probe](https://www.google.com/search?q=https://github.com/knurling-rs/panic-probe)|Debugging tool|Helps you see errors in VS Code if the sensor fails to initialize|
|[embassy-time](https://github.com/embassy-rs/embassy)|Timekeeping and delays|Provides the microsecond-precision delays needed to "handshake" with the DHT22 and measure its data pulses|


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Embassy Book](https://embassy.dev/book/#_what_is_embassy)
2. [link](https://example3.com)
...