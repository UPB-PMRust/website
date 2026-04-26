---
title: IoT Smartwatch with Health Monitoring
---

# IoT Smartwatch with Health Monitoring

A smartwatch based on RP2040 developed in Rust using the Embassy framework.

:::info

**Author**: Mihai Alexandru Vătafu \
**GitHub Project Link**: [https://github.com/upb-pmrust/project-mihai_alexandru_vatafu](https://github.com/upb-pmrust/project-mihai_alexandru_vatafu)

:::

## Description

This project aims to create a functional IoT smartwatch. It uses a Raspberry Pi Pico W to collect data and display a user interface on a circular IPS screen, focusing on power efficiency and real-time task management.

## Motivation

I chose this project to explore the capabilities of the Rust programming language in the embedded world, specifically using the Embassy framework for asynchronous multitasking on the RP2040.

## Architecture

The system architecture consists of several interconnected modules:
- **Display Driver Layer**: Manages SPI communication with the GC9A01 circular screen.
- **Power Management Module**: Monitors battery levels via ADC and handles low-power states.
- **UI Engine**: A lightweight graphics stack to render the clock face and sensor data.
- **Asynchronous Task Executor**: The Embassy executor that schedules tasks for sensor polling and display updates.