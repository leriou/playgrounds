package dp_test

import (
	"testing"

	dp "../dp"
)

func TestKnapsack(t *testing.T) {
	c := dp.Knapsack([]int{1, 2, 3}, []int{3, 2, 1}, 2, 4)
	if c != 5 {
		t.Error("c -> ", c)
	}
}
