use crate::unit::Unit;
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
}

impl Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} {}", self.quantity, self.unit)
    }
}
