#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::*;
use embassy_time::{Duration, Timer};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Rânduri (output): GPIO9 → R1, GPIO8 → R2, GPIO7 → R3, GPIO6 → R4
    let mut rows = [
        Output::new(p.PIN_9, Level::Low), // R1
        Output::new(p.PIN_8, Level::Low), // R2
        Output::new(p.PIN_7, Level::Low), // R3
        Output::new(p.PIN_6, Level::Low), // R4
    ];

    // Coloane (input): GPIO5 → C1, GPIO4 → C2, GPIO3 → C3, GPIO2 → C4
    let cols = [
        Input::new(p.PIN_5, Pull::Down), // C1
        Input::new(p.PIN_4, Pull::Down), // C2
        Input::new(p.PIN_3, Pull::Down), // C3
        Input::new(p.PIN_2, Pull::Down), // C4
    ];

    // Matrice taste [rând][coloană]
    let keys: [[char; 4]; 4] = [
        ['1', '2', '3', 'A'],
        ['4', '5', '6', 'B'],
        ['7', '8', '9', 'C'],
        ['*', '0', '#', 'D'],
    ];

    info!("Tastatură 4x4 activă. Apasă taste:");
    
    let mut key_pressed = false;
    
    loop {
        // Asigură-te că niciun rând nu este activ la începutul scanării
        for row in rows.iter_mut() {
            row.set_low();
        }
        
        // Verificare dacă o tastă este apăsată
        if key_pressed {
            // Așteaptă ca tasta să fie eliberată înainte de a continua scanarea
            let mut all_released = true;
            
            for row in rows.iter_mut() {
                row.set_high();
                
                for col in cols.iter() {
                    if col.is_high() {
                        all_released = false;
                    }
                }
                
                row.set_low();
            }
            
            if all_released {
                key_pressed = false;
                Timer::after(Duration::from_millis(50)).await; // Debounce după eliberare
            }
        } else {
            // Scanarea normală a tastaturii
            for (i, row) in rows.iter_mut().enumerate() {
                row.set_high(); // Activează rândul curent
                Timer::after(Duration::from_micros(50)).await; // Mică întârziere pentru stabilizare
                
                for (j, col) in cols.iter().enumerate() {
                    if col.is_high() {
                        // Debounce
                        Timer::after(Duration::from_millis(20)).await;
                        
                        if col.is_high() {
                            info!("Tasta apăsată: {}", keys[i][j]);
                            key_pressed = true;
                            // Nu mai continua scanarea după ce ai găsit o tastă
                            break;
                        }
                    }
                }
                
                row.set_low(); // Dezactivează rândul curent
                
                if key_pressed {
                    break; // Ieși din bucla de scanare a rândurilor dacă o tastă a fost apăsată
                }
            }
        }
        
        Timer::after(Duration::from_millis(10)).await; // Întârziere între cicluri de scanare
    }
}