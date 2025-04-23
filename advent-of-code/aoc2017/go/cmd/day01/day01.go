package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	data, err := os.ReadFile("../input/01.txt")
	if err != nil {
		panic(err)
	}

	input := string(data)

	input = strings.TrimRight(input, "\r\n")

	fmt.Println(part1(input))
	fmt.Println(part2(input))
}

func part1(input string) int {
	result := 0
	var last byte
	for i := 1; i < len(input); i++ {
		if digit := b2i(input[i]); digit < 10 {
			last = input[i]
		}
		if input[i] == input[i-1] {
			result += b2i(input[i])
		}
	}

	if input[0] == last {
		result += b2i(last)
	}

	return result
}

func part2(input string) int {
	result := 0
	half := len(input) / 2
	for i := 0; i < len(input); i++ {
		j := (i + half) % len(input)
		if input[i] == input[j] {
			result += b2i(input[i])
		}
	}
	return result
}

func b2i(b byte) int {
	return int(b - '0')
}
