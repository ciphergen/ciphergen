use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

/// Creates a simple pronounceable username by alternating random vowels and consonants.
/// Usernames created in this fashion are guaranteed to be pronouncable,
/// but may not be aesthetically pleasing.
///
/// # Arguments
///
/// * `length` - The length of the username
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
    let start = rng.gen_bool(1.0 / 2.0);

    for index in 0..*length {
        // Randomly choose between starting with a vowel or a consonant.
        if start {
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
