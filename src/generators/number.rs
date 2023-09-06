use rand::{thread_rng, Rng};

pub fn generate_number(minimum: &u64, maximum: &u64) -> Result<u64, ()> {
    let range = *minimum..*maximum;
    let mut rng = thread_rng();
    let number = rng.gen_range(range);

    Ok(number)
}
