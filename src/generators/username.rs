use std::fmt;
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use super::letters::{add_consonant, add_vowel, choose_random_consonant, choose_random_vowel};

#[derive(Debug)]
pub enum GenerateUsernameError {
    InvalidLength(u64)
}

impl fmt::Display for GenerateUsernameError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GenerateUsernameError::InvalidLength(length) => {
                write!(formatter, "expected a positive integer but got {} instead", length)
            }
        }
    }
}

impl std::error::Error for GenerateUsernameError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            GenerateUsernameError::InvalidLength(_) => None
        }
    }
}

/// Generate a simple pronounceable username by alternating random vowels and consonants.
///
/// Usernames created in this fashion are guaranteed to be pronouncable,
/// but are likely to be flagged as suspicious by automated tools and may not be aesthetically pleasing.
pub fn generate_simple_username(length: &u64) -> Result<String, GenerateUsernameError> {
    if length <= &0 {
        return Err(
            GenerateUsernameError::InvalidLength(*length)
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
pub fn generate_syllabic_username(count: &u64) -> Result<String, GenerateUsernameError> {
    if count <= &0 {
        return Err(
            GenerateUsernameError::InvalidLength(*count)
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
