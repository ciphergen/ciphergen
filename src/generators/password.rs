use rand::{Rng, thread_rng};
use rand::distributions::{Alphanumeric, Standard, DistString};

use super::SecretKeyLength;

pub fn generate_password(expanded: &bool, length: &SecretKeyLength) -> Result<String, String> {
    if length <= &0 {
        return Err(
            format!("Cannot generate a password of length {}", length)
        );
    }

    if *expanded {
        Ok(
            thread_rng()
            .sample_iter::<char, Standard>(Standard)
            .take(*length as usize)
            .map(char::from)
            .collect()
        )
    }
    else {
        Ok(
            Alphanumeric.sample_string(
                &mut thread_rng(),
                *length as usize
            )
        )
    }
}
