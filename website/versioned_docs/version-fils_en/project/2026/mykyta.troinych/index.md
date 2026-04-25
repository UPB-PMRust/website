---
Audio Spectrum Analyzer on STM32 by Mykyta Troinych
---
## Description
   This project implements a real-time audio spectrum analyzer on an embedded system.  
The device captures audio input through a 3.5 mm jack, converts it into a digital signal using an external ADC, processes it using Fast Fourier Transform (FFT), and displays the frequency spectrum on an SPI-connected TFT screen.

   The purpose of the project is to provide a visual tool for analyzing audio signals in real time, useful for sound engineering and signal analysis.
---
## System Architecture
   The signal processing pipeline is structured as follows:
- Audio Input (3.5 mm jack)  
- External ADC (PCM1808 – I2S output)  
- I2S Interface + DMA (STM32)  
- Double Buffering  
- FFT Processing  
- SPI Display Output  
---
## Hardware
- Microcontroller: STM32 Nucleo-U545RE-Q  
- Audio ADC: PCM1808 (24-bit stereo ADC, I2S interface)  
- Display: 2.4" TFT SPI (ST7789V, 240x320)  
- Input: 3.5 mm audio jack  
## Current Status
- SPI display initialized and tested  
- Hardware connections completed  
- ADC integration in progress  
---
## Software
   The firmware is developed in Rust for embedded systems.
   Main components:
- I2S interface for audio data acquisition  
- DMA for continuous data streaming  
- Double buffering for real-time processing  
- FFT algorithm for spectral analysis  
- SPI driver for display rendering  
---
## Signal Processing
- Sampling rate: 48 kHz  
- Input mode: Mono (single channel extracted from stereo stream)  
   Processing steps:
1. Continuous sampling using I2S + DMA  
2. Buffer filling using double buffering  
3. Windowing function (planned: Hann/Hamming)  
4. FFT computation  
5. Magnitude spectrum visualization  
FFT size will be determined experimentally (initial target: 256–1024 samples), depending on performance constraints.
---
## Prototype
   A PC-based prototype was implemented to validate the DSP pipeline before porting to embedded hardware.
prototype.webp
   The prototype demonstrates:
- Frequency spectrum visualization  
- Real-time signal analysis  
---
## Device Concept
device.webp
   The final device is designed as a compact standalone spectrum analyzer with an integrated display.
---
## Challenges
- Real-time FFT on a constrained microcontroller  
- Efficient use of DMA and buffering  
- Trade-off between latency and frequency resolution  
- SPI bandwidth limitations for smooth rendering  
---
## Future Work
- Optimize FFT using DSP libraries (e.g. CMSIS-DSP)  
- Improve graphical interface  
- Add user controls (gain, scaling)  
- Implement logarithmic frequency scaling  
---

