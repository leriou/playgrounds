package main

import (
	"fmt"

	bt "./src/backtrack"
	dp "./src/dp"
)

func main() {
	fmt.Println(dp.Knapsack([]int{1, 2, 3}, []int{3, 2, 1}, 2, 4))

	bt.Nqueen()

}
