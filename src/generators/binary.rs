use base64::Engine;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use hex::encode;
use base64::engine::general_purpose::STANDARD;

use super::SecretKeyLength;

pub fn generate_binary(length: &SecretKeyLength) -> Result<Vec<u8>, String> {
    if length <= &0 {
        return Err(
            format!("Cannot generate a secret key of length {}", length)
        );
    }

    let range = Uniform::new(u8::MIN, u8::MAX);

    Ok(
        thread_rng()
            .sample_iter(range)
            .take(*length as usize)
            .collect()
    )
}

pub fn generate_hexadecimal(length: &SecretKeyLength) -> Result<String, String> {
    let bytes = generate_binary(length)?;

    Ok(
        encode(bytes)
    )
}

pub fn generate_base64(length: &SecretKeyLength) -> Result<String, String> {
    let bytes = generate_binary(length)?;

    Ok(
        STANDARD.encode(bytes)
    )
}
