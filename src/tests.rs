#[cfg(test)]
mod generate_tests {
    use crate::generate::{generate_binary, generate_hexadecimal, generate_password, generate_passphrase};

    #[test]
    fn can_generate_eight_random_bytes() {
        let bytes = generate_binary(&8);

        assert_eq!(
            bytes.len(),
            8
        )
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
}
