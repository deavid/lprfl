use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing the number to convert: {0:?}")]
    MissingNumber(String),
    #[error("Missing the unit to convert to: {0:?}")]
    MissingUnit(String),
    #[error("Unit not found or not understood: {0:?}")]
    UnitNotFound(String),
    #[error("Units are incompatible for conversion: {0} -> {1}")]
    IncompatibleUnits(String, String),
}
