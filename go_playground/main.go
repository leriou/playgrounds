package main

import (
	"fmt"
	"go_playground/src/dp"
)

func main() {
	// c, _ := redis.Dial("tcp", "127.0.0.1:6379")
	// defer c.Close()
	// a := "123"

	// cal.Cal()
	// closure.Closure()

	// n, _ := c.Do("SADD", a, "10086")

	v := dp.Knapsack([]int{1, 2, 3}, []int{3, 2, 1}, 2, 3)

	fmt.Println(v)
}
