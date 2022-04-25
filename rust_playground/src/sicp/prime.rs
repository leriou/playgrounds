use std::ops::Index;

pub fn prime_question(n: usize) -> Vec<usize> {
    let (mut primes, mut is_prime): (Vec<usize>, Vec<usize>) = (vec![], vec![1; n]);
    for i in 2..n {
        if (is_prime[i] == 1) {
            primes.push(i);
        }
        for j in 0..primes.len() {
            if i * primes[j] < n {
                is_prime[i * primes[j]] = 0;
                if i % primes[j] == 0 {
                    break;
                }
            }
        }
    }
    primes
}
