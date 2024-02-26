use rand::{thread_rng, Rng, distributions::Slice};

fn split_to_characters(character_set: &str) -> Vec<char> {
    character_set.chars().collect()
}

pub fn generate_password(numbers: bool, symbols: bool, length: usize) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let characters = if numbers && symbols {
        split_to_characters("!@*-_.0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }
    else if numbers {
        split_to_characters("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }
    else if symbols {
        split_to_characters("!@*-_.abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }
    else {
        split_to_characters("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
    };

    let distribution = Slice::new(&characters).unwrap();

    thread_rng()
        .sample_iter(distribution)
        .take(length)
        .collect::<String>()
        .into_bytes()
}
