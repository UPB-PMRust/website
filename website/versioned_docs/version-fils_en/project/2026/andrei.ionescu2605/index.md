# Thermal Imaging Embedded System (STM32 + MLX90640)
A portable handheld thermal camera that captures, processes, and displays live infrared data in real time.

:::info

**Author**: Ionescu Andrei \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-ionescuaandrei

:::

<!-- do not delete the \ after your name -->

## Description

This project is a battery-powered embedded thermal imaging system built on STM32, using the MLX90640 (32x24 IR array) to capture temperature frames and render them on a TFT display.

## Motivation

I chose this project because it combines real-time data acquisition, embedded data processing, and graphical rendering in a single portable system. It is a practical and technically challenging alternative to simpler IoT projects.

## Architecture

Main software and system components:
- Sensor Acquisition Module: reads raw IR frames from MLX90640 over I2C.
- Processing Pipeline: calibration, normalization, temperature-to-color mapping.
- Rendering Engine: draws thermal image and overlays on TFT display.
- Storage Manager: saves captured frames to microSD.
- UI Controller: joystick-driven menus and capture actions.
- Communication Module (optional): sends frames to mobile app via WiFi.

## Log
<!-- write your progress here every week -->

### Week 5 - 11 May
- Got the hardware working on the breadboard.
- Next step is moving to a protoboard with soldering.
- The thermal camera is running, but I am still tuning it to work perfectly.
- The app menu is still buggy, but I will handle that next week.

<div style={{ display: 'flex', gap: '12px', justifyContent: 'center', flexWrap: 'nowrap', margin: '1.5rem 0' }}>
     <img src="https://ionescuandrei.tech/pm1.jpg" alt="Thermal camera prototype 1" style={{ width: 'clamp(180px, 22vw, 260px)', height: 'auto', borderRadius: '14px', boxShadow: '0 10px 30px rgba(0, 0, 0, 0.18)' }} />
     <img src="https://ionescuandrei.tech/pm2.jpg" alt="Thermal camera prototype 2" style={{ width: 'clamp(180px, 22vw, 260px)', height: 'auto', borderRadius: '14px', boxShadow: '0 10px 30px rgba(0, 0, 0, 0.18)' }} />
</div>

### Week 12 - 18 May
- Tried to use I²C as the communication protocol for the MLX90640; failed due to missing 4.7 kΩ pull-up resistors on SDA/SCL.
- Pivoted to the GY-MCU90640BAB bridge board which speaks UART (PA9 TX / PA10 RX, 115200 baud) — gives a reliable frame stream straight out of the box.
- Boosted and tuned the thermal math and physics pipeline (dechess FPN correction, 3×3 median, spatial smoothing, auto-range).

### Week 19 - 25 May
- Finished the UI menu and all joystick controls (Up/Down/Left/Right + Press).
- Tried to solder everything onto a protoboard — worked for one day, then the layout broke. Instead of chasing solder-joint debugging, migrated back to the breadboard.
- Started enhancing the aesthetics with a custom box so the whole device looks like a real camera.
- The AliExpress SD card module turned out to be dead — switched to the SD slot on the back of the TFT module. If the SD card still fails, the firmware now falls back to on-chip STM32 flash storage so captured photos persist across power cycles.

<div style={{ display: 'flex', gap: '12px', justifyContent: 'center', flexWrap: 'nowrap', margin: '1.5rem 0' }}>
     <img src="https://ionescuandrei.tech/final1.jpeg" alt="Thermal camera protoboard attempt" style={{ width: 'clamp(180px, 22vw, 260px)', height: 'auto', borderRadius: '14px', boxShadow: '0 10px 30px rgba(0, 0, 0, 0.18)' }} />
</div>

### Week 26 May - 1 June
- Built a professional-grade thermal image pipeline based on the FLIR Lepton AGC literature: percentile auto-range (P2/P98) with EMA-damped bounds, FLIR-style histogram-equalisation AGC with plateau clipping and a linear-percent term, temporal EMA frame blending for noise reduction, and switched the active palette to FLIR Ironbow.
- Added per-screen joystick guidance legend bars on every UI screen so first-time users can operate the device without instructions.
- Implemented a photo delete flow with a red confirmation modal (Right = prompt, Press = confirm, Left = cancel) — works for both the SD and on-chip flash backends.
- Polished the menu UI: bigger title bar with accent line, roomier menu rows, friendlier item labels.
- Force-pushed the cleaned-up state to GitHub and removed ~3500 stale `target/` build artefacts from version control.

