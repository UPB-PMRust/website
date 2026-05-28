# Smart Training Station
A smart reaction-training station that uses lights and sensors to measure hit force while logging scores online in real-time.

:::info 

**Author**: Cucerave Adelina-Maria 1222EEB \
**GitHub Project Link**: (https://github.com/UPB-PMRust-Students/fils-project-2026-AdelinaMariaCucerave)

:::

<!-- do not delete the \ after your name -->

## Description

This project is an interactive training station that uses a network of sensors to track physical hits and share the results over the internet. The system is built with several "interaction nodes" that connect to a main controller and a central dashboard.

The station works by using piezo sensors to measure exactly how hard and how fast a user hits a target. Because hitting these sensors can create high-voltage spikes, the system includes a protection circuit (Schottky-clamping). This keeps the sensitive electronics safe while ensuring the hit data is recorded accurately.

Users get instant feedback through three different channels:

    1. Visual: Bright RGB LED rings change colors to show the intensity of the hit.

    2. Audio: A buzzer plays different sounds for successful hits or errors.

    3. Data: A color screen displays live scores and menus, while a knob (potentiometer) lets the user manually adjust how sensitive the sensors are.

The "brain" of the system is the STM32U545RE-Q microcontroller. It runs on a modern software framework called Embassy-rs, which allows the code to handle many tasks at once without slowing down. Finally, an ESP32-C3 Wi-Fi module acts as a bridge, sending the performance data to an online dashboard so users can track their progress over time.

## Motivation

I have always been interested in how technology can be used to measure and improve human performance. Reaction time and physical precision are important in many areas, such as professional sports, physical therapy, and even industrial safety.

By building a system that gives instant feedback through lights and sounds, users can train their reflexes in a more engaging and effective way. Additionally, by adding Wi-Fi connectivity to the station, performance data can be tracked and saved over time. This allows users to stay aware of their progress, set personal goals, and find specific areas where they can improve their speed and accuracy.

## Architecture 

 Main Architectural Components:

1. Signal Acquisition Unit (Input)

    Role: Detects physical impacts and measures the force of each hit.

    Components: 3x Piezo Diaphragms and 1x Potentiometer (used for adjusting sensor sensitivity).

2. User Interface and Dashboard (I/O)

    Role: Manages the menu system and displays live scores and training data.

    Components: 1.8" SPI TFT Display and 3x Tactile Push Buttons (for navigation and selection).

3. Haptic Feedback System (Output)

    Role: Provides immediate visual and audio alerts to the user when a target is hit.

    Components: 3x 16-LED RGB Rings and a Passive Buzzer.

4. Wireless Telemetry Bridge (Connectivity)

    Role: Sends performance results and hit data to a remote web dashboard via Wi-Fi.

    Components: ESP32-C3 (RISC-V) Module.


## Log


### Week 5:
- Research, decided on an idea
- Asked lab assistant about it, decided to improve it

### Week 6:
- Started to look at components 
- Decided on what I wanted to buy

### Week 7: 
- Waited for lab feedback
- Placed the order

### Week 8:
- Some components came, started to look into Ubuntu for this assignment
- Having problems with Windows/Linux environment but fixing them
- Received help with soldering some wires to my piezo sensors and RGB LED rings

## Hardware

The main component is the Nucleo STM32U545RE-Q board that acts as the central unit, analyzing real-time vibration data based on signals from three piezoelectric diaphragms taped underneath the training cups. To protect the microcontroller's ADC pins from high-voltage spikes caused by physical "thumps," each piezo is connected through a protection circuit using BAT41 Schottky diodes and high-value resistors. A potentiometer is integrated to provide a variable voltage signal, allowing the user to manually adjust the "Global Sensitivity" threshold for the training session.

For user feedback, the STM32 controls RGB LED rings to provide instant visual cues—turning green for a perfect hit, yellow for soft, and red for too hard—while a passive buzzer is tied to a PWM channel to generate audible warning tones. Detailed performance data, including reaction latency and impact force percentage, is transmitted via SPI to the display, which serves as a real-time scoreboard. Finally, the ESP32-C3 module handles Wi-Fi and Bluetooth connectivity, allowing the training data to be transmitted wirelessly for long-term progress tracking.

### Schematics

<center>
![Diagram](diagram.svg)
</center>

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32U545RE](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | [107 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D) |
| [4 * 35mm Piezoelectric Diaphragm Sensor](https://www.piezoelements.com/piezo-diaphragm/external-drive-piezo-diaphram/35mm-2600-hz-piezoelec-diaphragm.html) | Sensor to convert physical thumps to electrical signals | [47 RON](https://ro.mouser.com/ProductDetail/490-CEB-35FD29) |
| [12 * Schottky Diodes](https://www.electronics-tutorials.ws/diode/schottky-diode.html) | Part of protection circuit to protect from high-voltage spikes from sensors(I might get a voltage stabilizer but I don't know yet) | [12.84 RON](https://ro.mouser.com/ProductDetail/78-BAT41) |
| [6 * 10M Ohm Resistors](https://www.chipspulse.com/news/Understanding-10-Ohm-Resistors-Types-Uses-and-Applications_i0944.html) | Resistors for piezo sensors | [13 RON](https://ro.mouser.com/ProductDetail/594-VR37000001004FR5) |
| [20 * 220M Ohm Resistors](https://www.flywing-tech.com/blog/220-ohm-resistor-color-code/) | Resistors for LEDs | [3.42 RON](https://ro.mouser.com/ProductDetail/603-MFR-25FRF52-220R) |
| [20 * 330M Ohm Resistors](https://electronics.stackexchange.com/questions/27561/why-we-use-330-ohm-resistor-to-connect-a-led) | Resistors for LEDs | [2.6 RON](https://ro.mouser.com/ProductDetail/603-MFR-25FRF52-330R) |
| [2 * Potentiometer](https://www.thomann.de/ro/onlineexpert_page_potentiometers_what_is_a_potentiometer.html) | For adjusting sensitivity threshold of sensors | [7.52 RON](https://ro.mouser.com/ProductDetail/652-PTV09A-4020UB103) |
| [10 * RGB LED](https://www.build-electronic-circuits.com/rgb-led/) | Visual color feedback (I bought simple LEDs as a backup in case the rings don't work) | [10 RON](https://www.optimusdigital.ro/en/leds/483-rgb-led-common-cathode.html?search_query=0104210000002096&results=1) |
| [10 * Push Buttons](https://www.sameskydevices.com/blog/push-button-switches-101) | User input for the display | [3.6 RON](https://www.optimusdigital.ro/en/buttons-and-switches/1119-6x6x6-push-button.html?search_query=0104210000010862&results=1) |
| [6 * Capacitor](https://learn.sparkfun.com/tutorials/capacitors/all) | Part of protection circuit to protect from high-voltage spikes from sensors | [3.54 RON](https://www.optimusdigital.ro/en/capacitors/7850-electrolytic-capacitor-220-uf-50-v.html?search_query=0104210000050189&results=1) |
| [2 * Passive Buzzers](https://www.circuitbasics.com/what-is-a-buzzer/) | Audio feedback (success/failure) | [2 RON](https://www.optimusdigital.ro/en/buzzers/12247-3-v-or-33v-passive-buzzer.html?search_query=0104210000081527&results=1) |
| [4 * RGB LED Rings (link is not the same one)](https://wiki.seeedstudio.com/Grove-LED_ring/) | Visual color feedback | [38.36 RON](https://www.optimusdigital.ro/en/others/12342-inel-de-led-uri-rgb-ws2812-cu-16-led-uri.html?search_query=0104110000082336&results=1) |
| [ESP32-C3](https://documentation.espressif.com/esp32-c3_datasheet_en.pdf) | Wi-Fi communication | [34 RON](https://www.emag.ro/placa-de-dezvoltare-esp32-c3-rqiurpn-cu-modul-esp32-c3-mini-1-wi-fi-si-bluetooth5-0-4mb-flash-dimensiuni-compacte-11-kaifaban-esp32-c3/pd/D6NX9S3BM/) |
| [Breadboard + Jumper Wires](https://learn.sparkfun.com/tutorials/how-to-use-a-breadboard/all) | Connecting components | [34 RON](https://www.emag.ro/kit-breadboard-830-gauri-65-fire-modul-tensiune-alimentare-mb102-jh027/pd/DY1YP6BBM/) |
| [ST7735 1.8" TFT LCD Display Screen](https://www.displayfuture.com/Display/datasheet/controller/ST7735.pdf) | Display for scoreboards | [64.59 RON](https://www.aliexpress.com/item/1005005978400236.html?spm=a2g0o.order_list.order_list_main.28.12091802wM2lXc) |
| [~20 MTM Jumper Wires](https://blog.sparkfuneducation.com/what-is-jumper-wire) | Connecting components | [15.74 RON](https://ro.mouser.com/ProductDetail/474-PRT-11026) |
| [~40 MTM Jumper Wires (different lengths)](https://blog.sparkfuneducation.com/what-is-jumper-wire) | Connecting components | [21.63 RON](https://ro.mouser.com/ProductDetail/713-110990029) |
| [40 MTM Jumper Wires](https://blog.sparkfuneducation.com/what-is-jumper-wire) | Connecting components | [10 RON](https://www.emag.ro/set-40-cabluri-jumper-tata-tata-pentru-breadboard-multicolore-20cm-034-030/pd/D18P4G3BM/) |
| [40 MTF Jumper Wires](https://blog.sparkfuneducation.com/what-is-jumper-wire) | Connecting components | [10 RON](https://www.emag.ro/set-40-cabluri-jumper-tata-mama-pentru-breadboard-multicolore-20cm-034-031/pd/DQ8P4G3BM/) |
**I bought many spares because this is my first project and I might need them if I break something + many other components like jumper wires for future components (also bought from different sites because of quality differences)


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [stm32u5xx-hal](https://github.com/stm32-rs/stm32h5xx-hal) | Used for STM32 | ADC for piezos, PWM for the buzzer, GPIO for LEDs etc.|
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Drawing, scoreboard, percentage bars to the display. |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Embedded hardware | "Translator" between sensor code and the screen/peripherals. |
| [cortex-m-rt](https://github.com/rust-embedded/cortex-m-rt) | ARM Cortex-M | Tells the processor how to start up and manage the basic execution loop. |
| [st7735-lcd](https://github.com/almindor/st7789) | Driver for the 1.8" TFT LCD | Sending pixel data to screen. |
| [nb](https://github.com/rust-embedded/nb) | Non-blocking multitasking support | Checking for "thumps" while simultaneously running the game timer. |
| [micromath](https://github.com/tarcieri/micromath) | Fast math operations | Converting raw electrical spikes into force percentages to display on screen. |
| [heapless](https://github.com/rust-embedded/heapless) | Memory-efficient data structures | Storing high scores or last hits without using dynamic memory. |
| [panic-probe](https://github.com/knurling-rs/probe-run) | Panic handler | Error debugging. |
**TBC

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Embassy book](https://docs.rust-embedded.org/book/)
2. [Piezo sensors guide](https://learn.sparkfun.com/tutorials/piezo-vibration-sensor-hookup-guide)
3. [STM32 Datasheet](https://www.alldatasheet.com/datasheet-pdf/pdf/1583960/STMICROELECTRONICS/STM32U545RE.html)
4. [Information on protection circuit with diodes](https://www.electronics-tutorials.ws/diode/diode-clipping-circuits.html)
5. [RGB LED Rings guide](https://www.embedded.com/coding-tips-tricks-for-led-ring-lighting-effects/)
**TBC
