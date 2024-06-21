use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;

pub fn create_password(pw_length: usize) -> String {
    let mut seed = thread_rng();
    let random_string = Alphanumeric.sample_string(&mut seed, pw_length);

    return random_string;
}
