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
