use rand::{Rng, thread_rng};

pub fn generate_pin(length: &u16) -> u64 {
    if length <= &0 {
        panic!("Cannot generate a PIN of length 0");
    }

    let radix = 10.0f64;
    let offset = length - 1;
    let maximum = radix.powi(*length as i32) as u64;
    let minimum = radix.powi(offset as i32) as u64;

    thread_rng().gen_range(minimum..maximum)
}
