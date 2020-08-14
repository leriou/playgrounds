package main

import (
	"fmt"

	"github.com/gomodule/redigo/redis"
)

func main() {
	c, _ := redis.Dial("tcp", "127.0.0.1:6379")
	defer c.Close()
	n, _ := c.Do("GET", "10086")
	fmt.Println(n)
}
