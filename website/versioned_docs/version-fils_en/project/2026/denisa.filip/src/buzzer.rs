use embassy_rp::pwm::{Config as PwmConfig, Pwm};
use embassy_time::{Duration, Timer};
use crate::distance::Zone;

/// Frequency of the tone emitted by the passive buzzer (Hz).
const BEEP_FREQ_HZ: u32 = 2_700;

/// System clock used for PWM calculations (125 MHz on RP2040).
const SYS_CLK_HZ: u32 = 125_000_000;

/// Build a PWM config that produces the desired frequency at ~50 % duty cycle.
pub fn make_beep_config() -> PwmConfig {
    let wrap = (SYS_CLK_HZ / BEEP_FREQ_HZ).saturating_sub(1) as u16;
    let mut cfg = PwmConfig::default();
    cfg.top = wrap;
    cfg.compare_a = wrap / 2; // 50 % duty
    cfg
}

/// Build a PWM config with zero duty (silent).
pub fn make_silent_config() -> PwmConfig {
    let mut cfg = PwmConfig::default();
    cfg.top = 0xFFFF;
    cfg.compare_a = 0; // 0 % duty → no sound
    cfg
}

/// Drive a single buzzer slice according to the given zone.
///
/// * `Zone::Clear`   → completely silent
/// * `Zone::Warning` → slow intermittent beep (handled by the caller loop)
/// * `Zone::Danger`  → continuous tone
///
/// The function returns immediately; timing is handled in the main task loop
/// so that both buzzers stay synchronised.
pub fn set_buzzer_zone(pwm: &mut Pwm<'_>, zone: Zone, beeping_on: bool) {
    let cfg = match zone {
        Zone::Clear => make_silent_config(),
        Zone::Warning => {
            if beeping_on {
                make_beep_config()
            } else {
                make_silent_config()
            }
        }
        Zone::Danger => make_beep_config(),
    };
    pwm.set_config(&cfg);
}

/// Await half a beep period; used by the main task to toggle the buzzer.
pub async fn half_period(zone: Zone) {
    let period_ms = zone.beep_period_ms();
    if period_ms > 0 {
        Timer::after(Duration::from_millis(period_ms / 2)).await;
    } else {
        // When silent or continuous, still yield briefly to avoid starving
        // other tasks.
        Timer::after(Duration::from_millis(50)).await;
    }
}
