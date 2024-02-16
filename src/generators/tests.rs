use std::str::from_utf8;

use super::binary::{generate_bytes, generate_hex, generate_base64};
use super::password::generate_password;
use super::passphrase::generate_passphrase;
use super::username::{generate_simple_username, generate_complex_username};
use super::digits::generate_digits;
use super::number::generate_number;

fn load_test_wordlist() -> Vec<String> {
    include_str!("../wordlist.txt").split('\n').map(|value| value.to_string()).collect()
}

#[test]
fn generates_eight_bytes() {
    let bytes = generate_bytes(8);

    assert_eq!(bytes.len(), 8)
}

#[test]
fn generates_zero_bytes() {
    let bytes = generate_bytes(0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_eight_bytes_as_hex() {
    let bytes = generate_hex(false, 8);
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 16)
}

#[test]
fn generates_zero_bytes_as_hex() {
    let bytes = generate_hex(false, 0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_eight_bytes_as_base64() {
    let bytes = generate_base64(false, 8);
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 12)
}

#[test]
fn generates_zero_bytes_as_base64() {
    let bytes = generate_base64(false, 0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_eight_character_password() {
    let bytes = generate_password(8);
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 8)
}

#[test]
fn generates_empty_password() {
    let bytes = generate_password(0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generate_four_word_passphrase() {
    let wordlist = load_test_wordlist();
    let bytes = generate_passphrase(&wordlist, &" ".to_string(), 4).unwrap();
    let string = from_utf8(&bytes).unwrap();
    let count = string.split_whitespace().count();

    assert_eq!(count, 4)
}

#[test]
fn generates_empty_passphrase() {
    let wordlist = load_test_wordlist();
    let bytes = generate_passphrase(&wordlist, &" ".to_string(), 0).unwrap();

    assert_eq!(bytes.len(), 0)
}

#[test]
#[should_panic]
fn empty_wordlist_panics() {
    let wordlist = Vec::<String>::new();

    generate_passphrase(&wordlist, &" ".to_string(), 1).unwrap();
}

#[test]
fn generates_eight_character_simple_username() {
    let bytes = generate_simple_username(8);
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 8)
}

#[test]
fn generates_empty_simple_username() {
    let bytes = generate_simple_username(0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_two_syllable_complex_username() {
    let bytes = generate_complex_username(2);
    let string = from_utf8(&bytes).unwrap();
    let length = string.chars().count();
    let range = 4..=6;

    assert!(range.contains(&length), "expected a number in {:?}, but got {} instead", range, length);
}

#[test]
fn generates_empty_complex_username() {
    let bytes = generate_complex_username(0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_six_digits() {
    let bytes = generate_digits(6);
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 6)
}

#[test]
fn generates_zero_digits() {
    let bytes = generate_digits(0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_number() {
    let bytes = generate_number(0, 1024);
    let string = from_utf8(&bytes).unwrap();
    let number = u64::from_str_radix(string, 10).unwrap();

    assert!(number < 1024)
}
