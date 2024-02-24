use log::trace;
use rand::{distributions::Slice, thread_rng, Rng};
use std::fmt;

#[derive(Debug)]
pub enum GeneratePassphraseError {
    EmptyWordlist
}

impl fmt::Display for GeneratePassphraseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GeneratePassphraseError::EmptyWordlist => {
                write!(formatter, "The wordlist is empty")
            }
        }
    }
}

impl std::error::Error for GeneratePassphraseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            GeneratePassphraseError::EmptyWordlist => None
        }
    }
}

/// Generate a passphrase.
pub fn generate_passphrase(wordlist: &[String], separator: &String, length: u64) -> Result<Vec<u8>, GeneratePassphraseError> {
    if length == 0 { return Ok(Vec::<u8>::new()); }
    if wordlist.is_empty() { return Err(GeneratePassphraseError::EmptyWordlist); }

    let distribution = Slice::new(wordlist).unwrap();

    let words = thread_rng()
        .sample_iter(distribution)
        .take(length as usize)
        .map(|value| value.to_owned())
        .collect::<Vec<String>>();
    let count = words.len();

    trace!("Generated a passphrase with {} words", count);

    let output = words.join(separator).into_bytes();

    Ok(output)
}
