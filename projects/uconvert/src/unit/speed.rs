use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum SpeedUnit {
    // metric
    MeterPerSecond,
    // NON-SI
    KilometerPerHour,
    // US
    MilePerHour,
    FootPerSecond,
    // Maritime
    Knot,
    // Other
    C, // lightspeed
}

impl Default for SpeedUnit {
    fn default() -> Self {
        Self::MeterPerSecond
    }
}

impl SpeedUnit {
    pub fn meterspersecond(&self) -> f64 {
        match self {
            SpeedUnit::MeterPerSecond => 1.0,
            SpeedUnit::KilometerPerHour => 0.27777777,
            SpeedUnit::MilePerHour => 0.44704,
            SpeedUnit::FootPerSecond => 0.3048,
            SpeedUnit::Knot => 0.51444444,
            SpeedUnit::C => 299792458.0,
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            SpeedUnit::MeterPerSecond => vec!["m/s"],
            SpeedUnit::KilometerPerHour => vec!["km/h", "kph"],
            SpeedUnit::MilePerHour => vec!["mi/h", "mph"],
            SpeedUnit::FootPerSecond => vec!["ft/s"],
            SpeedUnit::Knot => vec!["kn", "knot", "knots"],
            SpeedUnit::C => vec!["c", "lightspeed"],
        }
    }
    pub fn _from_text(t: &str) -> Result<Self> {
        let t = t.to_lowercase();
        let t = t.trim();
        for unit in enum_iterator::all::<Self>() {
            let names = unit.names();
            for name in names {
                if name == t {
                    return Ok(unit);
                }
            }
        }
        Err(AppError::UnitNotFound(t.into()).into())
    }
}

impl Display for SpeedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
