use std::fmt;
use std::fs::read_to_string;

use crate::generators::binary::{generate_bytes, generate_hex, generate_base64};
use crate::generators::password::generate_password;
use crate::generators::passphrase::{generate_passphrase, GeneratePassphraseError};
use crate::generators::username::{generate_simple_username, generate_complex_username};
use crate::generators::digits::generate_digits;
use crate::generators::number::generate_number;

use super::arguments::{GenerateCommands, UsernameCommands};

#[derive(Debug)]
pub enum GenerateError {
    IO(std::io::Error),
    Passphrase(GeneratePassphraseError)
}

impl fmt::Display for GenerateError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenerateError::IO(error) => write!(formatter, "{}", error),
            GenerateError::Passphrase(error) => write!(formatter, "{}", error)
        }
    }
}

impl std::error::Error for GenerateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            GenerateError::IO(ref error) => Some(error),
            GenerateError::Passphrase(ref error) => Some(error)
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

fn bytes(length: u64) -> Vec<u8> {
    generate_bytes(length)
}

fn hex(uppercase: bool, length: u64) -> Vec<u8> {
    generate_hex(uppercase, length)
}

fn base64(url_safe: bool, length: u64) -> Vec<u8> {
    generate_base64(url_safe, length)
}

fn password(length: u64, count: Option<u64>) -> Vec<u8> {
    let max = count.unwrap_or(1);
    let mut output = Vec::<u8>::new();

    if max == 0 { return output; }

    for _ in 0..max {
        let mut value = generate_password(length);

        output.append(&mut value);
        output.push(b'\n');
    }

    output
}

fn passphrase(path: &Option<String>, delimiter: &String, separator: &String, length: u64, count: Option<u64>) -> Result<Vec<u8>, GenerateError> {
    let max = count.unwrap_or(1);
    let mut output = Vec::<u8>::new();

    if max == 0 { return Ok(output); }

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

fn username(command: &UsernameCommands) -> Vec<u8> {
    match *command {
        UsernameCommands::Simple { length, count } => {
            let max = count.unwrap_or(1);
            let mut output = Vec::<u8>::new();

            if max == 0 { return output; }

            for _ in 0..max {
                let mut value = generate_simple_username(length);

                output.append(&mut value);
                output.push(b'\n');
            }

            output
        }
        UsernameCommands::Complex { length, count } => {
            let max = count.unwrap_or(1);
            let mut output = Vec::<u8>::new();

            if max == 0 { return output; }

            for _ in 0..max {
                let mut value = generate_complex_username(length);

                output.append(&mut value);
                output.push(b'\n');
            }

            output
        }
    }
}

fn digits(length: u64, count: Option<u64>) -> Vec<u8> {
    let max = count.unwrap_or(1);
    let mut output = Vec::<u8>::new();

    if max == 0 { return output; }

    for _ in 0..max {
        let mut value = generate_digits(length);

        output.append(&mut value);
        output.push(b'\n');
    }

    output
}

fn number(minimum: u64, maximum: u64, count: Option<u64>) -> Vec<u8> {
    let max = count.unwrap_or(1);
    let mut output = Vec::<u8>::new();

    if max == 0 { return output; }

    for _ in 0..max {
        let mut value = generate_number(minimum, maximum);

        output.append(&mut value);
        output.push(b'\n');
    }

    output
}

pub fn generate(command: GenerateCommands) -> Result<Vec<u8>, GenerateError> {
    match command {
        GenerateCommands::Bytes { length } => Ok(bytes(length)),
        GenerateCommands::Hex { uppercase, length } => Ok(hex(uppercase, length)),
        GenerateCommands::Base64 { url_safe, length } => Ok(base64(url_safe, length)),
        GenerateCommands::Password { length, count } => Ok(password(length, count)),
        GenerateCommands::Passphrase { path, delimiter, separator, length, count }
            => passphrase(&path, &delimiter, &separator, length, count),
        GenerateCommands::Username { command } => Ok(username(&command)),
        GenerateCommands::Digits { length, count } => Ok(digits(length, count)),
        GenerateCommands::Number { minimum, maximum, count } => Ok(number(minimum, maximum, count))
    }
}
