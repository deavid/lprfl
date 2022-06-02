use crate::unit::{Unit, UnitStandard};
use crate::AppError;
use anyhow::Result;
use std::fmt::Display;

#[derive(Debug)]
pub struct Measure {
    pub quantity: f64,
    pub unit: Unit,
}

impl Measure {
    pub fn to_unit(&self, dest_unit: Unit) -> Result<Self> {
        let quantity = match (&self.unit, &dest_unit) {
            (Unit::Area(s), Unit::Area(d)) => s.meters2() / d.meters2() * self.quantity,
            (Unit::Length(s), Unit::Length(d)) => s.meters() / d.meters() * self.quantity,
            (Unit::Mass(s), Unit::Mass(d)) => s.kilograms() / d.kilograms() * self.quantity,
            (Unit::Temperature(s), Unit::Temperature(d)) => s.convert_to(self.quantity, d),
            (Unit::Density(s), Unit::Density(d)) => s.kgm3() / d.kgm3() * self.quantity,
            (Unit::Energy(s), Unit::Energy(d)) => s.joules() / d.joules() * self.quantity,
            (Unit::Force(s), Unit::Force(d)) => s.newtons() / d.newtons() * self.quantity,
            (Unit::FuelEfficiency(s), Unit::FuelEfficiency(d)) => s.convert_to(self.quantity, d),
            (Unit::Pressure(s), Unit::Pressure(d)) => s.pascals() / d.pascals() * self.quantity,
            (Unit::Speed(s), Unit::Speed(d)) => {
                s.meterspersecond() / d.meterspersecond() * self.quantity
            }
            (Unit::Torque(s), Unit::Torque(d)) => {
                s.newtonsmeter() / d.newtonsmeter() * self.quantity
            }
            (Unit::Volume(s), Unit::Volume(d)) => s.meters3() / d.meters3() * self.quantity,
            (f, t) => Err(AppError::IncompatibleUnits(f.to_string(), t.to_string()))?,
        };
        Ok(Self {
            quantity,
            unit: dest_unit,
        })
    }

    pub fn _to_iso_unit(&self) -> Option<Measure> {
        let units = UnitStandard::Iso.units();
        self.to_best_unit(units, Some(UnitStandard::Iso))
    }

    pub fn to_best_unit(
        &self,
        units: Vec<Unit>,
        standard: Option<UnitStandard>,
    ) -> Option<Measure> {
        let mut measures: Vec<(f64, Self)> = units
            .into_iter()
            .filter_map(|u| self.to_unit(u).ok())
            .map(|m| (m.score(standard), m))
            .collect();
        measures.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        measures.pop().map(|x| x.1)
    }

    pub fn score(&self, standard: Option<UnitStandard>) -> f64 {
        let mut points = self.quantity.abs();
        points /= 20.0;
        points = points.log10().abs();
        let base = match standard {
            Some(s) => s.unit_score(self.unit),
            None => 1.0,
        };
        base / points.max(0.00001)
    }
}

impl Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} {}", self.quantity, self.unit)
    }
}
