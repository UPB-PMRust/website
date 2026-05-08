# Smart Dual-Barrier Parking System with NFC and Coin Detection


A smart parking barrier system designed to simulate a realistic automated parking management 

::info 
**Author**: Mira Mehdi Bark \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-MiraB8421

:::

## Description

This project demonstrates a smart parking barrier system built using the STM32 NUCLEO-U545RE microcontroller. When a vehicle arrives at the entrance, an ultrasonic sensor detects it and opens the entry barrier while registering the vehicle as unpaid.Moreover, at the payment station the driver must scan an NFC tag and insert a coin that is detected by an IR sensor to confirm payment, after which the system updates the vehicle status. At the exit barrier, the NFC tag is scanned again and the system verifies payment if the payment is confirmed, the barrier opens with a green LED and confirmation message on the LCD shows, otherwise the barrier stays closed and a red LED appears, buzzer alerts, and payment is denied.

## Motivation

The motivation of this project is to show how a real life parking control works to improve efficiency, security, and automation in everyday services.Also, It provides practical experience in working with hardware components, system integration, and verification logic, making it a valuable learning platform for modern intelligent transportation and smart city applications.

## Architecture

This system is built using **STM32 NUCLEO-U545RE-Q** which coordinates all functions:

* **Vehicle Detection Module:** Detects the arrival of a car at the entry gate and sends a signal to the control unit to open the entry barrier that the car is registered as **NOT PAID**.
* **Payment Processing Module:** Manages NFC identification and coin confirmation together before sending payment data to the controller and verifies the two payment methods to change the status in the system as **PAID**.
* **Exit Verification Module:** Checks the vehicle’s payment status using the second NFC scan at the exit gate and if the payment is confirmed or denied.
* **User Notification Module:** Displays messages and activates visual and audio alerts depending on payment status.

![Schematic diagram](diagram.webp)
## Log

## Week 2
* Worked on the word documentation to be able to implement the idea of the project.

## Week 4 - 5
* Ordered the hardware components for the project

## Week 7
* Started to connect components

## Hardware

The system is built around the STM32 NUCLEO-U545RE microcontroller, which acts as the main controller responsible for processing data and coordinating all modules. Two NFC readers are used to scan the vehicle’s NFC tag at the payment and exit stations for verification. Two SG90 servo motors control the opening and closing of the entry and exit barriers, while a LCD display shows system messages such as payment status. A green LED indicates successful payment and access approval, whereas a red LED with a buzzer signals payment denied. An ultrasonic sensor detects vehicle presence at the entrance to open the berrier, and an IR sensor detects coin insertion at the payment station. However, all the coennections are on the breadboard and using jumper wires.

## Schematics



## Bill of Materials

