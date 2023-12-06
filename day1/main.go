package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
)

func main() {

	f, _ := os.Open("input")

	b, _ := io.ReadAll(f)
	lines := bytes.Split(b, []byte{'\n'})
	lines = lines[:len(lines)-1]

	fmt.Println(challenge2(lines))
}

func challenge1(lines [][]byte) int {

	var sum int

	for _, line := range lines {

		var i int
		for ; i < len(line); i++ {
			if line[i] >= '0' && line[i] <= '9' {
				sum += int((line[i] - '0')) * 10
				break
			}
		}

		for j := len(line) - 1; j >= i; j-- {
			if line[j] >= '0' && line[j] <= '9' {
				sum += int((line[j] - '0'))
				break
			}
		}

	}

	return sum
}

func challenge2(lines [][]byte) int {

	var sum int

	numbers := []string{"!", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

	for _, line := range lines {

	prefix:
		for i := 0; i < len(line); i++ {
			if line[i] >= '0' && line[i] <= '9' {
				sum += int((line[i] - '0')) * 10
				break
			}

			for n, number := range numbers {
				if bytes.HasPrefix(line[i:], []byte(number)) {
					sum += n * 10
					break prefix
				}
			}

		}

	suffix:
		for j := len(line) - 1; j >= 0; j-- {
			if line[j] >= '0' && line[j] <= '9' {
				sum += int((line[j] - '0'))
				break
			}

			for n, number := range numbers {
				if bytes.HasSuffix(line[:j+1], []byte(number)) {
					sum += n
					break suffix
				}
			}

		}

	}

	return sum
}
