use std::fs::{create_dir_all, read, File};
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
mod markov;

use bincode::{deserialize, serialize};
use config::{parse, setup_logging, Commands, GenerateCommands, UsernameCommands};
use analyze::analyze;
use directories::ProjectDirs;
use generate::{create_base64, create_bytes, create_digits, create_hex, create_markov, create_number, create_passphrase, create_password, create_username, UsernameKind};
use log::{debug, trace, warn};
use markov::{load_corpus, load_default_corpus, Generator, MarkovGenerator};
use rand::Rng;
use rand::{thread_rng, prelude::SliceRandom};
use visualize::visualize;
use wordlist::{load_default_wordlist, load_wordlist};
use panic::setup_panic;
use zstd::{Decoder, Encoder};

type BoxedError<'a> = Box<dyn std::error::Error + Send + Sync + 'a>;
type UnitResult<'a> = Result<(), BoxedError<'a>>;
type VecByteResult<'a> = Result<Vec<u8>, BoxedError<'a>>;
type VecStringResult<'a> = Result<Vec<String>, BoxedError<'a>>;
type GeneratorResult<'a> = Result<Generator, BoxedError<'a>>;

const QUALIFIER: &str = "io.github";
const ORGANIZATION: &str = "ciphergen";
const APPLICATION: &str = "ciphergen";
const MODEL_FILE_NAME: &str = "model.bin.zst";

/// Read data from a file or STDIN
fn read_in<'a>(path: Option<PathBuf>) -> VecByteResult<'a> {
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
fn get_character_set(numbers: bool, symbols: bool) -> Vec<char> {
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

fn get_wordlist<'a, R: Rng + Sized>(path: Option<PathBuf>, delimiter: &str, rng: &mut R) -> VecStringResult<'a> {
    let wordlist = match path {
        Some(path) => load_wordlist(&path, &delimiter, rng)?,
        None => load_default_wordlist(rng)?,
    };

    Ok(wordlist)
}

fn get_corpus<'a, R: Rng + Sized>(path: Option<PathBuf>, rng: &mut R) -> VecStringResult<'a> {
    let corpus = match path {
        Some(path) => load_corpus(&path, rng)?,
        None => load_default_corpus(rng)?
    };

    Ok(corpus)
}

fn read_generator<'a>(path: &PathBuf) -> GeneratorResult<'a> {
    let file = File::options()
        .read(true)
        .open(path)?;
    let mut decoder = Decoder::new(file)?;
    let mut buffer = Vec::<u8>::new();

    decoder.read_to_end(&mut buffer)?;

    let generator = deserialize::<Generator>(&buffer)?;

    Ok(generator)
}

fn write_generator<'a>(path: &PathBuf, generator: &Generator) -> UnitResult<'a> {
    let buffer = serialize(&generator)?;
    let file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path.clone())?;
    let mut encoder = Encoder::new(file, 3)?.auto_finish();

    encoder.write_all(&buffer)?;

    debug!("Wrote model file to cache at {path:#?}");

    Ok(())
}

fn get_generator<'a>(corpus_path: Option<PathBuf>, order: usize, prior: f64, backoff: bool, no_cache: bool) -> GeneratorResult<'a> {
    let generator = if no_cache {
        let mut rng = thread_rng();
        let corpus = get_corpus(corpus_path, &mut rng)?;

        warn!("Building model because caching is disabled; this may take awhile...");

        Generator::new(&corpus, order, prior, backoff)
    }
    else {
        let project_dir = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .ok_or("Failed to find cache path on filesystem")?;
        let cache_dir = project_dir.cache_dir();
        let cache_path = &cache_dir.join(MODEL_FILE_NAME);

        create_dir_all(cache_dir)?;

        match read_generator(cache_path) {
            Ok(generator) => {
                debug!("Using cached model file at {cache_path:#?}");

                generator
            },
            Err(error) => {
                trace!("{error:#}");

                let mut rng = thread_rng();
                let corpus = get_corpus(corpus_path, &mut rng)?;

                warn!("Building model because cache is missing; this may take awhile...");

                let generator = Generator::new(&corpus, order, prior, backoff);

                write_generator(cache_path, &generator)?;

                generator
            }
        }
    };

    Ok(generator)
}

fn execute() -> UnitResult<'static> {
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
                        let character_set = get_character_set(numbers, symbols);

                        spawn(move || create_password(sender, &character_set, length, count))
                    },
                GenerateCommands::Passphrase { path, delimiter, separator, length, count }
                    => {
                        let mut rng = thread_rng();
                        let wordlist = get_wordlist(path, &delimiter, &mut rng)?;

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
                    => spawn(move || create_number(sender, minimum, maximum, count)),
                GenerateCommands::Markov { capitalize, path, order, prior, backoff, no_cache, count }
                    => {
                        let generator = get_generator(path, order, prior, backoff, no_cache)?;

                        spawn(move || create_markov(sender, capitalize, generator, count))
                    }
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

fn main() {
    if let Err(error) = execute() {
        panic!("{error}");
    }
}
