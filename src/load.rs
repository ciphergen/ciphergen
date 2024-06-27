use std::fs::{create_dir_all, read, File};
use std::io::{stdin, Read, Write};
use std::path::PathBuf;

use bincode::{deserialize, serialize};
use directories::ProjectDirs;
use log::{debug, trace, warn};
use rand::Rng;
use rand::{thread_rng, prelude::SliceRandom};
use zstd::{Decoder, Encoder};

use crate::corpus::{load_corpus, load_default_corpus};
use crate::markov::{Generator, MarkovGenerator};
use crate::wordlist::{load_default_wordlist, load_wordlist};

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
pub fn read_in<'a>(path: Option<PathBuf>) -> VecByteResult<'a> {
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
pub fn get_character_set(numbers: bool, symbols: bool) -> Vec<char> {
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

/// Retrieves a wordlist
pub fn get_wordlist<'a, R: Rng + Sized>(path: Option<PathBuf>, delimiter: &str, rng: &mut R) -> VecStringResult<'a> {
    let wordlist = match path {
        Some(path) => load_wordlist(&path, &delimiter, rng)?,
        None => load_default_wordlist(rng)?,
    };

    Ok(wordlist)
}

/// Retrieves a corpus
fn get_corpus<'a>(path: Option<PathBuf>) -> VecStringResult<'a> {
    let corpus = match path {
        Some(path) => load_corpus(&path)?,
        None => load_default_corpus()?
    };

    Ok(corpus)
}

/// Reads a cached model file
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

/// Writes a model file to cache
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

/// Gets a generator instance
pub fn get_generator<'a>(corpus_path: Option<PathBuf>, model_parameters: (usize, f64, bool), cache_control: (bool, bool)) -> GeneratorResult<'a> {
    let project_dir = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or("Failed to find cache path on filesystem")?;
    let cache_dir = project_dir.cache_dir();
    let cache_path = &cache_dir.join(MODEL_FILE_NAME);

    create_dir_all(cache_dir)?;

    let (order, prior, backoff) = model_parameters;
    let (no_cache, rebuild_cache) = cache_control;

    if no_cache {
        let corpus = get_corpus(corpus_path)?;

        warn!("Building model because caching is disabled; this may take awhile...");

        let generator = Generator::new(&corpus, order, prior, backoff);

        trace!("\n{generator}");

        return Ok(generator);
    };

    if rebuild_cache {
        let corpus = get_corpus(corpus_path)?;

        warn!("Rebuilding model; this may take awhile...");

        let generator = Generator::new(&corpus, order, prior, backoff);

        trace!("\n{generator}");

        write_generator(cache_path, &generator)?;

        return Ok(generator);
    };

    let generator = match read_generator(cache_path) {
        Ok(generator) => {
            debug!("Using cached model file at {cache_path:#?}");
            trace!("\n{generator}");

            generator
        },
        Err(error) => {
            trace!("{error:#}");

            let corpus = get_corpus(corpus_path)?;

            warn!("Building model because cache is missing; this may take awhile...");

            let generator = Generator::new(&corpus, order, prior, backoff);

            trace!("\n{generator}");

            write_generator(cache_path, &generator)?;

            generator
        }
    };

    Ok(generator)
}
