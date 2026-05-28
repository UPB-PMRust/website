#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::adc::{self, Adc, Channel as AdcChannel, Config as AdcConfig};
use embassy_rp::bind_interrupts;
use embassy_rp::dma::InterruptHandler as DmaInterruptHandler;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler as PioInterruptHandler, Pio};
use embassy_rp::pio_programs::i2s::{PioI2sOut, PioI2sOutProgram};
use embassy_rp::spi::{self, Spi};
use embassy_time::{Delay, Instant};
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::{SdCard, VolumeManager, VolumeIdx, Mode, ShortFileName};
use heapless::Vec;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0   => PioInterruptHandler<PIO0>;
    DMA_IRQ_0    => DmaInterruptHandler<DMA_CH0>;
    ADC_IRQ_FIFO => adc::InterruptHandler;
});

const SAMPLE_RATE: u32 = 48_000;
const BUFFER_SAMPLES: usize = 480;
const MAX_FILES: usize = 32;

struct DummyTimesource;
impl embedded_sdmmc::TimeSource for DummyTimesource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 54,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

fn parse_wav_header(header: &[u8; 44]) -> Option<(u32, u16, u16, u32)> {
    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" || &header[12..16] != b"fmt " {
        return None;
    }
    let channels = u16::from_le_bytes([header[22], header[23]]);
    let sample_rate = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
    let bit_depth = u16::from_le_bytes([header[34], header[35]]);
    let data_size = u32::from_le_bytes([header[40], header[41], header[42], header[43]]);

    Some((sample_rate, channels, bit_depth, data_size))
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("=== WAV Player RP235x ===");

    let p = embassy_rp::init(Default::default());

    let btn_play = Input::new(p.PIN_14, Pull::Up);
    let btn_next = Input::new(p.PIN_15, Pull::Up);

    let mut adc = Adc::new(p.ADC, Irqs, AdcConfig::default());
    let mut pot = AdcChannel::new_pin(p.PIN_26, Pull::None);

    // I2S
    let Pio { mut common, sm0, .. } = Pio::new(p.PIO0, Irqs);
    let i2s_prog = PioI2sOutProgram::new(&mut common);
    let mut i2s = PioI2sOut::new(
        &mut common, sm0, p.DMA_CH0, Irqs,
        p.PIN_9, p.PIN_10, p.PIN_11,
        SAMPLE_RATE, 16, &i2s_prog,
    );
    i2s.start();

    // SD Card
    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = 400_000;
    let spi = Spi::new_blocking(p.SPI0, p.PIN_2, p.PIN_3, p.PIN_4, spi_cfg);
    let cs = Output::new(p.PIN_5, Level::High);
    let spi_dev = ExclusiveDevice::new(spi, cs, Delay).unwrap();
    let sdcard = SdCard::new(spi_dev, Delay);

    info!("Initializare SD...");
    match sdcard.num_bytes() {
        Ok(size) => info!("SD card: {} MB", size / 1024 / 1024),
        Err(e) => {
            error!("SD eroare: {:?}", defmt::Debug2Format(&e));
            loop {}
        }
    }
    // Crește SPI după init
    sdcard.spi(|dev| dev.bus_mut().set_config(&{
        let mut c = spi::Config::default();
        c.frequency = 16_000_000;
        c
    }));

    let mut vol_mgr = VolumeManager::new(sdcard, DummyTimesource);
    let mut volume = vol_mgr.open_volume(VolumeIdx(0)).unwrap();
    let mut root = volume.open_root_dir().unwrap();

    // === Scanare fișiere .wav ===
    let mut files: Vec<ShortFileName, MAX_FILES> = Vec::new();
    info!("Scanare fisiere WAV...");
    root.iterate_dir(|entry| {
        let name = entry.name.base_name();
        let ext = entry.name.extension();
        if ext.eq_ignore_ascii_case(b"WAV") && !entry.attributes.is_directory() {
            let _ = files.push(entry.name.clone());
            info!("  Gasit: {}.{} ({} bytes)",
                core::str::from_utf8(name).unwrap_or("?"),
                core::str::from_utf8(ext).unwrap_or("?"),
                entry.size
            );
        }
    }).unwrap();

    if files.is_empty() {
        error!("Niciun fisier WAV pe card!");
        loop {}
    }
    info!("Total: {} fisiere", files.len());

    // === State playerul ===
    let mut current_file: usize = 0;
    let mut playing: bool = true;
    let mut volume_scale: f32 = 0.5;  // 0.0 - 1.0
    
    // State butoane (pentru edge detection)
    let mut btn_play_prev: bool = true;  // pull-up = HIGH normal
    let mut btn_next_prev: bool = true;
    let mut last_btn_check = Instant::now();
    let mut last_pot_log = Instant::now();

    // Buffer pentru I2S
    let mut i2s_buf = [0u32; BUFFER_SAMPLES];

    // === Loop principal ===
    'outer: loop {
        info!("Redare: {}", core::str::from_utf8(files[current_file].base_name()).unwrap_or("?"));

        // Deschide fișierul curent
        let mut file = match root.open_file_in_dir(files[current_file].clone(), Mode::ReadOnly) {
            Ok(f) => f,
            Err(e) => {
                error!("Nu pot deschide: {:?}", defmt::Debug2Format(&e));
                current_file = (current_file + 1) % files.len();
                continue;
            }
        };

        // Citește header WAV
        let mut header = [0u8; 44];
        if file.read(&mut header).is_err() {
            error!("Eroare citire header");
            current_file = (current_file + 1) % files.len();
            continue;
        }

        let (sr, ch, bd, data_size) = match parse_wav_header(&header) {
            Some(h) => h,
            None => {
                error!("Header WAV invalid");
                current_file = (current_file + 1) % files.len();
                continue;
            }
        };

        info!("WAV: {} Hz, {} ch, {} bit, {} bytes", sr, ch, bd, data_size);

        if sr != SAMPLE_RATE || ch != 2 || bd != 16 {
            warn!("Format diferit ({}Hz {}ch {}bit), reproducere posibil incorecta", sr, ch, bd);
        }

        // === Inner loop: redare fișier ===
        let mut bytes_read: u32 = 0;
        let mut raw_buf = [0u8; BUFFER_SAMPLES * 4];  // 4 bytes per stereo frame

        loop {
            // === Verifică butoane (la fiecare ~5ms) ===
            if last_btn_check.elapsed().as_millis() >= 5 {
                last_btn_check = Instant::now();
                
                let play_now = btn_play.is_high();
                let next_now = btn_next.is_high();

                // Edge falling = apăsare (pull-up: HIGH→LOW când apeși)
                if !play_now && btn_play_prev {
                    playing = !playing;
                    info!(">> {}", if playing { "PLAY" } else { "PAUSE" });
                }
                if !next_now && btn_next_prev {
                    info!(">> NEXT");
                    current_file = (current_file + 1) % files.len();
                    file.close().unwrap_or(());
                    continue 'outer;
                }
                btn_play_prev = play_now;
                btn_next_prev = next_now;
            }

            // === Citește pot la fiecare ~50ms ===
            if last_pot_log.elapsed().as_millis() >= 50 {
                last_pot_log = Instant::now();
                if let Ok(raw) = adc.read(&mut pot).await {
                    let new_vol = raw as f32 / 4095.0;
                    // Smoothing simplu
                    volume_scale = volume_scale * 0.7 + new_vol * 0.3;
                }
            }

            // === Dacă playing, redă audio ===
            if playing {
                // Citește chunk din WAV
                let bytes_to_read = core::cmp::min(
                    raw_buf.len(),
                    (data_size - bytes_read) as usize,
                );
                
                if bytes_to_read == 0 {
                    info!("Sfarsit fisier - trec la urmatorul");
                    current_file = (current_file + 1) % files.len();
                    file.close().unwrap_or(());
                    continue 'outer;
                }

                let n = match file.read(&mut raw_buf[..bytes_to_read]) {
                    Ok(n) => n,
                    Err(_) => {
                        error!("Eroare citire");
                        current_file = (current_file + 1) % files.len();
                        file.close().unwrap_or(());
                        continue 'outer;
                    }
                };
                bytes_read += n as u32;

                // Converteste raw bytes la samples I2S, aplica volum
                let frames = n / 4;
                for i in 0..frames {
                    let l_raw = i16::from_le_bytes([raw_buf[i*4],     raw_buf[i*4 + 1]]);
                    let r_raw = i16::from_le_bytes([raw_buf[i*4 + 2], raw_buf[i*4 + 3]]);
                    
                    let l = ((l_raw as f32) * volume_scale) as i16;
                    let r = ((r_raw as f32) * volume_scale) as i16;
                    
                    let l_u = l as u16 as u32;
                    let r_u = r as u16 as u32;
                    i2s_buf[i] = (l_u << 16) | r_u;
                }
                
                // Zero rest dacă buffer parțial
                for i in frames..BUFFER_SAMPLES {
                    i2s_buf[i] = 0;
                }

                i2s.write(&i2s_buf).await;
            } else {
                // PAUSE - scrie liniște la I2S (0) ca să nu se audă pop-uri
                i2s_buf.iter_mut().for_each(|x| *x = 0);
                i2s.write(&i2s_buf).await;
            }
        }
    }
}