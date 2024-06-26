use rand::Rng;
use serde::{Deserialize, Serialize};

use super::model::{MarkovModel, Model};

pub trait MarkovGenerator {
    /// Creates a new procedural word generator.
    fn new(data: &[String], order: usize, prior: f64, backoff: bool) -> Self;

    /// Generates a word.
    fn generate<R: Rng + Sized>(&self, rng: &mut R) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Generator {
    order: usize,
    models: Vec<Model>
}

impl MarkovGenerator for Generator {
    fn new(data: &[String], order: usize, prior: f64, backoff: bool) -> Self {
        let mut alphabet = data.iter().map(|value| value.chars()).flatten().collect::<Vec<_>>();

        alphabet.sort();
        alphabet.dedup();

        let mut models = Vec::<Model>::new();

        if backoff {
            for index in 0..order {
                let model = Model::new(data, order - index, prior, &alphabet);

                models.push(model);
            }
        }
        else {
            let model = Model::new(data, order, prior, &alphabet);

            models.push(model);
        };

        Generator {
            order,
            models
        }
    }

    fn generate<R: Rng + Sized>(&self, rng: &mut R) -> String {
        let mut word = "#".repeat(self.order);

        loop {
            match self.get_letter(&word, rng) {
                Some(letter) if letter != "#" => word += &letter,
                _ => break
            };
        };

        word
    }
}

impl Generator {
    /// Generates the next letter in a word.
    fn get_letter<R: Rng + Sized>(&self, word: &String, rng: &mut R) -> Option<String> {
        if word.is_empty() { panic!("Word is empty"); }

        let length = word.len();
        let mut context = word[length - self.order..length].to_string();

        for model in self.models.iter() {
            match model.generate(&context, rng) {
                Some(value) if value != "#" => {
                    return Some(value);
                }
                _ => {
                    context = context[1..context.len()].to_string();
                }
            }
        }

        None
    }
}
