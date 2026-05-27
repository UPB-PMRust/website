# Slot Machine
A mini slot machine that accepts real coins, spins three physical reels, and dispenses a payout on win.

:::info

**Author**: Teodor Adrian Miron \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/acs-project-2026-teodoradriann](https://github.com/UPB-PMRust-Students/acs-project-2026-teodoradriann)

:::

<!-- do not delete the \ after your name -->

## Description

My project consists in building a mini Slot Machine where you can insert coins and bet X amount of credited money. The system uses a load cell to count coins and three stepper motors to physically roll the reels. The game is initiated via a dedicated Spin button, and if the player wins, the amount is dispensed physically using a servo-driven payout mechanism.

## Motivation

I wanted to do something fun and help students control their gambling addiction with tiny amounts of money.

## Architecture

The firmware is written in Rust on top of **Embassy** (`embassy-executor` async runtime, `embassy-stm32` HAL, `embassy-time` time driver) targeting `thumbv8m.main-none-eabihf` (Cortex-M33). The whole game runs as a **single cooperative async task** inside `#[embassy_executor::main]`; concurrency comes from `embassy_futures::select` and `embassy_futures::join3` rather than from spawning extra tasks, which keeps shared state (credit, coin count, baseline weight) in plain stack variables instead of behind a mutex.

 - The crate is split into a reusable library plus three binaries:
   - `src/lib.rs` exposes three driver modules — `load_cell`, `stepper`, `display`.
   - `src/main.rs` is the game itself (`cargo run`).
   - `src/bin/calibrate.rs` auto-tares on the empty cell, waits for a known reference mass, and prints the `SCALE_FACTOR` (counts/g) to copy into `main.rs`.
   - `src/bin/motor_test.rs` rotates all three reels by a hard-coded number of half-steps in parallel for sanity-checking the wiring.

 - The three driver modules:
   - **`load_cell`** wraps the HX711 amplifier. A conversion is bit-banged inside `cortex_m::interrupt::free` because the 24 SCK pulses are microsecond-sensitive and can't tolerate interrupt jitter; everything around it (`wait_ready`, `tare`, `average_reading`, `read_net`, `read_grams`) is async and yields to the executor between samples via `Timer::after_millis`. Also exports a generic `MovingAverage<const N: usize>` filter used to smooth the runtime grams readings.
   - **`stepper`** drives a 28BYJ-48 via a ULN2003 with the standard 8-step half-step sequence. Each `Stepper` instance owns its four `Output` pins and tracks an internal `position: u32` modulo `STEPS_PER_REV (= 4096)`. Public API: `step_forward(n, delay_us)` for raw motion and `spin_to_symbol(target, extra_revs, delay_us)` for game-level moves — the latter lands exactly on `target * STEPS_PER_SYMBOL (= 1024)`, with extra travel expressed in **whole** revolutions because partial revs shift the modular landing and the reel would end up on the wrong symbol.
   - **`display`** wraps the ST7735 LCD. The chosen pins (`A1`-`A5`) have no SPI alternate function on STM32U545RE, so the module includes a small `BitBangSpi` that implements `embedded-hal::spi::SpiDevice` by toggling GPIO; the standard `st7735-lcd` + `embedded-graphics` crates then sit on top unchanged. The wrapper exposes screen-shaped methods (`show_boot`, `show_idle`, `show_coin`, `show_spinning`, `show_won`, `show_lose`), each one re-rendering a full frame with a coloured title (cyan / green / yellow / red) and the running credit balance pinned to the bottom. `Orientation::PortraitSwapped` is set in `init` so the screen reads the right way up despite being mounted upside-down on the box.

 - Concurrency model — single task, two primitives:
   - **`select(cell_step, btn_step).await`** — the main loop races a paced load-cell read (`Timer::after_millis(150)` followed by `cell.read_net()`, paces the cell at ~7 Hz) against an EXTI-driven button wait (`spin_btn.wait_for_falling_edge()`, so the MCU actually sleeps until the player presses Spin). Whichever future completes first runs its handler; the other is dropped (drop-safe in Embassy by construction).
   - **`join3(left.spin_to_symbol(...), mid.spin_to_symbol(...), right.spin_to_symbol(...)).await`** — during a spin, the three reels turn in parallel within the same task. Each future yields between half-steps via `Timer::after_micros`, so the executor advances all three roughly together. The right reel is given more `extra_revs` than the left one, so it stops last (slot-machine cascade effect).

 - Coin detection is **the only explicit state machine in the code**. The cell-read branch runs `enum DetectState { Idle, Settling { last_g, stable_count } }`: in `Idle`, any sample where `|grams − baseline| > COIN_DELTA_G (4 g)` arms the detector; in `Settling`, the detector counts consecutive samples within `STABLE_TOL_G (1 g)` of the previous, and once `STABLE_SAMPLES (5)` stable readings accumulate (~750 ms at the 150 ms read period), one coin is registered (`coin_count += 1`, `credit += CREDIT_PER_COIN (50)`, baseline updated) and the detector returns to `Idle`. A sample further than the tolerance resets the stability counter. The detector is direction-agnostic on purpose — a settled `|Δ| > 4 g` jump counts as one coin whether positive or negative, because the mechanical mount can make some coins drop the reading instead of raising it.

 - Game flow maps the classic slot-machine sequence (idle → bet → spin → payout) onto the natural control flow of the `select` loop rather than onto an explicit `state: GameState` variable. `IDLE / WAITING_FOR_BET` is `select.await` parked between events; `COIN_DETECTED` is the `Either::First` branch running the `Idle/Settling` micro-FSM above; `SPINNING` is `join3.await` for the three reels, after credit has been deducted and the RNG has chosen the targets; `PAYOUT (logical)` is the `if left == mid && mid == right { credit += coin_count × CREDIT_PER_COIN }` branch right after `join3` returns. Each transition is also an explicit LCD refresh (`display.show_coin`, `show_spinning`, `show_won`, `show_lose`), so the UI mirrors the implicit state of the loop.

 - RNG and the win rule — randomness comes from the STM32U5's hardware TRNG (`embassy_stm32::rng::Rng`). HSI48 is already enabled by the default RCC config, so no extra clock setup is needed; `rng.async_fill_bytes(&mut [0u8; 3])` yields three random bytes which are masked with `% 4` (no modulo bias since `256 mod 4 == 0`) and mapped to the four-variant `Symbol` enum (`Seven`, `Cireasa`, `Diamant`, `Inima`). Each `Symbol::index()` is passed straight to `Stepper::spin_to_symbol` so the reel lands physically on the matching position. Win condition is `left == mid && mid == right` — probability `4 / 64 = 6.25 %` per spin under uniform reels. On a win, `credit += coin_count × CREDIT_PER_COIN` (everything in the box becomes new credit); on a loss, credit is untouched.

 - The DS04-NFC continuous-rotation servo planned for the **physical** payout is deliberately not wired in this iteration: without an enclosure with a coin track and an ejection slot, the servo would spin against air. The payout exists only as the credit-level operation. The pinmap reserves a PWM-capable pin and a TIM channel for the servo, and the software hook is a one-liner (`coin_count = 0` plus a `servo.eject().await`) — both can be added once the enclosure is built.

 - Boot sequence inside `main`: `embassy_stm32::init(Default::default())` on the default 4 MHz MSI clock → constructors for HX711 (`PB5 SCK`, `PB4 DOUT`), Spin button (`PA0` with EXTI + internal pull-up), three steppers, RNG, and display in that order → `display.show_boot()` → 3 s HX711 thermal warm-up → `cell.tare(20)` averages 20 readings of the empty box as the new zero offset → `display.show_idle(0)` → the main `select` loop starts.

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

Ordered all the components from Optimus Digital and Ardushop: three 28BYJ-48 steppers with ULN2003 drivers, the ST7735 1.44" SPI display, the HX711 + 5 kg load cell pair, an arcade Spin button, the DS04-NFC servo, and a 5 V / 5 A stabilised supply to handle the motor peak current without browning out the Nucleo. Sketched the overall FSM (`IDLE` → `WAITING_FOR_BET` → `SPINNING` → `PAYOUT`) and picked Embassy as the async framework for the firmware. Drafted a first-pass pin allocation: HX711 on the digital header so it stays close to the Arduino-style D-pins, the arcade button on `PA0`, SPI1 reserved for the LCD, and three groups of four GPIOs lined up consecutively on CN10 so each stepper has its four IN-pins next to each other on the connector. Most of the architectural reading went into how to keep the cell read and the button reactive at the same time without spawning extra tasks too early.

### Week 12 - 18 May

Hardware assembly started with the load cell because it gates everything else — without a stable weight signal, the coin-driven game flow doesn't work. Wired HX711 with `SCK = PB5 (D4)` and `DOUT = PB4 (D5)`, powered the module from the external 5 V rail (PB4 is 5 V tolerant on STM32U5, so the DOUT line into the MCU is safe). Wrote a small `LoadCell` driver module with `tare`, `read_raw`, `read_net`, plus a calibration binary (`cargo run --bin calibrate`) that auto-tares on the empty cell, waits for a known reference mass, runs a stability detector, and prints the resulting `SCALE_FACTOR` in counts per gram. Built a `MovingAverage<N>` filter to keep the runtime readings usable.

After that, the arcade Spin button on `PA0` went on with internal pull-up and EXTI on the falling edge, and the three steppers were wired and tested with `motor_test.rs`, a separate binary that rotates all three motors by a hard-coded number of half-steps in parallel via `embassy_futures::join3`. Wrote the `Stepper` driver with the canonical 8-step half-step sequence, kept an internal `position` counter modulo 4096, and added a `spin_to_symbol(target, extra_revs, ...)` helper that lands exactly on `target * STEPS_PER_SYMBOL` (1024) — the API is in whole revolutions because partial revs shift the modular landing and the motor ends up one symbol off the target. The main game loop became a `select(cell_step, btn_step)` so coin detection and button presses are reactive at the same time.

### Week 19 - 25 May

Most of the software week. Two big chunks: making the coin detection actually work, and wiring up the rest of the IO (RNG, LCD).

The **load cell was, by far, the hardest part of the project**. The mechanical mount turned out to be wrong in a non-obvious way: the cell sandwich was incorrectly built so the load wasn't transferred to the strain gauge as expected, and depending on where in the box a coin landed, the reading either *increased* or *decreased*. Reseated the cell multiple times, re-derived `SCALE_FACTOR` between attempts (it drifted between roughly −35 and −90 counts/g across remounts — a hint that even the calibration wasn't transferring force consistently), and tried the floor-of-grams coin counter (1 coin = `floor(grams / COIN_MASS_G)`) but it failed because the first coin always read about double the others (~21 g vs ~11 g due to mechanical settling) and individual coins could even produce negative steps. Switched to **symmetric edge detection** — a small `Idle/Settling` state machine that registers exactly one coin per stabilised jump of `|Δ| > 4 g` regardless of sign — and that finally worked reliably (verified 4-out-of-4 detections across consecutive coin drops).

