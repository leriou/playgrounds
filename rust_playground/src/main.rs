mod dp;
mod linkedlist;

fn main() {
    let op = Option::from(1);
    println!("{:?}", op.map(|x| x + 1));

    let mut a = "";
    let mut b = "iisis";
    let c = std::mem::replace(&mut a, b);
    println!("a -> {:?}", a);
    println!("b -> {:?}", b);
    println!("c -> {:?}", c);
    println!("hello")
}
