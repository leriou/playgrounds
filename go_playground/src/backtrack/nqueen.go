package backtrack

import "fmt"

var (
	n      = 8
	count  = 0
	matrix = make([][]int, n+1)
)

func init_matrix() {
	for i := 0; i < n+1; i++ {
		matrix[i] = make([]int, n+1)
	}
}

func Nqueen() {
	init_matrix()
	trail(1)
}

func trail(i int) {
	if i > n {
		print()
	} else {
		for j := 1; j <= n; j++ {
			matrix[i][j] = 1
			if is_valid(i, j) {
				trail(i + 1)
			}
			matrix[i][j] = 0
		}
	}
}

func abs(i int) int {
	if i > 0 {
		return i
	}
	return -i
}

func print() {
	count++
	for i := 1; i <= n; i++ {
		fmt.Println(matrix[i][1:])
	}
	fmt.Println("---------", count)
}
func is_valid(i, j int) bool {
	for m := 1; m < i; m++ {
		for p := 1; p <= n; p++ {
			if matrix[m][p] == 1 {
				if j == p || abs(j-p) == abs(i-m) {
					return false
				}
			}
		}
	}
	return true
}
