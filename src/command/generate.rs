use std::fmt;

use crate::generators::binary::{generate_bytes, generate_hex, generate_base64, GenerateBytesError};
use crate::generators::password::{generate_password, GeneratePasswordError};
use crate::generators::passphrase::{generate_passphrase, GeneratePassphraseError};
use crate::generators::username::{generate_simple_username, generate_syllabic_username, GenerateUsernameError};
use crate::generators::digits::{generate_digits, GenerateDigitsError};
use crate::generators::number::generate_number;

use super::arguments::{GenerateCommands, UsernameCommands};

fn bytes(count: &u64) -> Result<Vec<u8>, GenerateError> {
    let bytes = generate_bytes(count).map_err(GenerateError::Bytes)?;

    Ok(bytes)
}

fn hex(uppercase: &bool, count: &u64) -> Result<Vec<u8>, GenerateError> {
    let hex = generate_hex(uppercase, count).map_err(GenerateError::Bytes)?;

    Ok(hex)
}

fn base64(url_safe: &bool, count: &u64) -> Result<Vec<u8>, GenerateError> {
    let base64 = generate_base64(url_safe, count).map_err(GenerateError::Bytes)?;

    Ok(base64)
}

fn password(length: &u64) -> Result<Vec<u8>, GenerateError> {
    let password = generate_password(length).map_err(GenerateError::Password)?;

    Ok(password)
}

fn passphrase(path: &Option<String>, delimiter: &String, length: &u64, separator: &Option<String>) -> Result<Vec<u8>, GenerateError> {
    let passphrase = generate_passphrase(path, delimiter, length, separator).map_err(GenerateError::Passphrase)?;

    Ok(passphrase)
}

fn username(command: &UsernameCommands) -> Result<Vec<u8>, GenerateError> {
    match command {
        UsernameCommands::Simple { length } => {
            let username = generate_simple_username(length).map_err(GenerateError::Username)?;

            Ok(username)
        }
        UsernameCommands::Syllabic { length } => {
            let username = generate_syllabic_username(length).map_err(GenerateError::Username)?;

            Ok(username)
        }
    }
}

fn digits(count: &u64) -> Result<Vec<u8>, GenerateError> {
    let digits = generate_digits(count).map_err(GenerateError::Digits)?;

    Ok(digits)
}

fn number(minimum: &u64, maximum: &u64) -> Result<Vec<u8>, GenerateError> {
    let number = generate_number(minimum, maximum).map_err(|_| GenerateError::Number)?;

    Ok(number)
}

#[derive(Debug)]
pub enum GenerateError {
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
            GenerateError::Bytes(ref error) => Some(error),
            GenerateError::Password(ref error) => Some(error),
            GenerateError::Passphrase(ref error) => Some(error),
            GenerateError::Username(ref error) => Some(error),
            GenerateError::Digits(ref error) => Some(error),
            GenerateError::Number => None,
        }
    }
}

pub fn generate(command: &GenerateCommands) -> Result<Vec<u8>, GenerateError> {
    match command {
        GenerateCommands::Bytes { count } => bytes(count),
        GenerateCommands::Hex { uppercase, count } => hex(uppercase, count),
        GenerateCommands::Base64 { url_safe, count } => base64(url_safe, count),
        GenerateCommands::Password { length } => password(length),
        GenerateCommands::Passphrase { path, delimiter, length, separator }
            => passphrase(path, delimiter, length, separator),
        GenerateCommands::Username { command } => username(command),
        GenerateCommands::Digits { count } => digits(count),
        GenerateCommands::Number { minimum, maximum } => number(minimum, maximum)
    }
}
