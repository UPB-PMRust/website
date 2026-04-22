# Automated Coffee Maker
A machine for iced coffee

:::info 

**Author**: Dumitrescu Teodora Cristina \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-t3o27-1

:::

<!-- do not delete the \ after your name -->

## Description

The project is an automated coffee machine that allows the user to choose exactly how much sugar and coffee they want. The process is simple: you turn on the device with a button, select your desired quantities from the menu, and then press a start button. The system then pours the sugar, coffee, and water in the correct order and mixes them together. The result is a finished iced coffee, ready to be served.

## Motivation

I chose this project because I wanted to build something functional that I can actually use at home after the course is finished. I also find it very interesting to take a common object that you can normally buy in a store and try to recreate it from scratch. This helps me understand the mechanics and logic behind everyday appliances.

## Architecture 

<!-- Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other -->

Input  Consists of push-buttons used to select the desired quantities of coffee and sugar, and an LCD display that provides real-time feedback to the user.

Processing : The STM32 Nucleo acts as the central controller. It processes the signals from the buttons and coordinates the other components, managing the timing and quantities needed for the recipe.

Output : The water pump, servo dispenser and mixer work together to prepare and deliver the final mixed iced coffee.

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Detail in a few words the hardware used.

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
| [STM32 Nucleo-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main microcontroller | Provided by Lab |
| [Servo SG90 (4 pieces)](https://www.temu.com/ro-en/5-10pcs--micro-servo-motors----9g-servo-kit-used-for-remote-control-helicopters-airplanes-cars-boats-robot--hands-walking-servo-door-lock-control-g-601100158512008.html?_oak_mp_inf=EIjXsMuo1ogBGiBiMjY5NGU5Nzk0OGU0YzJiODY4OWNjZWFlZDU1NWUxMiDmp6uw2zM%3D&top_gallery_url=https%3A%2F%2Fimg.kwcdn.com%2Fproduct%2Ffancy%2F793816fb-d6b7-4516-aa4b-e1deca287b15.jpg&spec_gallery_id=86194&refer_page_sn=10009&freesia_scene=2&_oak_freesia_scene=2&_oak_rec_ext_1=NDU1NQ&_oak_gallery_order=368504330%2C1949439591%2C930347573%2C743949400%2C1107688170&search_key=servo%20sg90&refer_page_el_sn=200049&ab_scene=1&enable_vqr=0&_x_sessn_id=fun9allzcx&refer_page_name=search_result&refer_page_id=10009_1776875656999_fapko68tx6) | Solid ingredients dispenser mechanism | 45 RON |
| [LCD 1602 with I2C](https://www.emag.ro/display-lcd-2-x-16-cu-convertor-i2c-80-x-35-mm-verde-albastru-negru-2-e-001/pd/DHRJ0LMBM/) | User menu and status display | 23 RON |
| [Tactile Buttons (3 pieces)](https://www.temu.com/ro-en/premium-skrgaed010--665-mm-mini-momentary-tactile-switch-robot-component-space-saving-design-ideal-for--great-for-or--robotics-applications-compatible-with--esp32---g-601104438055515.html?_oak_mp_inf=ENukg8S41ogBGiBmN2RmMzBlNWEwNWQ0NjU5YTAyNGQ4YzM5NmJkMTkwMSCQ%2Bq6w2zM%3D&top_gallery_url=https%3A%2F%2Fimg.kwcdn.com%2Fproduct%2Fopen%2F6b3673bdeeee43f085c9a852c5650cb3-goods.jpeg&spec_gallery_id=25439598800&refer_page_sn=10009&freesia_scene=2&_oak_freesia_scene=2&_oak_rec_ext_1=OTY3&_oak_gallery_order=1707219694%2C1401704462%2C373535030%2C1348726518%2C1578384367&search_key=buttons&refer_page_el_sn=200049&ab_scene=1&enable_vqr=0&refer_page_name=search_result&refer_page_id=10009_1776875656999_fapko68tx6&_x_sessn_id=fun9allzcx) | User input for quantity and start | 3 RON |
| [5mm LEDs (2 pieces)](https://www.temu.com/ro-en/100pcs-lot-3mm-5mm-f3-f5-round-led--white-green-yellow-blue-white-red--for-diy-kit-g-601099813800288.html?_oak_mp_inf=EOCSgaen1ogBGiA4MzQ1ZjE1MWEzMjA0ZjRmYTMwM2U2ZmM4YjhjNmIzMiCgo7Gw2zM%3D&top_gallery_url=https%3A%2F%2Fimg.kwcdn.com%2Fproduct%2Ffancy%2Fc7bcca00-600a-4233-905a-f14e37760769.jpg&spec_gallery_id=32380&refer_page_sn=10009&freesia_scene=2&_oak_freesia_scene=2&_oak_rec_ext_1=NzQ4&_oak_gallery_order=1399236407%2C643712276%2C691833858%2C1803860912%2C624254038&search_key=leds&refer_page_el_sn=200049&ab_scene=1&enable_vqr=0&_x_sessn_id=fun9allzcx&refer_page_name=search_result&refer_page_id=10009_1776875755012_83l35kd3hh) | System status indicators | 1 RON |
| [DC Mixer Motor](https://www.emag.ro/mixer-spuma-de-lapte-awwaline-negru-ideal-pentru-cappuccino-caffe-latte-ciocolata-calda-cafea-cu-lapte-105466021/pd/DQVQF4MBM/?ref=history-shopping_482864334_51399_1) | Homogenizing the coffee (extracted from handheld mixer) | 18 RON |
| [Water Pump 3.7V](https://www.temu.com/ro-en/1p-mini-370-vacuum-water-pump-electric--motor--instrument---high-pressure-dual-voltage-3-7v-6v-12v--for-home-and-professional-use-hydraulic-pumping-durable-plastic-pump-mini-vacuum-pump-g-601105003866108.html?_oak_mp_inf=EPzP6dG61ogBGiAzYTk2OGE4NTM4MDE0NmIwODEzMDlmN2U1ZGQ3ZTU0MSCP%2BLOw2zM%3D&top_gallery_url=https%3A%2F%2Fimg.kwcdn.com%2Fproduct%2Ffancy%2F9d19e137-dc05-4b27-acd3-9fbbb7f2ca5d.jpg&spec_gallery_id=29368686273&refer_page_sn=10009&freesia_scene=2&_oak_freesia_scene=2&_oak_rec_ext_1=MjA0OA&_oak_gallery_order=424776565%2C820203985%2C1431435730%2C1215000053%2C1777123575&search_key=water%20pump&refer_page_el_sn=200049&ab_scene=1&enable_vqr=0&_x_sessn_id=fun9allzcx&refer_page_name=search_result&refer_page_id=10009_1776875798082_vrwboacf4l) | Liquid dispensing | 20 RON |
| [Water hose](https://www.temu.com/ro-en/2m-4m-6m-transparent--elastic-soft-and-lightweight-pvc-hose-4mm-6mm-8mm-10mm-with-inner-diameter-suitable-for-small-water-pumps--aquariums-gardens--industrial-pumping-water-dispensers-etc-g-606064460319831.html?_oak_mp_inf=ENeY%2BYbm5okBGiAwMTFhYzU3YzZhYmM0Y2U4YmVmOTY5MTBjNWY4NzczYSCz77mw2zM%3D&top_gallery_url=https%3A%2F%2Fimg.kwcdn.com%2Fproduct%2Ffancy%2F614f0c69-e0e4-4185-81cd-c7b1b5b7d4c1.jpg&spec_gallery_id=38055145245&refer_page_sn=10009&freesia_scene=2&_oak_freesia_scene=2&_oak_rec_ext_1=MTE3OA&_oak_gallery_order=2125134407%2C1108771025%2C1250978232%2C1655611518%2C536406149&search_key=furtun&refer_page_el_sn=200049&ab_scene=1&enable_vqr=0&_x_sessn_id=fun9allzcx&refer_page_name=search_result&refer_page_id=10009_1776875895636_ofu63k3z5k) | The hose for the water pump | 12 RON |
| [HC-SR04 Sensor](https://www.temu.com/ro-en/1pc-hc-sr04--module-distance-measurement-module-sensor8-g-605514972921860.html?_oak_mp_inf=EISA%2BIbn1okBGiBiNjJlZWI3MWI0NzE0YjMxOWZhMDY4ZDRkMzhmNWEyMSD9p7aw2zM%3D&top_gallery_url=https%3A%2F%2Fimg.kwcdn.com%2Fproduct%2Falgo_framework%2FImageCm2InAlgo%2F0dc4a960-1e1d-11f1-a664-0a580aa946c4.jpg&spec_gallery_id=35673484673&refer_page_sn=10009&freesia_scene=2&_oak_freesia_scene=2&_oak_rec_ext_1=MTk0Mw&_oak_gallery_order=138927405%2C1361481989%2C2039794419%2C515447104%2C1381815348&search_key=sensor%20hc%20sr04&refer_page_el_sn=200049&ab_scene=1&enable_vqr=0&refer_page_name=search_result&refer_page_id=10009_1776875823674_lc4i9zqrjt&_x_sessn_id=fun9allzcx) | Measuring water level in the tank | 18 RON |




## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async Rust framework | Main hardware abstraction |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Drawing UI on the LCD |
| [panic-halt](https://crates.io/crates/panic-halt) | Panic handler | Basic error management |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...



