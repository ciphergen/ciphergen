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

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn generates_number() {
        let bytes = generate_number(0, 1024);
        let string = from_utf8(&bytes).unwrap();
        let number = u64::from_str_radix(string, 10).unwrap();

        assert!(number < 1024)
    }
}
