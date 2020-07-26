mod dp;
mod linkedlist;

fn main() {
    let op = Some(Box::new(8));
    op.map(|x| println!("{:?}", x));

    let mut a = "";
    let mut b = "iisis";
    let c = std::mem::replace(&mut a, b);
    println!("a -> {:?}", a);
    println!("b -> {:?}", b);
    println!("c -> {:?}", c);
    println!("hello")
}
