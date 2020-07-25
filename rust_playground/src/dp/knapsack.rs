pub struct Knapsack;

impl Knapsack {
    pub fn knapsack(wt: Vec<usize>, val: Vec<usize>, n: usize, w: usize) -> usize {
        let mut dp = vec![vec![0; w + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=w {
                dp[i][j] = if j < wt[i - 1] {
                    dp[i - 1][j]
                } else {
                    dp[i - 1][j].max(dp[i - 1][j - wt[i - 1]] + val[i - 1])
                }
            }
        }
        dp[n][w]
    }
}

//// tests
///

#[cfg(test)]
mod test {
    use super::Knapsack;

    #[test]
    fn test_knapsack() {
        assert_eq!(Knapsack::knapsack(vec![1, 2, 3], vec![3, 2, 1], 2, 4), 5)
    }
}
