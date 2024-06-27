use rand::Rng;

use crate::markov::{Generator, MarkovGenerator};

trait Capitalize {
    fn capitalize(&mut self);
}

impl Capitalize for String {
    fn capitalize(&mut self) {
        if let Some(character) = self.chars().next() {
            self.replace_range(0..1, &character.to_uppercase().to_string());
        }
    }
}

pub fn generate_markov<R: Rng + Sized>(generator: &Generator, capitalize: bool, minimum: usize, maximum: usize, rng: &mut R) -> Vec<u8> {
    let mut username = String::new();
    let mut length = 0;

    while !(minimum..maximum).contains(&length) {
        username = generator.generate(rng).replace("#", "");
        length = username.len();
    }

    if capitalize { username.capitalize(); }

    return username.into_bytes()
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use rand::thread_rng;

    use crate::corpus::load_default_corpus;

    use super::*;

    #[test]
    fn generates_a_username() {
        let corpus = load_default_corpus().unwrap();

        let prior: f64 = 0.0;
        let order = 3;
        let backoff = false;
        let minimum = 2;
        let maximum = 10;

        let generator = Generator::new(&corpus, order, prior, backoff);
        let mut rng = thread_rng();
        let bytes = generate_markov(&generator, false, minimum, maximum, &mut rng);
        let username = from_utf8(&bytes).unwrap();

        assert!(!username.is_empty());
    }
}
