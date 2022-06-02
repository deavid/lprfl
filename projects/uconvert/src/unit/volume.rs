use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum VolumeUnit {
    // SI
    CubicMeter,
    CubicCentiMeter,
    CubicMiliMeter,
    // non SI metric
    KiloLiter,
    Liter,
    CentiLiter,
    MiliLiter,
    // US & Imperial
    CubicYard,
    CubicFoot,
    CubicInch,
    // Imperial
    ImperialBarrel,
    ImperialBushel,
    ImperialGallon,
    ImperialQuart,
    ImperialPint,
    ImperialFluidOunce,
    // US liquid
    USBarrel,
    Barrel,
    USBeerBarrel,
    USGallon,
    USQuart,
    USPint,
    USFluidOunce,
    // US dry
    USDryBarrel,
    USBushel,
    USDryGallon,
    USDryQuart,
    USDryPint,
}

impl Default for VolumeUnit {
    fn default() -> Self {
        Self::CubicMeter
    }
}

impl VolumeUnit {
    pub fn meters3(&self) -> f64 {
        const LITER: f64 = 0.001;
        match self {
            VolumeUnit::CubicMeter => 1.0,
            VolumeUnit::CubicCentiMeter => 1.0E-6,
            VolumeUnit::CubicMiliMeter => 1.0E-9,
            VolumeUnit::KiloLiter => 1.0,
            VolumeUnit::Liter => LITER,
            VolumeUnit::CentiLiter => 0.00001,
            VolumeUnit::MiliLiter => 0.000001,
            VolumeUnit::CubicYard => 0.76,
            VolumeUnit::CubicFoot => 0.028,
            VolumeUnit::CubicInch => 16.0E-6,
            VolumeUnit::ImperialBarrel => 160.0 * LITER,
            VolumeUnit::ImperialBushel => 36.0 * LITER,
            VolumeUnit::ImperialGallon => 4.5 * LITER,
            VolumeUnit::ImperialQuart => 1.1 * LITER,
            VolumeUnit::ImperialPint => 0.57 * LITER,
            VolumeUnit::ImperialFluidOunce => 0.028 * LITER,
            VolumeUnit::USBarrel => 120.0 * LITER,
            VolumeUnit::Barrel => 0.16,
            VolumeUnit::USBeerBarrel => 120.0 * LITER,
            VolumeUnit::USGallon => 3.8 * LITER,
            VolumeUnit::USQuart => 0.95 * LITER,
            VolumeUnit::USPint => 0.47 * LITER,
            VolumeUnit::USFluidOunce => 0.030 * LITER,
            VolumeUnit::USDryBarrel => 0.12,
            VolumeUnit::USBushel => 35.0 * LITER,
            VolumeUnit::USDryGallon => 4.4 * LITER,
            VolumeUnit::USDryQuart => 1.1 * LITER,
            VolumeUnit::USDryPint => 0.55 * LITER,
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            VolumeUnit::CubicMeter => vec!["m^3", "m³"],
            VolumeUnit::CubicCentiMeter => vec!["cm^3", "cm³"],
            VolumeUnit::CubicMiliMeter => vec!["mm^3", "mm³"],
            VolumeUnit::KiloLiter => vec!["kL"],
            VolumeUnit::Liter => vec!["L", "liter", "liters"],
            VolumeUnit::CentiLiter => vec!["cl", "centiliter"],
            VolumeUnit::MiliLiter => vec!["ml", "milliliter"],
            VolumeUnit::CubicYard => vec!["yd³", "yd^3"],
            VolumeUnit::CubicFoot => vec!["ft^3", "ft³"],
            VolumeUnit::CubicInch => vec!["in^3", "in³"],
            VolumeUnit::ImperialBarrel => vec!["impbb"],
            VolumeUnit::ImperialBushel => vec!["impbsh"],
            VolumeUnit::ImperialGallon => vec!["impgal"],
            VolumeUnit::ImperialQuart => vec!["impqt"],
            VolumeUnit::ImperialPint => vec!["imppt"],
            VolumeUnit::ImperialFluidOunce => vec!["impfloz"],
            VolumeUnit::USBarrel => vec!["USbbl"],
            VolumeUnit::Barrel => vec!["oilbbl"],
            VolumeUnit::USBeerBarrel => vec!["USbeerbbl"],
            VolumeUnit::USGallon => vec!["USgal"],
            VolumeUnit::USQuart => vec!["USquart"],
            VolumeUnit::USPint => vec!["USpt"],
            VolumeUnit::USFluidOunce => vec!["USfloz"],
            VolumeUnit::USDryBarrel => vec!["USdrybbl"],
            VolumeUnit::USBushel => vec!["USbsh"],
            VolumeUnit::USDryGallon => vec!["USdrygal"],
            VolumeUnit::USDryQuart => vec!["USdryqt"],
            VolumeUnit::USDryPint => vec!["USdrypt"],
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

impl Display for VolumeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
