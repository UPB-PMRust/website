
# Orion
An autonomous guide assistant featuring smartphone-linked vector navigation and real-time intelligent disobedience.

:::info 

**Author**: Dorian-Gabriel Toros \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-TorosDorian-Gabriel

:::

<!-- do not delete the \ after your name -->

## Description

Orion is a robotic guide designed to assist visually impaired users with navigation. The system uses a smartphone for high-level tasks like voice processing and environmental recognition (identifying crosswalks or stoplights), while an STM32 handles the real-time movement and safety logic. The phone provides navigation targets—defined by a distance and an angle—which the robot executes. Running an Async Rust stack, the microcontroller manages these instructions while continuously monitoring a triple-ultrasonic sensor array. This enables "Intelligent Disobedience": if a command would lead to a collision or hazard, the robot overrides the input to stop or navigate around the obstacle, prioritizing the user's safety in both indoor and outdoor settings.

## Motivation

I chose this project because I wanted to make guidance more accessible to visually impaired people. While real guide dogs offer incredible emotional support, they aren't a realistic option for everyone due to the high costs involved. A trained guide dog can cost upwards of $50,000, and that’s before you factor in years of food, water, healthcare, and daily care, such as skill maintenance, grooming, and even playing with it. Plus, a dog’s lifespan is limited, so the process and expenses eventually have to start over. Orion is much more accessible. At a much cheaper total build cost, it provides a technical alternative that doesn't need food or vet visits. It has a virtually unlimited lifespan, as long as the hardware is powered and not faulty. While a robot might occasionally need a cheap spare part if something breaks, it’s nothing compared to the ongoing costs of a living animal.

## Architecture 
<div align="center">
  <img src="images/OrionArchitecture.svg" alt="Orion Architecture" style={{width: '60%'}} />
</div>
The system follows a decentralized command structure, where high-level strategy is handled by a mobile device and real-time safety is handled locally by the robot.

### Main Components
* **Instruction Layer:** A smartphone application that processes user commands and camera data to generate high-level navigation goals.

* **Communication Layer:** A wireless relay (ESP32) that bridges the gap between the smartphone's high-level commands and the robot’s low-level hardware.

* **Control Core:** This unit consists of the STM32 and the ultrasonic sensors array. It acts as the "source of truth", merging high-level instructions with real-time environmental data to manage the robot's physical behavior. It maintains a state history to recognize navigation deadlocks—where the robot is repeatedly blocked—allowing it to halt and alert the user independently.

* **Actuation Layer:** The physical hardware that turns logic into action, including the 4WD drive system and haptic feedback.

### Component Connection
The components are linked via a prioritized data chain:

* **Wireless Link (Wi-Fi/BLE):** Connects the Instruction Layer to the Communication Layer. It uses Wi-Fi for vision data and Bluetooth for low-latency destination commands.

* **Data Bridge (UART):** Connects the Communication Layer to the Control Core. It passes directional vectors (angle and distance) from the ESP32 to the STM32.

* **Safety Loop (GPIO/Timers):** Connects the sensors directly to the Control Core. This link allows the STM32 to measure distance and override navigation commands if an obstacle is determined as unavoidable.

* **Execution Link (PWM/GPIO):** Connects the Control Core to the motor drivers within the Actuation Layer for precise speed and direction control.

* **Haptic Feedback Link (Digital Out):** Connects the Control Core to the coin vibrators. This provides physical alerts to the user, signaling system status or immediate danger.

This architecture establishes the Control Core as the ultimate safety authority. Because the Safety Loop is hard-wired directly to the STM32, the robot maintains a real-time 'safety shield' that operates independently of the smartphone. In any conflict between user instructions and environmental reality—such as a detected unavoidable obstacle or a connection lag—the Control Core is programmed to prioritize safety by overriding external commands. Furthermore, the core is capable of Tactical Halts: if the robot determines it is physically stuck or unable to find a safe path after multiple attempts, it will cease movement and trigger haptic feedback to notify the user of the navigation deadlock, regardless of the smartphone's continued output.

## Log

<!-- write your progress here every week -->
### Week 23 - 29 March
Thought about and planned the project.
### Week 30 March - 5 April
Got my idea approved, started ordering materials needed.
### Week 6 - 12 April
All ordered materials arrived. I started doing small functionality tests to make sure I understand how they work.
### Week 13 - 19 April
Worked on the website for the project.
### Week 20 - 26 April

