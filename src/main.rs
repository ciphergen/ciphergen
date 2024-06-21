use std::fs::{read, File};
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread::spawn;

mod wordlist;
mod config;
mod analyze;
mod generate;
mod generators;
mod visualize;
mod panic;

use config::{parse, setup_logging, Commands, GenerateCommands, UsernameCommands};
use analyze::analyze;
use generate::{create_base64, create_bytes, create_digits, create_hex, create_number, create_passphrase, create_password, create_username, UsernameKind};
use rand::{thread_rng, prelude::SliceRandom};
use visualize::visualize;
use wordlist::{load_default_wordlist, load_wordlist};
use panic::setup_panic;

type UnitResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type VecByteResult = Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>;

/// Read data from a file or STDIN
fn read_in(path: Option<PathBuf>) -> VecByteResult {
    let mut buffer = Vec::<u8>::new();

    match path {
        Some(value) => {
            buffer = read(value)?;
        },
        None => {
            stdin().read_to_end(&mut buffer)?;
        }
    };

    if buffer.is_empty() { return Err("There is no data to read".into()); }

    Ok(buffer)
}

/// Loads the correct character set based on the available symbols
fn load_character_set(numbers: bool, symbols: bool) -> Vec<char> {
    let flags = (numbers, symbols);

    let mut character_set: Vec<char> = match flags {
        (true, true) => "0123456789!@*-_.abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
        (true, false) => "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
        (false, true) => "!@*-_.abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
        (false, false) => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect()
    };

    let rng = &mut thread_rng();

    character_set.shuffle(rng);

    character_set
}

fn main() -> UnitResult {
    let arguments = parse();

    setup_panic();

    setup_logging(&arguments.verbosity)?;

    match arguments.command {
        Commands::Generate { command } => {
            let (sender, receiver) = channel::<Vec<u8>>();

            let handle = match command {
                GenerateCommands::Bytes { length }
                    => spawn(move || create_bytes(sender, length)),
                GenerateCommands::Hex { uppercase, length }
                    => spawn(move || create_hex(sender, uppercase, length)),
                GenerateCommands::Base64 { url_safe, length }
                    => spawn(move || create_base64(sender, url_safe, length)),
                GenerateCommands::Password { numbers, symbols, length, count }
                    => {
                        let character_set = load_character_set(numbers, symbols);

                        spawn(move || create_password(sender, &character_set, length, count))
                    },
                GenerateCommands::Passphrase { path, delimiter, separator, length, count }
                    => {
                        let mut rng = thread_rng();

                        let wordlist = match path {
                            Some(path) => load_wordlist(&path, &delimiter, &mut rng)?,
                            None => load_default_wordlist(&mut rng),
                        };

                        spawn(move || create_passphrase(sender, &wordlist, &separator, length, count))
                    },
                GenerateCommands::Username { capitalize, command }
                    => match command {
                        UsernameCommands::Simple { length, count }
                            => spawn(move || create_username(sender, capitalize, UsernameKind::Simple, length, count)),
                        UsernameCommands::Complex { length, count }
                            => spawn(move || create_username(sender, capitalize, UsernameKind::Complex, length, count))
                    },
                GenerateCommands::Digits { length, count }
                    => spawn(move || create_digits(sender, length, count)),
                GenerateCommands::Number { minimum, maximum, count }
                    => spawn(move || create_number(sender, minimum, maximum, count))
            };

            let mut stdout = stdout();

            for message in receiver {
                stdout.write_all(&message)?;
            }

            stdout.flush()?;

            handle.join().unwrap();
        }
        Commands::Analyze { input } => {
            let buffer = read_in(input)?;

            let report = analyze(buffer);

            println!("{report}");
        }
        Commands::Visualize { input, output } => {
            let buffer = read_in(input)?;

            if let Some(path) = output {
                let file = File::options()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path)?;

                visualize(file, &buffer)?;
            }
            else {
                let stdout = stdout();

                visualize(stdout, &buffer)?;
            };
        }
    }

    Ok(())
}