With detection working, added the hardware RNG (TRNG peripheral, no extra clock config needed — HSI48 is on by default), four symbols (`7`, `CIREASA`, `DIAMANT`, `INIMA`), and the win-on-3-of-a-kind rule. On win, the player's credit is increased by the value of every coin currently in the box (`coin_count × CREDIT_PER_COIN`).

The ST7735 LCD went in last. **Hit a wiring constraint here**: ran out of female-female jumpers to reach the digital header where SPI1 is, so the display ended up on the analog header (`A1-A5`). None of those pins have an SPI alternate function on STM32U545RE, so wrote a small ~40-line `BitBangSpi` that implements `embedded-hal::spi::SpiDevice` and toggles `SCL`/`SDA`/`CS` by hand — the standard `st7735-lcd` + `embedded-graphics` crates then sit on top unchanged. Built a `Display` wrapper with `show_boot`, `show_idle`, `show_coin`, `show_spinning`, `show_won`, `show_lose` — each one re-renders the screen with a coloured title (cyan for boot, green for coin/win, yellow for spinning, red for lose), an optional sub-line for the credit delta, and the running credit balance always at the bottom. The module is mounted upside-down on the case so the driver uses `Orientation::PortraitSwapped` to keep the text readable.

**One thing didn't make it**: the physical enclosure. Without a proper case with a coin track and an ejection chute, the DS04-NFC servo can't actually push coins out of a storage tube — it would just spin against air. The payout mechanism is therefore implemented only at the software / credit level for now (the `credit += coin_count × 50` step on a win), and the servo is left unwired. Re-introducing the servo + the physical payout will need an enclosure pass; the GPIO and PWM budget for it are already reserved.

