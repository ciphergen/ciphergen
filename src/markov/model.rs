use std::collections::HashMap;

use log::trace;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Sha512, Digest};

pub trait MarkovModel {
    /// Creates a new Markov model.
    fn new(data: &[String], order: usize, prior: f64, alphabet: &[char]) -> Self;

    /// Retrains the model on the newly supplied data, regenerating the Markov chains.
    #[allow(dead_code)]
    fn retrain(&mut self);

    /// Attempts to generate the next letter in the word given the `context` (the previous "order" letters).
    fn generate<R: Rng + Sized>(&self, context: &String, rng: &mut R) -> Option<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    data: Vec<String>,
    order: usize,
    prior: f64,
    alphabet: Vec<char>,
    observations: HashMap<String, Vec<String>>,
    chains: HashMap<String, Vec<f64>>
}

impl MarkovModel for Model {
    fn new(data: &[String], order: usize, prior: f64, alphabet: &[char]) -> Self {
        let mut model = Model {
            data: data.to_vec(),
            order,
            prior,
            alphabet: alphabet.to_vec(),
            observations: HashMap::new(),
            chains: HashMap::new()
        };

        model.train();
        model.build_chains();

        let report = (model.order, model.prior, model.alphabet.clone(), model.observations.clone());

        trace!("{:#?}", report);

        model
    }

    fn retrain(&mut self) {
        self.train();
        self.build_chains();
    }

    fn generate<R: Rng + Sized>(&self, context: &String, rng: &mut R) -> Option<String> {
        match self.chains.get(context) {
            Some(chain) => {
                let index = select_index(chain, rng);
                let choice = self.alphabet[index].to_string();

                Some(choice)
            },
            None => None
        }
    }
}

impl Model {
    fn train(&mut self) {
        let mut data = self.data.clone();

        while let Some(item) = data.pop() {
            let mut value = item.clone();

            value = "#".to_string().repeat(self.order) + &value + "#";

            for index in 0..value.len() - self.order {
                let key = value[index..index + self.order].to_string();
                let character = value.chars().nth(index + self.order).unwrap().to_string();

                match self.observations.get_mut(&key) {
                    Some(observation) => {
                        observation.push(character);
                    },
                    None => {
                        self.observations.insert(key, vec![character]);
                    }
                };
            }
        }
    }

    fn build_chains(&mut self) {
        for context in self.observations.keys() {
            for prediction in self.alphabet.iter() {
                let count = count_matches(self.observations.get(context).unwrap(), &prediction.to_string());
                let chain = self.prior + count;

                match self.chains.get_mut(context) {
                    Some(value) => {
                        value.push(chain);
                    },
                    None => {
                        self.chains.insert(context.to_string(), vec![chain]);
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn calculate_checksum(&self) -> Vec<u8> {
        let mut digest = Sha512::new();
        let data = self.data.iter()
            .map(|value| value.to_owned().into_bytes())
            .flatten()
            .collect::<Vec<_>>();
        let order = self.order.to_le_bytes();
        let prior = self.prior.to_le_bytes();

        digest.update(data);
        digest.update(order);
        digest.update(prior);

        digest.finalize().to_vec()
    }
}

fn count_matches(observation: &Vec<String>, prediction: &String) -> f64 {
    let mut index: f64 = 0.0;

    for value in observation {
        if value == prediction {
            index += 1.0;
        }
    }

    index
}

fn select_index<R: Rng + Sized>(chain: &Vec<f64>, rng: &mut R) -> usize {
    let mut totals = Vec::<f64>::new();
    let mut accumulator: f64 = 0.0;

    for weight in chain {
        accumulator += weight;
        totals.push(accumulator);
    }

    let random = rng.gen::<f64>() * accumulator;

    for index in 0..totals.len() {
        if random < totals[index] {
            return index;
        }
    }

    0
}
