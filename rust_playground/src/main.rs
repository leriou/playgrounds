mod dp;

use dp::knapsack::Knapsack;

fn main() {
    println!(
        "{:?}",
        Knapsack::knapsack(vec![1, 2, 3], vec![3, 2, 1], 2, 4)
    );
}
