use std::io::{Write, stdout};
use std::fmt;

use crate::generators::binary::{generate_bytes, generate_hex, generate_base64, GenerateBytesError};
use crate::generators::password::{generate_ascii_password, generate_full_password, GeneratePasswordError};
use crate::generators::passphrase::{generate_passphrase, GeneratePassphraseError};
use crate::generators::username::{generate_simple_username, generate_syllabic_username, GenerateUsernameError};
use crate::generators::digits::{generate_digits, GenerateDigitsError};
use crate::generators::number::generate_number;

use super::arguments::{GenerateCommands, PasswordCommands, UsernameCommands};

fn bytes(count: &u64) -> Result<(), GenerateError> {
    let bytes = generate_bytes(count)
        .map_err(GenerateError::Bytes)?;

    let mut stdout = stdout();

    stdout
        .write_all(&bytes)
        .map_err(GenerateError::IO)?;

    stdout
        .flush()
        .map_err(GenerateError::IO)?;

    Ok(())
}

fn hex(uppercase: &bool, count: &u64) -> Result<(), GenerateError> {
    let hex = generate_hex(uppercase, count)
        .map_err(GenerateError::Bytes)?;

    println!("{}", hex);

    Ok(())
}

fn base64(url_safe: &bool, count: &u64) -> Result<(), GenerateError> {
    let base64 = generate_base64(url_safe, count)
        .map_err(GenerateError::Bytes)?;

    println!("{}", base64);

    Ok(())
}

fn password(command: &PasswordCommands) -> Result<(), GenerateError> {
    match command {
        PasswordCommands::Alphabetic { length } => {
            let password = generate_ascii_password(length)
                .map_err(GenerateError::Password)?;

            println!("{}", password);

            Ok(())
        },
        PasswordCommands::Full { length } => {
            let password = generate_full_password(length)
                .map_err(GenerateError::Password)?;

            println!("{}", password);

            Ok(())
        }
    }
}

fn passphrase(length: &u64, delimiter: &Option<String>, path: &Option<String>) -> Result<(), GenerateError> {
    let passphrase = generate_passphrase(length, delimiter, path)
        .map_err(GenerateError::Passphrase)?;

    println!("{}", passphrase);

    Ok(())
}

fn username(command: &UsernameCommands) -> Result<(), GenerateError> {
    match command {
        UsernameCommands::Simple { length } => {
            let username = generate_simple_username(length)
                .map_err(GenerateError::Username)?;

            println!("{}", username);

            Ok(())
        }
        UsernameCommands::Syllabic { length } => {
            let username = generate_syllabic_username(length)
                .map_err(GenerateError::Username)?;

            println!("{}", username);

            Ok(())
        }
    }
}

fn digits(count: &u64) -> Result<(), GenerateError> {
    let value = generate_digits(count)
        .map_err(GenerateError::Digits)?;

    println!("{}", value);

    Ok(())
}

fn number(minimum: &u64, maximum: &u64) -> Result<(), GenerateError> {
    let number = generate_number(minimum, maximum)
        .map_err(|_| GenerateError::Number)?;

    println!("{}", number);

    Ok(())
}

#[derive(Debug)]
pub enum GenerateError {
    Bytes(GenerateBytesError),
    Password(GeneratePasswordError),
    Passphrase(GeneratePassphraseError),
    Username(GenerateUsernameError),
    Digits(GenerateDigitsError),
    Number,
    IO(std::io::Error)
}

impl fmt::Display for GenerateError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
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

pub fn generate(command: &GenerateCommands) -> Result<(), GenerateError> {
    match command {
        GenerateCommands::Bytes { count } => bytes(count),
        GenerateCommands::Hex { uppercase, count } => hex(uppercase, count),
        GenerateCommands::Base64 { url_safe, count } => base64(url_safe, count),
        GenerateCommands::Password { command } => password(command),
        GenerateCommands::Passphrase { length, delimiter, path }
            => passphrase(length, delimiter, path),
        GenerateCommands::Username { command } => username(command),
        GenerateCommands::Digits { count } => digits(count),
        GenerateCommands::Number { minimum, maximum } => number(minimum, maximum)
    }
}
