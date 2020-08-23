mod dp;
mod leetcode;
mod unionfind;

fn main() {
    let op = Option::from(1);
    println!("{:?}", op.map(|x| x + 1));
    let arr = [10u32, 14, 5, 76, 84, 35, 90, 109, 9, 1, 2, 3, 4, 7, 5];
    let res = arr.iter().fold(0u32, |acc, x| acc + x);
    println!("{:?}", res);

    let mut c = arr.clone();
    c[0] = 100;
    println!(
        "origin {:?} , clone {:?}",
        arr,
        c.iter()
            .enumerate()
            .filter(|(x, _)| *x % 2 == 1)
            .map(|x| x.1)
            .collect::<Vec<_>>()
    );
}
