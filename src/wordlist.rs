use std::fs::read_to_string;

use log::info;
use rand::{seq::SliceRandom, thread_rng};

pub fn load_wordlist(path: &String, delimiter: &String) -> Result<Vec<String>, std::io::Error> {
    let input = read_to_string(path)?;
    let mut wordlist = input
        .split(delimiter)
        .map(|value| value.to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<String>>();
    let count = wordlist.len();
    let mut rng = thread_rng();

    wordlist.shuffle(&mut rng);

    info!("Loaded {} words from the wordlist at {}", count, path);

    Ok(wordlist)
}

pub fn load_default_wordlist() -> Vec<String> {
    let mut rng = thread_rng();
    let mut wordlist = include_str!("./wordlist.txt")
        .split('\n')
        .map(|value| value.to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<String>>();
    let count = wordlist.len() as u64;

    wordlist.shuffle(&mut rng);

    info!("Loaded {} words from the default wordlist", count);

    wordlist
}
