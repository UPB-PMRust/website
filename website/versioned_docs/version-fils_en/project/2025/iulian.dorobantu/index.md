# Simulated Automotive ECU
A simulated automotive ECU that reads throttle and coolant temperature sensors, displays them on an LCD digital dashboard, and controls engine behavior, warning indicators, and a cooling fan using Rust on a microcontroller.

:::info 

**Author**: Dorobantu Iulian-Adrian \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-adriandorobantu

:::

## Description

This project implements a simulated Electronic Control Unit (ECU) for an automotive system using the Raspberry Pi Pico WH microcontroller. It reads analog inputs from a throttle position sensor (potentiometer) and a coolant temperature sensor (thermistor), processes the data in real time, and outputs relevant system states.

A key feature of the project is its simulated digital dashboard, displayed on an LCD screen, which shows live throttle percentage and coolant temperature, mimicking a real vehicle interface. Based on sensor values, the ECU controls various outputs: activating a cooling fan when temperature exceeds a threshold, adjusting a simulated engine RPM, and triggering visual and audible warnings for overheating or abnormal behavior.

## Motivation

I chose to develop this project because I’m deeply passionate about cars and fascinated by the technology that powers them. The Electronic Control Unit (ECU) has revolutionized the automotive world by shifting control from purely mechanical systems to intelligent, software-driven logic. 

## Architecture 

![Schematic diagram](project_schematic.webp)

**Central control unit**
- **Role**: Acts as the brain of the ECU system. It reads input from sensors, executes control logic in real time, updates the digital dashboard, and manages actuator outputs based on current conditions.
- **Key functions**: Input acquisition, data processing, threshold evaluation, system coordination.

**Warning System**
- **Interface**: GPIO, PWM 
- **Role**: Provides visual and audible alerts when critical conditions occur, such as engine overheating, sensor faults, or when the rev limiter is reached.
- **Components Managed**: LEDs for visual warnings, passive buzzer for audible alerts. 
- **Control Logic**: LEDs for visual warnings, passive buzzer for audible alerts.

**Throttle Input Module**
- **Interface**: ADC
- **Role**: Simulates the accelerator pedal by measuring voltage changes from a user-controlled input (e.g., potentiometer). Converts analog signal to throttle percentage for use in engine behavior simulation.
- **Signal Processing**: Reads analog value, applies scaling or smoothing, and forwards to control logic.

**Temperature Monitoring Module**
- **Interface**: ADC
- **Role**: Monitors coolant temperature using a thermistor sensor. Interprets temperature readings and flags overheat conditions for system response.
- **Function**: Inputs temperature data into the control unit for fan activation and warning generation.

**Output Control Module**
- **Interface**: GPIO, PWM
- **Role**: Controls actuators such as the cooling fan, warning LEDs, and buzzer based on logic provided by the Central Control Unit.
- **Response Handling**: Turns fan on/off, manages alerts, and simulates engine RPM response.

**Digital Dashboard Interface**
- **Interface**: SPI
- **Role**: Simulates a vehicle’s dashboard by displaying live data such as throttle percentage, engine RPM, coolant temperature, and fault warnings.
- **Function**: Receives data from the control unit and presents it to the user through a graphical interface.


## Log

### Week 5 - 11 May
I began to search for all of the neccesary components, in order to start putting the project together. The first lines of code are beggining to be written, and all of the hardware details are being sorted out. While I wait for the components to arrive, I am also looking into making a 3D printed case for my ECU.

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

- **Raspberry Pi Pico WH**: Main microcontroller, reads sensors and controls outputs.
- **10kΩ Potentiometer**: Simulates throttle input (gas pedal).
- **10kΩ NTC Thermistor**: Measures coolant temperature.
- **Passive Buzzer**: Emits audible alerts for overheating or faults.
- **LEDs with 220Ω Resistors**: Provide visual warnings and rev limit indication.
- **DC Fan**: Simulates engine cooling, activated based on temperature.
- **SPI LCD Display**: Acts as a digital dashboard for real-time data visualization.
- **Breadboard & Jumper Wires**: For prototyping and interconnecting components.

### Schematics

![KiCAD schematic](kicad.webp)

### Bill of Materials


| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico WH](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [39 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/12395-raspberry-pi-pico-wh.html?search_query=Raspberry+Pi+Pico+WH&results=26) |
| [10kΩ Potentiometer](https://www.optimusdigital.ro/ro/componente-electronice-potentiometre/1886-potentiometru-stereo-10k.html?search_query=potentiometru+10k&results=19) | Throttle input | [1,99 RON](https://www.optimusdigital.ro/ro/componente-electronice-potentiometre/1886-potentiometru-stereo-10k.html?search_query=potentiometru+10k&results=19) |
| [10kΩ NTC Thermistor](https://www.optimusdigital.ro/ro/senzori-senzori-de-temperatura/8203-termistor-ntc-de-10k-rezistent-la-apa-cu-cablu-de-05-m.html?s) | Measuring the coolant temperature | [6,49 RON](https://www.optimusdigital.ro/ro/senzori-senzori-de-temperatura/8203-termistor-ntc-de-10k-rezistent-la-apa-cu-cablu-de-05-m.html?s) |
| [Passive Buzzer](https://www.optimusdigital.ro/ro/audio-buzzere/634-buzzer-pasiv-de-5-v.html?search_query=buzzer+pasiv&results=15) | Audible warnings | [1,40 RON](https://www.optimusdigital.ro/ro/audio-buzzere/634-buzzer-pasiv-de-5-v.html?search_query=buzzer+pasiv&results=15) |
| [LEDs with 220Ω, 1k, 10k Resistors](https://www.optimusdigital.ro/en/optimus-digital-kits/9517-set-de-led-uri-asortate-de-5-mm-si-3-mm-310-buc-cu-rezistoare-bonus.html?search_query=led+&results=2049) | Visual Warnings | [Already owned](https://www.optimusdigital.ro/en/optimus-digital-kits/9517-set-de-led-uri-asortate-de-5-mm-si-3-mm-310-buc-cu-rezistoare-bonus.html?search_query=led+&results=2049) |
| [DC Fan](https://www.optimusdigital.ro/en/others/7966-cmp-fan23-12-v-80x80x25-mm-fan-with-sensor.html?search_query=dc+fan&results=565) | Simulates coolant temperature regulation | [9,99 RON](https://www.optimusdigital.ro/en/others/7966-cmp-fan23-12-v-80x80x25-mm-fan-with-sensor.html?search_query=dc+fan&results=565) |
| [LCD Display](https://www.optimusdigital.ro/en/lcds/3554-modul-lcd-de-18-cu-spi-i-controller-st7735-128x160-px.html?srsltid=AfmBOorvbmQ0W7oenMF3GbKjJvYaAwmlKVeNrUUD92jn7lJLUyzJnsMu) | Digital Dashboard | [29,99 RON](https://www.optimusdigital.ro/en/lcds/3554-modul-lcd-de-18-cu-spi-i-controller-st7735-128x160-px.html?srsltid=AfmBOorvbmQ0W7oenMF3GbKjJvYaAwmlKVeNrUUD92jn7lJLUyzJnsMu) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7735-lcd](https://crates.io/crates/st7735-lcd) | ST7735-specific driver crate to send commands/data to the display | Used for the LCD display |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |
| [embassy-rp](https://github.com/embassy-rs/embassy/tree/main/embassy-rp) | RP2040 Peripherals | Used for accessing the peripherals |
| [heapless](https://github.com/rust-embedded/heapless) | String Library | 	Used for making strings for writiing to the lcd |
| [embbeded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer | 	Used for interacting with hardware components. |
| [defmt](https://github.com/knurling-rs/defmt) | Logging crate (used with Embassy)| 	Used for debugging/logging to RTT (Real Time Terminal)|
| [embassy-time](https://github.com/embassy-rs/embassy/tree/main/embassy-time) | Async timers/delays | 	Used for delays, blinking LEDs, buzzer timing, fan debounce|
| [embassy-executor](https://github.com/embassy-rs/embassy/tree/main/embassy-executor) | Async runtime | 	Used for running async fn tasks in Embassy|


## Links

1. [Testing analogue throttle on Arduino](https://www.youtube.com/watch?v=HzXL2NGaWAs)
2. [Arduino DS18B20 temperature sensor tutorial](https://www.youtube.com/watch?v=lIpgGru2Wv0)
3. [Arduino Dashboard](https://www.youtube.com/watch?v=L3ufJTFX2v0)
