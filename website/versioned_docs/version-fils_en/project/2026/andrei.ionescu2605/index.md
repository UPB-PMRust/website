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

How components connect:

```
MLX90640 Sensor
     |
     | I2C
     v
Sensor Acquisition Module
     |
     v
Processing Pipeline
     |
     +---------------------> Storage Manager (microSD)
     |
     +---------------------> Communication Module (WiFi -> Mobile App)
     |
     v
Rendering Engine -> TFT Display
     ^
     |
UI Controller (Joystick)
```

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May
- Initial idea was a reflex-based system.
- Pivoted to thermal imaging after feedback.
- Selected MLX90640 as core sensor.
- Studied MLX90640 communication and frame format.

### Week 12 - 18 May
- Defined system blocks and data flow.
- Selected peripherals: TFT display, microSD, joystick, optional WiFi.
- Planned local processing pipeline (normalization and color mapping).

### Week 19 - 25 May
- Planned hardware integration order.
- Planned first bring-up tests for I2C communication with MLX90640.
- Planned display pipeline validation with test frames.

## Hardware

The system uses an STM32 NUCLEO-U545RE-Q for development, an MLX90640 thermal sensor, a TFT display for visualization, a microSD module for storage, and a joystick for navigation. An ESP8266/ESP32 module can be added for WiFi transfer to a mobile app.

### Schematics

<svg width="820" height="260" viewBox="0 0 820 260" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Project hardware schematic overview">
  <rect x="10" y="20" width="170" height="70" fill="none" stroke="black"/>
  <text x="95" y="60" text-anchor="middle" font-size="14">MLX90640 (I2C)</text>

  <rect x="250" y="20" width="220" height="100" fill="none" stroke="black"/>
  <text x="360" y="60" text-anchor="middle" font-size="14">STM32 NUCLEO-U545RE-Q</text>
  <text x="360" y="82" text-anchor="middle" font-size="12">Main Controller</text>

  <rect x="540" y="20" width="160" height="70" fill="none" stroke="black"/>
  <text x="620" y="60" text-anchor="middle" font-size="14">TFT Display (SPI)</text>

  <rect x="540" y="110" width="160" height="60" fill="none" stroke="black"/>
  <text x="620" y="145" text-anchor="middle" font-size="14">microSD (SPI)</text>

  <rect x="250" y="160" width="220" height="70" fill="none" stroke="black"/>
  <text x="360" y="200" text-anchor="middle" font-size="14">Joystick + WiFi (UART)</text>

  <line x1="180" y1="55" x2="250" y2="55" stroke="black"/>
  <line x1="470" y1="55" x2="540" y2="55" stroke="black"/>
  <line x1="470" y1="120" x2="540" y2="140" stroke="black"/>
  <line x1="360" y1="120" x2="360" y2="160" stroke="black"/>
</svg>

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
