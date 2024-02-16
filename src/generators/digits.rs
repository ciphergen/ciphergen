use rand::{thread_rng, Rng};

pub fn generate_digits(length: u64) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let mut output = String::new();

    for _ in 0..length {
        let digit = thread_rng().gen_range(0..=9);
        let character = char::from_digit(digit, 10).unwrap();

        output.push(character);
    }

    output.into_bytes()
}
