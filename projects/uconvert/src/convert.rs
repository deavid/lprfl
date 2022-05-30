use crate::measure::Measure;
use crate::unit::Unit;
use crate::AppError;
use anyhow::Result;

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

#[derive(Debug)]
pub struct ConvertRequest {
    pub input: Measure,
    pub to_unit: Option<Unit>,
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
        let to_unit = to_unit.map(|x| Unit::from_text(&x)).transpose()?;

        Ok(Self {
            input: Measure {
                quantity: number,
                unit: from_unit,
            },
            to_unit,
        })
    }

    pub fn convert(&self) -> Result<Measure> {
        match self.to_unit {
            Some(dest_unit) => self.input.to_unit(dest_unit),
            None => self
                .input
                .to_iso_unit()
                .ok_or_else(|| AppError::NoSuitableUnitFound(self.input.unit.to_string()).into()),
        }
    }
}
