use rand::thread_rng;
use rand::seq::IteratorRandom;

use super::SecretKeyLength;

pub fn generate_passphrase(length: &SecretKeyLength) -> Result<String, String> {
    if length <= &0 {
        return Err(
            format!("Cannot generate a passphrase of length {}", length)
        );
    }

    Ok(
        include_str!("wordlist.txt")
            .to_string()
            .split('\n')
            .map(|value| value.to_string())
            .choose_multiple(&mut thread_rng(), *length as usize)
            .join(" ")
    )
}
