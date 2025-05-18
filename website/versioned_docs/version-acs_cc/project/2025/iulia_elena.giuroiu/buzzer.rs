#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::pwm::{Config as PwmConfig, Pwm};
use embassy_time::{Timer, Duration};
use panic_probe as _;

// Import interrupts definition module
mod irqs;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let p = embassy_rp::init(Default::default());

    info!("Buzzer Demo - Pornire!");

    // Configurare pentru buzzer
    let mut buzzer_config = PwmConfig::default();
    
    // Setăm frecvența PWM pentru a genera un ton distinct (aproximativ 2kHz)
    buzzer_config.top = 0x0FFF;  // Valoare TOP
    buzzer_config.divider = 10.into();  // Divider pentru frecvența ceasului
    
    // Setăm duty cycle-ul la 50% pentru ton clar
    buzzer_config.compare_b = buzzer_config.top / 2;

    // Initializare PWM pentru buzzer pe GP14
    // GP14 este la PWM_SLICE7, Channel A
    let mut buzzer = Pwm::new_output_a(
        p.PWM_SLICE7,
        p.PIN_14,
        buzzer_config.clone(),
    );

    // Configurăm două configurații - una pentru ton activ, una pentru silențios
    let mut tone_on = buzzer_config.clone();
    let mut tone_off = buzzer_config.clone();
    
    // Pentru ton activ, folosim 50% duty cycle
    tone_on.compare_a = buzzer_config.top / 2;
    
    // Pentru silențios, setăm compare_a la 0 (0% duty cycle)
    tone_off.compare_a = 0;

    // Inițial oprim buzzer-ul
    buzzer.set_config(&tone_off);
    
    loop {
        // Activăm buzzer-ul
        info!("Buzzer PORNIT");
        buzzer.set_config(&tone_on);
        
        // Ținem buzzer-ul pornit timp de 1 secundă
        Timer::after(Duration::from_secs(1)).await;
        
        // Oprim buzzer-ul
        info!("Buzzer OPRIT");
        buzzer.set_config(&tone_off);
        
        // Așteptăm 3 secunde înainte de a porni din nou
        Timer::after(Duration::from_secs(3)).await;
    }
}