package algorithm

func Max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func Knapsack(wt, val []int, n, w int) int {
	dp := [100][100]int{}
	for i := 1; i <= n; i++ {
		for j := 1; j <= w; j++ {
			if j >= wt[i-1] {
				dp[i][j] = max(dp[i-1][j], dp[i-1][j-wt[i-1]]+val[i-1])
			} else {
				dp[i][j] = dp[i-1][j]
			}
		}
	}
	return dp[n][w]
}
