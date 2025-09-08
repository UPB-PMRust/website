#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Input, Pull};
use embassy_rp::spi::{Spi, Config};
use embassy_time::{Timer, Instant};

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

// JSON parsing imports
use serde::Deserialize;
use serde_json_core;
use heapless::Vec;

// Game states
#[derive(PartialEq, Clone, Copy)]
enum GameState {
    Welcome,
    ShowTitle,
    ShowStartPrompt,
    AlphabetSelection,
    ChooseExposeOrExplore,
    ShowExpose,
    ShowExplore,
    GameOver, // New game over state
}

// JSON structure
#[derive(Deserialize, Debug)]
struct QuestionList(Vec<heapless::String<64>, 60>); // Max 60 items, each max 64 chars

// Embedded JSON data as string constants
const TRUTHS_JSON: &str = include_str!("truths.json");
const DARES_JSON: &str = include_str!("dares.json");

// Timer constants
const CHALLENGE_TIME_LIMIT: u64 = 30000; // 30 seconds

// Get current time in milliseconds
fn get_current_time_ms() -> u64 {
    Instant::now().as_millis()
}

// Simple 5x8 font for display
const FONT: [[u8; 5]; 26] = [
    [0x7E, 0x09, 0x09, 0x09, 0x7E], // A
    [0x7F, 0x49, 0x49, 0x49, 0x36], // B  
    [0x3E, 0x41, 0x41, 0x41, 0x22], // C
    [0x7F, 0x41, 0x41, 0x22, 0x1C], // D
    [0x7F, 0x49, 0x49, 0x49, 0x41], // E
    [0x7F, 0x09, 0x09, 0x09, 0x01], // F
    [0x3E, 0x41, 0x49, 0x49, 0x7A], // G
    [0x7F, 0x08, 0x08, 0x08, 0x7F], // H
    [0x00, 0x41, 0x7F, 0x41, 0x00], // I
    [0x20, 0x40, 0x41, 0x3F, 0x01], // J
    [0x7F, 0x08, 0x14, 0x22, 0x41], // K
    [0x7F, 0x40, 0x40, 0x40, 0x40], // L
    [0x7F, 0x02, 0x0C, 0x02, 0x7F], // M
    [0x7F, 0x04, 0x08, 0x10, 0x7F], // N
    [0x3E, 0x41, 0x41, 0x41, 0x3E], // O
    [0x7F, 0x09, 0x09, 0x09, 0x06], // P
    [0x3E, 0x41, 0x51, 0x21, 0x5E], // Q
    [0x7F, 0x09, 0x19, 0x29, 0x46], // R
    [0x46, 0x49, 0x49, 0x49, 0x31], // S
    [0x01, 0x01, 0x7F, 0x01, 0x01], // T
    [0x3F, 0x40, 0x40, 0x40, 0x3F], // U
    [0x1F, 0x20, 0x40, 0x20, 0x1F], // V
    [0x3F, 0x40, 0x38, 0x40, 0x3F], // W
    [0x63, 0x14, 0x08, 0x14, 0x63], // X
    [0x07, 0x08, 0x70, 0x08, 0x07], // Y
    [0x61, 0x51, 0x49, 0x45, 0x43], // Z
];

// Cool emoji patterns
const COOL_EMOJI: [u8; 8] = [
    0b00111100, 
    0b01111110, 
    0b11111111, 
    0b11111111,   
    0b01111110, 
    0b00111100, 
    0b01000010, 
    0b00111100, 
];

const FIRE_EMOJI: [u8; 8] = [
    0b00010000,   
    0b00111000,  
    0b01111100,   
    0b11111110,   
    0b11111110,  
    0b01111100,   
    0b00111000,   
    0b00010000,  
];

// Buzzer sound functions
async fn quick_beep(buzzer: &mut Output<'_>) {
    buzzer.set_high();
    Timer::after_millis(100).await;
    buzzer.set_low();
}

async fn game_over_buzzer(buzzer: &mut Output<'_>) {
    // buzzer pattern for game over
    for _ in 0..5 {
        buzzer.set_high();
        Timer::after_millis(500).await;  // 500ms ON
        buzzer.set_low();
        Timer::after_millis(200).await;  // 200ms OFF
    }
}

async fn button_sound(buzzer: &mut Output<'_>) {
    buzzer.set_high();
    Timer::after_millis(50).await;
    buzzer.set_low();
}

// Display "GAME OVER" in big font like welcome screen
async fn display_game_over(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>) {
    clear_screen(spi, cs, dc).await;
    
    // Display "GAME" on first line
    display_colored_single_big_letter(spi, cs, dc, "G", 15, 10, false).await; // Blue
    display_colored_single_big_letter(spi, cs, dc, "A", 30, 10, false).await; // Blue
    display_colored_single_big_letter(spi, cs, dc, "M", 45, 10, false).await; // Blue
    display_colored_single_big_letter(spi, cs, dc, "E", 60, 10, false).await; // Blue
    
    // Display "OVER" on second line
    display_colored_single_big_letter(spi, cs, dc, "O", 15, 35, true).await;  // Yellow
    display_colored_single_big_letter(spi, cs, dc, "V", 30, 35, true).await;  // Yellow
    display_colored_single_big_letter(spi, cs, dc, "E", 45, 35, true).await;  // Yellow
    display_colored_single_big_letter(spi, cs, dc, "R", 60, 35, true).await;  // Yellow
}

