package main

import (
	"fmt"

	sa "./src/algorithm"
)

func main() {
	fmt.Println(sa.Knapsack([]int{1, 2, 3}, []int{3, 2, 1}, 2, 4))
}
