use crate::measure::Measure;
use crate::unit::{Unit, UnitStandard};
use crate::AppError;
use anyhow::Result;
use enum_iterator::Sequence;

pub fn ask_user(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{}: ", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Cannot read from STDIN");

    // Remove the last return - CRLF
    s.trim().to_owned()
}

pub fn parse_to(txt_unit: &str) -> (String, Option<String>) {
    const TO: &str = " to ";
    let to_pos = txt_unit.find(TO);
    if let Some(to) = to_pos {
        // We have a "to"
        let txt_to = &txt_unit[to + TO.len()..txt_unit.len()];
        let txt_unit = &txt_unit[0..to];
        (txt_unit.to_owned(), Some(txt_to.to_owned()))
    } else {
        (txt_unit.to_owned(), None)
    }
}

#[derive(Debug, Sequence)]
pub enum DestUnit {
    Standard(UnitStandard),
    Unit(Unit),
}

impl Default for DestUnit {
    fn default() -> Self {
        Self::Standard(UnitStandard::Iso)
    }
}

impl DestUnit {
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
            DestUnit::Standard(s) => s.names(),
            DestUnit::Unit(c) => c.names(),
        }
    }
    pub fn units(&self) -> Vec<Unit> {
        match self {
            DestUnit::Standard(s) => s.units(),
            DestUnit::Unit(c) => vec![*c],
        }
    }
    pub fn standard(&self) -> Option<UnitStandard> {
        match self {
            DestUnit::Standard(s) => Some(*s),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ConvertRequest {
    pub input: Measure,
    pub to_unit: DestUnit,
}

impl ConvertRequest {
    pub fn from_text(input: &str) -> Result<Self> {
        let last_num = input.rfind(|c: char| matches!(c, '0'..='9' | '.' | '-'));
        if last_num.is_none() {
            return Err(AppError::MissingNumber(input.into()).into());
        }
        let last_num = last_num.unwrap();
        let txt_num = &input[0..=last_num];
        let number: f64 = txt_num.parse()?;

        let txt_unit = input[last_num + 1..input.len()].trim();
        if txt_unit.is_empty() {
            return Err(AppError::MissingUnit(input.into()).into());
        }
        // Check if the unit contains ' to ', like "12kg to lb"
        let (from_unit, to_unit) = parse_to(txt_unit);
        let from_unit = Unit::from_text(&from_unit)?;
        let to_unit = to_unit
            .map(|x| DestUnit::from_text(&x))
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            input: Measure {
                quantity: number,
                unit: from_unit,
            },
            to_unit,
        })
    }

    pub fn convert(&self) -> Result<Measure> {
        let units = self.to_unit.units();
        self.input
            .to_best_unit(units, self.to_unit.standard())
            .ok_or_else(|| AppError::NoSuitableUnitFound(self.input.unit.to_string()).into())
    }
}
