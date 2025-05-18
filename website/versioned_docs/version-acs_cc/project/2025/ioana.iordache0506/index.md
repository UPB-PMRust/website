# Mini Smart Home
A tiny Rust-powered smart house that senses, blinks, and moves — just like a real one, but way cooler.😁


:::info

**Author**: Iordache Ioana-Diana \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/proiect-ioanaior](https://github.com/UPB-PMRust-Students/proiect-ioanaior)

:::

## Description  
This project aims to simulate a miniature smart home environment, focusing on practical energy-efficient and comfort-improving features such as RGB lighting, temperature and humidity monitoring, automatic window closing when rain is detected, and door control via a secure method (I'm thinking Morse code).


## Motivation  
I find it fascinating how sensors can interact with the environment and create real-life automation scenarios. I wanted to work on a project that’s both educational and fun, while also touching on concepts related to green homes, security, and interactivity. The idea of a miniature smart home felt like a great fit for demonstrating embedded programming in Rust. That is why I decided to create a small-scale model that could be both a learning experience and a nice gadget.😊


## Architecture  
The final system will include modules for:

- **Environment sensing**: temperature and humidity (DHT sensor), rain sensor  
- **Output feedback**: LCD display for status and sensor values, RGB LED lighting (WS2812) - this one might need a bit of tweaking to work with the Pico W, so TBD
- **Actuators**: servo motors for door and window control  
- **Optional modules**: motion or gas sensors for enhanced interactivity and safety  
- **Power**: USB power supply (batteries/solar are considered optional extensions)


```mermaid
graph TD
  subgraph Sensors
    DHT["🌡️ DHT Sensor\n(Temp & Humidity)"]
    Rain["🌧️ Rain Sensor"]
    PIR["🚶 PIR Motion Sensor (optional)"]
    Gas["🧪 Gas Sensor (optional)"]
  end

  subgraph Actuators
    ServoDoor["🚪 Servo Motor - Door"]
    ServoWindow["🪟 Servo Motor - Window"]
  end

  subgraph Display
    LCD["🖥️ I2C LCD 1602"]
  end

  subgraph Control
    Pico["🧠 Raspberry Pi Pico W"]
  end

  subgraph Optional
    WS2812["💡 WS2812 RGB LED (optional)"]
    Solar["☀️ Solar Panel"]
    Debug["🐞 RTT Debug Interface"]
  end

  %% Connections
  DHT --> Pico
  Rain --> Pico
  PIR --> Pico
  Gas --> Pico
  Pico --> LCD
  Pico --> ServoDoor
  Pico --> ServoWindow
  Pico --> WS2812
  Pico --> Debug
  Solar -. optional charging .-> Pico

  classDef optional fill:#f9f,stroke:#333,stroke-dasharray: 5 5;
  class Solar,Gas,PIR,WS2812 optional
  ```

> **NOTE**: A complete block diagram and wiring schematic will be added once hardware connections are finalized.


### Log

- **Week 21 – 27 April**  
  Reviewed previously owned components (sensors, display, servo motors) to assess reusability for the project. Placed an order for a Raspberry Pi Pico 2 W as the main controller board. Researched compatibility and pin availability for each part to ensure smooth integration.

- **Week 28 April – 4 May**  
  Drafted the initial project documentation. Defined the scope, motivation, and key goals. Outlined the planned system architecture and reviewed optional extensions. Started organizing the hardware list and validating available Rust crates for each module.

- **Week 5 – 11 May**  
  Placed an order for a second Raspberry Pi Pico 2 W to use it for debugging.  
  TBD

- **Week 12 – 18 May**  
  TBD

- **Week 19 – 25 May**  
  TBD


## Hardware  

| Component              | Function                          |
|------------------------|-----------------------------------|
| Raspberry Pi Pico 2W   | Main microcontroller with WiFi    |
| WS2812 RGB LED         | Smart lighting / system status    |
| DHT sensor             | Measures temperature and humidity |
| 1602 I2C LCD           | Displays environment info/status  |
| Servo motor #1         | Automatic door opening            |
| Servo motor #2         | Automatic window closing (rain)   |
| Rain sensor            | Triggers window close             |        |
| *Optional: PIR sensor* | Motion detection                  |
| *Optional: Gas sensor* | Safety feature                    |
| *Optional: Solar module* | Green energy charging           |

> **Note**: Solar power is planned as an optional future extension, not core requirements.


## Schematics  
**TBD** – Will be provided after hardware setup is complete.

## Bill of Materials  

| Device                | Usage                        | Price (approx.) |
|------------------------|-----------------------------|-----------------|
| Raspberry Pi Pico 2W   | Microcontroller with WiFi    | 35 RON          |
| WS2812 RGB LED module  | Smart light / system status  | 10 RON          |
| DHT11 sensor           | Temp & humidity measurement  | 15 RON          |
| 1602 LCD I2C module    | Display                      | 20 RON          |
| 2x Servo motors        | Door and window control      | 40 RON          |
| Rain sensor            | Detects rain                 | 10 RON          |
| PIR/Gas sensor (opt.)  | Optional safety feature      | 15–25 RON       |
| USB power supply       | Stable, easy power source    | -               |


## Software  

| Crate (Library)      | Description                              | Usage                         |
|----------------------|------------------------------------------|-------------------------------|
| `embassy`            | Async framework for embedded Rust        | Task scheduling and concurrency |
| `embedded-hal`       | Abstraction for microcontroller hardware | GPIO, I2C, PWM, etc.          |
| `ws2812-pio`         | RGB LED control via PIO                  | For WS2812 lighting           |
| `dht-sensor`         | Reads DHT11/22 values                    | Temperature & humidity data   |
| `servo-motor`        | PWM-based servo control                  | Door and window motors        |
| `lcd-i2c-driver`     | LCD control over I2C                     | Display status and readings   |
| `rtt-target`         | Debug logging                            | Development and debugging     |


## Links  
- TBD
