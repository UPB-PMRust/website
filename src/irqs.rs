// COMPLETE SOLUTION - Replace these sections in your main code

// 1. ADD THIS VARIABLE at the top of your main function (after let mut random_counter = 0u32;)
let mut game_over_led_state = false; // Track LED state for game over

// 2. REPLACE the entire timer logic section with this:
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
        // IMMEDIATE ACTIONS WHEN TIME ENDS
        game_state = GameState::GameOver;
        timer_started = false;
        green_led.set_low(); // Turn off green LED IMMEDIATELY
        
        // FORCE RED LED ON IMMEDIATELY - NO CONDITIONS
        red_led.set_high();
        game_over_led_state = true;
        
        game_over_drawn = false; // Need to draw game over screen
        info!("⏰ TIME'S UP! GAME OVER! - RED LED FORCED ON");
    }
}

// 3. REPLACE the game over handling section with this:
// LED BLINKING LOGIC - RUNS EVERY LOOP REGARDLESS OF DRAWING STATE
if game_state == GameState::GameOver {
    // IMMEDIATE LED BLINKING - This runs EVERY single loop iteration
    if (current_time / 300) % 2 == 0 { // 300ms blink cycle for visibility
        red_led.set_high();
        game_over_led_state = true;
    } else {
        red_led.set_low();
        game_over_led_state = false;
    }
    
    // Debug info to confirm LED code is running
    if random_counter % 500 == 0 { // More frequent debug messages
        info!("🔴 BLINKING! LED State: {} at time: {}", game_over_led_state, current_time);
    }
} else {
    // Make sure red LED is off when not in game over
    red_led.set_low();
    game_over_led_state = false;
}

// SEPARATE GAME OVER SCREEN DRAWING - This only runs once
if game_state == GameState::GameOver && !game_over_drawn {
    display_smaller_game_over(&mut spi, &mut cs, &mut dc).await;
    
    // Quick buzzer - but LED is already blinking above
    buzzer.set_high();
    Timer::after_millis(100).await;
    buzzer.set_low();
    
    game_over_drawn = true;
    info!("💀 Game Over screen displayed - LED should already be blinking!");
}

// 4. ADD these new functions before your main function:

// Smaller GAME OVER display function
async fn display_smaller_game_over(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>) {
    clear_screen(spi, cs, dc).await;
    
    // Display "GAME" on first line - compact size
    display_compact_letter(spi, cs, dc, "G", 25, 5, false).await; // Blue
    display_compact_letter(spi, cs, dc, "A", 35, 5, false).await; // Blue
    display_compact_letter(spi, cs, dc, "M", 45, 5, false).await; // Blue
    display_compact_letter(spi, cs, dc, "E", 55, 5, false).await; // Blue
    
    // Display "OVER" on second line - compact size
    display_compact_letter(spi, cs, dc, "O", 25, 20, true).await;  // Yellow
    display_compact_letter(spi, cs, dc, "V", 35, 20, true).await;  // Yellow
    display_compact_letter(spi, cs, dc, "E", 45, 20, true).await;  // Yellow
    display_compact_letter(spi, cs, dc, "R", 55, 20, true).await;  // Yellow
}

// Compact letter display function
async fn display_compact_letter(spi: &mut Spi<'_, embassy_rp::peripherals::SPI0, embassy_rp::spi::Async>, cs: &mut Output<'_>, dc: &mut Output<'_>, letter: &str, x: u8, y: u8, is_yellow: bool) {
    if let Some(ch) = letter.chars().next() {
        if ch.is_ascii_alphabetic() {
            let char_index = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
            if char_index < FONT.len() {
                // Set color contrast
                cs.set_low();
                dc.set_low();
                if is_yellow {
                    let _ = spi.write(&[0x81, 0xFF]).await; // Full brightness for "yellow"
                } else {
                    let _ = spi.write(&[0x81, 0x9F]).await; // Medium brightness for "blue"
                }
                cs.set_high();
                
                // Display compact letter
                cs.set_low();
                dc.set_low();
                let _ = spi.write(&[0x21, x, x + 7]).await; // Compact width
                let _ = spi.write(&[0x22, y / 8, (y + 10) / 8]).await; // Compact height
                cs.set_high();
                
                cs.set_low();
                dc.set_high();
                for &byte in &FONT[char_index] {
                    let compact_byte = byte | (byte >> 1); // Good thickness
                    let _ = spi.write(&[compact_byte]).await;
                }
                cs.set_high();
            }
        }
    }
}

// 5. REPLACE the GameState::GameOver match case with this:
GameState::GameOver => {
    // Use the exact same scrolling as challenges - perfectly clear
    clear_text_area(&mut spi, &mut cs, &mut dc, 35, 64).await; // More space due to compact GAME OVER
    restart_scroll_offset -= 1; // Same speed as challenge text
    if restart_scroll_offset < -200 {
        restart_scroll_offset = 128;
    }
    // Use the SAME function as ShowExpose/ShowExplore for identical clarity
    display_scrolling_text(&mut spi, &mut cs, &mut dc, "PRESS WHITE TO RESTART", restart_scroll_offset, 40).await;
},