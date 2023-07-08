use rand::{Rng, thread_rng};
use rand::distributions::{Alphanumeric, Standard, DistString};

pub fn generate_password(expanded: &bool, length: &u16) -> String {
    if length <= &0 {
        panic!("Cannot generate a password of length 0");
    }

    if *expanded {
        thread_rng()
            .sample_iter::<char, Standard>(Standard)
            .take(*length as usize)
            .map(char::from)
            .collect()
    }
    else {
        Alphanumeric.sample_string(
            &mut thread_rng(),
            *length as usize
        )
    }
}
