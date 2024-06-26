use rand::distributions::{DistString, Distribution, Uniform, Alphanumeric};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use std::vec::Vec;

static CHAR_COLLECTION: &'static [char] = &[
    // Lowercase letters
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',

    // Uppercase letters
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',

    // Numbers
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',

    // Special characters
    '!', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.',
    '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_',
    '`', '{', '|', '}', '~'
];

pub struct PasswordDist;

pub trait Sampleable<T, U> {
    const LOWER_RANGE: &'static usize = &0;
    fn sample(&self, num_samples: usize) -> U {
        let mut element_list: Vec<T> = Vec::new();
        while element_list.len() < num_samples {
            element_list.push(Self::sample_from_pool());
        }

        let result: U = Self::concatenate(element_list);

        return result;
    }
    fn sample_from_pool() -> T;
    fn concatenate(elements: Vec<T>) -> U;
    fn pool() -> &'static [T];
}

impl Sampleable<char, String> for PasswordDist {
    fn pool() -> &'static [char] {
        return CHAR_COLLECTION;
    }

    fn sample_from_pool() -> char {
        let range: usize = Self::pool().len();
        let mut seed: ThreadRng = thread_rng();
        let sampler: Uniform<usize> = Uniform::new(Self::LOWER_RANGE, range);
        let sample: usize = sampler.sample(&mut seed);
        let new_element: char = Self::pool()[sample];

        return new_element;
    }

    fn concatenate(elements: Vec<char>) -> String {
        return elements.into_iter().collect();
    }

}

pub fn create_simple_password(pw_length: usize) -> String {
    let mut seed = thread_rng();
    let random_string = Alphanumeric.sample_string(&mut seed, pw_length);

    return random_string;
}
