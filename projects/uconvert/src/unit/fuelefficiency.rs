use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum FuelEfficiencyUnit {
    KiloMetersPerLiter,
    LitersPer100KiloMeters,
    LitersPerKiloMeter,
    MilesPerImperialGallon,
    MilesPerUSGallon,
    ImperialGallonsPerMile,
    USGallonsPerMile,
}

impl Default for FuelEfficiencyUnit {
    fn default() -> Self {
        Self::LitersPerKiloMeter
    }
}

impl FuelEfficiencyUnit {
    pub fn convert_to(&self, value: f64, dest: &Self) -> f64 {
        todo!()
    }
    pub fn litersperkm(&self, value: f64) -> f64 {
        match self {
            FuelEfficiencyUnit::KiloMetersPerLiter => todo!(),
            FuelEfficiencyUnit::LitersPer100KiloMeters => todo!(),
            FuelEfficiencyUnit::LitersPerKiloMeter => todo!(),
            FuelEfficiencyUnit::MilesPerImperialGallon => todo!(),
            FuelEfficiencyUnit::MilesPerUSGallon => todo!(),
            FuelEfficiencyUnit::ImperialGallonsPerMile => todo!(),
            FuelEfficiencyUnit::USGallonsPerMile => todo!(),
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            FuelEfficiencyUnit::KiloMetersPerLiter => vec!["km/L"],
            FuelEfficiencyUnit::LitersPer100KiloMeters => vec!["L/100km"],
            FuelEfficiencyUnit::LitersPerKiloMeter => vec!["L/km"],
            FuelEfficiencyUnit::MilesPerImperialGallon => vec!["mpgimp"],
            FuelEfficiencyUnit::MilesPerUSGallon => vec!["mpgus"],
            FuelEfficiencyUnit::ImperialGallonsPerMile => vec!["impgal/mi"],
            FuelEfficiencyUnit::USGallonsPerMile => vec!["usgal/mi"],
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

impl Display for FuelEfficiencyUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
