
#[derive(Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum Zone {
    Clear,
    Warning,
    Danger,
}

impl Zone {
   
    pub fn from_sensors(s1_triggered: bool, s2_triggered: bool) -> Self {
        match (s1_triggered, s2_triggered) {
            (false, false) => Zone::Clear,
            (true, false) | (false, true) => Zone::Warning,
            (true, true) => Zone::Danger,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Zone::Clear   => "CLEAR   >100cm ",
            Zone::Warning => "WARNING  ~50cm ",
            Zone::Danger  => "DANGER!  <20cm ",
        }
    }

    pub fn beep_period_ms(self) -> u64 {
        match self {
            Zone::Clear   => 0,    // silent
            Zone::Warning => 600,  // slow beep
            Zone::Danger  => 150,  // rapid beep
        }
    }
}
