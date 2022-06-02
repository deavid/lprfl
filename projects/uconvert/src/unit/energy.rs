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
    pub fn joules(&self) -> f64 {
        match self {
            EnergyUnit::GigaJoule => 1000000000.0,
            EnergyUnit::MegaJoule => 1000000.0,
            EnergyUnit::KiloJoule => 1000.0,
            EnergyUnit::HectoJoule => 100.0,
            EnergyUnit::DecaJoule => 10.0,
            EnergyUnit::Joule => 1.0,
            EnergyUnit::DeciJoule => 0.1,
            EnergyUnit::CentiJoule => 0.01,
            EnergyUnit::MilliJoule => 0.001,
            EnergyUnit::MicroJoule => 0.000001,
            EnergyUnit::NanoJoule => 0.000000001,
            EnergyUnit::MegaErg => 0.1,
            EnergyUnit::KiloErg => 0.0001,
            EnergyUnit::Erg => 0.0000001,
            EnergyUnit::TeraWattHour => 3600000000000000.0,
            EnergyUnit::GigaWattHour => 3600000000000.0,
            EnergyUnit::MegaWattHour => 3600000000.0,
            EnergyUnit::KiloWattHour => 3600000.0,
            EnergyUnit::WattHour => 3600.0,
            EnergyUnit::GigaElectronVolt => 1.602176634E-11,
            EnergyUnit::MegaElectronVolt => 1.602176634E-13,
            EnergyUnit::KiloElectronVolt => 1.602176634E-16,
            EnergyUnit::ElectronVolt => 1.602176634E-19,
            EnergyUnit::MilliElectronVolt => 1.602176634E-21,
            EnergyUnit::MilliCalore => 0.0041868,
            EnergyUnit::Calorie => 4.1868,
            EnergyUnit::KiloCalorie => 4186.8,
            EnergyUnit::MegaCalorie => 4186800.0,
            EnergyUnit::FootPoundal => 0.04214011009,
            EnergyUnit::FootPoundForce => 1.355817948,
            EnergyUnit::InchPoundForce => 0.1129848290,
            EnergyUnit::InchOunceForce => 0.0071,
            EnergyUnit::HorsepowerHour => 2684519.954,
            EnergyUnit::BritishTermalUnit => 1100.0,
            EnergyUnit::GigaTonTNT => 4.2E18,
            EnergyUnit::MegaTonTNT => 4.2E15,
            EnergyUnit::KiloTonTNT => 4.2E12,
            EnergyUnit::TonTNT => 4.2E9,
            EnergyUnit::Hartree => 27.0 * EnergyUnit::ElectronVolt.joules(),
            EnergyUnit::Rydberg => 14.0 * EnergyUnit::ElectronVolt.joules(),
            EnergyUnit::TonneOilEquivalent => 42.0E9,
            EnergyUnit::BarrelOilEquivalent => 6.1E9,
            EnergyUnit::CubicFootNaturalGas => 1.1E6,
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            EnergyUnit::GigaJoule => vec!["GJ", "gigajoule"],
            EnergyUnit::MegaJoule => vec!["MJ", "megajoule"],
            EnergyUnit::KiloJoule => vec!["kJ", "kilojoule"],
            EnergyUnit::HectoJoule => vec!["hJ", "hectojoule"],
            EnergyUnit::DecaJoule => vec!["daJ", "decajoule"],
            EnergyUnit::Joule => vec!["J", "joule"],
            EnergyUnit::DeciJoule => vec!["dJ", "decijoule"],
            EnergyUnit::CentiJoule => vec!["cJ", "centijoule"],
            EnergyUnit::MilliJoule => vec!["mJ", "millijoule"],
            EnergyUnit::MicroJoule => vec!["uJ", "microjoule"],
            EnergyUnit::NanoJoule => vec!["nJ", "nanojoule"],
            EnergyUnit::MegaErg => vec!["Merg", "megaerg"],
            EnergyUnit::KiloErg => vec!["kerg", "kiloerg"],
            EnergyUnit::Erg => vec!["erg", "erg"],
            EnergyUnit::TeraWattHour => vec!["Twh", "terawatthour"],
            EnergyUnit::GigaWattHour => vec!["Gwh", "gigawatthour"],
            EnergyUnit::MegaWattHour => vec!["Mwh", "megawatthour"],
            EnergyUnit::KiloWattHour => vec!["kwh", "kilowatthour"],
            EnergyUnit::WattHour => vec!["wh", "watthour"],
            EnergyUnit::GigaElectronVolt => vec!["GeV", "gigaelectronvolt"],
            EnergyUnit::MegaElectronVolt => vec!["MeV", "megaelectronvolt"],
            EnergyUnit::KiloElectronVolt => vec!["keV", "kiloelectronvolt"],
            EnergyUnit::ElectronVolt => vec!["eV", "electronvolt"],
            EnergyUnit::MilliElectronVolt => vec!["meV", "millielectronvolt"],
            EnergyUnit::Calorie => vec!["cal", "calorie"],
            EnergyUnit::MegaCalorie => vec!["Mcal", "megacalorie"],
            EnergyUnit::KiloCalorie => vec!["kcal", "kilocalorie"],
            EnergyUnit::MilliCalore => vec!["mcal", "millicalorie"],
            EnergyUnit::FootPoundal => vec!["ftpdl", "footpoundal"],
            EnergyUnit::FootPoundForce => vec!["ftlbf", "footpoundforce"],
            EnergyUnit::InchPoundForce => vec!["inlbf", "inchpoundforce"],
            EnergyUnit::InchOunceForce => vec!["inozf", "inchouceforce"],
            EnergyUnit::HorsepowerHour => vec!["hph", "horsepowerhour"],
            EnergyUnit::BritishTermalUnit => vec!["btu", "britishtermalunit"],
            EnergyUnit::GigaTonTNT => vec!["GtTNT", "gigatontnt"],
            EnergyUnit::MegaTonTNT => vec!["MtTNT", "megatontnt"],
            EnergyUnit::KiloTonTNT => vec!["ktTNT", "kilotontnt"],
            EnergyUnit::TonTNT => vec!["tTNT", "tontnt"],
            EnergyUnit::Hartree => vec!["Eh", "hartree"],
            EnergyUnit::Rydberg => vec!["Ry", "rydberg"],
            EnergyUnit::TonneOilEquivalent => vec!["toe", "toneofoilequvialent"],
            EnergyUnit::BarrelOilEquivalent => vec!["boe", "barrelofoilequivalent"],
            EnergyUnit::CubicFootNaturalGas => vec!["ft³naturalgas"],
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
