#[allow(unused_imports)]
use super::{
    generate_binary,
    generate_hexadecimal,
    generate_password,
    generate_passphrase,
    generate_username,
    generate_pin
};

#[test]
fn can_generate_eight_random_bytes() {
    let bytes = generate_binary(&8);

    assert_eq!(
        bytes.len(),
        8
    )
}

#[test]
#[should_panic]
fn invalid_bytes_length_fails() {
    generate_binary(&0);
}

#[test]
fn can_generate_eight_random_bytes_in_hex() {
    let hex = generate_hexadecimal(&8);

    assert_eq!(
        hex.chars().count(),
        16
    )
}

#[test]
fn can_generate_eight_character_password() {
    let password = generate_password(&true, &8);

    assert_eq!(
        password.chars().count(),
        8
    )
}

#[test]
#[should_panic]
fn invalid_password_length_fails() {
    generate_password(&false, &0);
}

#[test]
fn can_generate_eight_character_ascii_password() {
    let password = generate_password(&false, &8);

    assert_eq!(
        password.chars().count(),
        8
    )
}

#[test]
fn can_generate_four_word_passphrase() {
    let passphrase = generate_passphrase(&4);
    let count = passphrase.split_whitespace().count();

    assert_eq!(
        count,
        4
    )
}

#[test]
#[should_panic]
fn invalid_passphrase_length_fails() {
    generate_passphrase(&0);
}

#[test]
fn can_generate_eight_character_username() {
    let username = generate_username(&8);

    assert_eq!(
        username.chars().count(),
        8
    )
}

#[test]
#[should_panic]
fn invalid_username_length_fails() {
    generate_username(&0);
}

#[test]
fn can_generate_six_digit_pin() {
    let pin = generate_pin(&6);

    println!("{}", pin);
    assert_eq!(
        pin.to_string().chars().count(),
        6
    )
}

#[test]
#[should_panic]
fn invalid_pin_length_fails() {
    generate_pin(&0);
}
