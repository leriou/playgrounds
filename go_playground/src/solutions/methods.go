package solutions

import "fmt"

func Max(a int, b int) int {
	if a > b {
		return a
	}
	return b
}

func Max_test() {

	fmt.Println("success")
}
