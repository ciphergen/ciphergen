use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::fs::read_to_string;

const DEFAULT_SEPARATOR: &str = " ";

fn load_wordlist(path: &String) -> Result<Vec<String>, GeneratePassphraseError> {
    let input = read_to_string(path).map_err(GeneratePassphraseError::IO)?;
    let wordlist = input.split('\n').map(|value| value.to_string()).collect();

    Ok(wordlist)
}

fn load_default_wordlist() -> Vec<String> {
    include_str!("wordlist.txt")
        .split('\n')
        .map(|value| value.to_string())
        .collect()
}

#[derive(Debug)]
pub enum GeneratePassphraseError {
    InvalidLength(u64),
    EmptyWordlist,
    IO(std::io::Error),
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
            GeneratePassphraseError::IO(error) => write!(formatter, "{}", error),
        }
    }
}

impl std::error::Error for GeneratePassphraseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            GeneratePassphraseError::InvalidLength(_) => None,
            GeneratePassphraseError::EmptyWordlist => None,
            GeneratePassphraseError::IO(ref error) => Some(error),
        }
    }
}

/// Generate a passphrase.
pub fn generate_passphrase(
    length: &u64,
    delimiter: &Option<String>,
    path: &Option<String>,
) -> Result<String, GeneratePassphraseError> {
    if *length == 0 {
        return Err(GeneratePassphraseError::InvalidLength(*length));
    }

    let wordlist = match path {
        Some(value) => load_wordlist(value)?,
        None => load_default_wordlist(),
    };

    if wordlist.is_empty() {
        return Err(GeneratePassphraseError::EmptyWordlist);
    }

    let words: Vec<String> = wordlist
        .choose_multiple(&mut thread_rng(), *length as usize)
        .map(|value| value.to_string())
        .collect();

    let separator = delimiter
        .clone()
        .unwrap_or(DEFAULT_SEPARATOR.to_string());

    let passphrase = words.join(&separator);

    Ok(passphrase)
}
