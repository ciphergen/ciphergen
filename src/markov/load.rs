use std::{fs::read, path::PathBuf, str::from_utf8};

use log::debug;
use rand::{Rng, prelude::SliceRandom};
use zstd::bulk::decompress;

type BoxedError<'a> = Box<dyn std::error::Error + Send + Sync + 'a>;
type VecStringResult<'a> = Result<Vec<String>, BoxedError<'a>>;

pub fn load_corpus<'a, R: Rng + Sized>(path: &PathBuf, rng: &mut R) -> VecStringResult<'a> {
    let bytes = read(path)?;
    let mut corpus = from_utf8(&bytes)?
        .split('\n')
        .map(|value| value.to_owned())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let count = corpus.len();

    corpus.shuffle(rng);

    if corpus.is_empty() { return Err(format!("The corpus at {path:?} contains no words").into()); }

    debug!("Loaded {count} words from the wordlist at {path:?}");

    Ok(corpus)
}

pub fn load_default_corpus<'a, R: Rng + Sized>(rng: &mut R) -> VecStringResult<'a> {
    let buffer = include_bytes!("corpus.txt.zst");
    let bytes = decompress(buffer, 4234903)?;
    let mut corpus = from_utf8(&bytes)?
        .split('\n')
        .map(|value| value.to_owned())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let count = corpus.len();

    corpus.shuffle(rng);

    assert_eq!(count, 370104);

    debug!("Loaded {count} words from the default corpus");

    Ok(corpus)
}
