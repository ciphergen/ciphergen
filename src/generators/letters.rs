use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
const CONSONANTS: [char; 20] = [
    'b', 'c', 'd', 'f', 'g',
    'h', 'j', 'k', 'l', 'm',
    'n', 'p', 'q', 'r', 's',
    't', 'v', 'w', 'x', 'z'
];

pub fn choose_random_vowel(rng: &mut ThreadRng) -> &char {
    VOWELS.choose(rng).unwrap()
}

pub fn choose_random_consonant(rng: &mut ThreadRng) -> &char {
    CONSONANTS.choose(rng).unwrap()
}

pub fn add_vowel(input: &mut Vec<char>, rng: &mut ThreadRng) {
    let value = choose_random_vowel(rng);

    input.push(*value);
}

pub fn add_consonant(input: &mut Vec<char>, rng: &mut ThreadRng) {
    let value = choose_random_consonant(rng);

    input.push(*value);
}
