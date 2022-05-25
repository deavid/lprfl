// Java: trait -> interfaces
// trait Unit {
//     fn in_baseunits(&self) -> f64;
// }

// trait UnitValue {
//     fn value(&self) -> f64;
//     fn unit(&self) -> dyn Unit;
//     fn to_baseunits(&self) -> f64 {
//         self.value() * self.unit().in_baseunits()
//     }
// }

use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LengthUnit {
    Kilometer,
    Meter,
    Millimeter,
    Inch,
    Angstrom,
    Mile,
    Furlong,
    Chain,
    Rod,
    Fathom,
    Yard,
    Foot,
    Parsec,
    LightYear,
    AstronomicalUnit,
}

// impl Unit for LengthUnit {
//     fn in_baseunits(&self) -> f64 {
//         self.meters()
//     }
// }

impl LengthUnit {
    pub fn unit(&self) -> &'static str {
        match self {
            LengthUnit::Kilometer => "km",
            LengthUnit::Meter => "m",
            LengthUnit::Millimeter => "mm",
            LengthUnit::Inch => "in",
            LengthUnit::Angstrom => "a",
            LengthUnit::Mile => "mi",
            LengthUnit::LightYear => "ly",
            LengthUnit::AstronomicalUnit => "AU",
            _ => self.unit_name(),
        }
    }
    pub fn unit_name(&self) -> &'static str {
        match self {
            LengthUnit::Kilometer => "kilometer",
            LengthUnit::Meter => "meter",
            LengthUnit::Millimeter => "milimeter",
            LengthUnit::Inch => "inch",
            LengthUnit::Angstrom => "angstrom",
            LengthUnit::Mile => "mile",
            LengthUnit::Furlong => "furlong",
            LengthUnit::Chain => "chain",
            LengthUnit::Rod => "rod",
            LengthUnit::Fathom => "fathom",
            LengthUnit::Yard => "yard",
            LengthUnit::Foot => "foot",
            LengthUnit::Parsec => "parsec",
            LengthUnit::LightYear => "lightyear",
            LengthUnit::AstronomicalUnit => "astronomicalunit",
        }
    }
    pub fn unit_name_plural(&self) -> &'static str {
        match self {
            LengthUnit::Kilometer => "kilometers",
            LengthUnit::Meter => "meters",
            LengthUnit::Millimeter => "milimeters",
            LengthUnit::Inch => "inches",
            LengthUnit::Angstrom => "angstroms",
            LengthUnit::Mile => "miles",
            LengthUnit::Furlong => "furlongs",
            LengthUnit::Chain => "chains",
            LengthUnit::Rod => "rods",
            LengthUnit::Fathom => "fathoms",
            LengthUnit::Yard => "yards",
            LengthUnit::Foot => "foots",
            LengthUnit::Parsec => "parsecs",
            LengthUnit::LightYear => "lightyears",
            LengthUnit::AstronomicalUnit => "astronomicalunits",
        }
    }
    pub fn meters(&self) -> f64 {
        match self {
            LengthUnit::Kilometer => 1000.0,
            LengthUnit::Meter => 1.0,
            LengthUnit::Millimeter => 0.001,
            LengthUnit::Inch => 2.51 / 1000.0,
            LengthUnit::Angstrom => todo!(),
            LengthUnit::Mile => todo!(),
            LengthUnit::Furlong => todo!(),
            LengthUnit::Chain => todo!(),
            LengthUnit::Rod => todo!(),
            LengthUnit::Fathom => todo!(),
            LengthUnit::Yard => todo!(),
            LengthUnit::Foot => todo!(),
            LengthUnit::Parsec => todo!(),
            LengthUnit::LightYear => todo!(),
            LengthUnit::AstronomicalUnit => todo!(),
        }
    }
}

struct Length {
    value: f64,
    unit: LengthUnit,
}

impl Length {
    pub fn to_meters(&self) -> f64 {
        self.value * self.unit.meters()
    }
    pub fn to_unit(&self, unit: LengthUnit) -> Self {
        let meters = self.value * self.unit.meters();
        let value = meters / unit.meters();
        Self { unit, value }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "text")
    }
}

impl Clone for Length {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            unit: self.unit.clone(),
        }
    }
}

enum MassUnit {
    Kilogram,
    Gram,
    Milligram,
    MetricTon,
    Avoirdupois,
    Pound,
    Ounce,
    Drachm,
    Grain,
    Troy,
}

// impl Unit for MassUnit {
//     fn in_baseunits(&self) -> f64 {
//         self.kg()
//     }
// }

impl MassUnit {
    pub fn kg(&self) -> f64 {
        todo!()
    }
}

/*
Rust ... C++/Java
f32   ->  float      (floating point number)
f64   ->  double     (double precisin float)
*/

struct Mass {
    value: f64,
    unit: MassUnit,
}

impl Mass {
    pub fn to_kg(&self) -> f64 {
        self.value * self.unit.kg()
    }
    pub fn to_unit(&self, unit: MassUnit) -> Self {
        let kg = self.value * self.unit.kg();
        let value = kg / unit.kg();
        Self { unit, value }
    }
}