// Helper function to find questions/dares starting with a specific letter
fn find_items_by_letter(items: &Vec<heapless::String<64>, 60>, letter: char) -> Vec<&heapless::String<64>, 10> {
    let mut result = Vec::new();
    let letter_upper = letter.to_ascii_uppercase();
    
    for item in items.iter() {
        if let Some(first_char) = item.chars().next() {
            if first_char.to_ascii_uppercase() == letter_upper {
                let _ = result.push(item);
                if result.len() >= 10 { // Limit to prevent overflow
                    break;
                }
            }
        }
    }
    
    result
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    
    info!("🎮 EXPOSE OR EXPLORE Game Starting!");
    info!("==================================");
    
    // Parse JSON data at startup with better error handling
    let truths_data: QuestionList = match serde_json_core::from_str::<QuestionList>(TRUTHS_JSON) {
        Ok((data, _)) => {
            info!("✅ Truths JSON parsed successfully! Count: {}", data.0.len());
            data
        },
        Err(_) => {
            info!("❌ Failed to parse truths JSON - using minimal fallback");
            let mut fallback = Vec::new();
            let _ = fallback.push(heapless::String::try_from("What's your biggest secret?").unwrap_or(heapless::String::new()));
            QuestionList(fallback)
        }
    };
    
    let dares_data: QuestionList = match serde_json_core::from_str::<QuestionList>(DARES_JSON) {
        Ok((data, _)) => {
            info!("✅ Dares JSON parsed successfully! Count: {}", data.0.len());
            data
        },
        Err(_) => {
            info!("❌ Failed to parse dares JSON - using minimal fallback");
            let mut fallback = Vec::new();
            let _ = fallback.push(heapless::String::try_from("Dance for 1 minute!").unwrap_or(heapless::String::new()));
            QuestionList(fallback)
        }
    };
    
    // Built-in LED for status
    let mut led = Output::new(p.PIN_25, Level::Low);
    
    // Dual-color LED pins (Red/Green Common Cathode)
    let mut dual_red = Output::new(p.PIN_5, Level::Low);
    let mut dual_green = Output::new(p.PIN_6, Level::Low);
    
    // Timer control LEDs and buzzer
    let mut buzzer = Output::new(p.PIN_28, Level::Low);        // GP28 for buzzer
    let mut green_led = Output::new(p.PIN_26, Level::Low);     // GP26 for green LED (challenge active)
    let mut red_led = Output::new(p.PIN_27, Level::Low);       // GP27 for red LED (game over)
    
    // Extra LED (free pin)
    let mut led1 = Output::new(p.PIN_22, Level::Low);  // GP22 (free pin)
    
    // Button inputs with pull-up resistors
    let white_btn = Input::new(p.PIN_2, Pull::Up);   // WHITE (Start) button
    let blue_btn = Input::new(p.PIN_3, Pull::Up);    // BLUE (Expose) button  
    let red_btn = Input::new(p.PIN_4, Pull::Up);     // RED (Explore) button
    
    // Rotary encoder inputs (matching your diagram)
    let encoder_clk = Input::new(p.PIN_9, Pull::Up);   // GP9 (Yellow wire)
    let encoder_dt = Input::new(p.PIN_10, Pull::Up);   // GP10 (Green wire)  
    let encoder_sw = Input::new(p.PIN_11, Pull::Up);   // GP11 (Pink wire)
    
    info!("🔘 Buttons: WHITE=GP2, BLUE=GP3, RED=GP4");
    info!("🎛️ Encoder: CLK=GP9, DT=GP10, SW=GP11");
    info!("🔊 Buzzer: GP28");
    info!("🔴🟢 Timer LEDs: GREEN=GP26, RED=GP27");
    info!("💡 Connect buttons between GPIO and GND");
    info!("🔴🟢 Dual LED: RED=GP5, GREEN=GP6, CATHODE=GND");
    info!("💡 Extra LEDs: LED1=GP22");
    
    // Simple pseudo-random counter for selection
    let mut random_counter = 0u32;
    
    // Timer variables
    let mut challenge_start_time = 0u64;
    let mut timer_started = false;
    
    // SPI Configuration
    let mut spi_config = Config::default();
    spi_config.frequency = 8_000_000;
    
    let mut spi = Spi::new(
        p.SPI0, p.PIN_18, p.PIN_19, p.PIN_16,
        p.DMA_CH0, p.DMA_CH1, spi_config,
    );
    
    // OLED Control pins
    let mut cs = Output::new(p.PIN_17, Level::High);
    let mut dc = Output::new(p.PIN_20, Level::Low);
    let mut res = Output::new(p.PIN_21, Level::High);
    
    // Hardware reset
    res.set_low();
    Timer::after_millis(10).await;
    res.set_high();
    Timer::after_millis(10).await;
    
    // Initialize OLED
    let init_commands = [
        0xAE, 0xD5, 0x80, 0xA8, 0x3F, 0xD3, 0x00, 0x40,
        0x8D, 0x14, 0x20, 0x00, 0xA1, 0xC8, 0xDA, 0x12,
        0x81, 0xCF, 0xD9, 0xF1, 0xDB, 0x40, 0xA4, 0xA6, 0xAF,
    ];
    
    cs.set_low();
    for &cmd in &init_commands {
        dc.set_low();
        let _ = spi.write(&[cmd]).await;
    }
    cs.set_high();
    
    info!("✅ OLED initialized!");
    
    // LED TEST - Test all LEDs
    info!("🔴🟢 Testing all LEDs...");
    dual_red.set_high();
    dual_green.set_high();
    green_led.set_high();
    red_led.set_high();
    led1.set_high();
    Timer::after_millis(2000).await;
    
    // Turn off all LEDs
    dual_red.set_low();
    dual_green.set_low();
    green_led.set_low();
    red_led.set_low();
    led1.set_low();
    
    // Test buzzer
    info!("🔊 Testing buzzer...");
    quick_beep(&mut buzzer).await;
    Timer::after_millis(500).await;
    quick_beep(&mut buzzer).await;
    info!("🔊 Buzzer test complete");
    
    let mut game_state = GameState::Welcome;
    
    // Button debounce helpers
    let mut white_pressed = false;
    let mut blue_pressed = false;  
    let mut red_pressed = false;
    
    // Horizontal scrolling animation variables
    let mut scroll_offset = 0i16;
    let mut restart_scroll_offset = 0i16; // Add separate scroll for restart text
    
    // Track if we need to draw expose/explore content once
    let mut expose_drawn = false;
    let mut explore_drawn = false;
    let mut scrolling_drawn = false; // Add flag for scrolling text
    let mut game_over_drawn = false; // Flag for game over screen
    
    // Alphabet selection variables
    let mut current_letter = 0u8; // 0=A, 1=B, ..., 25=Z
    let mut selected_letter = 0u8;
    
    // Encoder state tracking
    let mut last_clk = encoder_clk.is_high();
    let mut encoder_pressed = false;
    
    loop {
        led.set_high();
        
        // Increment random counter continuously for pseudo-randomness
        random_counter = random_counter.wrapping_add(1);
        
        // Get current time for timer logic
        let current_time = get_current_time_ms();
        
        // TIMER LOGIC - Check for 30-second timeout during challenges
        if (game_state == GameState::ShowExpose || game_state == GameState::ShowExplore) {
            // Start timer when challenge first appears
            if !timer_started {
                challenge_start_time = current_time;
                timer_started = true;
                green_led.set_high(); // Turn on green LED at start
                button_sound(&mut buzzer).await; // Sound when challenge starts
                info!("⏰ Challenge timer started - 30 seconds!");
            }
            
            // Blink green LED during challenge (every 500ms)
            if (current_time / 500) % 2 == 0 {
                green_led.set_high();
            } else {
                green_led.set_low();
            }
            
            // Check if time is up (30 seconds)
            if current_time - challenge_start_time >= CHALLENGE_TIME_LIMIT {
                game_state = GameState::GameOver;
                timer_started = false;
                green_led.set_low(); // Turn off green LED
                game_over_drawn = false; // Need to draw game over screen
                info!("⏰ TIME'S UP! GAME OVER!");
            }
        }
        
        // Handle Game Over state
        if game_state == GameState::GameOver {
            if !game_over_drawn {
                display_game_over(&mut spi, &mut cs, &mut dc).await;
                game_over_buzzer(&mut buzzer).await; // Play game over sound
                game_over_drawn = true;
                info!("💀 Game Over screen displayed");
            }
            
            // Blink red LED rapidly during game over (every 250ms)
            if (current_time / 250) % 2 == 0 {
                red_led.set_high();
            } else {
                red_led.set_low();
            }
        } else {
            // Make sure red LED is off when not in game over
            red_led.set_low();
        }
        
        // Read rotary encoder
        let current_clk = encoder_clk.is_high();
        let encoder_sw_state = encoder_sw.is_low(); // Active low
        
        // Detect encoder rotation
        if current_clk != last_clk && current_clk {
            // CLK changed from low to high
            if encoder_dt.is_low() {
                // Clockwise
                current_letter = (current_letter + 1) % 26;
                button_sound(&mut buzzer).await; // Sound when rotating
                info!("🔤 Letter: {}", (b'A' + current_letter) as char);
            } else {
                // Counter-clockwise  
                current_letter = if current_letter == 0 { 25 } else { current_letter - 1 };
                button_sound(&mut buzzer).await; // Sound when rotating
                info!("🔤 Letter: {}", (b'A' + current_letter) as char);
            }
        }
        last_clk = current_clk;
        
        // Handle encoder button press
        if encoder_sw_state && !encoder_pressed {
            encoder_pressed = true;
            button_sound(&mut buzzer).await; // Sound when pressing encoder
            if game_state == GameState::AlphabetSelection {
                selected_letter = current_letter;
                game_state = GameState::ChooseExposeOrExplore;
                scrolling_drawn = false; // Reset scrolling when entering new state
                info!("✅ Selected letter: {}", (b'A' + selected_letter) as char);
            }
        } else if !encoder_sw_state {
            encoder_pressed = false;
        }
        
        // Read buttons (active low - pressed = false)
        let white_btn_state = white_btn.is_low();
        let blue_btn_state = blue_btn.is_low();
        let red_btn_state = red_btn.is_low();
        
        // Handle button presses with debouncing
        if white_btn_state && !white_pressed {
            white_pressed = true;
            // Button press flash - quick yellow flash (both colors)
            dual_red.set_high();
            dual_green.set_high();
            button_sound(&mut buzzer).await; // Sound when pressing button
            Timer::after_millis(100).await;
            
            info!("🔘 WHITE button pressed!");
            
            match game_state {
                GameState::ShowStartPrompt => {
                    game_state = GameState::AlphabetSelection;
                    current_letter = 0; // Start with 'A'
                    info!("🔤 Choose your letter!");
                },
                GameState::ShowExpose | GameState::ShowExplore => {
                    game_state = GameState::AlphabetSelection;
                    current_letter = 0; // Reset to 'A'
                    expose_drawn = false;
                    explore_drawn = false;
                    scrolling_drawn = false; // Reset scrolling flag
                    scroll_offset = 128; // Reset scroll positions
                    restart_scroll_offset = 128;
                    timer_started = false; // Reset timer
                    green_led.set_low(); // Turn off green LED
                    info!("🔄 New round - choose letter!");
                },
                GameState::GameOver => {
                    // Reset from game over
                    game_state = GameState::AlphabetSelection;
                    current_letter = 0;
                    expose_drawn = false;
                    explore_drawn = false;
                    scrolling_drawn = false;
                    game_over_drawn = false;
                    timer_started = false;
                    green_led.set_low();
                    red_led.set_low();
                    info!("🔄 Game reset - choose letter!");
                },
                _ => {}
            }
        } else if !white_btn_state {
            white_pressed = false;
        }
        
        if blue_btn_state && !blue_pressed {
            blue_pressed = true;
            // Button press flash - quick red flash (for EXPOSE/truth)
            dual_red.set_high();
            dual_green.set_low();
            led1.set_high();
            button_sound(&mut buzzer).await; // Sound when pressing button
            Timer::after_millis(150).await;
            
            if game_state == GameState::ChooseExposeOrExplore {
                game_state = GameState::ShowExpose;
                expose_drawn = false; // Mark that we need to draw
                info!("💭 EXPOSE selected for letter {}!", (b'A' + selected_letter) as char);
            }
        } else if !blue_btn_state {
            blue_pressed = false;
        }
        
        if red_btn_state && !red_pressed {
            red_pressed = true;
            // Button press flash - quick green flash (for EXPLORE/dare)
            dual_red.set_low();
            dual_green.set_high();
            led1.set_high();
            button_sound(&mut buzzer).await; // Sound when pressing button
            Timer::after_millis(150).await;
            
            if game_state == GameState::ChooseExposeOrExplore {
                game_state = GameState::ShowExplore;
                explore_drawn = false; // Mark that we need to draw
                info!("⚡ EXPLORE selected for letter {}!", (b'A' + selected_letter) as char);
            }
        } else if !red_btn_state {
            red_pressed = false;
        }
        
        // Clear screen for all states except when content is already drawn
        match game_state {
            GameState::ShowExpose | GameState::ShowExplore => {
                if !expose_drawn && !explore_drawn {
                    clear_screen(&mut spi, &mut cs, &mut dc).await;
                }
            },
            GameState::ChooseExposeOrExplore => {
                if !scrolling_drawn {
                    clear_screen(&mut spi, &mut cs, &mut dc).await;
                }
            },
            GameState::GameOver => {
                // Don't clear during game over - screen already drawn
            },
            _ => {
                clear_screen(&mut spi, &mut cs, &mut dc).await;
            }
        }
        
        // Display current state
        match game_state {
            GameState::Welcome => {
                // Display WELCOME using colored single big letters - W,E,L in yellow, C,O,M,E in blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "W", 10, 15, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "E", 25, 15, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "L", 40, 15, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "C", 55, 15, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "O", 70, 15, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "M", 85, 15, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "E", 100, 15, false).await; // Blue
                
            
                display_emoji(&mut spi, &mut cs, &mut dc, &COOL_EMOJI, 40, 45).await;
                display_emoji(&mut spi, &mut cs, &mut dc, &FIRE_EMOJI, 60, 45).await;
                display_emoji(&mut spi, &mut cs, &mut dc, &COOL_EMOJI, 80, 45).await;
                
                // Auto-advance after 5 seconds
                Timer::after_millis(5000).await;
                game_state = GameState::ShowTitle;
                info!("⏰ Welcome complete - showing title");
            },
            
            GameState::ShowTitle => {
                // EXPOSE in yellow (using colored letters)
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "E", 15, 5, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "X", 30, 5, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "P", 45, 5, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "O", 60, 5, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "S", 75, 5, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "E", 90, 5, true).await;  // Yellow
                
                // OR in yellow (centered)
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "O", 50, 25, true).await; // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "R", 65, 25, true).await; // Yellow
                
                // EXPLORE in blue (bottom)
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "E", 10, 45, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "X", 25, 45, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "P", 40, 45, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "L", 55, 45, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "O", 70, 45, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "R", 85, 45, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "E", 100, 45, false).await; // Blue
                
                // Auto-advance after 5 seconds
                Timer::after_millis(5000).await;
                game_state = GameState::ShowStartPrompt;
                info!("⏰ Title complete - showing start prompt");
            },
            
            GameState::ShowStartPrompt => {
                // PRESS in yellow (top line)
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "P", 15, 5, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "R", 30, 5, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "E", 45, 5, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "S", 60, 5, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "S", 75, 5, true).await;  // Yellow
                
                // WHITE in blue (middle line)
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "W", 10, 25, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "H", 25, 25, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "I", 40, 25, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "T", 55, 25, false).await; // Blue
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "E", 70, 25, false).await; // Blue
                
                // BUTTON in yellow (bottom line)
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "B", 5, 45, true).await;   // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "U", 20, 45, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "T", 35, 45, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "T", 50, 45, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "O", 65, 45, true).await;  // Yellow
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, "N", 80, 45, true).await;  // Yellow
            },
            
            GameState::AlphabetSelection => {
                // Display current letter prominently using single big letter (not 3x3 grid)
                let current_char = (b'A' + current_letter) as char;
                let letter_byte = current_char as u8;
                let letter_display = core::str::from_utf8(core::slice::from_ref(&letter_byte)).unwrap_or("?");
                display_colored_single_big_letter(&mut spi, &mut cs, &mut dc, letter_display, 56, 25, true).await; // Yellow, centered
                
                display_text(&mut spi, &mut cs, &mut dc, "TURN ENCODER", 22, 5).await;
                display_text(&mut spi, &mut cs, &mut dc, "PRESS TO", 30, 45).await;
                display_text(&mut spi, &mut cs, &mut dc, "SELECT", 35, 55).await;
            },
            
            GameState::ChooseExposeOrExplore => {
                if !scrolling_drawn {
                    // Display static text without horizontal scrolling to prevent blinking
                    display_text(&mut spi, &mut cs, &mut dc, "PRESS BLUE FOR", 15, 15).await;
                    display_text(&mut spi, &mut cs, &mut dc, "EXPOSE", 35, 25).await;
                    display_text(&mut spi, &mut cs, &mut dc, "PRESS RED FOR", 18, 40).await;
                    display_text(&mut spi, &mut cs, &mut dc, "EXPLORE", 32, 50).await;
                    scrolling_drawn = true; // Mark as drawn to prevent redrawing
                }
            },
            
            GameState::ShowExpose => {
                if !expose_drawn {
                    // Find questions starting with selected letter
                    let selected_char = (b'A' + selected_letter) as char;
                    let letter_questions = find_items_by_letter(&truths_data.0, selected_char);
                    
                    // Choose a question - use letter-specific if available, otherwise random
                    let selected_question = if !letter_questions.is_empty() {
                        letter_questions[(random_counter as usize) % letter_questions.len()].as_str()
                    } else if !truths_data.0.is_empty() {
                        truths_data.0[(random_counter as usize) % truths_data.0.len()].as_str()
                    } else {
                        // Ultimate fallback
                        "What's your biggest secret?"
                    };
                    
                    // Display challenge text completely on screen - NO SCROLLING
                    display_big_wrapped_text(&mut spi, &mut cs, &mut dc, selected_question, 5, 5).await;
                    
                    // Start restart text scrolling from right side
                    restart_scroll_offset = 128;
                    expose_drawn = true;
                }
                
                // Handle restart text scrolling (keep this scrolling nicely)
                clear_text_area(&mut spi, &mut cs, &mut dc, 45, 60).await;
                restart_scroll_offset -= 1;
                if restart_scroll_offset < -200 {
                    restart_scroll_offset = 128;
                }
                display_scrolling_text(&mut spi, &mut cs, &mut dc, "PRESS WHITE TO RESTART", restart_scroll_offset, 50).await;
            },
            
            GameState::ShowExplore => {
                if !explore_drawn {
                    // Find challenges starting with selected letter
                    let selected_char = (b'A' + selected_letter) as char;
                    let letter_challenges = find_items_by_letter(&dares_data.0, selected_char);
                    
                    // Choose a challenge - use letter-specific if available, otherwise random
                    let selected_challenge = if !letter_challenges.is_empty() {
                        letter_challenges[(random_counter as usize) % letter_challenges.len()].as_str()
                    } else if !dares_data.0.is_empty() {
                        dares_data.0[(random_counter as usize) % dares_data.0.len()].as_str()
                    } else {
                        // Ultimate fallback
                        "Dance for 1 minute!"
                    };
                    
                    // Display challenge text completely on screen - NO SCROLLING
                    display_big_wrapped_text(&mut spi, &mut cs, &mut dc, selected_challenge, 5, 5).await;
                    
                    // Start restart text scrolling from right side
                    restart_scroll_offset = 128;
                    explore_drawn = true;
                }
                
                // Handle restart text scrolling (keep this scrolling nicely)
                clear_text_area(&mut spi, &mut cs, &mut dc, 45, 60).await;
                restart_scroll_offset -= 1;
                if restart_scroll_offset < -200 {
                    restart_scroll_offset = 128;
                }
                display_scrolling_text(&mut spi, &mut cs, &mut dc, "PRESS WHITE TO RESTART", restart_scroll_offset, 50).await;
            },
            
            GameState::GameOver => {
                // Game over screen is handled above in the timer logic
                // Just display scrolling restart text at bottom
            },
        }
        
        // Turn off LED flashes after delay
        Timer::after_millis(20).await;
        dual_red.set_low();
        dual_green.set_low();
        led1.set_low();
        
        Timer::after_millis(80).await; // Total loop delay = 100ms
    }
}

