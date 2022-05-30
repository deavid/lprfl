pub mod length;
pub mod mass;
pub mod temperature;
use std::fmt::Display;

use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;
use length::LengthUnit;
use mass::MassUnit;

use self::temperature::TemperatureUnit;

#[derive(Debug, Sequence, Clone, Copy)]
pub enum Unit {
    Length(LengthUnit),
    Mass(MassUnit),
    Temperature(TemperatureUnit),
}

impl Unit {
    pub fn from_text(t: &str) -> Result<Self> {
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
    pub fn names(&self) -> Vec<&str> {
        match self {
            Unit::Length(x) => x.names(),
            Unit::Mass(x) => x.names(),
            Unit::Temperature(x) => x.names(),
        }
    }
    pub fn default_for(dest_unit: &Self) -> Self {
        match dest_unit {
            Unit::Length(_) => Unit::Length(LengthUnit::default()),
            Unit::Mass(_) => Unit::Mass(MassUnit::default()),
            Unit::Temperature(_) => Unit::Temperature(TemperatureUnit::default()),
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Length(x) => x.fmt(f),
            Unit::Mass(x) => x.fmt(f),
            Unit::Temperature(x) => x.fmt(f),
        }
    }
}
