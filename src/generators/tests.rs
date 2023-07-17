#[allow(unused_imports)]
use super::{
    generate_binary,
    generate_hexadecimal,
    generate_base64,
    generate_password,
    generate_passphrase,
    generate_simple_username,
    generate_syllabic_username,
    generate_pin
};

#[test]
fn can_generate_eight_random_bytes() {
    let bytes = generate_binary(&8).unwrap();

    assert_eq!(
        bytes.len(),
        8
    )
}

#[test]
#[should_panic]
fn invalid_bytes_length_fails() {
    generate_binary(&0).unwrap();
}

#[test]
fn can_generate_eight_random_bytes_in_hex() {
    let hex = generate_hexadecimal(&8).unwrap();

    assert_eq!(
        hex.chars().count(),
        16
    )
}

#[test]
fn can_generate_eight_random_bytes_in_base64() {
    let base64 = generate_base64(&8).unwrap();

    assert_eq!(
        base64.chars().count(),
        12
    )
}

#[test]
fn can_generate_eight_character_password() {
    let password = generate_password(&true, &8).unwrap();

    assert_eq!(
        password.chars().count(),
        8
    )
}

#[test]
#[should_panic]
fn invalid_password_length_fails() {
    generate_password(&false, &0).unwrap();
}

#[test]
fn can_generate_eight_character_ascii_password() {
    let password = generate_password(&false, &8).unwrap();

    assert_eq!(
        password.chars().count(),
        8
    )
}

#[test]
fn can_generate_four_word_passphrase() {
    let passphrase = generate_passphrase(&4).unwrap();
    let count = passphrase.split_whitespace().count();

    assert_eq!(
        count,
        4
    )
}

#[test]
#[should_panic]
fn invalid_passphrase_length_fails() {
    generate_passphrase(&0).unwrap();
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
fn can_generate_six_digit_pin() {
    let pin = generate_pin(&6).unwrap();

    println!("{}", pin);
    assert_eq!(
        pin.to_string().chars().count(),
        6
    )
}

#[test]
#[should_panic]
fn invalid_pin_length_fails() {
    generate_pin(&0).unwrap();
}
