package main

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func main() {

	f, _ := os.Open("input")

	b, _ := io.ReadAll(f)
	lines := strings.Split(string(b), "\n")
	lines = lines[:len(lines)-1]

	var sum int
	var max [3]int

	for i, line := range lines {

		index := strings.Index(line, ":") + 1
		rounds := strings.Split(line[index:], ";")

		for _, round := range rounds {

			balls := strings.Split(round, ",")

			for _, ball := range balls {

				amount, colorIndex := getBalls(ball)
				if amount > max[colorIndex] {
					max[colorIndex] = amount
				}

			}

		}

		if max[0] <= 12 && max[1] <= 13 && max[2] <= 14 {
			sum += i + 1
		}

		max = [3]int{}

	}

	fmt.Println(sum)
}

func getBalls(ball string) (int, int) {

	// Find first character

	var i int
	var c rune
	for i, c = range ball {

		switch c {

		case 'r':
			n, _ := strconv.Atoi(ball[1 : i-1])
			return n, 0
		case 'g':
			n, _ := strconv.Atoi(ball[1 : i-1])
			return n, 1
		case 'b':
			n, _ := strconv.Atoi(ball[1 : i-1])
			return n, 2
		}
	}

	return 0, 0
}
