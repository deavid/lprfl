use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum ForceUnit {
    // Metric - SI
    GigaNewton,
    MegaNewton,
    KiloNewton,
    Newton,
    MilliNewton,
    MicroNewton,
    NanoNewton,
    // Cm-G-sec
    MegaDyne,
    KiloDyne,
    Dyne,
    MilliDyne,
    // Metric gravitational units
    TonneForce,
    KiloGramForce,
    GramForce,
    MilliGramForce,
    // Avoirdupois
    Poundal,
    LongTonForce,
    ShortTonForce,
    PoundForce,
    GrainForce,
}

impl Default for ForceUnit {
    fn default() -> Self {
        Self::Newton
    }
}

impl ForceUnit {
    pub fn meters2(&self) -> f64 {
        match self {
            ForceUnit::GigaNewton => todo!(),
            ForceUnit::MegaNewton => todo!(),
            ForceUnit::KiloNewton => todo!(),
            ForceUnit::Newton => todo!(),
            ForceUnit::MilliNewton => todo!(),
            ForceUnit::MicroNewton => todo!(),
            ForceUnit::NanoNewton => todo!(),
            ForceUnit::MegaDyne => todo!(),
            ForceUnit::KiloDyne => todo!(),
            ForceUnit::Dyne => todo!(),
            ForceUnit::MilliDyne => todo!(),
            ForceUnit::TonneForce => todo!(),
            ForceUnit::KiloGramForce => todo!(),
            ForceUnit::GramForce => todo!(),
            ForceUnit::MilliGramForce => todo!(),
            ForceUnit::Poundal => todo!(),
            ForceUnit::LongTonForce => todo!(),
            ForceUnit::ShortTonForce => todo!(),
            ForceUnit::PoundForce => todo!(),
            ForceUnit::GrainForce => todo!(),
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            ForceUnit::GigaNewton => todo!(),
            ForceUnit::MegaNewton => todo!(),
            ForceUnit::KiloNewton => todo!(),
            ForceUnit::Newton => todo!(),
            ForceUnit::MilliNewton => todo!(),
            ForceUnit::MicroNewton => todo!(),
            ForceUnit::NanoNewton => todo!(),
            ForceUnit::MegaDyne => todo!(),
            ForceUnit::KiloDyne => todo!(),
            ForceUnit::Dyne => todo!(),
            ForceUnit::MilliDyne => todo!(),
            ForceUnit::TonneForce => todo!(),
            ForceUnit::KiloGramForce => todo!(),
            ForceUnit::GramForce => todo!(),
            ForceUnit::MilliGramForce => todo!(),
            ForceUnit::Poundal => todo!(),
            ForceUnit::LongTonForce => todo!(),
            ForceUnit::ShortTonForce => todo!(),
            ForceUnit::PoundForce => todo!(),
            ForceUnit::GrainForce => todo!(),
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

impl Display for ForceUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