async fn clear_screen(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>) {
    cs.set_low();
    dc.set_low();
    let _ = spi.write(&[0x21, 0x00, 0x7F]).await; // Column 0-127
    let _ = spi.write(&[0x22, 0x00, 0x07]).await; // Page 0-7
    cs.set_high();
    
    cs.set_low();
    dc.set_high();
    for _ in 0..1024 {
        let _ = spi.write(&[0x00]).await;
    }
    cs.set_high();
}

async fn display_text(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>, text: &str, x: u8, y: u8) {
    let mut col = x;
    
    for ch in text.chars() {
        if ch == ' ' {
            col += 8; // Moderate space width
            continue;
        }
        
        if ch.is_ascii_alphabetic() {
            let char_index = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
            if char_index < FONT.len() && col + 7 < 128 && y < 60 {
                // Set position - 1.5x scaling
                cs.set_low();
                dc.set_low();
                let _ = spi.write(&[0x21, col, col + 6]).await; // Slightly wider
                let _ = spi.write(&[0x22, y / 8, y / 8]).await; // Single page
                cs.set_high();
                
                // Send slightly enlarged character data
                cs.set_low();
                dc.set_high();
                for &byte in &FONT[char_index] {
                    let enlarged_byte = byte | (byte >> 1); // Slightly thicker
                    let _ = spi.write(&[enlarged_byte]).await;
                }
                cs.set_high();
                
                col += 8; // Moderate character spacing
            }
        }
    }
}

