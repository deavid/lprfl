use enum_iterator::Sequence;
use std::fmt::Display;

#[derive(Debug, Sequence, Clone, Copy)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
}

impl Default for TemperatureUnit {
    fn default() -> Self {
        Self::Celsius
    }
}

impl TemperatureUnit {
    pub fn convert_to(&self, temp: f64, unit: &Self) -> f64 {
        unit.convert_from_kelvin(self.convert_to_kelvin(temp))
    }
    pub fn convert_to_kelvin(&self, t: f64) -> f64 {
        match self {
            TemperatureUnit::Celsius => t + 273.15,
            TemperatureUnit::Fahrenheit => (t - 32.0) / 1.8 + 273.15,
            TemperatureUnit::Kelvin => t,
            TemperatureUnit::Rankine => t / 1.8,
        }
    }
    pub fn convert_from_kelvin(&self, t: f64) -> f64 {
        match self {
            TemperatureUnit::Celsius => t - 273.15,
            TemperatureUnit::Fahrenheit => (t - 273.15) * 1.8 - 32.0,
            TemperatureUnit::Kelvin => t,
            TemperatureUnit::Rankine => t * 1.8,
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            TemperatureUnit::Celsius => vec!["ºc", "celsius"],
            TemperatureUnit::Fahrenheit => vec!["ºf", "fahrenheit"],
            TemperatureUnit::Kelvin => vec!["k", "kelvin", "ºk"],
            TemperatureUnit::Rankine => vec!["ºr", "rankine"],
        }
    }
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self.names();
        let name = names.first().expect("Unit does not have names");
        write!(f, "{}", name)
    }
}
