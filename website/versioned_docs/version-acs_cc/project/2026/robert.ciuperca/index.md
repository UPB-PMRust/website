# Braille Bidirectional Trainer
An interactive system for learning Braille through visual, tactile and auditory feedback.

:::info 

**Author**: Ciuperca Robert-Mihai \
**GitHub Project Link**: [Project Repository](https://github.com/UPB-PMRust-Students/acs-project-2026-robert1724)

:::

<!-- do not delete the \ after your name -->

## Description

The Braille Bidirectional Trainer is a dual-mode educational device. Mode 1 teaches sighted users to write Braille using a 3x2 button matrix based on screen prompts. Mode 2 helps visually impaired users practice reading via a 6-solenoid tactile display, with answers entered through a T9-style keypad and validated by audio feedback.

## Motivation

I chose this project after meeting a visually impaired person and realizing how much a basic knowledge of Braille would have helped our communication. This experience inspired me to create a tool that assists parents and teachers in supporting blind children, while providing young learners with an interactive, fun way to practice their tactile skills.

## Architecture 

![Project Diagram](./braille_trainer_architecture.svg)

The system operates in two modes, selected at startup by pressing a blue (Mode 1) or red button (Mode 2). Each mode has 3 levels of increasing difficulty, and the user must achieve at least 80% accuracy to advance.
### Mode 1 — Learn to Write Braille (Sighted Users)
The OLED display shows a random letter or word. The user encodes it in Braille using 6 blue buttons arranged in a 3×2 cell, then presses the yellow button to confirm. A green LED lights up for correct answers, red for incorrect ones.
### Mode 2 — Learn to Read Braille (Blind Users)
The STM32 activates solenoids through MOSFETs to physically raise Braille dots on a tactile plate. The user feels the pattern and types the corresponding letter on 9 red buttons arranged as a phone keypad (multi-press input, like old Nokia phones). Feedback is given via buzzer: one beep for correct, two beeps for incorrect.
The STM32 NUCLEO-U545RE-Q acts as the central controller, managing all input/output:
- **Input**: 6 blue buttons + 9 red buttons (GPIO) + 1 yellow confirm button
- **Display**: 0.91" OLED 128×32 via I2C — shows letters, words and game status
- **Tactile output**: 6 push-pull solenoids (5V 0.7A) driven by IRLZ44N MOSFETs with flyback diodes
- **Feedback**: Green/red LEDs (Mode 1) and passive buzzer via PWM (Mode 2)
- **Power**: USB-C for STM32, separate 5V 6A PSU for solenoids, with bulk and decoupling capacitors

## Log

<!-- write your progress here every week -->

### Week 20 - 26 April

- Drafted the initial hardware documentation (block structure, component list, wiring notes)
- Ordered all required components from Optimus Digital, eMAG, Farnell, AliExpress and Ground Studio
- Refined the project concept: defined the two operating modes (sighted / blind), the level progression logic, and how the selection screen switches between them

### Week 27 April - 3 May

### Week 4 - 10 May

### Week 11 - 17 May

## Hardware

**1. STM32 Nucleo-U545RE-Q - Main Microcontroller Unit**

- Handles: buttons (GPIO), OLED (I2C), MOSFETs (GPIO), LEDs (GPIO), buzzer (PWM)

**2. 6 Mini Push-Pull Solenoids (5V, 0.7A)**

- Tactile Braille output: 3×2 dot matrix under the tactile plate
- Powered from external 5V/6A supply, switched by MOSFETs
- Each solenoid driven through its own MOSFET: High -> piston extends, Low -> piston retracts

**3. 0.91" OLED Display (128×32, I2C)**

- Visual output for Mode 1: displays random letters, short words and complex words that the user must encode in Braille

**4. Push Buttons (16 total)**

- 6 blue buttons form the Braille cell input (3×2) for Mode 1
- 9 red buttons form a Nokia T9 keypad (3x3) for Mode 2
- 1 yellow button to validate input
- All buttons are debounced in software

**5. MOSFET Driver Stage (6× IRLZ44N + 6× 1N4007)**

- One MOSFET per solenoid
- Low-side switching for solenoids (GPIO can't source 0.7A)
- Per channel: 220Ω gate series, 10kΩ gate pull-down, 1N4007 flyback diode across coil

**6. Feedback Outputs**

- 2 LEDs (green/red) for Mode 1 (correct/wrong) working via 220Ω resistors
- 1 passive buzzer for Mode 2 (1 beep = correct, 2 beeps = wrong) working via PWM through 2N2222 + 1kΩ base resistor

**7. Power Subsystem**

- External 5V/6A feeds the solenoids only: solenoids use up to ~4.2A peak (6 * 0.7A)
- MCU powered via USB-C, only GND shared between the two supplies
- Decoupling: 100µF electrolytic on 5V rail + 100nF ceramics near MOSFETs/ICs

**8. Passives & Wiring**

- Resistors: 220Ω (LEDs, MOSFET gates), 10kΩ (pull-downs), 1kΩ (buzzer transistor base)
- 2× breadboard 830pt — Board A (logic), Board B (power)
- M-M, M-F, F-F jumpers; plexiglass tactile plate with 6 holes for solenoid pistons

**9. Physical Assembly**

- Plexiglass tactile plate with 6 holes drilled for solenoid pistons
- Solenoids mounted underneath, aligned with the holes
- Buttons arranged on top (Braille cell + T9 keypad + yellow confirm)

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
| [STM32 Nucleo-U545RE-Q](https://ro.farnell.com/stmicroelectronics/nucleo-u545re-q/development-brd-32bit-arm-cortex/dp/4216396?gross_price=true&CMP=KNC-GRO-GEN-SHOPPING-PMax_Test_840_Lowmargin&mckv=_dc%7Cpcrid%7C%7Cplid%7C%7Ckword%7C%7Cmatch%7C%7Cslid%7C%7Cproduct%7C4216396%7Cpgrid%7C%7Cptaid%7C%7C&gad_source=1&gad_campaignid=20659611307&gbraid=0AAAAAD8yeHkSK63CAR63cuQ9pFy5lNcjZ&gclid=Cj0KCQjwj47OBhCmARIsAF5wUEH6Kym6sSE14nqDDJKp0NwKgYvIjM8EmyS4uOmAS9zt2HrlzObdZDQaAiu0EALw_wcB) | Main microcontroller | 130 RON |
| [Mini Push-Pull Solenoids](https://www.aliexpress.com/item/1005007163422306.html?spm=a2g0o.productlist.main.1.11c5sxGhsxGhC6&algo_pvid=cb5e2006-7204-416e-9f57-7f2ee3d3e57b&algo_exp_id=cb5e2006-7204-416e-9f57-7f2ee3d3e57b-0&pdp_ext_f=%7B%22order%22%3A%223795%22%2C%22eval%22%3A%221%22%2C%22fromPage%22%3A%22search%22%7D&pdp_npi=6%40dis%21RON%2119.07%2114.62%21%21%2129.26%2122.43%21%40210384b917758799842563173e4715%2112000039662693694%21sea%21RO%210%21ABX%211%210%21n_tag%3A-29910%3Bd%3Aa45a2b33%3Bm03_new_user%3A-29895%3BpisId%3A5000000201130828&curPageLogUid=kN2AjBacVNhk&utparam-url=scene%3Asearch%7Cquery_from%3A%7Cx_object_id%3A1005007163422306%7C_p_origin_prod%3A#nav-specification) | Raises/Lowers Braille dots | 130 RON |
| [OLED Display](https://ro.farnell.com/dfrobot/dfr0648/oled-display-module-0-91-128x32/dp/4308185) | Displays letters/words | 38 RON |
| [6 blue buttons](https://www.optimusdigital.ro/en/buttons-and-switches/1118-blue-round-button-with-cover.html?search_query=button&results=377) | Braille cell (3x2 matrix) | 12 RON |
| [9 red buttons](https://www.optimusdigital.ro/en/buttons-and-switches/1114-red-button-with-round-cover.html?search_query=red+round+button+with+cover&results=8) | Nokita T9 keypad (3x3 matrix) | 18 RON |
| [1 yellow button](https://www.optimusdigital.ro/en/buttons-and-switches/13604-yellow-button-with-round-cover.html?search_query=button&results=377) | Confirms user input | 3 RON|
| [LEDs (green+red)](https://www.optimusdigital.ro/en/leds/13608-set-of-25-leds-5mm.html?search_query=5mm+LED&results=310) | Visual feedback | 5 RON |
| [MOSFET IRLZ44N](https://www.aliexpress.com/item/1005007174659364.html?spm=a2g0o.order_list.order_list_main.4.44a31802u3zJQ6) | Power switch per solenoid | 18 RON |
| [5V6A Power Supply](https://www.emag.ro/sursa-in-comutatie-ac-dc-30w-5v-6a-well-143x59x40mm-psup-so-5v30w-wl/pd/DXZ2MJ3BM/) | Powers solenoids | 52 RON |
| [Breadboard](https://www.emag.ro/breadboard-h-hct-tronic-830-puncte-de-conectare-abs-200x630-puncte-034-066/pd/DBNQ7R3BM/) | Connects all electronic parts | 10 RON|
| [Buzzer](https://www.optimusdigital.ro/en/buzzers/12247-3-v-or-33v-passive-buzzer.html?search_query=buzzer+5v&results=62) | Audio feedback | 1 RON |
| Resistors, capacitors, diodes, wires | Support the main components | ~30 RON |
| Perforated board | Solenoids mounted underneath raise through the holes | ~10RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://docs.embassy.dev/embassy-stm32/) | Hardware Interface | Acts as a bridge between the Rust code and the physical pins (GPIO, I2C, PWM) |
| [defmt](https://defmt.ferrous-systems.com/) | Debug Logging | Sends status messages to the computer for debugging via probe |
| [panic-probe](https://docs.rs/panic-probe/) | Error Handling | Reports crashes and errors through the debug probe |

The rest will be added as I develop the code

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [The Braille Alphabet](https://www.pharmabraille.com/pharmaceutical-braille/the-braille-alphabet/)
2. [Interactive Braille Trainer](https://www.michalpaszkiewicz.co.uk/brailletraining/)
3. [People's opinion on learning Braille](https://www.reddit.com/r/Blind/comments/1lehwx9/the_importance_of_braille_in_todays_technology/)