async fn display_single_big_letter(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>, letter: &str, x: u8, y: u8) {
    if let Some(ch) = letter.chars().next() {
        if ch.is_ascii_alphabetic() {
            let char_index = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
            if char_index < FONT.len() {
                // Display letter 2x larger (single copy, not 3x3 grid)
                cs.set_low();
                dc.set_low();
                let _ = spi.write(&[0x21, x, x + 10]).await; // Wider
                let _ = spi.write(&[0x22, y / 8, (y + 16) / 8]).await; // Taller
                cs.set_high();
                
                cs.set_low();
                dc.set_high();
                for &byte in &FONT[char_index] {
                    let enlarged_byte = byte | (byte >> 1) | (byte << 1); // Make thicker
                    let _ = spi.write(&[enlarged_byte, enlarged_byte]).await; // Write twice for width
                }
                cs.set_high();
            }
        }
    }
}

async fn display_colored_single_big_letter(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>, letter: &str, x: u8, y: u8, is_yellow: bool) {
    if let Some(ch) = letter.chars().next() {
        if ch.is_ascii_alphabetic() {
            let char_index = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
            if char_index < FONT.len() {
                // Set color contrast (simulate colors on monochrome OLED)
                cs.set_low();
                dc.set_low();
                if is_yellow {
                    let _ = spi.write(&[0x81, 0xFF]).await; // Full brightness for "yellow"
                } else {
                    let _ = spi.write(&[0x81, 0x8F]).await; // Medium brightness for "blue"
                }
                cs.set_high();
                
                // Display letter 2x larger
                cs.set_low();
                dc.set_low();
                let _ = spi.write(&[0x21, x, x + 12]).await; // Wider
                let _ = spi.write(&[0x22, y / 8, (y + 20) / 8]).await; // Taller
                cs.set_high();
                
                cs.set_low();
                dc.set_high();
                for &byte in &FONT[char_index] {
                    let enlarged_byte = byte | (byte >> 1) | (byte << 1); // Make thicker
                    let _ = spi.write(&[enlarged_byte, enlarged_byte]).await; // Write twice for width
                }
                cs.set_high();
            }
        }
    }
}

