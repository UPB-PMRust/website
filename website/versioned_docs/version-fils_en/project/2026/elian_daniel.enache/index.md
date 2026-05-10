# RustyPlotter
A 2-axis pen plotter with a solenoid-actuated Z-axis

:::info 

**Author**: Enache Elian-Daniel \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-ibbnog

:::

<!-- do not delete the \ after your name -->

## Description

The RustyPlotter is a 2-axis CNC pen plotter using the STM32 NUCLEO-U545RE-Q as its microcontroller. It uses the Embassy asynchronous firmware stack in Rust to concurrently read the .gcode files from the SD-card, drive the motors for X/Y motion and actuate the solenoid for the pen to move and write. Because it uses a display with an integrated SD-card, it can run entirely without any other computer, as it takes its power from an external power supply.

## Motivation

My initial inspiration definitely came from my passion for 3D printers. While I initially wanted to do a full 3D printer build, I ended up taking notes from Creality Ender 5-style machines for the skeleton and motion system of this plotter. There is also a great aspect of upcycling parts, as some of the hardware comes from a disassembled Tronxy X5SA-Pro I had lying around. Because the software needs to be written entirely in Rust (unlike standard C++ 3D printer firmware like Marlin), this project presents a great opportunity to explore asynchronous embedded programming. Furthermore, I wanted to execute the plotter idea differently compared to most DIY solutions by using a solenoid to push and pull the pen, rather than the usual servomotor handling its position.

## Architecture 

The main components of the plotter work together to complete the build:
 - **STM32 NUCLEO-U545RE-Q**: The brain of the machine, running the Embassy executor
 - **NEMA17 Stepper Motors (with A4988 Drivers)**: Provide X and Y-axis planar motion via GPIO step/direction signals
 - **JF-0530B Solenoid (with a Dual MOSFET Module)**: Moves the pen up and down, actuated via PWM
 - **Mechanical Endstops**: Get the origin (home) 0,0 position for X and Y-axis via GPIO input
 - **RepRapDiscount Full Graphic Smart Controller (with an SD-card)**: Manages the entire user interface and reads the .gcode files for plotting over SPI
 

![RustyPlotter Architecture](rustyplotterschematic.svg)

## Log

<!-- write your progress here every week -->

### Weeks 1 - 5
Thought of the initial project idea. Drafted the initial project proposal and documentation. The original concept was a 3-axis plotter inspired by 3D printer cube frames, using an aluminum bed driven by two Z-axis stepper motors on leadscrews.

### Week 6

Received feedback on the initial project proposal from the lab assistant. As per suggestion, I fundamentally changed the Z-axis mechanical design to instead use a fixed bed with a solenoid mounted on the X-axis carriage to handle the pen movement, reducing mechanical complexity.

### Weeks 7-8

I started more research on the components, especially what solenoid to use and what its requirements are. I figured out the necessary driver components (JF-0530B solenoid, Dual MOSFET module as suggested, flyback diode and decoupling capacitors). Also started purchasing the necessary hardware components and began testing the stepper motors in Rust.

### Week 9

I mapped out the hardware architecture diagram and wrote the system documentation for the first documentation milestone.

## Hardware

The plotter is built upon a rigid skeleton of 2020/2040 aluminum extrusions. Planar motion across the X and Y-axis is achieved using GT2 belts, pulleys, and V-slot wheels driven by two NEMA17 stepper motors. The electrical components are all connected through an 830-point breadboard with breadboard wires and jumpers. The NEMA17 steppers are driven by 2 A4988 stepper drivers, interfacing with the STM32 NUCLEO-U545RE-Q via GPIO. For protection of said drivers, 100uF 35V decoupling capacitors are added to help against voltage spikes. For the pen actuation, a JF-0530B 12V push-pull solenoid is used. It is controlled by a 15A 400W Dual Mosfet module, interfacing with the microcontroller via PWM. To prevent reverse voltage spikes from the solenoid getting to the MOSFET module, a 1N4007 flyback diode is used. The mechanical endstops, via GPIO, establish a known origin position for the X and Y-axis. In order to not get false triggers, 10kOhm external pull-up resistors and 0.47uF 50V debouncing capacitors are added. The machine is controlled (via SPI) with the help of the RepRapDiscount Full Graphic Smart Controller, which contains a 12864 LCD with a rotary encoder, used for navigating through the menus, showing status and selecting the .gcode files for plotting, which are stored on an SD-card plugged into the SD-card reader, the other component of the Smart Controller. The entire system is powered by a 12V 6A power supply with a DC barrel-jack, with 5V particularly going to the STM32 via a 12-24V to 5V 5A step-down module.

<!--### Schematics

