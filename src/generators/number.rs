use std::process::exit;

use log::error;
use rand::{thread_rng, Rng};

pub fn generate_number(minimum: usize, maximum: usize) -> Vec<u8> {
    if minimum >= maximum {
        error!("{} is greater than or equal to {}", minimum, maximum);
        exit(-1);
    }

    let range = minimum..=maximum;

    thread_rng()
        .gen_range(range)
        .to_string()
        .into_bytes()
}
