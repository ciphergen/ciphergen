use rand::{thread_rng, Rng};

pub fn generate_number(minimum: u64, maximum: u64) -> Vec<u8> {
    thread_rng()
        .gen_range(minimum..maximum)
        .to_string()
        .into_bytes()
}