Place your KiCAD or similar schematics here in SVG format.-->

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [x1 STM32 NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | The microcontroller | [112.47 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D) |
| [x1 12V 6A Power Supply](https://www.optimusdigital.ro/ro/surse-ac-dc-de-12-v/8016-alimentator-stabilizat-12-v-6000-ma.html) | Supplies power to the board and the other components | [49.99 RON](https://www.optimusdigital.ro/ro/surse-ac-dc-de-12-v/8016-alimentator-stabilizat-12-v-6000-ma.html) |
| [x1 12/24V to 5V 5A Step-down Module](https://www.optimusdigital.ro/ro/surse-coboratoare/12484-sursa-coboratoare-dc-dc-de-la-24v12v-la-5v-5a.html) | Provides the STM32 with 5V | [already owned](https://www.optimusdigital.ro/ro/surse-coboratoare/12484-sursa-coboratoare-dc-dc-de-la-24v12v-la-5v-5a.html) |
| [x1 RepRapDiscount Full Graphic Smart Controller](https://reprap.org/wiki/RepRapDiscount_Full_Graphic_Smart_Controller) | The display with encoder and SD-card reader | [already owned](https://sigmanortec.ro/modul-control-imprimanta-3d-display-12864-ramps-14-card-sd?SubmitCurrency=1&id_currency=2&gad_source=5&gad_campaignid=23069763085&gclid=EAIaIQobChMIvdmqh96JlAMV0KeDBx3roxqiEAQYASABEgJshfD_BwE) |
| [x1 SanDisk microSDHC Ultra 16GB C10/UHS-I](https://www.evomag.ro/foto-video-carduri-memorie/sandisk-card-de-memorie-sandisk-ultra-android-microsdhc-16gb-80-mb-s-citire-clasa10-uhs-i-3645969.html) | MicroSD-card (with SD-card adapter) | [already owned](https://www.evomag.ro/foto-video-carduri-memorie/sandisk-card-de-memorie-sandisk-ultra-android-microsdhc-16gb-80-mb-s-citire-clasa10-uhs-i-3645969.html) |
| [x2 NEMA17 SL42STH40-1684A-23 Stepper Motor](https://forum.duet3d.com/assets/uploads/files/1694961895301-sl42sth40-1684a-23%E7%94%B5%E6%9C%BA%E5%9B%BE.pdf) | The motors that will move the pen on the X and Y axis | [already owned](https://www.aliexpress.com/i/1005008532513345.html) |
| [x2 A4988 Stepper Driver](https://www.pololu.com/file/0j450/a4988_dmos_microstepping_driver_with_translator.pdf) | The drivers that control the stepper motors | [15.98 RON](https://www.optimusdigital.ro/ro/drivere-de-motoare-pas-cu-pas/866-driver-pentru-motoare-pas-cu-pas-a4988-rosu.html) |
| [x2 100uF 35V Capacitor](https://www.tme.eu/en/details/rd1v107m6l011bb/tht-electrolytic-capacitors/samwha/) | Decoupling capacitors for protecting the stepper drivers | [0.60 RON](https://electroniclight.ro/ce-10035pht-condensator-electrolitic-tht-100uf-35vdc-63x12mm/8734.htm) |
| [x1 JF-0530B 12V Push-pull Solenoid](https://www.hobbytronics.co.za/Content/external/1274/D2512640695.pdf) | Moves the pen up and down | [24.38 RON](https://sigmanortec.ro/piston-electromagnetic-jf-0530b-cu-solenoid-12v-push-pull) |
| [x1 Dual MOSFET PWM Module, 15A, 400W](https://www.mouser.com/datasheet/2/149/FDD8782-1007378.pdf) | Helps control the solenoid | [2.83 RON](https://sigmanortec.ro/Modul-dual-MOSFET-de-putere-15A-400W-p187778881) |
| [x1 1N4007 Diode](https://www.vishay.com/docs/88503/1n4001.pdf) | Flyback diode for the solenoid | [0.49 RON](https://www.optimusdigital.ro/ro/componente-electronice-diode/7457-dioda-1n4007.html) |
| [x2 Mechanical Endstop](https://www.robotics.org.za/END-HOR) | Used for getting the origin position on the X and Y axis | [13.98 RON](https://www.optimusdigital.ro/ro/piese-imprimante-3d-altele/7200-intrerupator-limitator-orizontal-mecanic-in-miniatura.html) |
| [x2 0.47uF 50V Capacitor](https://www.tme.eu/en/details/pf1hr47mnn0511u/tht-electrolytic-capacitors/elite/) | Debouncing capacitors for the endstop false triggers | [0.50 RON](https://electroniclight.ro/pf1hr47mnn0511u-condensator-electrolitic-047uf-50vdc-5x1mm-tht/10019.htm) |
| [x2 10kOhm resistor](https://docs.rs-online.com/7a5c/A700000008919930.pdf) | 10kOhm external pull-up resistors for the endstops | [0.20 RON](https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/1088-rezistor-025w-100k.html) |
| [x1 GL-12 830 Point Breadboard](https://electroniclight.ro/kit0586-placa-breadboard-830-puncte-gl-12/9816.htm) | The central prototyping platform | [15.00 RON](https://electroniclight.ro/kit0586-placa-breadboard-830-puncte-gl-12/9816.htm) |
| [x1 DC Jack Module](https://www.optimusdigital.ro/ro/conectori/177-modul-mufa-dc.html) | Gives the power to the breadboard | [4.99 RON](https://www.optimusdigital.ro/ro/conectori/177-modul-mufa-dc.html) |
| [x1 40 Breadboard (DuPont) M-M 20cm Wires](https://electroniclight.ro/40pe279l100--set-40-fire-breadboard-colorate-tata-tata-lungime-10mm/10368.htm) | For connecting the components | [15.00 RON](https://electroniclight.ro/40pe279l100--set-40-fire-breadboard-colorate-tata-tata-lungime-10mm/10368.htm) |
| [x1 140 Breadboard Jumpers](https://www.emag.ro/set-140-fire-jumper-u-shape-pentru-breadboard-si-pcb-ai2098/pd/DBR4Z6YBM/) | For connecting the components and short breadboard connections | [14.82 RON](https://www.emag.ro/set-140-fire-jumper-u-shape-pentru-breadboard-si-pcb-ai2098/pd/DBR4Z6YBM/) |
| [x12 2020/2040 Aluminum Extrusions](https://www.reprapmania.ro/cumpara/profil-aluminiu-20x20-slot-6mm-456) | The skeleton of the build | [already owned](https://www.reprapmania.ro/cumpara/profil-aluminiu-20x20-slot-6mm-456) |
| [x1 Aluminum Bed](https://www.spindle.ro/material-de-lucru/tabla-aluminiu-3mm-300mmx300mm-5754.html) | The surface on which the paper will sit | [already owned](https://www.spindle.ro/material-de-lucru/tabla-aluminiu-3mm-300mmx300mm-5754.html) |
| [x6 Bed Screws with Springs](https://sigmanortec.ro/set-surub-m3-cu-arc-si-piulita-pat-imprimanta-3d?SubmitCurrency=1&id_currency=2) | For securing the bed and making sure it's flat | [already owned](https://sigmanortec.ro/set-surub-m3-cu-arc-si-piulita-pat-imprimanta-3d?SubmitCurrency=1&id_currency=2) |
| [GT2 Belt](https://sigmanortec.ro/Curea-GT2-6mm-p125814436?SubmitCurrency=1&id_currency=2) | X and Y axis motion | [already owned](https://sigmanortec.ro/Curea-GT2-6mm-p125814436?SubmitCurrency=1&id_currency=2) |
| [GT2 Pulleys](https://www.optimusdigital.ro/ro/mecanica-roti-scripete/12641-roata-scripete-din-aluminiu-cu-diametru-intern-de-8-mm-30-dinti-pentru-curea-gt2-6mm.html) | Guides the belt | [already owned](https://www.optimusdigital.ro/ro/mecanica-roti-scripete/12641-roata-scripete-din-aluminiu-cu-diametru-intern-de-8-mm-30-dinti-pentru-curea-gt2-6mm.html) |
| [V-Slot Wheels](https://sigmanortec.ro/Rola-V-Slot-POM-p128663792) | X and Y axis motion | [already owned](https://sigmanortec.ro/Rola-V-Slot-POM-p128663792) |
| Total | Total cost of the components | 271.23 RON |
## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | Embassy Hardware Abstraction Layer (HAL) for ST STM32 series microcontrollers | Async task management |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async/await executor designed for embedded usage | Runs the asynchronous tasks |
| [gcode](https://crates.io/crates/gcode) | A gcode parser for no-std applications | For parsing the .gcode files |
| [st7920](https://crates.io/crates/st7920) | SPI driver for the ST7920 LCD display controller | Display driver library |
| [embedded-graphics](https://crates.io/crates/embedded-graphics) | Embedded graphics library for small hardware displays | For displaying the system on the screen |
| [embedded-sdmmc](https://crates.io/crates/embedded-sdmmc) | A basic SD/MMC driver for Embedded Rust | For storing and using the .gcode files |
| [micromath](https://crates.io/crates/micromath) | Embedded-friendly math library | For calculating motion |
| [defmt](https://crates.io/crates/defmt) | A highly efficient logging framework that targets resource-constrained devices, like microcontrollers | For debugging |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Embassy Framework Documentation](https://embassy.dev/book/)
2. [RepRapDiscount Full Graphic Smart Controller Wiki](https://reprap.org/wiki/RepRapDiscount_Full_Graphic_Smart_Controller)
3. [A4988 Stepper Driver Datasheet](https://www.pololu.com/file/0j450/a4988_dmos_microstepping_driver_with_translator.pdf)
4. [JF-0530B Solenoid Datasheet](https://www.hobbytronics.co.za/Content/external/1274/D2512640695.pdf)
5. [STM32 NUCLEO-U545RE-Q Specifications](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html)
