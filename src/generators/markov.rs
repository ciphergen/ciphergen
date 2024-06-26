use rand::Rng;

use crate::markov::{Generator, MarkovGenerator};

trait Capitalize {
    fn capitalize(&self) -> String;
}

impl Capitalize for String {
    fn capitalize(&self) -> String {
        let mut characters = self.chars();

        match characters.next() {
            Some(first) => first.to_uppercase().collect::<String>() + characters.as_str(),
            None => String::new()
        }
    }
}

pub fn generate_markov<R: Rng + Sized>(generator: Generator, capitalize: bool, rng: &mut R) -> Vec<u8> {
    let mut username = generator.generate(rng).replace("#", "");

    if capitalize { username = username.capitalize() }

    return username.into_bytes()
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use rand::thread_rng;

    use super::*;

    #[test]
    fn generates_a_username() {
        let corpus = vec![
            "alice", "bob", "charlie", "dave", "eve", "frank", "grace", "hank",
            "ivy", "jack", "kara", "liam", "mona", "nina", "oscar", "paul", "quinn",
            "rose", "sam", "tina", "uma", "vera", "will", "xena", "yara", "zane"
        ];
        let data = corpus.iter().map(|value| value.to_owned().to_owned()).collect::<Vec<_>>();

        let prior: f64 = 0.0;
        let order = 2;
        let backoff = false;

        let generator = Generator::new(&data, order, prior, backoff);
        let mut rng = thread_rng();

        let bytes = generate_markov(generator, false, &mut rng);
        let username = from_utf8(&bytes).unwrap();

        assert!(!username.is_empty());
    }
}
