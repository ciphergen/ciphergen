use std::{fs::read_to_string, path::PathBuf, str::from_utf8};

use log::debug;
use rand::{seq::SliceRandom, Rng};
use zstd::bulk::decompress;

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

    wordlist.sort();
    wordlist.dedup();
    wordlist.shuffle(rng);

    if wordlist.is_empty() { return Err(format!("The wordlist at {path:?} contains no words").into()); }

    debug!("Loaded {count} words from the wordlist at {path:?}");

    Ok(wordlist)
}

pub fn load_default_wordlist<'a, R: Rng + Sized>(rng: &mut R) -> StringVecResult<'a> {
    let buffer = include_bytes!("wordlist.txt.zst");
    let bytes = decompress(buffer, 62144)?;
    let mut wordlist = from_utf8(&bytes)?
        .split('\n')
        .map(|value| value.to_owned())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let count = wordlist.len();

    wordlist.sort();
    wordlist.dedup();
    wordlist.shuffle(rng);

    assert_eq!(count, 7776);

    debug!("Loaded {count} words from the default wordlist");

    Ok(wordlist)
}
