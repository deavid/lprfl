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
    pub fn newtons(&self) -> f64 {
        match self {
            ForceUnit::GigaNewton => 1.0E9,
            ForceUnit::MegaNewton => 1.0E6,
            ForceUnit::KiloNewton => 1.0E3,
            ForceUnit::Newton => 1.0E0,
            ForceUnit::MilliNewton => 1.0E-3,
            ForceUnit::MicroNewton => 1.0E-6,
            ForceUnit::NanoNewton => 1.0E-9,
            ForceUnit::MegaDyne => 10.0E0,
            ForceUnit::KiloDyne => 10.0E-3,
            ForceUnit::Dyne => 10.0E-6,
            ForceUnit::MilliDyne => 10.0E-9,
            ForceUnit::TonneForce => 9.8E3,
            ForceUnit::KiloGramForce => 9.8,
            ForceUnit::GramForce => 9.8E-3,
            ForceUnit::MilliGramForce => 9.8E-6,
            ForceUnit::Poundal => 0.14,
            ForceUnit::LongTonForce => 10.0E3,
            ForceUnit::ShortTonForce => 8.9E3,
            ForceUnit::PoundForce => 4.4,
            ForceUnit::GrainForce => 640E-6,
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            ForceUnit::GigaNewton => vec!["GN", "giganewton"],
            ForceUnit::MegaNewton => vec!["MN", "meganewton"],
            ForceUnit::KiloNewton => vec!["kN", "kilonewton"],
            ForceUnit::Newton => vec!["N", "newton"],
            ForceUnit::MilliNewton => vec!["mN", "millinewton"],
            ForceUnit::MicroNewton => vec!["uN", "micronewton"],
            ForceUnit::NanoNewton => vec!["nN", "nanonewton"],
            ForceUnit::MegaDyne => vec!["Mdyn", "megadyne"],
            ForceUnit::KiloDyne => vec!["kdyn", "kilodyne"],
            ForceUnit::Dyne => vec!["dyn", "dyne"],
            ForceUnit::MilliDyne => vec!["mdyn", "millidyne"],
            ForceUnit::TonneForce => vec!["tf", "tonneforce"],
            ForceUnit::KiloGramForce => vec!["kgf", "kilogramforce"],
            ForceUnit::GramForce => vec!["gf", "gramforce"],
            ForceUnit::MilliGramForce => vec!["mgf", "milligramforce"],
            ForceUnit::Poundal => vec!["pdl", "poundal"],
            ForceUnit::LongTonForce => vec!["LT", "longton"],
            ForceUnit::ShortTonForce => vec!["ST", "shortton"],
            ForceUnit::PoundForce => vec!["lbf", "poundforce"],
            ForceUnit::GrainForce => vec!["grf", "grainforce"],
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
