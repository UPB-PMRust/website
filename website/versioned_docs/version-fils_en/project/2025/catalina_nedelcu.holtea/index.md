
# HydroSense

An ESP32-based irrigation controller written in Rust, using soil moisture + rain sensing and MQTT telemetry.

:::info
**Author:** Cătălina Nedelcu-Holtea (`catalina_nedelcu.holtea`)  
:::

## Description

HydroSense is an IoT irrigation system that makes watering decisions based on **real sensor data**, not fixed schedules. The ESP32 reads a **soil moisture sensor** (GPIO4) and a **rain sensor** (GPIO16). If the soil is below a chosen threshold and rain is not detected, the controller enables a **12 V water pump**.

Because the pump is an inductive load, it is not driven directly from the ESP32. Instead, the pump is switched using an **N-channel MOSFET** controlled from GPIO5, with a protection/stabilization stage (snubber + filtering) to reduce switching noise and improve reliability.

On the networking side, HydroSense connects to Wi-Fi and publishes data to an **MQTT broker** (soil moisture level, rain status, pump state). This allows the system to be monitored remotely and makes debugging/testing much easier compared to a “blind” embedded device. :contentReference[oaicite:0]{index=0}
