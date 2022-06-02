use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[derive(Debug, Sequence, Clone, Copy)]
pub enum LengthUnit {
    // Metric
    NanoMeter,
    MicroMeter,
    MiliMeter,
    CentiMeter,
    KiloMeter,
    Meter,
    MegaMeter,
    // Non SI metric
    Angstrom,
    // US
    Mile,
    Foot,
    Inch,
    Furlong,
    Chain,
    Rod,
    Fathom,
    Yard,
    // Other
    NauticalMile,
    Parsec,
    LightYear,
    AstronomicalUnit,
}

impl Default for LengthUnit {
    fn default() -> Self {
        Self::Meter
    }
}

impl LengthUnit {
    pub fn meters(&self) -> f64 {
        match self {
            LengthUnit::NanoMeter => 0.000000001,
            LengthUnit::MicroMeter => 0.000001,
            LengthUnit::MiliMeter => 0.001,
            LengthUnit::CentiMeter => 0.01,
            LengthUnit::KiloMeter => 1000.0,
            LengthUnit::MegaMeter => 1000000.0,
            LengthUnit::Meter => 1.0,
            LengthUnit::Mile => 1609.344,
            LengthUnit::Foot => 0.3048,
            LengthUnit::Inch => 0.0254,
            LengthUnit::Angstrom => 1E-10,
            LengthUnit::Furlong => 201.168,
            LengthUnit::Chain => 20.1168,
            LengthUnit::Rod => 5.02921,
            LengthUnit::Fathom => 1.8288,
            LengthUnit::Yard => 0.9144,
            LengthUnit::NauticalMile => 1852.0,
            LengthUnit::Parsec => 3.085677581E16,
            LengthUnit::LightYear => 9.460730473E15,
            LengthUnit::AstronomicalUnit => 149597870700.0,
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            LengthUnit::KiloMeter => vec!["km", "kilometer", "kilometers"],
            LengthUnit::Meter => vec!["m", "meter", "meters", "mts"],
            LengthUnit::MiliMeter => vec!["mm", "milimeter", "milimeters"],
            LengthUnit::CentiMeter => vec!["cm", "centimeter"],
            LengthUnit::NanoMeter => vec!["nm", "nanometer"],
            LengthUnit::MicroMeter => vec!["um", "micrometer"],
            LengthUnit::MegaMeter => vec!["Mm", "megameter"],
            LengthUnit::Mile => vec!["mi", "mile", "miles"],
            LengthUnit::Foot => vec!["ft", "foot", "feet"],
            LengthUnit::Inch => vec!["in", "inch", "inches"],
            LengthUnit::Angstrom => vec!["Å", "angstrom"],
            LengthUnit::Furlong => vec!["furlong"],
            LengthUnit::Chain => vec!["chain"],
            LengthUnit::Rod => vec!["rd", "rod"],
            LengthUnit::Fathom => vec!["fathom"],
            LengthUnit::Yard => vec!["yd", "yard", "yards"],
            LengthUnit::NauticalMile => vec!["nmi", "nauticalmile"],
            LengthUnit::Parsec => vec!["pc", "parsec"],
            LengthUnit::LightYear => vec!["ly", "lightyear"],
            LengthUnit::AstronomicalUnit => vec!["AU", "astronomicalunit"],
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
