use base64::Engine;
use rand::{Rng, thread_rng, distributions::Uniform};
use hex::{encode, encode_upper};
use base64::engine::general_purpose::{STANDARD, URL_SAFE};

pub fn generate_bytes(length: usize) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let range = Uniform::new_inclusive(u8::MIN, u8::MAX);

    thread_rng()
        .sample_iter(range)
        .take(length)
        .collect()
}

pub fn generate_hex(uppercase: bool, length: usize) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let bytes = generate_bytes(length);

    if uppercase { encode_upper(bytes).into_bytes() }
    else { encode(bytes).into_bytes() }
}

pub fn generate_base64(url_safe: bool, length: usize) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let bytes = generate_bytes(length);

    if url_safe { URL_SAFE.encode(bytes).into_bytes() }
    else { STANDARD.encode(bytes).into_bytes() }
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

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
}
