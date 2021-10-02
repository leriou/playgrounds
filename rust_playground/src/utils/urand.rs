use rand::prelude::*;

pub fn get_rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn get_rand_i32() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen::<i32>()
}