use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[derive(Debug)]
pub enum GeneratePassphraseError {
    InvalidLength(u64),
    EmptyWordlist
}

impl fmt::Display for GeneratePassphraseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GeneratePassphraseError::InvalidLength(length) => {
                write!(formatter, "expected positive integer but got {}", length)
            }
            GeneratePassphraseError::EmptyWordlist => {
                write!(formatter, "The provided wordlist is empty")
            }
        }
    }
}

impl std::error::Error for GeneratePassphraseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            GeneratePassphraseError::InvalidLength(_) => None,
            GeneratePassphraseError::EmptyWordlist => None
        }
    }
}

/// Generate a passphrase.
pub fn generate_passphrase(wordlist: &Vec<String>, separator: &String, length: u64) -> Result<Vec<u8>, GeneratePassphraseError> {
    if length == 0 { return Err(GeneratePassphraseError::InvalidLength(length)); }
    if wordlist.is_empty() { return Err(GeneratePassphraseError::EmptyWordlist); }

    let words: Vec<String> = wordlist.choose_multiple(&mut thread_rng(), length as usize).map(|value| value.to_string()).collect();
    let passphrase = words.join(separator).into_bytes();

    Ok(passphrase)
}