| Device | Usage | Price |
| :--- | :--- | :--- |
| [STM32 NUCLEO-U545RE-Q](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D&utm_id=6470900573&utm_source=google&utm_medium=cpc&utm_marketing_tactic=emeacorp&gad_source=1&gad_campaignid=6470900573&gbraid=0AAAAADn_wf1Bn3QAN7vnR3zNOR5YbzCJB&gclid=CjwKCAjw46HPBhAMEiwASZpLRGsr429DKmxJjS_l8ra5qBHsnehbK18Lb_vNyvXPiFPbeGzHYWjdFRoCBhgQAvD_BwE) | The main microcontroller | [125 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D) |
| [Set of two NFC Reader](https://www.st.com/en/nfc.html) | Card/User authentication | [84 RON](https://www.emag.ro/set-de-2-module-pn532-nfc-rfid-v3-kit-cititor-nfc-si-rfid-pn532-include-card-nfc-cip-rfid-si-card-rfid-compatibil-cu-arduino-telefoane-android-si-microcontrolere-et000008/pd/DSM0JR3BM/?ref=fav_pd-title) |
| [Servo Motor](https://components101.com/motors/servo-motor-basics-pinout-datasheet) | Barrier Movement | [56 RON](https://www.emag.ro/servomotor-sg90-180-de-grade-ai156-s297/pd/D33V1GMBM/) |
| [LCD](https://www.vishay.com/docs/37484/lcd016n002bcfhet.pdf) | Display Status Message | [24 RON](https://www.emag.ro/display-lcd-2-x-16-cu-convertor-i2c-80-x-35-mm-verde-albastru-negru-2-e-001/pd/DHRJ0LMBM/?ref=fav_pd-title) |
| [LED Kit](https://www.emag.ro/kit-200-buc-led-3mm-5mm-de-diferite-culori-ai707/pd/D4DJYTMBM/?ref=fav_pd-title) | Access indicator | [25 RON](https://www.emag.ro/kit-200-buc-led-3mm-5mm-de-diferite-culori-ai707/pd/D4DJYTMBM/?ref=fav_pd-title) |
| [Buzzer](https://components101.com/misc/buzzer-pinout-working-datasheet) | Warning sound output | [19 RON](https://www.emag.ro/buzzer-activ-12v-compatibil-arduino-raspberry-oky0151-dh000016/pd/DH5QQL3BM/?ref=fav_pd-title) |
| [Jumper Wires](https://www.emag.ro/set-40-cabluri-jumper-tata-tata-pentru-breadboard-multicolore-20cm-034-030/pd/D18P4G3BM/?ref=fav_pd-title) | Components connections | [10 RON](https://www.emag.ro/set-40-cabluri-jumper-tata-tata-pentru-breadboard-multicolore-20cm-034-030/pd/D18P4G3BM/?ref=fav_pd-title) |
| [Ultrasonic distance sensor](https://components101.com/sensors/ultrasonic-sensor-working-pinout-datasheet)  | Sensor detection | [37 RON](https://www.emag.ro/modul-senzor-ultrasonic-detector-distanta-hc-sr04-xbaxah-ultrasonic/pd/D5HMPD2BM/) |
| [Breadboard](https://www.emag.ro/breadboard-h-hct-tronic-830-puncte-de-conectare-abs-200x630-puncte-034-066/pd/DBNQ7R3BM/?ref=fav_pd-title) | Circuit Prototyping | [10 RON](https://www.emag.ro/breadboard-h-hct-tronic-830-puncte-de-conectare-abs-200x630-puncte-034-066/pd/DBNQ7R3BM/?ref=fav_pd-title) |
| [IR sensor](https://www.vishay.com/docs/82843/tssp940.pdf) | Coin detection | [19 RON](https://www.emag.ro/modul-senzor-evitare-obstacol-ir-ai395-s333/pd/DNB572MBM/?ref=fav_pd-title) |


## Software

| Library | Description | Usage |
| :--- | :--- | :--- |
|[embassy-executor](https://crates.io/crates/embassy-executor) | Await executor optimized for embedded systems |  Without an operating system it runs asynchronous |
| [embassy-embedded-hal](https://crates.io/crates/embassy-embedded-hal) | Async hardware interface for embedded-hal | Used for control of SPI communication like sensors and peripherals |
| [embassy-time](https://crates.io/crates/embassy-time) | Timekeeping, delays, and timeouts | Used for timers,delays and timeouts |
| [embassy-sync](https://crates.io/crates/embassy-sync) | no-std, no synchronization primitives with async support. | Used to create safe communication between tasks |
| [embassy-defmt](https://defmt.ferrous-systems.com/) | Debug logging system | Used for printing debug message fromthe microcontroller |
| [embassy-futures](https://docs.embassy.dev/embassy-futures/git/default/index.html) | Async help utilities | Used to mange tasks and futures embedded programs |




## Links
1. [NFC implementation](https://www.compilenrun.com/docs/iot/stm32/stm32-connectivity/stm32-nfc-implementation/)
2. [Inspriation](https://www.udemy.com/course/arduino-parking-system/?utm_campaign=BG-Search_DSA_Beta_Prof_la.EN_cc.ROW-English&utm_source=bing&utm_medium=paid-search&portfolio=Bing-ROW-English&utm_audience=mx&utm_tactic=nb&utm_term=_._ag_1326013411676364_._ad__._kw_IT+en&utm_content=o&funnel=&test=&utm_campaign_id=638596229&msclkid=fe60804b87d2130a5d24e9a273befa2b&couponCode=PMNVD2025)

