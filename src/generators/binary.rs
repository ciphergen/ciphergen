use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

fn to_hex_string(bytes: Vec<u8>) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join("")
}

pub fn generate_binary(length: &u16) -> Vec<u8> {
    if length <= &0 {
        panic!("Cannot generate a secret key of length 0");
    }

    let range = Uniform::new(u8::MIN, u8::MAX);

    thread_rng()
        .sample_iter(range)
        .take(*length as usize)
        .collect()
}

pub fn generate_hexadecimal(length: &u16) -> String {
    let bytes = generate_binary(length);

    to_hex_string(bytes)
}
