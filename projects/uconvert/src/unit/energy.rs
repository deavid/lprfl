use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum EnergyUnit {
    // metric
    GigaJoule,
    MegaJoule,
    KiloJoule,
    HectoJoule,
    DecaJoule,
    Joule,
    DeciJoule,
    CentiJoule,
    MilliJoule,
    MicroJoule,
    NanoJoule,
    // cm-gram-sec units
    MegaErg,
    KiloErg,
    Erg,
    // Watt.hour
    TeraWattHour,
    GigaWattHour,
    MegaWattHour,
    KiloWattHour,
    WattHour,
    // ElectronVolt
    GigaElectronVolt,
    MegaElectronVolt,
    KiloElectronVolt,
    ElectronVolt,
    MilliElectronVolt,
    // Calorie
    Calorie,
    MegaCalorie,
    KiloCalorie,
    MilliCalore,
    // Imperial
    FootPoundal,
    FootPoundForce,
    InchPoundForce,
    InchOunceForce,
    HorsepowerHour,
    // BTU - British termal unit
    BritishTermalUnit,
    // TNT units
    GigaTonTNT,
    MegaTonTNT,
    KiloTonTNT,
    TonTNT,
    // Other
    Hartree,
    Rydberg,
    TonneOilEquivalent,
    BarrelOilEquivalent,
    CubicFootNaturalGas,
}

impl Default for EnergyUnit {
    fn default() -> Self {
        Self::Joule
    }
}

impl EnergyUnit {
    pub fn meters2(&self) -> f64 {
        match self {
            EnergyUnit::GigaJoule => todo!(),
            EnergyUnit::MegaJoule => todo!(),
            EnergyUnit::KiloJoule => todo!(),
            EnergyUnit::HectoJoule => todo!(),
            EnergyUnit::DecaJoule => todo!(),
            EnergyUnit::Joule => todo!(),
            EnergyUnit::DeciJoule => todo!(),
            EnergyUnit::CentiJoule => todo!(),
            EnergyUnit::MilliJoule => todo!(),
            EnergyUnit::MicroJoule => todo!(),
            EnergyUnit::NanoJoule => todo!(),
            EnergyUnit::MegaErg => todo!(),
            EnergyUnit::KiloErg => todo!(),
            EnergyUnit::Erg => todo!(),
            EnergyUnit::TeraWattHour => todo!(),
            EnergyUnit::GigaWattHour => todo!(),
            EnergyUnit::MegaWattHour => todo!(),
            EnergyUnit::KiloWattHour => todo!(),
            EnergyUnit::WattHour => todo!(),
            EnergyUnit::GigaElectronVolt => todo!(),
            EnergyUnit::MegaElectronVolt => todo!(),
            EnergyUnit::KiloElectronVolt => todo!(),
            EnergyUnit::ElectronVolt => todo!(),
            EnergyUnit::MilliElectronVolt => todo!(),
            EnergyUnit::Calorie => todo!(),
            EnergyUnit::MegaCalorie => todo!(),
            EnergyUnit::KiloCalorie => todo!(),
            EnergyUnit::MilliCalore => todo!(),
            EnergyUnit::FootPoundal => todo!(),
            EnergyUnit::FootPoundForce => todo!(),
            EnergyUnit::InchPoundForce => todo!(),
            EnergyUnit::InchOunceForce => todo!(),
            EnergyUnit::HorsepowerHour => todo!(),
            EnergyUnit::BritishTermalUnit => todo!(),
            EnergyUnit::GigaTonTNT => todo!(),
            EnergyUnit::MegaTonTNT => todo!(),
            EnergyUnit::KiloTonTNT => todo!(),
            EnergyUnit::TonTNT => todo!(),
            EnergyUnit::Hartree => todo!(),
            EnergyUnit::Rydberg => todo!(),
            EnergyUnit::TonneOilEquivalent => todo!(),
            EnergyUnit::BarrelOilEquivalent => todo!(),
            EnergyUnit::CubicFootNaturalGas => todo!(),
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            EnergyUnit::GigaJoule => todo!(),
            EnergyUnit::MegaJoule => todo!(),
            EnergyUnit::KiloJoule => todo!(),
            EnergyUnit::HectoJoule => todo!(),
            EnergyUnit::DecaJoule => todo!(),
            EnergyUnit::Joule => todo!(),
            EnergyUnit::DeciJoule => todo!(),
            EnergyUnit::CentiJoule => todo!(),
            EnergyUnit::MilliJoule => todo!(),
            EnergyUnit::MicroJoule => todo!(),
            EnergyUnit::NanoJoule => todo!(),
            EnergyUnit::MegaErg => todo!(),
            EnergyUnit::KiloErg => todo!(),
            EnergyUnit::Erg => todo!(),
            EnergyUnit::TeraWattHour => todo!(),
            EnergyUnit::GigaWattHour => todo!(),
            EnergyUnit::MegaWattHour => todo!(),
            EnergyUnit::KiloWattHour => todo!(),
            EnergyUnit::WattHour => todo!(),
            EnergyUnit::GigaElectronVolt => todo!(),
            EnergyUnit::MegaElectronVolt => todo!(),
            EnergyUnit::KiloElectronVolt => todo!(),
            EnergyUnit::ElectronVolt => todo!(),
            EnergyUnit::MilliElectronVolt => todo!(),
            EnergyUnit::Calorie => todo!(),
            EnergyUnit::MegaCalorie => todo!(),
            EnergyUnit::KiloCalorie => todo!(),
            EnergyUnit::MilliCalore => todo!(),
            EnergyUnit::FootPoundal => todo!(),
            EnergyUnit::FootPoundForce => todo!(),
            EnergyUnit::InchPoundForce => todo!(),
            EnergyUnit::InchOunceForce => todo!(),
            EnergyUnit::HorsepowerHour => todo!(),
            EnergyUnit::BritishTermalUnit => todo!(),
            EnergyUnit::GigaTonTNT => todo!(),
            EnergyUnit::MegaTonTNT => todo!(),
            EnergyUnit::KiloTonTNT => todo!(),
            EnergyUnit::TonTNT => todo!(),
            EnergyUnit::Hartree => todo!(),
            EnergyUnit::Rydberg => todo!(),
            EnergyUnit::TonneOilEquivalent => todo!(),
            EnergyUnit::BarrelOilEquivalent => todo!(),
            EnergyUnit::CubicFootNaturalGas => todo!(),
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

impl Display for EnergyUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
