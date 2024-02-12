use rand::{Rng, thread_rng};
use std::fmt;

#[derive(Debug)]
pub enum GenerateDigitsError {
    InvalidLength(u64)
}

impl fmt::Display for GenerateDigitsError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenerateDigitsError::InvalidLength(length) => { write!(formatter, "expected a positive integer but got {} instead", length) }
        }
    }
}

impl std::error::Error for GenerateDigitsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            GenerateDigitsError::InvalidLength(_) => None
        }
    }
}

pub fn generate_digits(length: &u64) -> Result<Vec<u8>, GenerateDigitsError> {
    if length <= &0 { return Err(GenerateDigitsError::InvalidLength(*length)); }

    let radix = 10.0f64;
    let offset = length - 1;
    let maximum = radix.powi(*length as i32) as u64;
    let minimum = radix.powi(offset as i32) as u64;
    let value = thread_rng().gen_range(minimum..maximum).to_string().into_bytes();

    Ok(value)
}
