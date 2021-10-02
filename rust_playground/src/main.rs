mod backtrace;
mod dp;
mod leetcode;
mod unionfind;
mod middleware;
mod utils;

fn main() {
    // println!("{:?}", utils::urand::get_rand());
    // println!("duration -> {:?}", utils::utime::now_unix_timestamp());
    // middleware::mg::test();
    // middleware::kafka::producer();
    // middleware::kafka::consumer();


    // let ans = backtrace::nqueen::Solution::solve_n_queens(8);
    // println!("{:?}", ans);


    let nums = 1000000i32;
    let mut arr:Vec<i32> = vec![];
    
    let mut arr2:Vec<i32> = vec![];
    let s1 = utils::utime::now_nanos();
    for _ in 0..nums {
        let t = utils::urand::get_rand_i32();
        arr.push(t);
        arr2.push(t);
    }

    let s2 = utils::utime::now_nanos();

    arr.sort();
    let s3 = utils::utime::now_nanos();

    arr2.sort_unstable();
    let s4 = utils::utime::now_nanos();

    println!("s2-s1 {:?}", (s2 - s1)/1000000);
    println!("s3-s2 {:?}", (s3 - s2)/1000000);
    println!("s4-s3 {:?}", (s4 - s3)/1000000);

}
