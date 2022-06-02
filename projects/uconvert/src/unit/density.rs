use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum DensityUnit {
    // metric
    KiloGramPerCubicMeter,
    GramPerCubicMeter,
    // US
    PoundPerCubicFoot,
    PoundPerCubicYard,
}

impl Default for DensityUnit {
    fn default() -> Self {
        Self::KiloGramPerCubicMeter
    }
}

impl DensityUnit {
    pub fn kgm3(&self) -> f64 {
        match self {
            DensityUnit::KiloGramPerCubicMeter => 1.0,
            DensityUnit::GramPerCubicMeter => 0.001,
            DensityUnit::PoundPerCubicFoot => 16.01846337,
            DensityUnit::PoundPerCubicYard => 0.5932764213,
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            DensityUnit::KiloGramPerCubicMeter => vec![
                "kg/m^3",
                "kg/m³",
                "kilogram per cubic meter",
                "kilograms per cubic meter",
            ],
            DensityUnit::GramPerCubicMeter => vec![
                "g/m^3",
                "g/m³",
                "gram per cubic meter",
                "grams per cubic meter",
            ],
            DensityUnit::PoundPerCubicFoot => vec![
                "lb/ft^3",
                "lb/ft³",
                "pound per cubic foot",
                "pounds per cubic foot",
            ],
            DensityUnit::PoundPerCubicYard => vec![
                "lb/yd^3",
                "lb/yd³",
                "pound per cubic yard",
                "pounds per cubic yard",
            ],
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

impl Display for DensityUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
