# Parking Sensor – Raspberry Pi Pico W (Rust / Embassy)

Smart parking sensor system using digital IR obstacle modules, RGB LEDs, passive buzzers and an LCD 1602 display.

## Wiring (from schematic)

| Signal        | Pico GPIO | Physical Pin |
|---------------|-----------|--------------|
| I²C0 SDA      | GPIO 4    | 6            |
| I²C0 SCL      | GPIO 5    | 7            |
| I²C1 SDA      | GPIO 8    | 11           |
| I²C1 SCL      | GPIO 9    | 12           |
| LED1 Red      | GPIO 10   | 14           |
| LED1 Green    | GPIO 11   | 15           |
| LED1 Blue     | GPIO 12   | 16           |
| LED2 Red      | GPIO 13   | 17           |
| LED2 Green    | GPIO 14   | 19           |
| LED2 Blue     | GPIO 15   | 20           |
| Buzzer 1      | GPIO 16   | 21           |
| Buzzer 2      | GPIO 17   | 22           |
| IR Sensor 1   | GPIO 20   | 26           |
| IR Sensor 2   | GPIO 21   | 27           |

> **RGB LEDs** are common-anode: connect the common pin to 3.3 V and drive each colour channel LOW to light it.  
> **IR sensors** output LOW when an obstacle is detected.

## Prerequisites

```bash
# Install Rust target for Cortex-M0+ (RP2040)
rustup target add thumbv6m-none-eabi

# Install the probe-rs flash/debug tool
cargo install probe-rs-tools --locked

# (Linux) Add yourself to the plugdev group so probe-rs can access the probe
sudo usermod -aG plugdev $USER
```

## Build

```bash
cargo build --release
```

## Flash

Connect a **Raspberry Pi Debug Probe** (or a second Pico flashed as picoprobe) to the SWD pins, then:

```bash
cargo run --release
```

Or flash the UF2 directly:

```bash
# Convert ELF → UF2
cargo install elf2uf2-rs --locked
elf2uf2-rs target/thumbv6m-none-eabi/release/parking_sensor parking_sensor.uf2

# Hold BOOTSEL while plugging in the Pico, then copy the UF2
cp parking_sensor.uf2 /media/$USER/RPI-RP2/
```

## Debug output

Real-time `defmt` logs are streamed over RTT.  With probe-rs connected:

```bash
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/parking_sensor
```

## Logic overview

| Sensors triggered | Zone    | LEDs   | Buzzer       | LCD                  |
|-------------------|---------|--------|--------------|----------------------|
| neither           | CLEAR   | Green  | Silent       | `CLEAR   >100cm`     |
| one               | WARNING | Yellow | Slow beep    | `WARNING  ~50cm`     |
| both              | DANGER  | Red    | Rapid beep   | `DANGER!  <20cm`     |

## Project structure

```
parking_sensor/
├── .cargo/
│   └── config.toml      # build target + linker flags
├── src/
│   ├── main.rs          # Embassy entry point, hardware init, main loop
│   ├── pins.rs          # GPIO / I²C address constants
│   ├── distance.rs      # Zone enum + sensor logic
│   ├── led.rs           # RgbLed helper
│   ├── buzzer.rs        # PWM buzzer helpers
│   └── lcd.rs           # LCD 1602 I²C driver (4-bit mode via PCF8574)
├── build.rs             # Linker script notification
├── memory.x             # RP2040 flash / RAM layout
├── Cargo.toml
└── README.md
```
