package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	part1()
	part2()
}

func part1() {
	lines := fileToLines("../input/03.txt")
	result := 0
	for _, line := range lines {
		p, v := biggest(line)
		if p == len(line)-1 {
			_, first := biggest(line[:p])
			result += first*10 + v
		} else {
			_, second := biggest(line[p+1:])
			result += v*10 + second
		}
	}
	fmt.Println(result)
}

func part2() {
	const limit int = 12
	lines := fileToLines("../input/03.txt")
	result := 0
	for _, line := range lines {
		var digits []byte
		for i := 0; i < limit; i++ {
			p, v := biggest(line)
			if len(line)-p > limit-i { // todo, check
				digits = append(digits, []byte(strconv.Itoa(v))...)
			}
		}
	}
}

func biggest(s string) (position int, value int) {
	value = -1
	for i, c := range s {
		v, _ := strconv.Atoi(string(c))
		if v > value {
			value = v
			position = i
		}
	}
	return
}

func fileToLines(filePath string) []string {
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(file)

	var result []string
	for scanner.Scan() {
		result = append(result, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	return result
}
