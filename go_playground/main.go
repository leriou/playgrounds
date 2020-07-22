package main

import (
	"fmt"

	dp "./src/dp"
)

func main() {
	fmt.Println(dp.Knapsack([]int{1, 2, 3}, []int{3, 2, 1}, 2, 4))
}