async fn display_emoji(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>, emoji_pattern: &[u8; 8], x: u8, y: u8) {
    // Set position for emoji
    cs.set_low();
    dc.set_low();
    let _ = spi.write(&[0x21, x, x + 7]).await; // 8 pixels wide
    let _ = spi.write(&[0x22, y / 8, y / 8]).await; // Single page
    cs.set_high();
    
    // Send emoji pattern
    cs.set_low();
    dc.set_high();
    for &byte in emoji_pattern {
        let _ = spi.write(&[byte]).await;
    }
    cs.set_high();
}

async fn display_big_letter(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>, letter: &str, x: u8, y: u8) {
    if let Some(ch) = letter.chars().next() {
        if ch.is_ascii_alphabetic() {
            let char_index = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
            if char_index < FONT.len() {
                // Display letter 3x larger
                for row in 0..3 {
                    for col in 0..3 {
                        cs.set_low();
                        dc.set_low();
                        let _ = spi.write(&[0x21, x + col * 6, x + col * 6 + 5]).await;
                        let _ = spi.write(&[0x22, (y + row * 8) / 8, (y + row * 8) / 8]).await;
                        cs.set_high();
                        
                        cs.set_low();
                        dc.set_high();
                        for &byte in &FONT[char_index] {
                            let enlarged_byte = byte | (byte >> 1) | (byte << 1); // Make thicker
                            let _ = spi.write(&[enlarged_byte]).await;
                        }
                        cs.set_high();
                    }
                }
            }
        }
    }
}

