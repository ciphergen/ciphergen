use rand::Rng;
use rand::distributions::{Alphanumeric, Uniform, Standard, DistString};
use rand::seq::{IteratorRandom, SliceRandom};

fn to_hex_string(bytes: Vec<u8>) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join("")
}

pub fn generate_binary(length: &u16) -> Vec<u8> {
    let range = Uniform::new(u8::MIN, u8::MAX);

    rand::thread_rng()
        .sample_iter(range)
        .take(*length as usize)
        .collect()
}

pub fn generate_hexadecimal(length: &u16) -> String {
    let bytes = generate_binary(length);

    to_hex_string(bytes)
}

pub fn generate_password(expanded: &bool, length: &u16) -> String {
    if *expanded {
        rand::thread_rng()
            .sample_iter::<char, Standard>(Standard)
            .take(*length as usize)
            .map(char::from)
            .collect()
    }
    else {
        Alphanumeric.sample_string(
            &mut rand::thread_rng(),
            *length as usize
        )
    }
}

pub fn generate_passphrase(length: &u16) -> String {
    let wordlist = include_str!("wordlist.txt").to_string();

    wordlist
        .split('\n')
        .map(|value| value.to_string())
        .choose_multiple(&mut rand::thread_rng(), *length as usize)
        .join(" ")
}

pub fn generate_username(length: &u16) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants = [
        'b', 'c', 'd', 'f', 'g',
        'h', 'j', 'k', 'l', 'm',
        'n', 'p', 'r', 's', 't',
        'v', 'w', 'x', 'y', 'z'
    ];
    let mut characters: Vec<char> = Vec::new();
    let rng = &mut rand::thread_rng();

    for _ in 0..*length {
        if rng.gen_bool(1.0 / 2.0) {
            characters.push(
                *vowels.choose(rng).unwrap()
            );
        }
        else {
            characters.push(
                *consonants.choose(rng).unwrap()
            );
        }
    }

    characters.iter().collect()
}
