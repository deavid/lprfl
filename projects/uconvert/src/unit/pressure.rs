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
    pub fn meters2(&self) -> f64 {
        match self {
            PressureUnit::GigaPascal => todo!(),
            PressureUnit::MegaPascal => todo!(),
            PressureUnit::KiloPascal => todo!(),
            PressureUnit::HectoPascal => todo!(),
            PressureUnit::Pascal => todo!(),
            PressureUnit::MilliPascal => todo!(),
            PressureUnit::MilliBar => todo!(),
            PressureUnit::DeciBar => todo!(),
            PressureUnit::Bar => todo!(),
            PressureUnit::KiloBarye => todo!(),
            PressureUnit::Barye => todo!(),
            PressureUnit::Atmosphere => todo!(),
            PressureUnit::MiliMeterMercury => todo!(),
            PressureUnit::Torr => todo!(),
            PressureUnit::InchMercury => todo!(),
            PressureUnit::PoundPerSquareInch => todo!(),
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            PressureUnit::GigaPascal => todo!(),
            PressureUnit::MegaPascal => todo!(),
            PressureUnit::KiloPascal => todo!(),
            PressureUnit::HectoPascal => todo!(),
            PressureUnit::Pascal => todo!(),
            PressureUnit::MilliPascal => todo!(),
            PressureUnit::MilliBar => todo!(),
            PressureUnit::DeciBar => todo!(),
            PressureUnit::Bar => todo!(),
            PressureUnit::KiloBarye => todo!(),
            PressureUnit::Barye => todo!(),
            PressureUnit::Atmosphere => todo!(),
            PressureUnit::MiliMeterMercury => todo!(),
            PressureUnit::Torr => todo!(),
            PressureUnit::InchMercury => todo!(),
            PressureUnit::PoundPerSquareInch => todo!(),
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
