use rand::{Rng, thread_rng};
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
    if length <= &0 {
        panic!("Cannot generate a secret key of length 0");
    }

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
    if length <= &0 {
        panic!("Cannot generate a password of length 0");
    }

    if *expanded {
        thread_rng()
            .sample_iter::<char, Standard>(Standard)
            .take(*length as usize)
            .map(char::from)
            .collect()
    }
    else {
        Alphanumeric.sample_string(
            &mut thread_rng(),
            *length as usize
        )
    }
}

pub fn generate_passphrase(length: &u16) -> String {
    if length <= &0 {
        panic!("Cannot generate a passphrase of length 0");
    }

    let wordlist = include_str!("wordlist.txt").to_string();

    wordlist
        .split('\n')
        .map(|value| value.to_string())
        .choose_multiple(&mut thread_rng(), *length as usize)
        .join(" ")
}

pub fn generate_username(length: &u16) -> String {
    if length <= &0 {
        panic!("Cannot generate a username of length 0");
    }

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants = [
        'b', 'c', 'd', 'f', 'g',
        'h', 'j', 'k', 'l', 'm',
        'n', 'p', 'r', 's', 't',
        'v', 'w', 'x', 'y', 'z'
    ];
    let mut characters: Vec<char> = Vec::new();
    let rng = &mut thread_rng();
    let switch = rng.gen_bool(1.0 / 2.0);

    for index in 0..*length {
        if switch {
            if index % 2 == 0 {
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
        else if index % 2 != 0 {
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

pub fn generate_pin(length: &u16) -> u64 {
    if length <= &0 {
        panic!("Cannot generate a PIN of length 0");
    }

    let digits = 10.0f32.powi(*length as i32) as u64;

    thread_rng().gen_range(0..digits + 1)
}
