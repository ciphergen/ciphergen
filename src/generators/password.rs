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

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn generates_ten_thousand_character_password() {
        let character_set: Vec<char> = "!@*-_.0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let bytes = generate_password(&character_set, 10000);
        let string = from_utf8(&bytes).unwrap();

        assert_eq!(string.chars().count(), 10000)
    }

    #[test]
    fn generates_empty_password() {
        let character_set: Vec<char> = "!@*-_.0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let bytes = generate_password(&character_set, 0);

        assert_eq!(bytes.len(), 0)
    }
}
