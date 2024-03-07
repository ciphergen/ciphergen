use rand::{thread_rng, Rng, distributions::Slice};

pub fn generate_password(character_set: &[char], length: usize) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let distribution = Slice::new(character_set).unwrap();

    thread_rng()
        .sample_iter(distribution)
        .take(length)
        .collect::<String>()
        .into_bytes()
}
