use std::fmt;
use base64::Engine;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use hex::{encode, encode_upper};
use base64::engine::general_purpose::{STANDARD, URL_SAFE};

#[derive(Debug)]
pub enum GenerateBytesError {
    InvalidLength(u64)
}

impl fmt::Display for GenerateBytesError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenerateBytesError::InvalidLength(length) => {
                write!(formatter, "expected a positive integer but got {} instead", length)
            }
        }
    }
}

impl std::error::Error for GenerateBytesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            GenerateBytesError::InvalidLength(_) => None
        }
    }
}

pub fn generate_bytes(length: &u64) -> Result<Vec<u8>, GenerateBytesError> {
    if length <= &0 { return Err(GenerateBytesError::InvalidLength(*length)); }

    let range = Uniform::new(u8::MIN, u8::MAX);
    let bytes = thread_rng().sample_iter(range).take(*length as usize).collect();

    Ok(bytes)
}

pub fn generate_hex(uppercase: &bool, length: &u64) -> Result<Vec<u8>, GenerateBytesError> {
    let bytes = generate_bytes(length)?;
    let hex = if *uppercase { encode_upper(bytes).into_bytes() } else { encode(bytes).into_bytes() };

    Ok(hex)
}

pub fn generate_base64(url_safe: &bool, length: &u64) -> Result<Vec<u8>, GenerateBytesError> {
    let bytes = generate_bytes(length)?;
    let base64 = if *url_safe { URL_SAFE.encode(bytes).into_bytes() } else { STANDARD.encode(bytes).into_bytes() };

    Ok(base64)
}
