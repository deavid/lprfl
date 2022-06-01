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
    pub fn meters2(&self) -> f64 {
        match self {
            SpeedUnit::MeterPerSecond => todo!(),
            SpeedUnit::KilometerPerHour => todo!(),
            SpeedUnit::MilePerHour => todo!(),
            SpeedUnit::FootPerSecond => todo!(),
            SpeedUnit::Knot => todo!(),
            SpeedUnit::C => todo!(),
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            SpeedUnit::MeterPerSecond => todo!(),
            SpeedUnit::KilometerPerHour => todo!(),
            SpeedUnit::MilePerHour => todo!(),
            SpeedUnit::FootPerSecond => todo!(),
            SpeedUnit::Knot => todo!(),
            SpeedUnit::C => todo!(),
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
