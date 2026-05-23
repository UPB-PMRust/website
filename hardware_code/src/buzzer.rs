use embassy_rp::pwm::{Config as PwmConfig, Pwm};
use embassy_time::{Duration, Timer};
use crate::distance::Zone;


const BEEP_FREQ_HZ: u32 = 2_700;


const SYS_CLK_HZ: u32 = 125_000_000;


pub fn make_beep_config() -> PwmConfig {
    let wrap = (SYS_CLK_HZ / BEEP_FREQ_HZ).saturating_sub(1) as u16;
    let mut cfg = PwmConfig::default();
    cfg.top = wrap;
    cfg.compare_a = wrap / 2; 
    cfg
}


pub fn make_silent_config() -> PwmConfig {
    let mut cfg = PwmConfig::default();
    cfg.top = 0xFFFF;
    cfg.compare_a = 0; // 0 % duty → no sound
    cfg
}


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

pub async fn half_period(zone: Zone) {
    let period_ms = zone.beep_period_ms();
    if period_ms > 0 {
        Timer::after(Duration::from_millis(period_ms / 2)).await;
    } else {
       
        Timer::after(Duration::from_millis(50)).await;
    }
}
