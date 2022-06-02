pub mod area;
pub mod density;
pub mod energy;
pub mod force;
pub mod fuelefficiency;
pub mod length;
pub mod mass;
pub mod pressure;
pub mod speed;
pub mod temperature;
pub mod torque;
pub mod volume;

use crate::AppError;
use anyhow::Result;
use area::AreaUnit;
use density::DensityUnit;
use energy::EnergyUnit;
use enum_iterator::Sequence;
use force::ForceUnit;
use fuelefficiency::FuelEfficiencyUnit;
use length::LengthUnit;
use mass::MassUnit;
use pressure::PressureUnit;
use std::fmt::Display;
use temperature::TemperatureUnit;
use torque::TorqueUnit;
use volume::VolumeUnit;

use self::speed::SpeedUnit;

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
            self.area_units()
                .into_iter()
                .map(Unit::Area)
                .collect::<Vec<Unit>>(),
        );
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
        units.extend(
            self.density_units()
                .into_iter()
                .map(Unit::Density)
                .collect::<Vec<Unit>>(),
        );
        units.extend(
            self.energy_units()
                .into_iter()
                .map(Unit::Energy)
                .collect::<Vec<Unit>>(),
        );
        units.extend(
            self.force_units()
                .into_iter()
                .map(Unit::Force)
                .collect::<Vec<Unit>>(),
        );
        units.extend(
            self.fuel_units()
                .into_iter()
                .map(Unit::FuelEfficiency)
                .collect::<Vec<Unit>>(),
        );
        units.extend(
            self.pressure_units()
                .into_iter()
                .map(Unit::Pressure)
                .collect::<Vec<Unit>>(),
        );
        units.extend(
            self.speed_units()
                .into_iter()
                .map(Unit::Speed)
                .collect::<Vec<Unit>>(),
        );
        units.extend(
            self.torque_units()
                .into_iter()
                .map(Unit::Torque)
                .collect::<Vec<Unit>>(),
        );
        units.extend(
            self.volume_units()
                .into_iter()
                .map(Unit::Volume)
                .collect::<Vec<Unit>>(),
        );
        units
    }
    pub fn area_units(&self) -> Vec<AreaUnit> {
        match self {
            UnitStandard::Iso => vec![
                AreaUnit::SquareKiloMeter,
                AreaUnit::SquareMeter,
                AreaUnit::SquareMiliMeter,
            ],
            UnitStandard::US => vec![
                AreaUnit::SquareMile,
                AreaUnit::SquareFoot,
                AreaUnit::SquareInch,
            ],
            UnitStandard::EU => vec![
                AreaUnit::SquareKiloMeter,
                AreaUnit::SquareMeter,
                AreaUnit::SquareCentiMeter,
                AreaUnit::SquareMiliMeter,
            ],
        }
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
                LengthUnit::CentiMeter,
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
    pub fn density_units(&self) -> Vec<DensityUnit> {
        match self {
            UnitStandard::Iso => vec![],
            UnitStandard::US => vec![],
            UnitStandard::EU => vec![],
        }
    }
    pub fn energy_units(&self) -> Vec<EnergyUnit> {
        match self {
            UnitStandard::Iso => vec![],
            UnitStandard::US => vec![],
            UnitStandard::EU => vec![],
        }
    }
    pub fn force_units(&self) -> Vec<ForceUnit> {
        match self {
            UnitStandard::Iso => vec![],
            UnitStandard::US => vec![],
            UnitStandard::EU => vec![],
        }
    }
    pub fn fuel_units(&self) -> Vec<FuelEfficiencyUnit> {
        match self {
            UnitStandard::Iso => vec![],
            UnitStandard::US => vec![],
            UnitStandard::EU => vec![],
        }
    }
    pub fn pressure_units(&self) -> Vec<PressureUnit> {
        match self {
            UnitStandard::Iso => vec![],
            UnitStandard::US => vec![],
            UnitStandard::EU => vec![],
        }
    }
    pub fn speed_units(&self) -> Vec<SpeedUnit> {
        match self {
            UnitStandard::Iso => vec![],
            UnitStandard::US => vec![],
            UnitStandard::EU => vec![],
        }
    }
    pub fn torque_units(&self) -> Vec<TorqueUnit> {
        match self {
            UnitStandard::Iso => vec![],
            UnitStandard::US => vec![],
            UnitStandard::EU => vec![],
        }
    }
    pub fn volume_units(&self) -> Vec<VolumeUnit> {
        match self {
            UnitStandard::Iso => vec![],
            UnitStandard::US => vec![],
            UnitStandard::EU => vec![],
        }
    }
    pub fn unit_score(&self, unit: Unit) -> f64 {
        match self {
            UnitStandard::Iso => match unit {
                Unit::Area(AreaUnit::SquareMeter) => 2.0,
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
                Unit::Area(AreaUnit::SquareCentiMeter) => 1.5,
                Unit::Length(LengthUnit::CentiMeter) => 5.0,
                Unit::Temperature(TemperatureUnit::Celsius) => 2.0,
                _ => 1.0,
            },
        }
    }
}

#[derive(Debug, Sequence, Clone, Copy)]
pub enum Unit {
    Area(AreaUnit),
    Length(LengthUnit),
    Mass(MassUnit),
    Temperature(TemperatureUnit),
    Density(DensityUnit),
    Energy(EnergyUnit),
    Force(ForceUnit),
    FuelEfficiency(FuelEfficiencyUnit),
    Pressure(PressureUnit),
    Speed(SpeedUnit),
    Torque(TorqueUnit),
    Volume(VolumeUnit),
}

impl Unit {
    pub fn from_text(t: &str) -> Result<Self> {
        let t = t.trim();
        for unit in enum_iterator::all::<Self>() {
            let names = unit.names();
            for name in names {
                if name == t {
                    return Ok(unit);
                }
            }
        }
        let t = t.to_lowercase();
        for unit in enum_iterator::all::<Self>() {
            let names = unit.names();
            for name in names {
                if name == t {
                    return Ok(unit);
                }
            }
        }
        Err(AppError::UnitNotFound(t).into())
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            Unit::Area(x) => x.names(),
            Unit::Length(x) => x.names(),
            Unit::Mass(x) => x.names(),
            Unit::Temperature(x) => x.names(),
            Unit::Density(x) => x.names(),
            Unit::Energy(x) => x.names(),
            Unit::Force(x) => x.names(),
            Unit::FuelEfficiency(x) => x.names(),
            Unit::Pressure(x) => x.names(),
            Unit::Speed(x) => x.names(),
            Unit::Torque(x) => x.names(),
            Unit::Volume(x) => x.names(),
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Area(x) => x.fmt(f),
            Unit::Length(x) => x.fmt(f),
            Unit::Mass(x) => x.fmt(f),
            Unit::Temperature(x) => x.fmt(f),
            Unit::Density(x) => x.fmt(f),
            Unit::Energy(x) => x.fmt(f),
            Unit::Force(x) => x.fmt(f),
            Unit::FuelEfficiency(x) => x.fmt(f),
            Unit::Pressure(x) => x.fmt(f),
            Unit::Speed(x) => x.fmt(f),
            Unit::Torque(x) => x.fmt(f),
            Unit::Volume(x) => x.fmt(f),
        }
    }
}