## Hardware

The build uses a microcontroller board driving a small TFT display, an HX711 + load cell pair for detecting inserted coins by weight, three 28BYJ-48 stepper motors with ULN2003 drivers for the physical reels, a continuous-rotation servo for the payout mechanism, arcade buttons for player input, and a 5 V stabilized power supply to handle motor peak currents.

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
|--------|-------|-------|
| [DS04-NFC Continous Rotation Servo](https://www.optimusdigital.ro/en/servomotors/1161-ds04-nfc-continous-rotation-servo.html) | Drives the coin payout mechanism | [39 RON](https://www.optimusdigital.ro/en/servomotors/1161-ds04-nfc-continous-rotation-servo.html) |
| [5 V 5000 mA Stabilized Power Supply](https://www.optimusdigital.ro/en/wall-socket-power-supplies/2890-5-v-5000-ma-stabilized-power-supply.html) | Powers the motors and the rest of the system | [30 RON](https://www.optimusdigital.ro/en/wall-socket-power-supplies/2890-5-v-5000-ma-stabilized-power-supply.html) |
| [1.44" SPI LCD Module with ST7735 Controller (128x128 px)](https://www.optimusdigital.ro/en/lcds/3552-modul-lcd-de-144-cu-spi-i-controller-st7735-128x128-px.html) | Displays credit balance, bet amount and reel animation | [30 RON](https://www.optimusdigital.ro/en/lcds/3552-modul-lcd-de-144-cu-spi-i-controller-st7735-128x128-px.html) |
| [ULN2003 Stepper Driver + 5V Stepper Motor](https://www.optimusdigital.ro/en/stepper-motors/101-stepper-motor-with-uln2003-driver.html) (x3) | Physically rolls the three reels | [17 RON](https://www.optimusdigital.ro/en/stepper-motors/101-stepper-motor-with-uln2003-driver.html) |
| [Arcade Button 24 mm - Green](https://www.optimusdigital.ro/en/buttons-and-switches/1851-buton-arcade-iluminat-24mm-verde.html) | Spin / bet input from the player | [10 RON](https://www.optimusdigital.ro/en/buttons-and-switches/1851-buton-arcade-iluminat-24mm-verde.html) |
| [HX711 GroundStudio Load Cell Amplifier](https://ardushop.ro/ro/groundstudio/2207-modul-citire-senzor-greutate-hx711-groundstudio-6427854000040.html) | Reads digital weight values from the load cell | [11 RON](https://ardushop.ro/ro/groundstudio/2207-modul-citire-senzor-greutate-hx711-groundstudio-6427854000040.html) |
| [Load Cell (max. 1 Kg)](https://ardushop.ro/ro/electronica/2418-1349-senzor-greutate.html#/246-greutate_maxima-1_kg) | Detects inserted coins by weight | [10 RON](https://ardushop.ro/ro/electronica/2418-1349-senzor-greutate.html#/246-greutate_maxima-1_kg) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | STM32 hardware driver | Controlling pins, timers (PWM for the servo) and SPI for the LCD |
| [embassy-time](https://github.com/embassy-rs/embassy) | Time and delay management | Non-blocking delays for stepper motor steps and animations |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task scheduler | Running multiple tasks (motors, UI, sensors) concurrently |
| [embassy-sync](https://github.com/embassy-rs/embassy) | Async sync primitives | Inter-task communication (e.g. sending coin weight data to the UI) |
| [cortex-m](https://github.com/rust-embedded/cortex-m) | Core processor access | Managing interrupts and CPU-specific instructions |
| [cortex-m-rt](https://github.com/rust-embedded/cortex-m) | Startup/Runtime for ARM | Initializing memory and the program entry point |
| [defmt](https://github.com/knurling-rs/defmt) | Low-overhead logger | Fast logging for debugging sensor data and game states |
| [defmt-rtt](https://github.com/knurling-rs/defmt) | RTT transport for logs | Viewing logs in real-time through the debugger |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Drawing fruit icons, text and shapes on the screen |
| [st7735-lcd](https://crates.io/crates/st7735-lcd) | Display driver for ST7735 | Managing the command set for the 1.44" color TFT screen |
| [hx711](https://crates.io/crates/hx711) | Load cell driver | Reading digital weight values from the load cell amplifier |
| [panic-probe](https://github.com/knurling-rs/defmt) | Debug panic handler | Reporting and stopping the CPU safely if a runtime crash occurs |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Slot machine reference video](https://youtu.be/ihVHIpEZ-Pw)
