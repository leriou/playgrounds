mod backtrace;
mod dp;
mod leetcode;
mod middleware;
mod sicp;
mod unionfind;
mod utils;

fn main() {
    // println!("gcd(30,42) -> {:?}", sicp::gcd::gcd(30,42));
    println!("prime -> {:?}", sicp::prime::prime_question(10000).len());
}
