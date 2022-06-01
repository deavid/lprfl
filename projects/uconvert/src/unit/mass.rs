use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[derive(Debug, Sequence, Clone, Copy)]
pub enum MassUnit {
    // metric
    Gram,
    KiloGram,
    MilliGram,
    MicroGram,
    // non-si metric
    MetricTon,
    // Avoirdupois
    LongTon,
    ShortTon,
    Stone,
    Pound,
    Ounce,
    Drachm,
    Grain,
    // Other
    TroyOunce,
    Carat,
}

impl Default for MassUnit {
    fn default() -> Self {
        Self::KiloGram
    }
}

impl MassUnit {
    pub fn kilograms(&self) -> f64 {
        match self {
            MassUnit::Gram => 0.001,
            MassUnit::KiloGram => 1.0,
            MassUnit::MetricTon => 1000.0,
            MassUnit::Stone => 6.35029318,
            MassUnit::Pound => 0.45359237,
            MassUnit::Ounce => 0.02834952313,
            MassUnit::MilliGram => todo!(),
            MassUnit::MicroGram => todo!(),
            MassUnit::LongTon => todo!(),
            MassUnit::ShortTon => todo!(),
            MassUnit::Drachm => todo!(),
            MassUnit::Grain => todo!(),
            MassUnit::TroyOunce => todo!(),
            MassUnit::Carat => todo!(),
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            MassUnit::Gram => vec!["g", "gram", "grams"],
            MassUnit::KiloGram => vec!["kg", "kilogram", "kilograms"],
            MassUnit::MetricTon => vec!["mt", "metric ton", "metric tonnes"],
            MassUnit::Stone => vec!["st", "stone", "stones"],
            MassUnit::Pound => vec!["lb", "pound", "pounds"],
            MassUnit::Ounce => vec!["oz", "ounce", "ounces"],
            MassUnit::MilliGram => todo!(),
            MassUnit::MicroGram => todo!(),
            MassUnit::LongTon => todo!(),
            MassUnit::ShortTon => todo!(),
            MassUnit::Drachm => todo!(),
            MassUnit::Grain => todo!(),
            MassUnit::TroyOunce => todo!(),
            MassUnit::Carat => todo!(),
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

impl Display for MassUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
