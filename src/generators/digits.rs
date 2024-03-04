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
