use rand::{thread_rng, Rng};

pub fn generate_number(minimum: &u64, maximum: &u64) -> Result<Vec<u8>, ()> {
    let range = *minimum..*maximum;
    let mut rng = thread_rng();
    let number = rng.gen_range(range).to_string().into_bytes();

    Ok(number)
}
