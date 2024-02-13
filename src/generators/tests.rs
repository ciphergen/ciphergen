use std::str::from_utf8;

use super::binary::{generate_bytes, generate_hex, generate_base64};
use super::password::generate_password;
use super::passphrase::generate_passphrase;
use super::username::{generate_simple_username, generate_syllabic_username};
use super::digits::generate_digits;
use super::number::generate_number;

fn load_test_wordlist() -> Vec<String> {
    include_str!("../wordlist.txt").split('\n').map(|value| value.to_string()).collect()
}

#[test]
fn can_generate_eight_random_bytes() {
    let bytes = generate_bytes(8).unwrap();

    assert_eq!(bytes.len(), 8)
}

#[test]
#[should_panic]
fn invalid_bytes_length_fails() {
    generate_bytes(0).unwrap();
}

#[test]
fn can_generate_eight_random_bytes_in_hex() {
    let bytes = generate_hex(false, 8).unwrap();
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 16)
}

#[test]
fn can_generate_eight_random_bytes_in_base64() {
    let bytes = generate_base64(false, 8).unwrap();
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 12)
}

#[test]
fn can_generate_eight_character_password() {
    let bytes = generate_password(8).unwrap();
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 8)
}

#[test]
#[should_panic]
fn invalid_password_length_fails() {
    generate_password(0).unwrap();
}

#[test]
fn can_generate_four_word_passphrase() {
    let wordlist = load_test_wordlist();
    let bytes = generate_passphrase(&wordlist, &" ".to_string(), 4).unwrap();
    let string = from_utf8(&bytes).unwrap();
    let count = string.split_whitespace().count();

    assert_eq!(count, 4)
}

#[test]
#[should_panic]
fn invalid_passphrase_length_fails() {
    let wordlist = load_test_wordlist();
    generate_passphrase(&wordlist, &" ".to_string(), 0).unwrap();
}

#[test]
fn can_generate_eight_character_simple_username() {
    let bytes = generate_simple_username(8).unwrap();
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 8)
}

#[test]
#[should_panic]
fn invalid_simple_username_length_fails() {
    generate_simple_username(0).unwrap();
}

#[test]
#[should_panic]
fn invalid_syllabic_username_length_fails() {
    generate_syllabic_username(0).unwrap();
}

#[test]
fn can_generate_six_digits() {
    let bytes = generate_digits(6).unwrap();
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 6)
}

#[test]
#[should_panic]
fn invalid_digits_length_fails() {
    generate_digits(0).unwrap();
}

#[test]
fn can_generate_number() {
    let bytes = generate_number(0, 1024).unwrap();
    let string = from_utf8(&bytes).unwrap();
    let number = u64::from_str_radix(string, 10).unwrap();

    assert!(number < 1024)
}
