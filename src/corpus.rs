use std::{fs::read, path::PathBuf, str::from_utf8};

use include_dir::{include_dir, Dir};
use log::debug;

type BoxedError<'a> = Box<dyn std::error::Error + Send + Sync + 'a>;
type VecStringResult<'a> = Result<Vec<String>, BoxedError<'a>>;

fn split_text(input: &str, delimiter: &str) -> Vec<String> {
    input.split(delimiter)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_owned())
        .collect()
}

pub fn load_corpus<'a>(path: &PathBuf) -> VecStringResult<'a> {
    let bytes = read(path)?;
    let string = from_utf8(&bytes)?;
    let mut corpus = split_text(string, "\n");
    let count = corpus.len();

    corpus.sort();

    if corpus.is_empty() { return Err(format!("The corpus at {path:?} contains no words").into()); }

    debug!("Loaded {count} words from the wordlist at {path:?}");

    Ok(corpus)
}

static CORPORA: Dir = include_dir!("src/corpora");

pub fn load_default_corpus<'a>() -> VecStringResult<'a> {
    let mut corpus = CORPORA.files()
        .map(|file| file.contents_utf8())
        .flatten()
        .map(|value| split_text(value, "\n"))
        .flatten()
        .collect::<Vec<_>>();
    let count = corpus.len();

    corpus.sort();

    debug!("Loaded {count} words from the default corpus");

    Ok(corpus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_correct_number_of_words() {
        let corpus = load_default_corpus().unwrap();
        let length = corpus.len();

        assert_eq!(length, 19726);
    }
}
