use rand::{self, Rng};

pub fn generate_random_value(f_max_value: usize) -> usize {
    return rand::thread_rng().gen_range(0..f_max_value);
}