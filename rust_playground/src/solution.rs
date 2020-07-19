pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (len, mut i, mut m, mut rm) = (prices.len(), (prices.len() - 1) as i32, 0i32, 0i32);
        if len > 1 {
            while i >= 0 {
                let iu = i as usize;
                m = m.max(0).max(rm - prices[iu]);
                rm = rm.max(prices[iu]);
                i -= 1;
            }
        }
        m
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        if l == 0 {
            return 0;
        }
        if l == 1 {
            return nums[0];
        }
        let (mut i, mut a, mut b, mut tmp) = (2usize, nums[0], nums[0].max(nums[1]), 0i32);
        while i < l {
            tmp = b;
            b = b.max(a + nums[i]);
            a = tmp;
            i += 1;
        }
        return b;
    }

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

pub struct Tests {}

impl Tests {
    pub fn knapsack_test() {
        println!(
            "{:?}",
            Solution::knapsack(vec![1, 2, 3], vec![3, 2, 1], 2, 4)
        );
    }
}
