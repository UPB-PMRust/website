# OxiNMP
Digital Signal Processing on a INMP microphone, having multiple voice effects chosen by the user.

:::info 

**Author**: Khalghamouz Ali-Nicolas \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-K-Nicolas-10

:::

<!-- do not delete the \ after your name -->

## Description

My project's purpose is to have multiple integrated DSP algorithms (aiming to implement 4) that transform the captured audio into audio with multiple effects, kind of like a Voice Changer. I aim to have an 8-bit Retro audio effect, a high pitch effect, a low pitch effect and a sci-fi sounding effect (kind of like Dalke).

## Motivation

Choosing this project was a mix of my interest in mathematics and wanting to learn DMA more in-depth.

## Architecture 

- **4x Buttons:** This acts as a control panel for all the audio effects. They are connected to the GPIO pins on the STM32 configured as hardware interrupts.
- **INMP441 Module:** It sends clean digital audio arryas straight to the STM32 via I2S
- **STM32:** Calculates FFT to draw on the screen and the algorithms for the audio effects.
- **UDA1334A:** Takes the I2S output from the STM32 and outputs it into a 3.5mm Jack where you connect speakers or headphones.
- **SPI TFT LCD Display:** Chose an SPI screen over a I2C screen because it handles video better than I2C. I2C is too slow for smooth video.


![Architecture Diagram](./architecture.svg)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Microcontroller: STM32U545RE Nucleo (Core logic)

Input: INMP441 I2S Microphone (Captures audio)

Output: UDA1334A I2S DAC (Plays audio)

Display: SPI TFT LCD (Shows visualizer)

Interface: 4x Tactile Buttons (Changes effects)

### Schematics

Not available right now.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
|[STM32 Nucleo-U545RE](https://www.st.com/en/evaluation-tools/nucleo-u545re.html) | The microcontroller | [Borrowed]() |
| [INMP441 I2S Microphone](https://invensense.tdk.com/products/inmp441/) | Captures ambient audio digitally via I2S, bypassing analog ADC noise. | [21 RON](https://www.bitmi.ro/module-electronice/modul-microfon-omnidirectional-interfata-i2s-mems-inmp441-11003.html) |
| [UDA1334A I2S DAC Module](https://learn.adafruit.com/adafruit-i2s-stereo-decoder-uda1334a) | Decodes processed digital audio to an analog signal via a 3.5mm jack. | [37 RON](https://www.emag.ro/decodor-dac-stereo-uda1334a-convertor-digital-analogic-i2s-3-3-v-5-v-s-038/pd/D9WPHR3BM) |
| [Starting Electronics Kit](https://www.emag.ro/kit-start-componente-electronice-ai777/pd/DXRJ4TMBM/) | Contains Miscellaneous components like resistors,breadboard,etc | [51 RON](https://www.emag.ro/kit-start-componente-electronice-ai777/pd/DXRJ4TMBM/) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | Hardware Abstraction Layer for STM32 | Used for accessing I2S, SPI, DMA, and GPIO peripherals asynchronously. |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async runtime for embedded systems | Used for managing and scheduling concurrent tasks (audio pipeline, UI, buttons). |
| [embassy-sync](https://crates.io/crates/embassy-sync) | Synchronization primitives for async | Used for safely passing audio data buffers between async tasks via channels. |
| [embedded-graphics](https://crates.io/crates/embedded-graphics) | `no_std` 2D graphics library | Used for drawing the real-time audio waveforms and frequency bars on the display. |
| [mipidsi](https://crates.io/crates/mipidsi) | Display driver for MIPI DSI displays | Used as the hardware driver for the ILI9341 or ST7789 SPI TFT LCD. |
| [microfft](https://crates.io/crates/microfft) | Fast Fourier Transform library for `no_std` | Used for calculating the frequency spectrum for the audio visualizer. |



<!-- Add a few links that inspired you and that you think you will use for your project -->
## Links

1. [Embassy-rs Official Book](https://embassy.dev/book/)
2. [STM32 NUCLEO-U545RE Board Documentation](https://www.st.com/en/evaluation-tools/nucleo-u545re.html#documentation)
3. [INMP441 I2S Digital Microphone Official Datasheet](https://invensense.tdk.com/wp-content/uploads/2015/02/INMP441.pdf)
4. [Adafruit UDA1334A I2S DAC Guide](https://learn.adafruit.com/adafruit-i2s-stereo-decoder-uda1334a/overview)
5. [embedded-graphics Rust Crate Documentation](https://docs.rs/embedded-graphics/latest/embedded_graphics/)
6. [mipidsi Rust Crate Documentation](https://docs.rs/mipidsi/latest/mipidsi/)
7. [microfft Rust Crate Documentation](https://docs.rs/microfft/latest/microfft/)
8. [STM32U5 Series Reference Manual](https://www.st.com/resource/en/reference_manual/rm0456-stm32u575585-armbased-32bit-mcus-stmicroelectronics.pdf)