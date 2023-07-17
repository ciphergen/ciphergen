use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

use super::SecretKeyLength;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const CONSONANTS: [char; 20] = [
    'b', 'c', 'd', 'f', 'g',
    'h', 'j', 'k', 'l', 'm',
    'n', 'p', 'r', 's', 't',
    'v', 'w', 'x', 'y', 'z'
];

fn choose_random_vowel(rng: &mut ThreadRng) -> &char {
    VOWELS.choose(rng).expect("Failed to choose a random vowel")
}

fn choose_random_consonant(rng: &mut ThreadRng) -> &char {
    CONSONANTS.choose(rng).expect("Failed to choose a random consonant")
}

fn add_vowel(input: &mut Vec<char>, rng: &mut ThreadRng) {
    let value = choose_random_vowel(rng);

    input.push(*value);
}

fn add_consonant(input: &mut Vec<char>, rng: &mut ThreadRng) {
    let value = choose_random_consonant(rng);

    input.push(*value);
}

/// Generate a simple pronounceable username by alternating random vowels and consonants.
///
/// Usernames created in this fashion are guaranteed to be pronouncable,
/// but are likely to be flagged as suspicious by automated tools and may not be aesthetically pleasing.
pub fn generate_simple_username(length: &SecretKeyLength) -> Result<String, String> {
    if length <= &0 {
        return Err(
            format!("Cannot generate a username of length {}", length)
        );
    }

    let mut output: Vec<char> = Vec::new();
    let rng = &mut thread_rng();
    let start = rng.gen_bool(1.0 / 2.0);

    for index in 0..*length {
        // Flip a coin to choose between starting with a vowel or a consonant.
        if start {
            // If the current index is even.
            if index % 2 == 0 {
                add_consonant(&mut output, rng);
            }
            else {
                add_vowel(&mut output, rng);
            }
        }
        // If the current index is odd.
        else if index % 2 != 0 {
            add_vowel(&mut output, rng);
        }
        else {
            add_consonant(&mut output, rng);
        }
    }

    let value = output.iter().collect();

    Ok(value)
}

fn create_closed_syllable(rng: &mut ThreadRng) -> String {
    let syllable = vec![
        *choose_random_consonant(rng),
        *choose_random_vowel(rng),
        *choose_random_consonant(rng)
    ];

    syllable.iter().collect()
}

/// Generate a pronounceable username from random syllables.
///
/// Syllabic usernames are less likely to be flagged as suspicious by automated tools,
/// and may be more aesthetically pleasing.
pub fn generate_syllabic_username(count: &SecretKeyLength) -> Result<String, String> {
    if count <= &0 {
        return Err(
            format!("Cannot generate a username with {} syllables", count)
        );
    }

    let rng = &mut thread_rng();

    let mut output = String::new();
    let limit = (*count * 2) - 1;

    for index in 0..limit {
        // If the number of syllables is greater than 1,
        // insert a random vowel between every syllable.
        if *count > 1 && index % 2 != 0 {
            output.push(
                *choose_random_vowel(rng)
            );
            continue
        }
        output.push_str(
            create_closed_syllable(rng).as_str()
        );
    }

    Ok(output)
}
