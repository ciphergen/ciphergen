use rand::{Rng, thread_rng};

pub fn generate_digits(length: u64) -> Vec<u8> {
    if length == 0 { return Vec::<u8>::new(); }

    let radix = 10.0f64;
    let offset = length - 1;
    let maximum = radix.powi(length as i32) as u64;
    let minimum = radix.powi(offset as i32) as u64;

    thread_rng().gen_range(minimum..maximum).to_string().into_bytes()
}
