package closure

import (
	"fmt"
	"time"
)

func Closure() {
	j := 1000
	a := func() func() {
		i := 0
		ch := make(chan int, 1)
		ch <- i
		return func() {
			t := <-ch
			t++
			fmt.Printf(" i %d, j %d\n", t, j)
			ch <- t
		}
	}()

	chs := make([]chan int, j)
	for p := 0; p < j; p++ {
		ch := make(chan int, 1)
		go a()
		ch <- p
		chs[p] = ch
	}

	for _, c := range chs {
		time.Sleep(1000)
		<-c
		// fmt.Println(<-c)
	}
}
