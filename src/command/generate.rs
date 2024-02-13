use std::fmt;
use std::fs::read_to_string;

use crate::generators::binary::{generate_bytes, generate_hex, generate_base64, GenerateBytesError};
use crate::generators::password::{generate_password, GeneratePasswordError};
use crate::generators::passphrase::{generate_passphrase, GeneratePassphraseError};
use crate::generators::username::{generate_simple_username, generate_syllabic_username, GenerateUsernameError};
use crate::generators::digits::{generate_digits, GenerateDigitsError};
use crate::generators::number::generate_number;

use super::arguments::{GenerateCommands, UsernameCommands};

#[derive(Debug)]
pub enum GenerateError {
    InvalidLength(u64),
    IO(std::io::Error),
    Bytes(GenerateBytesError),
    Password(GeneratePasswordError),
    Passphrase(GeneratePassphraseError),
    Username(GenerateUsernameError),
    Digits(GenerateDigitsError),
    Number
}

impl fmt::Display for GenerateError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenerateError::InvalidLength(length) => { write!(formatter, "expected a positive integer but got {} instead", length) },
            GenerateError::IO(error) => write!(formatter, "{}", error),
            GenerateError::Bytes(error) => write!(formatter, "{}", error),
            GenerateError::Password(error) => write!(formatter, "{}", error),
            GenerateError::Passphrase(error) => write!(formatter, "{}", error),
            GenerateError::Username(error) => write!(formatter, "{}", error),
            GenerateError::Digits(error) => write!(formatter, "{}", error),
            GenerateError::Number => panic!()
        }
    }
}

impl std::error::Error for GenerateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            GenerateError::InvalidLength(_) => None,
            GenerateError::IO(ref error) => Some(error),
            GenerateError::Bytes(ref error) => Some(error),
            GenerateError::Password(ref error) => Some(error),
            GenerateError::Passphrase(ref error) => Some(error),
            GenerateError::Username(ref error) => Some(error),
            GenerateError::Digits(ref error) => Some(error),
            GenerateError::Number => None,
        }
    }
}

fn load_wordlist(path: &String, delimiter: &String) -> Result<Vec<String>, GenerateError> {
    let input = read_to_string(path).map_err(GenerateError::IO)?;
    let wordlist = input.split(delimiter).map(|value| value.to_string()).collect();

    Ok(wordlist)
}

fn load_default_wordlist() -> Vec<String> {
    include_str!("../wordlist.txt").split('\n').map(|value| value.to_string()).collect()
}

fn bytes(length: u64) -> Result<Vec<u8>, GenerateError> {
    let bytes = generate_bytes(length).map_err(GenerateError::Bytes)?;

    Ok(bytes)
}

fn hex(uppercase: bool, length: u64) -> Result<Vec<u8>, GenerateError> {
    let hex = generate_hex(uppercase, length).map_err(GenerateError::Bytes)?;

    Ok(hex)
}

fn base64(url_safe: bool, length: u64) -> Result<Vec<u8>, GenerateError> {
    let base64 = generate_base64(url_safe, length).map_err(GenerateError::Bytes)?;

    Ok(base64)
}

fn password(length: u64, count: Option<u64>) -> Result<Vec<u8>, GenerateError> {
    let max = count.unwrap_or(1);

    if max == 0 { return Err(GenerateError::InvalidLength(max)); }

    let mut output = Vec::<u8>::new();

    for _ in 0..max {
        let mut value = generate_password(length).map_err(GenerateError::Password)?;

        output.append(&mut value);
        output.push(b'\n');
    }

    Ok(output)
}

fn passphrase(path: &Option<String>, delimiter: &String, separator: &String, length: u64, count: Option<u64>) -> Result<Vec<u8>, GenerateError> {
    let max = count.unwrap_or(1);

    if max == 0 { return Err(GenerateError::InvalidLength(max)); }

    let mut output = Vec::<u8>::new();

    let wordlist = match path {
        Some(value) => load_wordlist(value, delimiter)?,
        None => load_default_wordlist(),
    };

    for _ in 0..max {
        let mut value = generate_passphrase(&wordlist, &separator, length).map_err(GenerateError::Passphrase)?;

        output.append(&mut value);
        output.push(b'\n');
    }

    Ok(output)
}

fn username(command: &UsernameCommands) -> Result<Vec<u8>, GenerateError> {
    match *command {
        UsernameCommands::Simple { length, count } => {
            let max = count.unwrap_or(1);

            if max == 0 { return Err(GenerateError::InvalidLength(max)); }

            let mut output = Vec::<u8>::new();

            for _ in 0..max {
                let mut value = generate_simple_username(length).map_err(GenerateError::Username)?;

                output.append(&mut value);
                output.push(b'\n');
            }

            Ok(output)
        }
        UsernameCommands::Syllabic { length, count } => {
            let max = count.unwrap_or(1);

            if max == 0 { return Err(GenerateError::InvalidLength(max)); }

            let mut output = Vec::<u8>::new();

            for _ in 0..max {
                let mut value = generate_syllabic_username(length).map_err(GenerateError::Username)?;

                output.append(&mut value);
                output.push(b'\n');
            }

            Ok(output)
        }
    }
}

fn digits(length: u64, count: Option<u64>) -> Result<Vec<u8>, GenerateError> {
    let max = count.unwrap_or(1);

    if max == 0 { return Err(GenerateError::InvalidLength(max)); }

    let mut output = Vec::<u8>::new();

    for _ in 0..max {
        let mut value = generate_digits(length).map_err(GenerateError::Digits)?;

        output.append(&mut value);
        output.push(b'\n');
    }

    Ok(output)
}

fn number(minimum: u64, maximum: u64, count: Option<u64>) -> Result<Vec<u8>, GenerateError> {
    let max = count.unwrap_or(1);

    if max == 0 { return Err(GenerateError::InvalidLength(max)); }

    let mut output = Vec::<u8>::new();

    for _ in 0..max {
        let mut value = generate_number(minimum, maximum).map_err(|_| GenerateError::Number)?;

        output.append(&mut value);
        output.push(b'\n');
    }

    Ok(output)
}

pub fn generate(command: &GenerateCommands) -> Result<Vec<u8>, GenerateError> {
    match command {
        GenerateCommands::Bytes { length } => bytes(*length),
        GenerateCommands::Hex { uppercase, length } => hex(*uppercase, *length),
        GenerateCommands::Base64 { url_safe, length } => base64(*url_safe, *length),
        GenerateCommands::Password { length, count } => password(*length, *count),
        GenerateCommands::Passphrase { path, delimiter, separator, length, count }
            => passphrase(path, delimiter, separator, *length, *count),
        GenerateCommands::Username { command } => username(command),
        GenerateCommands::Digits { length, count } => digits(*length, *count),
        GenerateCommands::Number { minimum, maximum, count } => number(*minimum, *maximum, *count)
    }
}
