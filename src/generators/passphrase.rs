use std::error::Error;
use std::fs::read_to_string;
use rand::thread_rng;
use rand::seq::SliceRandom;
use thiserror::Error;

use super::types::SecretKeyLength;

type WordlistReturnType = Result<Vec<String>, Box<dyn Error>>;

const SPACE: &str = " ";

fn load_wordlist(path: &String) -> WordlistReturnType {
    let input = read_to_string(path)?;

    Ok(
        input
            .split('\n')
            .map(|value| value.to_string())
            .collect()
    )
}

fn load_default_wordlist() -> WordlistReturnType {
    Ok(
        include_str!("wordlist.txt")
            .to_string()
            .split('\n')
            .map(|value| value.to_string())
            .collect()
    )
}

#[derive(Debug, Error)]
enum GeneratePassphraseError {
    #[error("Cannot generate a passphrase of length {0}")]
    InvalidLength(SecretKeyLength),
    #[error("No word list available")]
    NoWordlist
}

/// Generate a passphrase.
pub fn generate_passphrase(
    length: &SecretKeyLength,
    delimiter: &Option<String>,
    path: &Option<String>
) -> Result<String, Box<dyn Error>> {
    if *length == 0 {
        return Err(
            Box::new(GeneratePassphraseError::InvalidLength(*length))
        );
    }

    let wordlist = match path {
        Some(value) => load_wordlist(value),
        None => load_default_wordlist()
    }?;

    if wordlist.is_empty() {
        return Err(
            Box::new(GeneratePassphraseError::NoWordlist)
        )
    }

    let words: Vec<String> = wordlist
        .choose_multiple(&mut thread_rng(), *length as usize)
        .map(|value| value.to_string())
        .collect();

    let separator = match delimiter {
        Some(value) => value.as_str(),
        None => SPACE
    };

    Ok(
        words.join(separator)
    )
}
