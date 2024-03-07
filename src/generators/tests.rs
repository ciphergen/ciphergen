use std::str::from_utf8;

use super::binary::{generate_bytes, generate_hex, generate_base64};
use super::password::generate_password;
use super::passphrase::generate_passphrase;
use super::username::{generate_simple_username, generate_complex_username};
use super::digits::generate_digits;
use super::number::generate_number;

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
fn generates_one_kilobyte() {
    let bytes = generate_bytes(1024);

    assert_eq!(bytes.len(), 1024)
}

#[test]
fn generates_zero_bytes() {
    let bytes = generate_bytes(0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_one_kilobyte_as_hex() {
    let bytes = generate_hex(false, 1024);
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 2048)
}

#[test]
fn generates_zero_bytes_as_hex() {
    let bytes = generate_hex(false, 0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_one_kilobyte_as_base64() {
    let bytes = generate_base64(false, 1024);
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 1368)
}

#[test]
fn generates_zero_bytes_as_base64() {
    let bytes = generate_base64(false, 0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_ten_thousand_character_password() {
    let character_set: Vec<char> = "!@*-_.0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let bytes = generate_password(&character_set, 10000);
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 10000)
}

#[test]
fn generates_empty_password() {
    let character_set: Vec<char> = "!@*-_.0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let bytes = generate_password(&character_set, 0);

    assert_eq!(bytes.len(), 0)
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

#[test]
fn generates_ten_thousand_character_simple_username() {
    let bytes = generate_simple_username(10000);
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 10000)
}

#[test]
fn generates_empty_simple_username() {
    let bytes = generate_simple_username(0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_ten_thousand_syllable_complex_username() {
    let bytes = generate_complex_username(10000);
    let string = from_utf8(&bytes).unwrap();
    let length = string.chars().count();
    let range = 20000..=30000;

    assert!(range.contains(&length), "expected a number in {:?}, but got {} instead", range, length);
}

#[test]
fn generates_empty_complex_username() {
    let bytes = generate_complex_username(0);

    assert_eq!(bytes.len(), 0)
}

#[test]
fn generates_ten_thousand_digits() {
    let bytes = generate_digits(10000);
    let string = from_utf8(&bytes).unwrap();

    assert_eq!(string.chars().count(), 10000)
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
