use rand::{distributions::Slice, thread_rng, Rng};

/// Generate a passphrase.
pub fn generate_passphrase(wordlist: &[String], separator: &str, length: usize) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let distribution = Slice::new(wordlist).unwrap();

    thread_rng()
        .sample_iter(distribution)
        .take(length)
        .map(|value| value.to_owned())
        .collect::<Vec<String>>()
        .join(separator)
        .into_bytes()
}
