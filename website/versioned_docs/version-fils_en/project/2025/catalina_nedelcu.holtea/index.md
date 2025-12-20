<<<<<<< HEAD
=======
---
title: HydroSense
---
>>>>>>> 19d62b6 (Add architecture diagram)

# HydroSense

An ESP32-based irrigation controller written in Rust, using soil moisture + rain sensing and MQTT telemetry.

:::info
<<<<<<< HEAD

**Author:** Cătălina Nedelcu-Holtea (`catalina_nedelcu.holtea`)  
**GitHub Project Link:**

=======
**Author:** Cătălina Nedelcu-Holtea (`catalina_nedelcu.holtea`)  
>>>>>>> 19d62b6 (Add architecture diagram)
:::

## Description

<<<<<<< HEAD
<div style="text-align: justify;">
HydroSense is an ESP32-based irrigation controller written in Rust, designed to automate watering decisions using real sensor data. The system reads a soil moisture sensor and a rain sensor, then switches a 12 V water pump through an N-channel MOSFET stage when irrigation is needed. In parallel, it publishes status and measurements via MQTT, which enables remote monitoring and makes testing/debugging significantly easier.
</div>

## Motivation

<div style="text-align: justify;">
I am particularly interested in embedded and IoT systems because they connect software to the physical world in a direct and measurable way—through sensing, decision-making, and reliable control of real hardware. These concepts are fundamental to understanding many technologies we interact with daily, from smart home devices and environmental monitoring to agriculture/greenhouse automation and industrial control.
</div>

<div style="text-align: justify;">
At the same time, HydroSense started from a personal need. I genuinely enjoy taking care of plants, but watering often becomes a routine task that is easy to forget during busy periods. I have experienced situations where inconsistent watering led to plants drying out and dying. This project is my practical solution: a small, dependable system that handles watering based on actual conditions, not memory or fixed schedules.
</div>

## Architecture

![Architecture diagram](./architecture.svg)
<<<<<<< HEAD

=======
=======
HydroSense is an IoT irrigation system that makes watering decisions based on **real sensor data**, not fixed schedules. The ESP32 reads a **soil moisture sensor** (GPIO4) and a **rain sensor** (GPIO16). If the soil is below a chosen threshold and rain is not detected, the controller enables a **12 V water pump**.

Because the pump is an inductive load, it is not driven directly from the ESP32. Instead, the pump is switched using an **N-channel MOSFET** controlled from GPIO5, with a protection/stabilization stage (snubber + filtering) to reduce switching noise and improve reliability.

On the networking side, HydroSense connects to Wi-Fi and publishes data to an **MQTT broker** (soil moisture level, rain status, pump state). This allows the system to be monitored remotely and makes debugging/testing much easier compared to a “blind” embedded device. :contentReference[oaicite:0]{index=0}

>>>>>>> 19d62b6 (Add architecture diagram)
>>>>>>> e2905b4 (Add architecture diagram)
