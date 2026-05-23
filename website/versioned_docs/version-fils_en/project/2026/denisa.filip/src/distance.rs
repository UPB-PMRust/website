/// Distance zone reported by each infrared obstacle sensor.
///
/// The digital infrared modules output a LOW signal when an obstacle
/// is detected within the adjustable range (3–100 cm).  Because the
/// module only gives a binary "detected / not-detected" signal we map
/// three zones by reading *both* sensors:
///
///   • CLEAR   – neither sensor triggered
///   • WARNING – one sensor triggered (object in peripheral range)
///   • DANGER  – both sensors triggered (object directly ahead)
///
/// A more sophisticated analog-distance approach would require an
/// ultrasonic (HC-SR04) or ToF (TMF8820) sensor with distance output.
/// This design matches the Digital IR Obstacle modules in the BOM.
#[derive(Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum Zone {
    Clear,
    Warning,
    Danger,
}

impl Zone {
    /// Derive zone from two boolean sensor readings.
    /// `triggered` means the digital output pin went LOW (object present).
    pub fn from_sensors(s1_triggered: bool, s2_triggered: bool) -> Self {
        match (s1_triggered, s2_triggered) {
            (false, false) => Zone::Clear,
            (true, false) | (false, true) => Zone::Warning,
            (true, true) => Zone::Danger,
        }
    }

    /// Human-readable distance label for the LCD.
    pub fn label(self) -> &'static str {
        match self {
            Zone::Clear   => "CLEAR   >100cm ",
            Zone::Warning => "WARNING  ~50cm ",
            Zone::Danger  => "DANGER!  <20cm ",
        }
    }

    /// Buzzer beep period in milliseconds (0 = continuous tone).
    pub fn beep_period_ms(self) -> u64 {
        match self {
            Zone::Clear   => 0,    // silent
            Zone::Warning => 600,  // slow beep
            Zone::Danger  => 150,  // rapid beep
        }
    }
}