<div style={{ display: 'flex', gap: '12px', justifyContent: 'center', flexWrap: 'nowrap', margin: '1.5rem 0' }}>
     <img src="https://ionescuandrei.tech/final2.jpeg" alt="Final thermal camera output 1" style={{ width: 'clamp(180px, 22vw, 260px)', height: 'auto', borderRadius: '14px', boxShadow: '0 10px 30px rgba(0, 0, 0, 0.18)' }} />
     <img src="https://ionescuandrei.tech/final3.jpeg" alt="Final thermal camera output 2" style={{ width: 'clamp(180px, 22vw, 260px)', height: 'auto', borderRadius: '14px', boxShadow: '0 10px 30px rgba(0, 0, 0, 0.18)' }} />
</div>

## Hardware

The system uses an STM32 NUCLEO-U545RE-Q for development, an MLX90640 thermal sensor, a TFT display for visualization, a microSD module for storage, and a joystick for navigation. An ESP8266/ESP32 module can be added for WiFi transfer to a mobile app.

### Schematics

![Thermal camera hardware schematic](./assets/thermal_cam_good%20resolution.svg)


### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Main controller | [125 RON](https://ro.mouser.com/) |
| [MLX90640](https://www.melexis.com/en/product/MLX90640) | Thermal sensor (32x24 IR array) | [~150 RON](https://www.optimusdigital.ro/) |
| [TFT Display](https://www.adafruit.com/category/63) | Real-time thermal visualization | [~40 RON](https://www.optimusdigital.ro/) |
| [microSD Module](https://components101.com/modules/microsd-card-module) | Frame storage | [~15 RON](https://www.optimusdigital.ro/) |
| [Joystick Module](https://components101.com/modules/joystick-module) | Menu navigation and input | [~10 RON](https://www.optimusdigital.ro/) |
| [ESP8266/ESP32](https://www.espressif.com/en/products) | Optional WiFi communication | [~25 RON](https://www.optimusdigital.ro/) |
| [Battery / Power Bank](https://www.emag.ro/) | Portable power supply | [~50 RON](https://www.emag.ro/) |
| [Misc. Wires and Connectors](https://www.emag.ro/) | Integration accessories | [~50 RON](https://www.emag.ro/) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Async HAL for STM32 | Hardware access for I2C/SPI/UART/GPIO |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor | Schedules concurrent firmware tasks |
| [embassy-time](https://github.com/embassy-rs/embassy) | Embedded timers | Frame timing and periodic operations |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | HAL traits | Driver abstraction across peripherals |
| [mlx9064x](https://docs.rs/mlx9064x) | MLX90640 driver | Sensor frame acquisition and temperature extraction |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D drawing library | Rendering thermal pixels and UI overlays |
| [st7735-lcd](https://crates.io/crates/st7735-lcd) / [ili9341](https://crates.io/crates/ili9341) | TFT display drivers | Sending rendered buffers to the display |
| [heapless](https://github.com/rust-embedded/heapless) | No-std data structures | Fixed-capacity buffers without allocator |
| [embedded-sdmmc](https://github.com/rust-embedded-community/embedded-sdmmc-rs) | FAT filesystem support | Saving captured thermal frames |
| [express](https://expressjs.com/) | Node.js web framework | Lightweight API backend for mobile add-on |
| [multer](https://github.com/expressjs/multer) / [sharp](https://sharp.pixelplumbing.com/) | Upload and image processing | Frame upload handling and optional JPEG encoding |
| [expo-image](https://docs.expo.dev/versions/latest/sdk/image/) / [expo-media-library](https://docs.expo.dev/versions/latest/sdk/media-library/) | React Native media tools | Display and save captured images on mobile |
| [axios](https://axios-http.com/) | HTTP client | API communication from mobile app |
| [zustand](https://github.com/pmndrs/zustand) | State management | Local app state for stream/history/settings |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [OpenTemp Thermal Imager Project](https://roboticworx.io/blogs/projects/opentemp-thermal-imager-infrared-thermometer)
2. [mlx9064x crate documentation](https://docs.rs/mlx9064x)
3. [Embassy framework](https://embassy.dev)
4. [MLX90640 Datasheet Page](https://www.melexis.com/en/product/MLX90640)
5. [Project Inspo](https://youtu.be/2-_Wgspjkdw?si=q2qfbzEd-fa41ypF)
6. [FLIR Lepton — Basic AGC for radiometric images](https://oem.flir.com/learn/discover/basic-agc-for-radiometric-lepton-images/) — the AGC / histogram-EQ pipeline driving the live thermal view is built on this.
