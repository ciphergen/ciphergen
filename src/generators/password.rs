use std::fmt;
use rand::thread_rng;
use rand::distributions::{Alphanumeric, DistString};

#[derive(Debug)]
pub enum GeneratePasswordError {
    InvalidLength(u64)
}

impl fmt::Display for GeneratePasswordError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GeneratePasswordError::InvalidLength(length) => {
                write!(formatter, "expected a positive integer but got {} instead", length)
            }
        }
    }
}

impl std::error::Error for GeneratePasswordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            GeneratePasswordError::InvalidLength(_) => None
        }
    }
}

pub fn generate_password(length: &u64) -> Result<Vec<u8>, GeneratePasswordError> {
    if length <= &0 { return Err(GeneratePasswordError::InvalidLength(*length)); }

    let password = Alphanumeric.sample_string(&mut thread_rng(), *length as usize).into_bytes();

    Ok(password)
}
