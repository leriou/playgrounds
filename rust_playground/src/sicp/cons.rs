type BF = Box<dyn Fn(i32, i32) -> i32>;
type BFI = Box<dyn Fn(Box<dyn Fn(i32, i32) -> i32>) -> i32>;
pub fn cons(x: i32, y: i32) -> BFI {
    Box::new(move |m: BF| m(x, y))
}
pub fn car(pair: BFI) -> i32 {
    pair(Box::new(|a: i32, _: i32| a))
}
pub fn cdr(pair: BFI) -> i32 {
    pair(Box::new(|_: i32, b: i32| b))
}
