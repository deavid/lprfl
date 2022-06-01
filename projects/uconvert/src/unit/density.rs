use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum DensityUnit {
    // metric
    KiloGramPerCubicCentimeter,
    GramPerCubicCentimeter,
    // US
    PoundPerCubicFoot,
    PoundPerCubicYard,
}

impl Default for DensityUnit {
    fn default() -> Self {
        Self::KiloGramPerCubicCentimeter
    }
}

impl DensityUnit {
    pub fn meters2(&self) -> f64 {
        match self {
            DensityUnit::KiloGramPerCubicCentimeter => todo!(),
            DensityUnit::GramPerCubicCentimeter => todo!(),
            DensityUnit::PoundPerCubicFoot => todo!(),
            DensityUnit::PoundPerCubicYard => todo!(),
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            DensityUnit::KiloGramPerCubicCentimeter => todo!(),
            DensityUnit::GramPerCubicCentimeter => todo!(),
            DensityUnit::PoundPerCubicFoot => todo!(),
            DensityUnit::PoundPerCubicYard => todo!(),
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
