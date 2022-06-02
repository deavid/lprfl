use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum TorqueUnit {
    // SI
    NewtonMeter,
    // non SI metric
    KiloGramMeter,
    // US
    PoundForceFoot,
}

impl Default for TorqueUnit {
    fn default() -> Self {
        Self::NewtonMeter
    }
}

impl TorqueUnit {
    pub fn newtonsmeter(&self) -> f64 {
        match self {
            TorqueUnit::NewtonMeter => 1.0,
            TorqueUnit::KiloGramMeter => 9.8,
            TorqueUnit::PoundForceFoot => 1.4,
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            TorqueUnit::NewtonMeter => vec!["Nm", "newtonmeter"],
            TorqueUnit::KiloGramMeter => vec!["kgm", "kilogrammeter"],
            TorqueUnit::PoundForceFoot => vec!["lbft", "poundforcefoot"],
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

impl Display for TorqueUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