### Week 27 April - 3 May

### Week 4 - 10 May

### Week 11 - 17 May

### Week 18 - 24 May

## Hardware

Orion’s hardware is built around a multi-processor architecture to separate real-time control from high-bandwidth data handling. The STM32 acts as the primary core, executing Async Rust logic to process safety data and motor vectors. Connectivity is managed via a dedicated ESP32 module, while a separate ESP32-CAM is utilized specifically for environmental image capture.

For movement, the system uses a 4WD motor and wheel set adapted into a "walker-style" configuration for stable mobility. Obstacle detection is handled by a triple ultrasonic sensor array positioned for wide-angle coverage, ensuring the robot can detect hazards in its immediate path.

The power system is split into two distinct rails for stability: the 4WD motors are powered directly from the Li-ion batteries to handle high current spikes, while a buck converter steps the voltage down to a stable 5V for the breadboard-mounted logic circuits, protecting the microcontrollers from electrical interference.

### Schematics
<div align="center">
  <img src="images/OrionSchematics.svg" alt="Orion Schematics" style={{width: '60%'}} />
</div>

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|-------|-------|
| [ESP 32 DEVKIT V1, 1 PC](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32/esp-dev-kits-en-master-esp32.pdf) | Sending instructions from phone through BLE | [53.08](https://www.emag.ro/placa-dezvoltare-esp32-devkit-v1-ai669/pd/DXV9FDMBM/) |
| [10 FEMALE TO MALE DUPONT WIRE JUMPER CABLES, 20 CM, 2 PCS](https://www.budgetronics.eu/en/search?query=Wire+Jumper) | Connections | [12.22](https://www.emag.ro/10-x-fire-dupont-mama-tata-20cm-ai306-s459/pd/DZJ66JBBM/) |
| [STM32-NUCLEO-U545RE-Q, 1 PC](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main control unit - Handles logic, safety, and real-time processing. | [106.59](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D) |
| [5 MM LED, BLUE, 5 PCS](https://sigmanortec.ro/led-5mm-albastru) | Debugging | [1.50](https://sigmanortec.ro/led-5mm-albastru) |
| [5 MM LED, GREEN, 5 PCS](https://sigmanortec.ro/led-5mm-verde) | Debugging | [1.50](https://sigmanortec.ro/led-5mm-verde) |
| [5 MM LED, YELLOW, 10 PCS](https://sigmanortec.ro/led-5mm-galben) | Debugging | [3.00](https://sigmanortec.ro/led-5mm-galben) |
| [5 MM LED, WHITE, 5 PCS](https://sigmanortec.ro/led-5mm-alb) | Debugging | [1.50](https://sigmanortec.ro/led-5mm-alb) |
| [5 MM LED, RED, 5 PCS](https://sigmanortec.ro/led-5mm-rosu) | Debugging | [1.50](https://sigmanortec.ro/led-5mm-rosu) |
| [ELECTRET MICROPHONE CAPSULE, 1 PC](https://sigmanortec.ro/Microfon-electret-capsula-p126469106) | Microphone for hazard detection | [1.21](https://sigmanortec.ro/Microfon-electret-capsula-p126469106) |
| [760 POINTS BREADBOARD, 1 PC](https://sigmanortec.ro/Breadboard-760-puncte-p190992404) | Connections board between devices | [10.59](https://sigmanortec.ro/Breadboard-760-puncte-p190992404) |
| [40 MALE TO MALE DUPONT WIRE JUMPER CABLES, 10 CM, 1 PC](https://www.budgetronics.eu/en/search?query=Wire+Jumper) | Connections | [7.73](https://sigmanortec.ro/40-Fire-Dupont-30cm-Tata-Tata-p210849599) |
| [40 MALE TO MALE DUPONT WIRE JUMPER CABLES, 20 CM, 1 PC](https://www.budgetronics.eu/en/search?query=Wire+Jumper) | Connections | [8.97](https://sigmanortec.ro/40-Fire-Dupont-20cm-Tata-Tata-p210851325) |
| [MICRO VIBRATION MOTORS, 3V, 5 PCS](https://precisionminidrives.com/product/10mm-coin-vibration-motor-3mm-type-model-nfp-c1030?_gl=1*xjkzbp*_up*MQ..*_gs*MQ..&gclid=Cj0KCQjwkYLPBhC3ARIsAIyHi3SQnjs6q14PMiZl5raiINrToDman7eGPuO6rXBETsWOfCBPPjlN3WMaAt5WEALw_wcB&gbraid=0AAAAAomY7RVV_VKawgmUUJGBq9vz8B5WF)  | Guidance components | [31.44](https://www.drot.ro/platforma-arduino/121940-motor-minim-vibra-ie-1027-3v.html) |
| [USB TO TTL BOARD, COMPATIBLE WITH PL2303HX, 1 PC](https://www.waveshare.com/wiki/USB_TO_TTL?srsltid=AfmBOopE7a5oNn1drona_-J4jysm84IjQdlXgz6UY-KJ24-BUQzjfEzY) | Program ESP32 devkit and camera | [7.93](https://sigmanortec.ro/Modul-USB-to-TTL-PL2303HX-p126258532) |
| [DOUBLE CHARGER FOR 3,7 LI-ION BATTERIES, 1 PC](https://vectro.ro/produs/incarcator-dublu-pentru-acumulatori-3-7v-li-ion-de-tip-18650/?utm_source=Google%20Shopping&utm_campaign=Vectro%20-%20Toate%20Produsele&utm_medium=cpc&utm_term=7373&gad_source=1&gad_campaignid=21293681757&gbraid=0AAAAAohTOtapEAB2LjlZoaRI9CASu43mJ&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP8Duu6AAQvmcCC5XaQ0Kupz-owREnMsVfL4K9AViDASbNQz9pYK6ZUaAsFjEALw_wcB) | Batteries charger | [22.71](https://sigmanortec.ro/Incarcator-dublu-acumulatori-li-ion-18650-p187763453) |
| [ADJUSTABLE VOLTAGE REDUCER MODULE LM2596, 1 PC](https://sigmanortec.ro/Modul-coborator-tensiune-adjustabil-LM2596-DC-DC-4-5-40V-3A-p134532509?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72Nf2qtNjpEw_H0ficLHfF9fb&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP_B68CMckJ2TWs5OrsHeqlGc_ggYvQCgUk1qPfQ91RUH20Hn2JJBCEaAlavEALw_wcB) | Lowering the voltage received by breadboard compared to the motors driver | [6.69](https://sigmanortec.ro/Modul-coborator-tensiune-adjustabil-LM2596-DC-DC-4-5-40V-3A-p134532509?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72Nf2qtNjpEw_H0ficLHfF9fb&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP_B68CMckJ2TWs5OrsHeqlGc_ggYvQCgUk1qPfQ91RUH20Hn2JJBCEaAlavEALw_wcB) |
| [600 RESISTORS SET, 1 PC](https://mikrobot.pl/zestaw-600szt-rezystorow-0-25w-20x30-typow-opornik) | Fine tuning current and voltage received by different components through the breadboard | [31.57](https://www.emag.ro/set-600pcs-rezistente-0-25w-30-tipuri-toleranta-1-20-de-bucati-pentru-fiecare-valoare-ideal-pentru-proiecte-electronice-si-ingineri-include-rezistente-10-100-1k-10k-100k-pana-la-1m-utilizat-pentru-pro/pd/DBV351YBM/) |
| [WOOD SAW 20 UNI, 1 PC](https://www.bricodepot.ro/fierastrau-lemn-opp1-40cm-negru/cpd/101497509/) | Cut wood | [9.90](https://www.bricodepot.ro/fierastrau-lemn-opp1-40cm-negru/cpd/101497509/) |
| [INSULATING TAPE, 1 PC](https://www.bricodepot.ro/banda-izolatoare-diall-100608800-19-mm-x-10-m-maro/cpd/100608800/) | Material for cable joints and fixing | [3.50](https://www.bricodepot.ro/banda-izolatoare-diall-100608800-19-mm-x-10-m-maro/cpd/100608800/) |
| [MYF CABLE 1,5D, 25M, RED, 1 PC](https://www.bricodepot.ro/cablu-electric-myf-1-x-1-5-mmp-cupru-flexibil-rosu-rola-100-m/cpd/100858555/) | Connections | [25.57](https://www.bricodepot.ro/cablu-electric-myf-1-x-1-5-mmp-cupru-flexibil-rosu-rola-100-m/cpd/100858555/) |
| [100 PCS PLASTIC TIES, 2.5 X 100 MM,1 PC](https://www.dedeman.ro/ro/banda-zimtata-friulsider-36300m251000c-alb-2-5-x-100-mm-100-bucati/p/1027620) | Holding cables in place | [4.11](https://www.dedeman.ro/ro/banda-zimtata-friulsider-36300m251000c-alb-2-5-x-100-mm-100-bucati/p/1027620) |
| [HOFF CABLE STRIPPING PLIERS, 15 CM, 1 PC](https://www.dedeman.ro/ro/cleste-dezizolat-cablu-hoff-15-cm/p/1057841) | Stripping tool for cable insulation removal | [26.67](https://www.dedeman.ro/ro/cleste-dezizolat-cablu-hoff-15-cm/p/1057841) |
| [HOFF DIGITAL MULTIMETER, 1 PC](https://www.dedeman.ro/ro/multimetru-digital-hoff-masurare-intensitate-curent-dc-masurare-tensiune-ac/-dc-masurare-rezistenta-tester-dioda/-tranzistor/p/1040177) | Measurement tool for intensity, voltage, resistance, tester for diodes / transistors | [38.90](https://www.dedeman.ro/ro/multimetru-digital-hoff-masurare-intensitate-curent-dc-masurare-tensiune-ac/-dc-masurare-rezistenta-tester-dioda/-tranzistor/p/1040177) |
| [AS4 12-POLE STRING TERMINAL, 1 PC](https://www.dedeman.ro/ro/cleme-sir-12-poli-as4-683-004/p/1009533) | Joining cables electrically without soldering | [3.99](https://www.dedeman.ro/ro/cleme-sir-12-poli-as4-683-004/p/1009533) |
| [ELECTRICAL CONDUCTOR MYF / H07V-K 1.5 SMM BLUE, COPPER, 1 M, 5 PCS](https://www.dedeman.ro/ro/conductor-electric-myf/-h07v-k-1-5-mmp-albastru-cupru/p/1014039) | Connections | [6.00](https://www.dedeman.ro/ro/conductor-electric-myf/-h07v-k-1-5-mmp-albastru-cupru/p/1014039) |
| [AIR DUSTER, SPRAY, 1 PC](https://www.bricodepot.ro/spray-gaz-comprimat-elico-air-duster-400-ml/cpd/101457811/) | Dust removal from electronic boards | [12.50](https://www.bricodepot.ro/spray-gaz-comprimat-elico-air-duster-400-ml/cpd/101457811/) |
| [SOLDERING ALLOY (TIN), 2.5 MM, 100 G CYNEL, 1 PC](https://www.bricodepot.ro/aliaj-cositor-de-lipit-2-5-mm-100-g-cynel/cpd/100710863) | Soldering wires | [50.00](https://www.bricodepot.ro/aliaj-cositor-de-lipit-2-5-mm-100-g-cynel/cpd/100710863) |
| [VOREL 79358 SOLDERING IRON, 40 W, 420 DEGREES C, 1 PC](https://www.bricodepot.ro/ciocan-de-lipit-vorel-79358-40-w-420-grade-c/cpd/101468984/) | Soldering wires | [49.90](https://www.bricodepot.ro/ciocan-de-lipit-vorel-79358-40-w-420-grade-c/cpd/101468984/) |
| [SOLDERING PASTE, HONEST 660069, 30 GRAMS, 1 PC](https://www.bricodepot.ro/pasta-decapanta-pentru-lipit-honest-660069-30-grame/cpd/100873934/) | Cleaning paste for soldering | [7.19](https://www.bricodepot.ro/pasta-decapanta-pentru-lipit-honest-660069-30-grame/cpd/100873934/) |
| [HC-SR04 ULTRASONIC SENSOR SET 3, 1 PC](https://www.bitmi.ro/senzor-ultrasonic-hc-sr04-10406.html?gad_source=1&gad_campaignid=21312430054&gbraid=0AAAAADLag-kY65avfg7fYwyFc_-mN_7T9&gclid=CjwKCAjwnZfPBhAGEiwAzg-VzL7p81cH_cfstDowyUPj8QIg6qOyC9zN7ajCw_kLp-hYWMshhzVoOxoCjyUQAvD_BwE) | Obstacle detection | [44.41](https://www.emag.ro/set-3x-senzor-ultrasonic-hc-sr04-modul-masurare-distanta-2-400cm-5v-compatibil-arduino-si-raspberry-pi-pentru-robotica-si-proiecte-diy-et000019/pd/D6SMS83BM/?ref=sponsored_products_search_f_b_1_1&recid=recads_1_68e27e25250c86a815ad0536a903b64d3d1b9bd7526c1f57e7c49d47044ca35d_1776669729&aid=35fea07d-3988-11f1-801c-06eaf0d4245d%7Ck%7C303565777&oid=303565777&scenario_ID=1) |
| [MOTOR SET TT DC WITH WHEEL, 4 PCS](https://sigmanortec.ro/Kit-Motor-reductor-Roata-plastic-cu-cauciuc-p134585625) | Motors for movement | [36.30](https://www.emag.ro/set-motoreductor-cu-roata-4-bucati-3874783591881/pd/DMXQ1DYBM/) |
| [SWITCHING DIODE 1N4148 THT 100V, 10 PCS](https://hobbymarket.ro/dioda-comutatie-1n4148-tht-100v.html?gad_source=1&gad_campaignid=20857847252&gbraid=0AAAAADBDNEv71EYkdhhWHsZW_515cLmC1&gclid=CjwKCAjw14zPBhAuEiwAP3-Eb8FkXWwwc_RVhUZJwbhq2WwDkfDc9zmRUIvvOIKu9gQUElQpdaAkFRoCA8YQAvD_BwE) | Protect transistors | [2.00](https://hobbymarket.ro/dioda-comutatie-1n4148-tht-100v.html?gad_source=1&gad_campaignid=20857847252&gbraid=0AAAAADBDNEv71EYkdhhWHsZW_515cLmC1&gclid=CjwKCAjw14zPBhAuEiwAP3-Eb8FkXWwwc_RVhUZJwbhq2WwDkfDc9zmRUIvvOIKu9gQUElQpdaAkFRoCA8YQAvD_BwE) |
| [3A 250V SWITCH MTS-102, 1 PC](https://sigmanortec.ro/Intrerupator-3A-250V-Switch-MTS-102-p140901357?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72Nf2qtNjpEw_H0ficLHfF9fb&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP8gQnGQ34wJPpWXY7FaiTQIdyoNSjhoZ8X1-auMHVYKOxmric1-xJoaApELEALw_wcB) | Battery switch | [2.19](https://sigmanortec.ro/Intrerupator-3A-250V-Switch-MTS-102-p140901357?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72Nf2qtNjpEw_H0ficLHfF9fb&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP8gQnGQ34wJPpWXY7FaiTQIdyoNSjhoZ8X1-auMHVYKOxmric1-xJoaApELEALw_wcB) |
| [1W 8 OHM MINI SPEAKER 15X10MM, 1 PC](https://electronicmarket.ro/mini-difuzor-1w-8-ohm-15x10mm?gad_source=1&gad_campaignid=21513542058&gbraid=0AAAAA-D1O9Z2KXrM8jS931hq9EL83sPFJ&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP_a2yCMv4AK6_UaiDeDs7tWybYWKjtvy7sLGye_PY2C33NM-HYdElMaAm8JEALw_wcB) | Speaker for sound effects | [3.82](https://electronicmarket.ro/mini-difuzor-1w-8-ohm-15x10mm?gad_source=1&gad_campaignid=21513542058&gbraid=0AAAAA-D1O9Z2KXrM8jS931hq9EL83sPFJ&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP_a2yCMv4AK6_UaiDeDs7tWybYWKjtvy7sLGye_PY2C33NM-HYdElMaAm8JEALw_wcB) |
| [CERAMIC CAPACITOR, 1µF 50V, 10 PCS](https://www.drot.ro/platforma-arduino/7831-condensator-1uf-50v.html) | Signal coupling & noise decoupling | [2.00](https://hobbymarket.ro/condensator-ceramic-100nf-50v-tht-2-54mm.html?gad_source=1&gad_campaignid=20857847252&gbraid=0AAAAADBDNEvLR754tfYetQhY_WCZl3Ead&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP9gf_lvvJqJuLN52qkuRLOGDWmkkc7w0ePMgD32SWxfI6tsAJK3flAaAkP_EALw_wcB) |
| [CERAMIC CAPACITOR, 100nF 50V, THT, 10 PCS](https://hobbymarket.ro/condensator-ceramic-100nf-50v-tht-2-54mm.html?gad_source=1&gad_campaignid=20857847252&gbraid=0AAAAADBDNEvLR754tfYetQhY_WCZl3Ead&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP9gf_lvvJqJuLN52qkuRLOGDWmkkc7w0ePMgD32SWxfI6tsAJK3flAaAkP_EALw_wcB) | Signal coupling & noise decoupling | [2.00](https://hobbymarket.ro/condensator-ceramic-100nf-50v-tht-2-54mm.html?gad_source=1&gad_campaignid=20857847252&gbraid=0AAAAADBDNEvLR754tfYetQhY_WCZl3Ead&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP9gf_lvvJqJuLN52qkuRLOGDWmkkc7w0ePMgD32SWxfI6tsAJK3flAaAkP_EALw_wcB) |
| [ELECTROLYTIC CAPACITOR, 100µF, 35V – 135074, 1 PC](https://www.emag.ro/condensator-electrolitic-100uf-35v-135074/pd/DB4LFMMBM/?cmpid=146618&utm_source=google&utm_medium=cpc&utm_campaign=%28RO:eMAG!%29_3P_NO_SALES_%3e_Iluminat_and_electrice&utm_content=76376892625&gad_source=1&gad_campaignid=2088295595&gbraid=0AAAAACvmxQi2FGCzf1B_tiviD3GSBXkyq&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP_fqiK72AVpEwWstyO95sBxWAiZhRUmCDcBdc9ou3Iyxpk34OzU_VUaApySEALw_wcB) | Power rail stabilizer | [4.40](https://www.emag.ro/condensator-electrolitic-100uf-35v-135074/pd/DB4LFMMBM/?cmpid=146618&utm_source=google&utm_medium=cpc&utm_campaign=%28RO:eMAG!%29_3P_NO_SALES_%3e_Iluminat_and_electrice&utm_content=76376892625&gad_source=1&gad_campaignid=2088295595&gbraid=0AAAAACvmxQi2FGCzf1B_tiviD3GSBXkyq&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP_fqiK72AVpEwWstyO95sBxWAiZhRUmCDcBdc9ou3Iyxpk34OzU_VUaApySEALw_wcB) |
| [MINIATURE AMPLIFIER MODULE, PAM8403, 2.2-5V, 1 PC](https://sigmanortec.ro/modul-amplificator-miniatura-pam8403-22-5v?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72Nf2qtNjpEw_H0ficLHfF9fb&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP8-G7VREEPbiB-0RCPnZ7qMJAiaeSx2vYZ7EkKk6PtkS0g6a27pHRoaAhjiEALw_wcB) | Amplifier for audio signal | [3.58](https://sigmanortec.ro/modul-amplificator-miniatura-pam8403-22-5v?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72Nf2qtNjpEw_H0ficLHfF9fb&gclid=Cj0KCQjw-pHPBhCdARIsAHXYWP8-G7VREEPbiB-0RCPnZ7qMJAiaeSx2vYZ7EkKk6PtkS0g6a27pHRoaAhjiEALw_wcB) |
| [GEAR WHEELS SET 34, P134867239, 1 PC](https://sigmanortec.ro/Kit-rotite-dintate-set-34-bucati-p134867239) | Torque-conversion | [17.62](https://sigmanortec.ro/Kit-rotite-dintate-set-34-bucati-p134867239) |
| [40 MALE TO MALE DUPONT WIRE JUMPER CABLES, 30 CM, 1 PC](https://sigmanortec.ro/40-Fire-Dupont-30cm-Tata-Tata-p210849599) | Connections | [8.39](https://sigmanortec.ro/40-Fire-Dupont-30cm-Tata-Tata-p210849599) |
| [WHITE HDF WOOD BOARD, 1 PC](https://www.bricodepot.ro/placa-hdf-kronospan-3-mm-x-2800-x-1032-mm-alb/cpd/101238184/?srsltid=AfmBOor_y4UX9uSHFdBWrywqH6-yZXWOCuzSwR0_p1FjsdFbU-iZSsoa) | Parts making | [79.00](https://www.bricodepot.ro/placa-hdf-kronospan-3-mm-x-2800-x-1032-mm-alb/cpd/101238184/?srsltid=AfmBOor_y4UX9uSHFdBWrywqH6-yZXWOCuzSwR0_p1FjsdFbU-iZSsoa) |
| [TRANSISTORS 2N2222 NPN BJT, 10PC](https://electronicmarket.ro/2n2222-npn-bjt-tranzistor?gad_source=1&gad_campaignid=21513542058&gbraid=0AAAAA-D1O9ZA0L4JDzzbJYoYmkXXuDY_4&gclid=CjwKCAjw14zPBhAuEiwAP3-Eb6EK_mGV1SePNx7A3Mm-tS8VSTG5fux7wbn0grCdo2XWrinshkSh1BoCdFgQAvD_BwE) | Protect coin vibrators by changing received current | [1.40](https://electronicmarket.ro/2n2222-npn-bjt-tranzistor?gad_source=1&gad_campaignid=21513542058&gbraid=0AAAAA-D1O9ZA0L4JDzzbJYoYmkXXuDY_4&gclid=CjwKCAjw14zPBhAuEiwAP3-Eb6EK_mGV1SePNx7A3Mm-tS8VSTG5fux7wbn0grCdo2XWrinshkSh1BoCdFgQAvD_BwE) |
| [METAL NUT, 10MM DIAMETER, 8 PC](https://www.bricodepot.ro/piulita-hexagonala-m10-diall-otel-inoxidabil-a2-argintiu-10-buc/cpd/100657607/) | Construction | [3.84](https://www.bricodepot.ro/piulita-hexagonala-m10-diall-otel-inoxidabil-a2-argintiu-10-buc/cpd/100657607/) |
| [METAL SCREW, M10X20MM BLACK 8 PC](https://www.bricodepot.ro/surub-metric-m10-x-20mm-otel-negru-vrac-50buc-kg/cpd/101310772/) | Construction | [10.03](https://www.bricodepot.ro/surub-metric-m10-x-20mm-otel-negru-vrac-50buc-kg/cpd/101310772/) |
| [MAGNUSSON PROFESSIONAL CUTTER, 1 PC](https://www.bricodepot.ro/cutter-utilitar-magnusson-61-mm-zinc/cpd/100640487/) | Construction | [37.00](https://www.bricodepot.ro/cutter-utilitar-magnusson-61-mm-zinc/cpd/100640487/) |
| [UNIVERSAL GLUE, 1 PC](https://www.soudal.ro/diy/produse/adezivi/adezivi-speciali/super-glue-lichid) | Construction | [28.46](https://www.outlet-ieftin.ro/cumpara/adeziv-profesional-super-glue-20g-soudal-12469?srsltid=AfmBOopcElHFufeojivBTbsLZOIlDgugPTIELOZKG0mPrbpLwB8NsM6p) |
| [COUNTERSUNK HEAD SCREW, PZ2, 1 SET](https://www.bricodepot.ro/surub-pentru-pal-lemn-cap-inecat-3-5-x-16-mm-pz2-za-vrac/cpd/101308115/) | Construction | [9.00](https://www.bricodepot.ro/surub-pentru-pal-lemn-cap-inecat-3-5-x-16-mm-pz2-za-vrac/cpd/101308115/) |
| [SCISSOR LIMITER 125X125MM, 4PC](https://www.bricodepot.ro/limitator-foarfece-300-05-2x125mm/cpd/101305288/) | Construction | [56.00](https://www.bricodepot.ro/limitator-foarfece-300-05-2x125mm/cpd/101305288/) |
| [HINGE 40X40MM, 4PC](https://www.bricodepot.ro/balama-40x40-mm-2-mm-grosime/cpd/101250188/) | Construction | [18.36](https://www.bricodepot.ro/balama-40x40-mm-2-mm-grosime/cpd/101250188/) |
| [ESP32-CAM, 1PC](https://www.espressif.com/en/news/ESP32_CAM) | Vision of surroundings | [69.73](https://sigmanortec.ro/placa-dezvoltare-esp32-cam-wifi-bluetooth-ov2640-2mp?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72NaeHUezr_l2tenEtN_DWuo-&gclid=CjwKCAjwwJzPBhBREiwAJfHRncs6iBeoE0wPaSECFaCZAgI91YlvsW3P0Mivj8F67mw68YUT3yr1gBoCHEMQAvD_BwE) |
| [MOTOR DRIVER L298N, 1PC](https:/www.handsontec.com/dataspecs/L298N%20Motor%20Driver.pdf) | Control the motors | [11.99](https://www.bitmi.ro/modul-driver-l298n-cu-punte-h-dubla-pentru-motoare-dc-stepper-10400.html?gad_source=1&gad_campaignid=22990790771&gbraid=0AAAAADLag-l-Qerv9f0lSY9XqqBlFnwNA&gclid=CjwKCAjwwJzPBhBREiwAJfHRnfinvN0rXoyvlGJ25nfHMtS9aOkRWboLQOHjwRcBMDjc36QEGa-khxoCv7QQAvD_BwE) |
| [BATTERIES 3.7V 2500mAh LI-ION CELLS 18650, 2PC](https://www.a2t.ro/default-category/acumulator-li-ion-sony-18650-25r-inr-3-7v-2500mah-descarcare-20a?gad_source=1&gad_campaignid=21154157017&gclid=CjwKCAjwwJzPBhBREiwAJfHRnY9QXvOL2PKbyJ5mpJ0N7jnGqOhSX-3I-1vSrV_tmaTLDerpL75RmxoCVAwQAvD_BwE) | Power | [57.98](https://www.a2t.ro/default-category/acumulator-li-ion-sony-18650-25r-inr-3-7v-2500mah-descarcare-20a?gad_source=1&gad_campaignid=21154157017&gclid=CjwKCAjwwJzPBhBREiwAJfHRnY9QXvOL2PKbyJ5mpJ0N7jnGqOhSX-3I-1vSrV_tmaTLDerpL75RmxoCVAwQAvD_BwE) |
| Total |     | 1057.46 |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [Rust](https://www.rust-lang.org/) | Systems programming language | Core implementation |
| [cortex-m](https://crates.io/crates/cortex-m) | ARM Cortex-M low-level access | Register and interrupt control |
| [cortex-m-rt](https://crates.io/crates/cortex-m-rt) | Runtime support | Startup and interrupt handling |
| [embedded-hal](https://crates.io/crates/embedded-hal) | Hardware abstraction layer | Peripheral interfaces |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async executor | Runs concurrent tasks |
| [embassy-time](https://crates.io/crates/embassy-time) | Time management | Delays and scheduling |
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | STM32 HAL | GPIO, UART, PWM control |
| [embassy-sync](https://crates.io/crates/embassy-sync) | Synchronization primitives | Task coordination |
| [heapless](https://crates.io/crates/heapless) | Fixed-size data structures | Memory-safe buffers |
| [defmt](https://crates.io/crates/defmt) | Logging framework | Debugging output |
| [defmt-rtt](https://crates.io/crates/defmt-rtt) | RTT transport | Real-time logs |
| [panic-probe](https://crates.io/crates/panic-probe) | Panic handler with logging | Debugging crashes |
| [nb](https://crates.io/crates/nb) | Non-blocking abstractions | Peripheral communication |
| [esp-hal](https://crates.io/crates/esp-hal) | ESP32 hardware abstraction | Control of ESP32 peripherals |
| [esp-radio](https://crates.io/crates/esp-radio) | Wireless communication stack | WiFi/BLE communication |
| [smoltcp](https://crates.io/crates/smoltcp) | Embedded TCP/IP stack | Networking for ESP32 |
| [esp-alloc](https://crates.io/crates/esp-alloc) | Heap allocator for ESP | Dynamic memory on ESP32 |


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [crates.io](https://crates.io/)
2. [embedded-rust](https://embedded-rust-101.wyliodrin.com/)
3. [STM32-U545RE-Q datasheet](https://www.st.com/resource/en/datasheet/stm32u545ce.pdf)
4. [embassy.dev](https://embassy.dev/)
