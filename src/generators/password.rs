use std::error::Error;

use rand::{Rng, thread_rng};
use rand::distributions::{Alphanumeric, Standard, DistString};
use thiserror::Error;

use super::SecretKeyLength;

type GeneratePasswordReturnType = Result<String, Box<dyn Error>>;

#[derive(Debug, Error)]
pub enum GeneratePasswordError {
    #[error("Cannot generate a password of length {0}")]
    InvalidLength(SecretKeyLength)
}

pub fn generate_ascii_password(length: &SecretKeyLength) -> GeneratePasswordReturnType {
    if length <= &0 {
        return Err(
            Box::new(GeneratePasswordError::InvalidLength(*length))
        );
    }

    Ok(
        Alphanumeric.sample_string(
            &mut thread_rng(),
            *length as usize
        )
    )
}

pub fn generate_full_password(length: &SecretKeyLength) -> GeneratePasswordReturnType {
    if length <= &0 {
        return Err(
            Box::new(GeneratePasswordError::InvalidLength(*length))
        );
    }

    Ok(
        thread_rng()
        .sample_iter::<char, Standard>(Standard)
        .take(*length as usize)
        .map(char::from)
        .collect()
    )
}
