package algorithm

func Max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func Knapsack(wt, val []int, n, w int) int {
	dp := make([][]int, n+1)
	for i := 0; i <= n; i++ {
		dp[i] = make([]int, w+1)
	}
	for i := 1; i <= n; i++ {
		for j := 1; j <= w; j++ {
			if j >= wt[i-1] {
				dp[i][j] = Max(dp[i-1][j], dp[i-1][j-wt[i-1]]+val[i-1])
			} else {
				dp[i][j] = dp[i-1][j]
			}
		}
	}
	return dp[n][w]
}
