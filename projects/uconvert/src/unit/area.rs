use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Sequence, Clone, Copy)]
pub enum AreaUnit {
    // metric
    SquareMiliMeter,
    SquareCentiMeter,
    SquareKiloMeter,
    SquareMeter,
    // Non si metric
    Hectare,
    // US
    SquareMile,
    Acre,
    SquareYard,
    SquareFoot,
    SquareInch,
    // Other
    SquareNauticalMile,
    Dunam,
    Tsubo,
}

impl Default for AreaUnit {
    fn default() -> Self {
        Self::SquareMeter
    }
}

impl AreaUnit {
    pub fn meters2(&self) -> f64 {
        match self {
            AreaUnit::SquareMiliMeter => 0.001_f64.powi(2),
            AreaUnit::SquareCentiMeter => 0.01_f64.powi(2),
            AreaUnit::SquareKiloMeter => 1000.0_f64.powi(2),
            AreaUnit::SquareMeter => 1.0_f64.powi(2),
            AreaUnit::SquareMile => 1609.344_f64.powi(2),
            AreaUnit::SquareFoot => 0.3048_f64.powi(2),
            AreaUnit::SquareInch => 0.0254_f64.powi(2),
            AreaUnit::Hectare => todo!(),
            AreaUnit::Acre => todo!(),
            AreaUnit::SquareYard => todo!(),
            AreaUnit::SquareNauticalMile => todo!(),
            AreaUnit::Dunam => todo!(),
            AreaUnit::Tsubo => todo!(),
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            AreaUnit::SquareKiloMeter => {
                vec!["km²", "km^2", "square kilometer", "square kilometers"]
            }
            AreaUnit::SquareMeter => vec!["m²", "m^2", "square meter", "square meters", "sq. mts"],
            AreaUnit::SquareCentiMeter => {
                vec!["cm²", "cm^2", "square centimeter", "square centimeters"]
            }
            AreaUnit::SquareMiliMeter => {
                vec!["mm²", "mm^2", "square milimeter", "square milimeters"]
            }
            AreaUnit::SquareMile => vec!["mi²", "mi^2", "square mile", "square miles"],
            AreaUnit::SquareFoot => vec!["ft²", "ft^2", "square foot", "square feet"],
            AreaUnit::SquareInch => vec!["in²", "in^2", "square inch", "square inches"],
            AreaUnit::Hectare => todo!(),
            AreaUnit::Acre => todo!(),
            AreaUnit::SquareYard => todo!(),
            AreaUnit::SquareNauticalMile => todo!(),
            AreaUnit::Dunam => todo!(),
            AreaUnit::Tsubo => todo!(),
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

impl Display for AreaUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
