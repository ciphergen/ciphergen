use rand::{distributions::Uniform, thread_rng, Rng};

pub fn generate_digits(length: usize) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let distribution = Uniform::new_inclusive(0, 9);

    thread_rng()
        .sample_iter(distribution)
        .take(length)
        .map(|value| char::from_digit(value, 10).unwrap())
        .collect::<String>()
        .into_bytes()
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn generates_ten_thousand_digits() {
        let bytes = generate_digits(10000);
        let string = from_utf8(&bytes).unwrap();

        assert_eq!(string.chars().count(), 10000)
    }


    #[test]
    fn generates_zero_digits() {
        let bytes = generate_digits(0);

        assert_eq!(bytes.len(), 0)
    }
}
