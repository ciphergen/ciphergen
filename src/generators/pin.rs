use rand::{Rng, thread_rng};

use super::SecretKeyLength;

pub fn generate_pin(length: &SecretKeyLength) -> Result<SecretKeyLength, String> {
    if length <= &0 {
        return Err(
            format!("Cannot generate a PIN of length {}", length)
        );
    }

    let radix = 10.0f64;
    let offset = length - 1;
    let maximum = radix.powi(*length as i32) as u64;
    let minimum = radix.powi(offset as i32) as u64;

    Ok(
        thread_rng().gen_range(minimum..maximum)
    )
}
