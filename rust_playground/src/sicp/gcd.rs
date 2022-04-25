pub fn gcd(a: i64, b: i64) -> i64 {
    if b != 0 {
        gcd(b, a%b)
    } else {
        a
    }
} 