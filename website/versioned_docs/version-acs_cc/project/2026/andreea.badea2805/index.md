# Mini Smart Home
A miniature smart home model featuring automated security, climate control, emergency hazard detection.

:::info

**Author**: Badea Andreea-Bianca \
**GitHub Project Link**: [link_to_github](https://github.com/UPB-PMRust-Students/acs-project-2026-badeabianca)

:::

<!-- do not delete the \ after your name -->

## Description

This project is a miniature smart home prototype designed to demonstrate practical home automation and safety features. It integrates an RFID-based security system for door access, alongside a climate control module that automatically adjusts windows based on real-time temperature and rain detection. For safety, it features an emergency hazard protocol that triggers an alarm and opens windows if smoke or gas is detected. Additionally, the system includes a built-in LCD screen for monitoring environmental data and a remote-controlled RGB lighting system, creating a complete, interactive model of modern smart living.

## Motivation

I chose this project because I wanted to build something highly interactive that you can see working in real life. It is fascinating to learn how data from sensors can control physical movements, like unlocking a door with a card or automatically closing the windows when it rains. I also wanted to create a system that reacts to dangers, such as turning on an alarm and opening windows if it detects smoke or gas. Building this model is a great and simple way to learn how to connect sensors, motors, and lights together.

## Architecture

The final system will include modules for:
Environment sensing:

- Digital temperature and humidity sensor (DHT11).

- Rain sensor (detects precipitation to trigger window and awning control).

- Gas and smoke sensor (MQ2) for safety monitoring.

Output feedback: LCD 1602 display with an I2C interface, for showing system status (e.g., temperature, humidity, "Emergency Mode", system alerts).

Motorized components: Two SG90 servo motors: one for door access, one for window control.

Input system: RC522 RFID module for scanning the access card, and an IR receiver for the remote control.

Indicators: RGB LED module (for ambient lighting and status) and an active buzzer (for audio alarms).

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The system is built around the STM32 Nucleo-U545RE-Q microcontroller. The sensory hardware includes a DHT11 temperature and humidity sensor , a rain sensor , an MQ2 gas/smoke sensor for safety , and an RC522 RFID module for secure access. Outputs and physical movements are driven by SG90 servomotors , an active buzzer , an RGB LED module , a ventilation fan , and an I2C-enabled LCD1602 screen for displaying system data.

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
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | Lab Provided
| [Servomotor SG90](https://www.bitmi.ro/componente-electronice/servomotor-sg90-180-grade-9g-10496.html) | Servomotor | [9.99 RON](https://www.bitmi.ro/componente-electronice/servomotor-sg90-180-grade-9g-10496.html) |
| [Modul RFID RC522](https://www.bitmi.ro/module-electronice/modul-rfid-rc522-13-59mhz-cu-card-si-tag-10468.html) | Modul RFID | [14.99 RON](https://www.bitmi.ro/module-electronice/modul-rfid-rc522-13-59mhz-cu-card-si-tag-10468.html) |
| [Modul senzor de umiditate si temperatura DHT11](https://www.bitmi.ro/senzori-electronici/modul-senzor-de-umiditate-si-temperatura-dht11-ky-015-10637.html) | Modul senzor de umiditate si temperatura  | [7.99 RON](https://www.bitmi.ro/senzori-electronici/modul-senzor-de-umiditate-si-temperatura-dht11-ky-015-10637.html) |
| [Modul senzor de gaze MQ2](https://www.bitmi.ro/senzori-electronici/modul-senzor-de-gaze-mq2-11214.html) | Modul senzor de gaze | [11.99 RON](https://www.bitmi.ro/senzori-electronici/modul-senzor-de-gaze-mq2-11214.html) |
| [Modul LED RGB ](https://www.bitmi.ro/module-electronice/modul-led-rgb-3-culori-10401.html) | Modul LED RGB | [2.13 RON](https://www.bitmi.ro/module-electronice/modul-led-rgb-3-culori-10401.html) |
| [Display LCD1602 HD44780](https://www.bitmi.ro/componente-electronice/display-lcd1602-hd44780-albastru-iluminat-10486.html) | Display | [13.99 RON](https://www.bitmi.ro/componente-electronice/display-lcd1602-hd44780-albastru-iluminat-10486.html) |
| [Modul interfata I2C/IIC pentru LCD1602](https://www.bitmi.ro/module-electronice/modul-interfata-i2c-pentru-lcd1602-10456.html) | Modul interfata I2C/IIC pentru LCD1602 | [9.99 RON](https://www.bitmi.ro/module-electronice/modul-interfata-i2c-pentru-lcd1602-10456.html) |
| [Senzor picaturi de ploaie](https://www.emag.ro/senzor-picaturi-de-ploaie-wlongc-rain-01/pd/DKXMPD2BM/?cmpid=148774&utm_source=google&utm_medium=cpc&utm_campaign=(RO:eMAG!)_3P_NO_SALES_%3e_Jucarii_hobby&utm_content=111476631565&gad_source=1&gad_campaignid=11606684347&gbraid=0AAAAACvmxQiuQWwRBmn2hIM6jL7g7XTSn&gclid=Cj0KCQjwkrzPBhCqARIsAJN460lHIac0SsnpISvLfFMwtyBHzZTLydA7KTRnV3tIgG6prJk7ttOOyS0aAq37EALw_wcB) | Senzor ploaie | [6.99 RON](https://www.emag.ro/senzor-picaturi-de-ploaie-wlongc-rain-01/pd/DKXMPD2BM/?cmpid=148774&utm_source=google&utm_medium=cpc&utm_campaign=(RO:eMAG!)_3P_NO_SALES_%3e_Jucarii_hobby&utm_content=111476631565&gad_source=1&gad_campaignid=11606684347&gbraid=0AAAAACvmxQiuQWwRBmn2hIM6jL7g7XTSn&gclid=Cj0KCQjwkrzPBhCqARIsAJN460lHIac0SsnpISvLfFMwtyBHzZTLydA7KTRnV3tIgG6prJk7ttOOyS0aAq37EALw_wcB) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [DIY Smart Home](https://youtu.be/Ohjlj0z85hQ?is=nvKfke6AahJwSeIl)
