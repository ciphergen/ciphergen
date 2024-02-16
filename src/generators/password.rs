use rand::thread_rng;
use rand::distributions::{Alphanumeric, DistString};

pub fn generate_password(length: u64) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    Alphanumeric.sample_string(&mut thread_rng(), length as usize).into_bytes()
}
