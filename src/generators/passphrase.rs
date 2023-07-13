use rand::thread_rng;
use rand::seq::IteratorRandom;

pub fn generate_passphrase(length: &u16) -> String {
    if length <= &0 {
        panic!("Cannot generate a passphrase of length 0");
    }

    include_str!("wordlist.txt")
        .to_string()
        .split('\n')
        .map(|value| value.to_string())
        .choose_multiple(&mut thread_rng(), *length as usize)
        .join(" ")
}
