use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum VolumeUnit {
    // SI
    CubicMeter,
    CubitCentiMeter,
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
    pub fn meters2(&self) -> f64 {
        match self {
            VolumeUnit::CubicMeter => todo!(),
            VolumeUnit::CubitCentiMeter => todo!(),
            VolumeUnit::CubicMiliMeter => todo!(),
            VolumeUnit::KiloLiter => todo!(),
            VolumeUnit::Liter => todo!(),
            VolumeUnit::CentiLiter => todo!(),
            VolumeUnit::MiliLiter => todo!(),
            VolumeUnit::CubicYard => todo!(),
            VolumeUnit::CubicFoot => todo!(),
            VolumeUnit::CubicInch => todo!(),
            VolumeUnit::ImperialBarrel => todo!(),
            VolumeUnit::ImperialBushel => todo!(),
            VolumeUnit::ImperialGallon => todo!(),
            VolumeUnit::ImperialQuart => todo!(),
            VolumeUnit::ImperialPint => todo!(),
            VolumeUnit::ImperialFluidOunce => todo!(),
            VolumeUnit::USBarrel => todo!(),
            VolumeUnit::Barrel => todo!(),
            VolumeUnit::USBeerBarrel => todo!(),
            VolumeUnit::USGallon => todo!(),
            VolumeUnit::USQuart => todo!(),
            VolumeUnit::USPint => todo!(),
            VolumeUnit::USFluidOunce => todo!(),
            VolumeUnit::USDryBarrel => todo!(),
            VolumeUnit::USBushel => todo!(),
            VolumeUnit::USDryGallon => todo!(),
            VolumeUnit::USDryQuart => todo!(),
            VolumeUnit::USDryPint => todo!(),
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            VolumeUnit::CubicMeter => todo!(),
            VolumeUnit::CubitCentiMeter => todo!(),
            VolumeUnit::CubicMiliMeter => todo!(),
            VolumeUnit::KiloLiter => todo!(),
            VolumeUnit::Liter => todo!(),
            VolumeUnit::CentiLiter => todo!(),
            VolumeUnit::MiliLiter => todo!(),
            VolumeUnit::CubicYard => todo!(),
            VolumeUnit::CubicFoot => todo!(),
            VolumeUnit::CubicInch => todo!(),
            VolumeUnit::ImperialBarrel => todo!(),
            VolumeUnit::ImperialBushel => todo!(),
            VolumeUnit::ImperialGallon => todo!(),
            VolumeUnit::ImperialQuart => todo!(),
            VolumeUnit::ImperialPint => todo!(),
            VolumeUnit::ImperialFluidOunce => todo!(),
            VolumeUnit::USBarrel => todo!(),
            VolumeUnit::Barrel => todo!(),
            VolumeUnit::USBeerBarrel => todo!(),
            VolumeUnit::USGallon => todo!(),
            VolumeUnit::USQuart => todo!(),
            VolumeUnit::USPint => todo!(),
            VolumeUnit::USFluidOunce => todo!(),
            VolumeUnit::USDryBarrel => todo!(),
            VolumeUnit::USBushel => todo!(),
            VolumeUnit::USDryGallon => todo!(),
            VolumeUnit::USDryQuart => todo!(),
            VolumeUnit::USDryPint => todo!(),
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
