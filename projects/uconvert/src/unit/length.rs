use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[derive(Debug, Sequence, Clone, Copy)]
pub enum LengthUnit {
    KiloMeter,
    Meter,
    Mile,
    Foot,
    Inch,
}

impl Default for LengthUnit {
    fn default() -> Self {
        Self::Meter
    }
}

impl LengthUnit {
    pub fn meters(&self) -> f64 {
        match self {
            LengthUnit::KiloMeter => 1000.0,
            LengthUnit::Meter => 1.0,
            LengthUnit::Mile => 1609.344,
            LengthUnit::Foot => 0.3048,
            LengthUnit::Inch => 0.0254,
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            LengthUnit::KiloMeter => vec!["km", "kilometer", "kilometers"],
            LengthUnit::Meter => vec!["m", "meter", "meters", "mts"],
            LengthUnit::Mile => vec!["mi", "mile", "miles"],
            LengthUnit::Foot => vec!["ft", "foot", "feet"],
            LengthUnit::Inch => vec!["in", "inch", "inches"],
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

impl Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
