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

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    fn word_count(buffer: &Vec<u8>) -> usize {
        from_utf8(buffer)
            .unwrap()
            .split(' ')
            .count()
    }

    fn load_test_wordlist() -> Vec<String> {
        include_str!("../wordlist.txt").split('\n').map(|value| value.to_string()).collect()
    }

    #[test]
    fn generates_ten_thousand_word_passphrase() {
        let wordlist = load_test_wordlist();
        let bytes = generate_passphrase(&wordlist, &" ".to_string(), 10000);
        let count = word_count(&bytes);

        assert_eq!(count, 10000)
    }

    #[test]
    fn generates_hundred_thousand_word_passphrase() {
        let wordlist = load_test_wordlist();
        let bytes = generate_passphrase(&wordlist, &" ".to_string(), 100000);
        let count = word_count(&bytes);

        assert_eq!(count, 100000)
    }

    #[test]
    fn generates_empty_passphrase() {
        let wordlist = load_test_wordlist();
        let bytes = generate_passphrase(&wordlist, &" ".to_string(), 0);

        assert_eq!(bytes.len(), 0)
    }

    #[test]
    #[should_panic]
    fn empty_wordlist_panics() {
        let wordlist = Vec::<String>::new();

        generate_passphrase(&wordlist, &" ".to_string(), 1);
    }
}
