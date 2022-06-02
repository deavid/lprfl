use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum PressureUnit {
    // SI
    GigaPascal,
    MegaPascal,
    KiloPascal,
    HectoPascal,
    Pascal,
    MilliPascal,
    MilliBar,
    DeciBar,
    // NON SI Metric
    Bar,
    KiloBarye,
    Barye,
    Atmosphere,
    MiliMeterMercury,
    // US
    Torr,
    InchMercury,
    PoundPerSquareInch,
}

impl Default for PressureUnit {
    fn default() -> Self {
        Self::Pascal
    }
}

impl PressureUnit {
    pub fn pascals(&self) -> f64 {
        match self {
            PressureUnit::GigaPascal => 1.0E12,
            PressureUnit::MegaPascal => 1.0E6,
            PressureUnit::KiloPascal => 1000.0,
            PressureUnit::HectoPascal => 100.0,
            PressureUnit::Pascal => 1.0,
            PressureUnit::MilliPascal => 1.0E-3,
            PressureUnit::MilliBar => 100.0,
            PressureUnit::DeciBar => 100.0E2,
            PressureUnit::Bar => 100.0E3,
            PressureUnit::KiloBarye => 100.0,
            PressureUnit::Barye => 0.1,
            PressureUnit::Atmosphere => 100.0E3,
            PressureUnit::MiliMeterMercury => 130.0,
            PressureUnit::Torr => 130.0,
            PressureUnit::InchMercury => 3400.0,
            PressureUnit::PoundPerSquareInch => 6900.0,
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            PressureUnit::GigaPascal => vec!["GP"],
            PressureUnit::MegaPascal => vec!["MP"],
            PressureUnit::KiloPascal => vec!["kP"],
            PressureUnit::HectoPascal => vec!["hP"],
            PressureUnit::Pascal => vec!["P"],
            PressureUnit::MilliPascal => vec!["mP"],
            PressureUnit::MilliBar => vec!["mbar"],
            PressureUnit::DeciBar => vec!["dbar"],
            PressureUnit::Bar => vec!["bar"],
            PressureUnit::KiloBarye => vec!["kBa"],
            PressureUnit::Barye => vec!["Ba"],
            PressureUnit::Atmosphere => vec!["atm"],
            PressureUnit::MiliMeterMercury => vec!["mmHg"],
            PressureUnit::Torr => vec!["Torr"],
            PressureUnit::InchMercury => vec!["inHg"],
            PressureUnit::PoundPerSquareInch => vec!["psi"],
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

impl Display for PressureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
