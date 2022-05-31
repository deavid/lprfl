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
            (Unit::Length(s), Unit::Length(d)) => s.meters() / d.meters() * self.quantity,
            (Unit::Mass(s), Unit::Mass(d)) => s.kilograms() / d.kilograms() * self.quantity,
            (Unit::Temperature(s), Unit::Temperature(d)) => s.convert_to(self.quantity, d),
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
        points /= 100.0;
        points = points.log10();
        if let Some(s) = standard {
            points /= s.unit_score(self.unit)
        }
        1.0 / points.abs().max(0.00001)
    }
}

impl Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} {}", self.quantity, self.unit)
    }
}
