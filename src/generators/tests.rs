#[allow(unused_imports)]
use super::binary::{generate_bytes, generate_hex, generate_base64};
#[allow(unused_imports)]
use super::password::{generate_ascii_password, generate_full_password};
#[allow(unused_imports)]
use super::passphrase::generate_passphrase;
#[allow(unused_imports)]
use super::username::{generate_simple_username, generate_syllabic_username};
#[allow(unused_imports)]
use super::digits::generate_digits;
#[allow(unused_imports)]
use super::number::generate_number;

#[test]
fn can_generate_eight_random_bytes() {
    let bytes = generate_bytes(&8).unwrap();

    assert_eq!(
        bytes.len(),
        8
    )
}

#[test]
#[should_panic]
fn invalid_bytes_length_fails() {
    generate_bytes(&0).unwrap();
}

#[test]
fn can_generate_eight_random_bytes_in_hex() {
    let hex = generate_hex(&false, &8).unwrap();

    assert_eq!(
        hex.chars().count(),
        16
    )
}

#[test]
fn can_generate_eight_random_bytes_in_base64() {
    let base64 = generate_base64(&false, &8).unwrap();

    assert_eq!(
        base64.chars().count(),
        12
    )
}

#[test]
fn can_generate_eight_character_ascii_password() {
    let password = generate_ascii_password(&8).unwrap();

    assert_eq!(
        password.chars().count(),
        8
    )
}

#[test]
fn can_generate_eight_character_full_password() {
    let password = generate_full_password(&8).unwrap();

    assert_eq!(
        password.chars().count(),
        8
    )
}

#[test]
#[should_panic]
fn invalid_ascii_password_length_fails() {
    generate_ascii_password(&0).unwrap();
}

#[test]
#[should_panic]
fn invalid_full_password_length_fails() {
    generate_full_password(&0).unwrap();
}

#[test]
fn can_generate_four_word_passphrase() {
    let passphrase = generate_passphrase(&4, &Option::None, &Option::None).unwrap();
    let count = passphrase.split_whitespace().count();

    assert_eq!(
        count,
        4
    )
}

#[test]
#[should_panic]
fn invalid_passphrase_length_fails() {
    generate_passphrase(&0, &Option::None, &Option::None).unwrap();
}

#[test]
fn can_generate_eight_character_simple_username() {
    let username = generate_simple_username(&8).unwrap();

    assert_eq!(
        username.chars().count(),
        8
    )
}

#[test]
#[should_panic]
fn invalid_simple_username_length_fails() {
    generate_simple_username(&0).unwrap();
}

#[test]
#[should_panic]
fn invalid_syllabic_username_length_fails() {
    generate_syllabic_username(&0).unwrap();
}

#[test]
fn can_generate_six_digits() {
    let digits = generate_digits(&6).unwrap();

    assert_eq!(
        digits.to_string().chars().count(),
        6
    )
}

#[test]
#[should_panic]
fn invalid_digits_length_fails() {
    generate_digits(&0).unwrap();
}

#[test]
fn can_generate_number() {
    let number = generate_number(&0, &1024).unwrap();

    assert!(
        number < 1024
    )
}
