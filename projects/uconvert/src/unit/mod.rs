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
pub enum UnitStandard {
    Iso,
    US,
    EU,
}
impl UnitStandard {
    pub fn names(&self) -> Vec<&str> {
        match self {
            UnitStandard::Iso => vec!["iso", "standard", "metric"],
            UnitStandard::US => vec!["us", "usa", "united states"],
            UnitStandard::EU => vec!["eu", "europe", "euro"],
        }
    }
    pub fn units(&self) -> Vec<Unit> {
        let mut units: Vec<Unit> = self.length_units().into_iter().map(Unit::Length).collect();
        units.extend(
            self.mass_units()
                .into_iter()
                .map(Unit::Mass)
                .collect::<Vec<Unit>>(),
        );
        units.extend(
            self.temperature_units()
                .into_iter()
                .map(Unit::Temperature)
                .collect::<Vec<Unit>>(),
        );
        units
    }
    pub fn length_units(&self) -> Vec<LengthUnit> {
        match self {
            UnitStandard::Iso => vec![
                LengthUnit::KiloMeter,
                LengthUnit::Meter,
                LengthUnit::MiliMeter,
            ],
            UnitStandard::US => vec![LengthUnit::Mile, LengthUnit::Foot, LengthUnit::Inch],
            UnitStandard::EU => vec![
                LengthUnit::KiloMeter,
                LengthUnit::Meter,
                LengthUnit::MiliMeter,
            ],
        }
    }
    pub fn mass_units(&self) -> Vec<MassUnit> {
        match self {
            UnitStandard::Iso => vec![MassUnit::Gram, MassUnit::KiloGram, MassUnit::MetricTon],
            UnitStandard::US => vec![MassUnit::Stone, MassUnit::Pound, MassUnit::Ounce],
            UnitStandard::EU => vec![MassUnit::Gram, MassUnit::KiloGram, MassUnit::MetricTon],
        }
    }
    pub fn temperature_units(&self) -> Vec<TemperatureUnit> {
        match self {
            UnitStandard::Iso => vec![TemperatureUnit::Celsius, TemperatureUnit::Kelvin],
            UnitStandard::US => vec![TemperatureUnit::Fahrenheit, TemperatureUnit::Rankine],
            UnitStandard::EU => vec![TemperatureUnit::Celsius, TemperatureUnit::Kelvin],
        }
    }
    pub fn unit_score(&self, unit: Unit) -> f64 {
        match self {
            UnitStandard::Iso => match unit {
                Unit::Length(LengthUnit::Meter) => 2.0,
                Unit::Mass(MassUnit::KiloGram) => 2.0,
                Unit::Temperature(TemperatureUnit::Kelvin) => 2.0,
                _ => 1.0,
            },
            UnitStandard::US => match unit {
                Unit::Temperature(TemperatureUnit::Fahrenheit) => 2.0,
                _ => 1.0,
            },
            UnitStandard::EU => match unit {
                Unit::Temperature(TemperatureUnit::Celsius) => 2.0,
                _ => 1.0,
            },
        }
    }
}

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
    pub fn _default_for(dest_unit: &Self) -> Self {
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
