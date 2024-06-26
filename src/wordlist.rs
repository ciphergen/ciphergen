use std::{fs::read_to_string, path::PathBuf};

use log::debug;
use rand::{seq::SliceRandom, Rng};

type BoxedError<'a> = Box<dyn std::error::Error + Send + Sync + 'a>;
type StringVecResult<'a> = Result<Vec<String>, BoxedError<'a>>;

pub fn load_wordlist<'a, R: Rng + Sized>(path: &PathBuf, delimiter: &str, rng: &mut R) -> StringVecResult<'a> {
    let input = read_to_string(path)?;
    let mut wordlist = input
        .split(delimiter)
        .map(|value| value.to_owned())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let count = wordlist.len();

    wordlist.shuffle(rng);

    if wordlist.is_empty() { return Err(format!("The wordlist at {path:?} contains no words").into()); }

    debug!("Loaded {count} words from the wordlist at {path:?}");

    Ok(wordlist)
}

pub fn load_default_wordlist<R: Rng + Sized>(rng: &mut R) -> Vec<String> {
    let mut wordlist = include_str!("./wordlist.txt")
        .split('\n')
        .map(|value| value.to_owned())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let count = wordlist.len();

    assert_eq!(count, 7776);

    wordlist.shuffle(rng);

    debug!("Loaded {count} words from the default wordlist");

    wordlist
}
