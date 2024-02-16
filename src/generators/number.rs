use rand::{thread_rng, Rng};

pub fn generate_number(minimum: u64, maximum: u64) -> Vec<u8> {
    let range = minimum..maximum;
    let mut rng = thread_rng();

    rng.gen_range(range).to_string().into_bytes()
}
