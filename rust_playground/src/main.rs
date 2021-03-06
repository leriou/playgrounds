mod backtrace;
mod dp;
mod leetcode;
mod unionfind;
use rand::prelude::*;
use time;

fn main() {
    let a = 0.1;
    let b = 0.2;
    println!("{:?}", a + b);

    println!("{}", get_rand());

    println!("{:?}", time::precise_time_s())
}


fn get_rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}