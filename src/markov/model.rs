use std::{collections::HashMap, fmt::Display};

use hex::encode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

pub trait MarkovModel {
    /// Creates a new Markov model.
    fn new(data: &[String], order: usize, prior: f64, alphabet: &[char]) -> Self;

    /// Retrains the model on the newly supplied data, regenerating the Markov chains.
    #[allow(dead_code)]
    fn retrain(&mut self);

    /// Attempts to generate the next letter in the word given the `context` (the previous "order" letters).
    fn generate<R: Rng + Sized>(&self, context: &String, rng: &mut R) -> Option<String>;

    /// Calculates the CID of this model.
    fn calculate_checksum(&self) -> Vec<u8>;
}

trait IntoBytes {
    fn into_bytes(&self) -> Vec<u8>;
}

impl IntoBytes for Vec<String> {
    fn into_bytes(&self) -> Vec<u8> {
        self.iter()
            .map(|value| value.to_owned().into_bytes())
            .flatten()
            .collect::<Vec<_>>()
    }
}

impl IntoBytes for Vec<char> {
    fn into_bytes(&self) -> Vec<u8> {
        self.iter()
            .collect::<String>()
            .into_bytes()
    }
}

impl IntoBytes for HashMap<String, Vec<String>> {
    fn into_bytes(&self) -> Vec<u8> {
        self.iter()
            .map(|(key, value)| {
                let mut buffer = Vec::<u8>::new();
                let mut key = key.to_owned().into_bytes();
                let mut value = value.iter()
                    .map(|value| value.to_owned().into_bytes())
                    .flatten()
                    .collect::<Vec<_>>();

                buffer.append(&mut key);
                buffer.append(&mut value);

                buffer
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}

impl IntoBytes for HashMap<String, Vec<f64>> {
    fn into_bytes(&self) -> Vec<u8> {
        self.iter()
            .map(|(key, value)| {
                let mut buffer = Vec::<u8>::new();
                let mut key = key.to_owned().into_bytes();
                let mut value = value.iter()
                    .map(|value| value.to_le_bytes())
                    .flatten()
                    .collect::<Vec<_>>();

                    buffer.append(&mut key);
                    buffer.append(&mut value);

                buffer
            })
            .flatten()
            .collect::<Vec<_>>()
    }
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

impl Display for Model {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cid = encode(self.calculate_checksum());
        let order = self.order.to_string();
        let prior = self.prior.to_string();
        let alphabet = self.alphabet.len().to_string();
        let observations = self.observations.len().to_string();
        let chains = self.chains.len().to_string();

        write!(
            formatter,
            "\tModel {{ \
            \n\t\t\tCID: {cid} \
            \n\t\t\tOrder: {order} \
            \n\t\t\tPrior: {prior} \
            \n\t\t\tAlphabet: {alphabet} \
            \n\t\t\tObservations: {observations} \
            \n\t\t\tChains: {chains} \
            \n\t\t}}"
        )
    }
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

    fn calculate_checksum(&self) -> Vec<u8> {
        let mut digest = Sha256::new();
        let data = self.data.into_bytes();
        let order = self.order.to_le_bytes();
        let prior = self.prior.to_le_bytes();
        let alphabet = self.alphabet.into_bytes();
        let observations = self.observations.into_bytes();
        let chains = self.chains.into_bytes();

        digest.update(data);
        digest.update(order);
        digest.update(prior);
        digest.update(alphabet);
        digest.update(observations);
        digest.update(chains);

        digest.finalize().to_vec()
    }
}

impl Model {
    fn train(&mut self) {
        let mut data = self.data.clone();

        while let Some(item) = data.pop() {
            let mut value = item.clone();

            value = "#".to_string().repeat(self.order) + &value + "#";

            for index in 0..value.len() - self.order {
                let key = value[index..index + self.order].to_owned();
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
