package cal

import (
	"fmt"
	"math"
	"math/rand"
)

func Cal() {
	x := 0.0
	y := 0.0
	n := 0.0
	for {
		rx := rand.Float64()
		ry := rand.Float64()
		y += 1.0
		ff := math.Sqrt(math.Pow(rx, float64(2.0)) + math.Pow(ry, float64(2.0)))
		if ff <= float64(1.0) {
			x += 1.0
		}
		n += 1.0
		if n >= 100000000.0 {
			n = 0.0
			fmt.Println(float64(x) / float64(y) * 4.0)
		}
	}
}
