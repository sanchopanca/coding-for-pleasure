package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	part1()
	part2()
}

func part1() {
	result := 0
	input := fileToTrimmedString("../input/02.txt")
	ranges := strings.Split(input, ",")
	for _, r := range ranges {
		f, t, _ := strings.Cut(r, "-")
		from, _ := strconv.Atoi(f)
		to, _ := strconv.Atoi(t)
		for i := from; i <= to; i++ {
			s := strconv.Itoa(i)
			if len(s)%2 != 0 {
				continue
			}
			if s[:len(s)/2] == s[len(s)/2:] {
				result += i
			}
		}
	}
	fmt.Println(result)
}

func part2() {
	result := 0
	input := fileToTrimmedString("../input/02.txt")
	ranges := strings.Split(input, ",")
	for _, r := range ranges {
		f, t, _ := strings.Cut(r, "-")
		from, _ := strconv.Atoi(f)
		to, _ := strconv.Atoi(t)
	Number:
		for i := from; i <= to; i++ {
			s := strconv.Itoa(i)
			l := len(s)
			primes := []int{2, 3, 5, 7}

			for _, p := range primes {
				if l%p == 0 {
					if partsEqual(s, p) {
						result += i
						continue Number
					}
				}
			}
		}
	}
	fmt.Println(result)
}

func fileToTrimmedString(filepath string) string {
	data, err := os.ReadFile(filepath)
	if err != nil {
		panic(err)
	}

	content := strings.TrimSpace(string(data))

	return content
}

func partsEqual(s string, n int) bool {
	l := len(s)
	for i := 0; i < n-1; i++ {
		if s[i*l/n:(i+1)*l/n] != s[(i+1)*l/n:(i+2)*l/n] {
			return false
		}
	}
	return true
}