async fn display_text_at_position(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>, text: &str, x: i16, y: u8) {
    let mut col = x;
    
    for ch in text.chars() {
        if ch == ' ' {
            col += 8; // Moderate space width
            continue;
        }
        
        // Only draw characters that are visible on screen
        if col >= 0 && col < 120 {
            if ch.is_ascii_alphabetic() {
                let char_index = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
                if char_index < FONT.len() && (col + 7) < 128 && y < 60 {
                    // Set position
                    cs.set_low();
                    dc.set_low();
                    let _ = spi.write(&[0x21, col as u8, (col + 6) as u8]).await;
                    let _ = spi.write(&[0x22, y / 8, y / 8]).await;
                    cs.set_high();
                    
                    // Send slightly enlarged character data
                    cs.set_low();
                    dc.set_high();
                    for &byte in &FONT[char_index] {
                        let enlarged_byte = byte | (byte >> 1); // Slightly thicker
                        let _ = spi.write(&[enlarged_byte]).await;
                    }
                    cs.set_high();
                }
            }
        }
        
        col += 8; // Moderate character spacing
    }
}

async fn clear_text_area(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>, y_start: u8, y_end: u8) {
    cs.set_low();
    dc.set_low();
    let _ = spi.write(&[0x21, 0x00, 0x7F]).await; // Full width
    let _ = spi.write(&[0x22, y_start / 8, y_end / 8]).await; // Specific height range
    cs.set_high();
    
    cs.set_low();
    dc.set_high();
    let bytes_to_clear = 128 * ((y_end / 8) - (y_start / 8) + 1) as usize;
    for _ in 0..bytes_to_clear {
        let _ = spi.write(&[0x00]).await;
    }
    cs.set_high();
}

