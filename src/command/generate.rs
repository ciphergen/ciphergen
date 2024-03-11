use std::process::exit;
use std::sync::mpsc::Sender;

use log::error;

use crate::generators::binary::{generate_bytes, generate_hex, generate_base64};
use crate::generators::password::generate_password;
use crate::generators::passphrase::generate_passphrase;
use crate::generators::username::{generate_simple_username, generate_complex_username};
use crate::generators::digits::generate_digits;
use crate::generators::number::generate_number;
use crate::wordlist::{load_wordlist, load_default_wordlist};

use super::arguments::{GenerateCommands, UsernameCommands};

const LINE_FEED: &[u8; 1] = b"\n";
const DEFAULT_CHUNK_SIZE: usize = 2_usize.pow(16);

/// Repeatedly invokes an enclosure with a subrange of the initial value.
fn repeat_while_remaining(initial: usize, repeats: Option<usize>, closure: impl Fn(usize, usize) -> ()) {
    let mut remaining = initial;

    for index in 0..repeats.unwrap_or(1) {
        loop {
            let count = initial.min(DEFAULT_CHUNK_SIZE);

            closure(count, index);

            match remaining.checked_sub(count) {
                Some(value) if value != 0 => remaining = value,
                _ => break
            }
        }
    }
}

fn split_to_characters(character_set: &str) -> Vec<char> {
    character_set.chars().collect()
}

fn bytes(sender: Sender<Vec<u8>>, length: usize) {
    repeat_while_remaining(length, None, |count, _| {
        let buffer = generate_bytes(count);

        sender.send(buffer).unwrap();
    });
}

fn hex(sender: Sender<Vec<u8>>, uppercase: bool, length: usize) {
    repeat_while_remaining(length, None, |count, _| {
        let buffer = generate_hex(uppercase, count);

        sender.send(buffer).unwrap();
    });
}

fn base64(sender: Sender<Vec<u8>>, url_safe: bool, length: usize) {
    repeat_while_remaining(length, None, |count, _| {
        let buffer = generate_base64(url_safe, count);

        sender.send(buffer).unwrap();
    });
}

fn password(sender: Sender<Vec<u8>>, numbers: bool, symbols: bool, length: usize, count: Option<usize>) {
    let max = count.unwrap_or(1);

    if max == 0 { return; }

    let character_set = if numbers && symbols {
        split_to_characters("!@*-_.0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }
    else if numbers {
        split_to_characters("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }
    else if symbols {
        split_to_characters("!@*-_.abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }
    else {
        split_to_characters("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
    };

    repeat_while_remaining(length, Some(max), |count, index| {
        let buffer = generate_password(&character_set, count);

        sender.send(buffer).unwrap();

        if index < max { sender.send(LINE_FEED.to_vec()).unwrap(); }
    });
}

fn passphrase(sender: Sender<Vec<u8>>, path: &Option<String>, delimiter: &String, separator: &String, length: usize, count: Option<usize>) {
    let max = count.unwrap_or(1);

    if max == 0 { return; }

    let wordlist = match path {
        Some(value) => match load_wordlist(value, delimiter) {
            Ok(value) => value,
            Err(error) => {
                error!("{}", error);
                exit(-1);
            }
        },
        None => load_default_wordlist(),
    };

    repeat_while_remaining(length, Some(max), |count, index| {
        let buffer = generate_passphrase(&wordlist, &separator, count);

        sender.send(buffer).unwrap();

        if index < max { sender.send(LINE_FEED.to_vec()).unwrap(); }
    });
}

fn username(sender: Sender<Vec<u8>>, capitalize: bool, command: UsernameCommands) {
    match command {
        UsernameCommands::Simple { length, count } => {
            let max = count.unwrap_or(1);

            if max == 0 { return; }

            repeat_while_remaining(length, Some(max), |count, index| {
                let buffer = generate_simple_username(capitalize, count);

                sender.send(buffer).unwrap();

                if index < max { sender.send(LINE_FEED.to_vec()).unwrap(); }
            });
        }
        UsernameCommands::Complex { length, count } => {
            let max = count.unwrap_or(1);

            if max == 0 { return; }

            repeat_while_remaining(length, Some(max), |count, index| {
                let buffer = generate_complex_username(capitalize, count);

                sender.send(buffer).unwrap();

                if index < max { sender.send(LINE_FEED.to_vec()).unwrap(); }
            });
        }
    }
}

fn digits(sender: Sender<Vec<u8>>, length: usize, count: Option<usize>) {
    let max = count.unwrap_or(1);

    if max == 0 { return; }

    repeat_while_remaining(length, Some(max), |count, index| {
        let buffer = generate_digits(count);

        sender.send(buffer).unwrap();

        if index < max { sender.send(LINE_FEED.to_vec()).unwrap(); }
    });
}

fn number(sender: Sender<Vec<u8>>, minimum: usize, maximum: usize, count: Option<usize>) {
    let max = count.unwrap_or(1);

    if max == 0 { return; }

    for index in 0..max {
        let buffer = generate_number(minimum, maximum);

        sender.send(buffer).unwrap();

        if index < max { sender.send(LINE_FEED.to_vec()).unwrap(); }
    }
}

pub fn generate(sender: Sender<Vec<u8>>, command: GenerateCommands) {
    match command {
        GenerateCommands::Bytes { length }
            => bytes(sender, length),
        GenerateCommands::Hex { uppercase, length }
            => hex(sender, uppercase, length),
        GenerateCommands::Base64 { url_safe, length }
            => base64(sender, url_safe, length),
        GenerateCommands::Password { numbers, symbols, length, count }
            => password(sender, numbers, symbols, length, count),
        GenerateCommands::Passphrase { path, delimiter, separator, length, count }
            => passphrase(sender, &path, &delimiter, &separator, length, count),
        GenerateCommands::Username { capitalize, command }
            => username(sender, capitalize, command),
        GenerateCommands::Digits { length, count }
            => digits(sender, length, count),
        GenerateCommands::Number { minimum, maximum, count }
            => number(sender, minimum, maximum, count)
    }
}