async fn display_big_wrapped_text(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>, text: &str, x: u8, y: u8) {
    let mut col = x;
    let mut row = y;
    const MAX_WIDTH: u8 = 120; // Use more screen width
    const LINE_HEIGHT: u8 = 10; // Smaller line spacing
    const CHAR_WIDTH: u8 = 6; // Smaller character width
    
    // Split text into words
    let words: heapless::Vec<&str, 32> = text.split(' ').collect();
    
    for word in words.iter() {
        // Check if current word fits on current line
        let word_width = word.len() as u8 * CHAR_WIDTH;
        
        // If word doesn't fit on current line, move to next line
        if col + word_width > MAX_WIDTH && col > x {
            col = x; // Start new line
            row += LINE_HEIGHT;
        }
        
        // Display each character in the word
        for ch in word.chars() {
            if ch.is_ascii_alphabetic() {
                let char_index = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
                if char_index < FONT.len() && row < 42 { // More space for text
                    // Set position for smaller text
                    cs.set_low();
                    dc.set_low();
                    let _ = spi.write(&[0x21, col, col + 4]).await; // Smaller chars
                    let _ = spi.write(&[0x22, row / 8, row / 8]).await; // Page
                    cs.set_high();
                    
                    // Send smaller character data
                    cs.set_low();
                    dc.set_high();
                    for &byte in &FONT[char_index] {
                        // Smaller, readable text
                        let _ = spi.write(&[byte]).await;
                    }
                    cs.set_high();
                    
                    col += CHAR_WIDTH; // Move to next character position
                }
            } else if ch.is_ascii_punctuation() || ch.is_ascii_digit() {
                // Handle punctuation and numbers
                if row < 42 {
                    cs.set_low();
                    dc.set_low();
                    let _ = spi.write(&[0x21, col, col + 3]).await; // Smaller punctuation
                    let _ = spi.write(&[0x22, row / 8, row / 8]).await;
                    cs.set_high();
                    
                    cs.set_low();
                    dc.set_high();
                    // Smaller punctuation patterns
                    let pattern = match ch {
                        '?' => [0x06, 0x09, 0x51, 0x09, 0x06], // ?
                        '!' => [0x00, 0x00, 0x5F, 0x00, 0x00], // !
                        '.' => [0x00, 0x60, 0x60, 0x00, 0x00], // .
                        ',' => [0x00, 0x80, 0x60, 0x00, 0x00], // ,
                        '\'' => [0x00, 0x05, 0x03, 0x00, 0x00], // '
                        '1' => [0x00, 0x42, 0x7F, 0x40, 0x00], // 1
                        '2' => [0x42, 0x61, 0x51, 0x49, 0x46], // 2
                        '3' => [0x21, 0x41, 0x45, 0x4B, 0x31], // 3
                        '4' => [0x18, 0x14, 0x12, 0x7F, 0x10], // 4
                        '5' => [0x27, 0x45, 0x45, 0x45, 0x39], // 5
                        '6' => [0x3C, 0x4A, 0x49, 0x49, 0x30], // 6
                        '7' => [0x01, 0x71, 0x09, 0x05, 0x03], // 7
                        '8' => [0x36, 0x49, 0x49, 0x49, 0x36], // 8
                        '9' => [0x06, 0x49, 0x49, 0x29, 0x1E], // 9
                        '0' => [0x3E, 0x51, 0x49, 0x45, 0x3E], // 0
                        _ => [0x1C, 0x1C, 0x1C, 0x1C, 0x1C], // Generic pattern
                    };
                    
                    for &byte in &pattern[0..4] { // Use fewer bytes for smaller size
                        let _ = spi.write(&[byte]).await;
                    }
                    cs.set_high();
                    
                    col += 5; // Smaller punctuation spacing
                }
            }
        }
        
        // Add smaller space after word
        col += 5; // Smaller space between words
    }
}

async fn display_scrolling_text(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>, text: &str, x: i16, y: u8) {
    let mut col = x;
    
    for ch in text.chars() {
        if ch == ' ' {
            col += 6; // Good spacing for readability
            continue;
        }
        
        // Only draw characters that are visible on screen
        if col >= -10 && col < 130 {
            if ch.is_ascii_alphabetic() {
                let char_index = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
                if char_index < FONT.len() && col >= 0 && col < 128 {
                    // Set position for good readable size
                    cs.set_low();
                    dc.set_low();
                    let _ = spi.write(&[0x21, col as u8, (col + 5) as u8]).await;
                    let _ = spi.write(&[0x22, y / 8, y / 8]).await;
                    cs.set_high();
                    
                    // Send character data with good thickness for readability
                    cs.set_low();
                    dc.set_high();
                    for &byte in &FONT[char_index] {
                        let readable_byte = byte | (byte >> 1); // Good thickness
                        let _ = spi.write(&[readable_byte]).await;
                    }
                    cs.set_high();
                }
            } else if (ch.is_ascii_punctuation() || ch.is_ascii_digit()) && col >= 0 && col < 128 {
                // Handle punctuation and numbers
                cs.set_low();
                dc.set_low();
                let _ = spi.write(&[0x21, col as u8, (col + 4) as u8]).await;
                let _ = spi.write(&[0x22, y / 8, y / 8]).await;
                cs.set_high();
                
                cs.set_low();
                dc.set_high();
                let pattern = match ch {
                    '?' => [0x06, 0x09, 0x51, 0x09, 0x06], // ?
                    '!' => [0x00, 0x00, 0x5F, 0x00, 0x00], // !
                    '.' => [0x00, 0x60, 0x60, 0x00, 0x00], // .
                    ',' => [0x00, 0x80, 0x60, 0x00, 0x00], // ,
                    '\'' => [0x00, 0x05, 0x03, 0x00, 0x00], // '
                    '1' => [0x00, 0x42, 0x7F, 0x40, 0x00], // 1
                    '2' => [0x42, 0x61, 0x51, 0x49, 0x46], // 2
                    '3' => [0x21, 0x41, 0x45, 0x4B, 0x31], // 3
                    '4' => [0x18, 0x14, 0x12, 0x7F, 0x10], // 4
                    '5' => [0x27, 0x45, 0x45, 0x45, 0x39], // 5
                    '6' => [0x3C, 0x4A, 0x49, 0x49, 0x30], // 6
                    '7' => [0x01, 0x71, 0x09, 0x05, 0x03], // 7
                    '8' => [0x36, 0x49, 0x49, 0x49, 0x36], // 8
                    '9' => [0x06, 0x49, 0x49, 0x29, 0x1E], // 9
                    '0' => [0x3E, 0x51, 0x49, 0x45, 0x3E], // 0
                    _ => [0x14, 0x14, 0x14, 0x14, 0x14], // Generic pattern
                };
                
                for &byte in &pattern[0..5] {
                    let _ = spi.write(&[byte]).await;
                }
                cs.set_high();
            }
        }
        
        col += 7; // Good character spacing for readability
    }
}