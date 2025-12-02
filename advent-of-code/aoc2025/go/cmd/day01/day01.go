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
	lines := fileToLines("../input/01.txt")

	dial := 50
	zeros := 0
	for _, line := range lines {
		dir := line[:1]
		distance, _ := strconv.Atoi(line[1:])
		if dir == "L" {
			distance = -distance
		}
		dial = (dial + distance + 100) % 100
		if dial == 0 {
			zeros++
		}
	}
	fmt.Println(zeros)
}

func part2() {
	lines := fileToLines("../input/01.txt")

	dial := 50
	zeros := 0
	for _, line := range lines {
		dir := line[:1]
		distance, _ := strconv.Atoi(line[1:])
		zeros += distance / 100
		distance = distance % 100
		if dir == "L" {
			distance = -distance
		}
		was_zero := dial == 0
		dial += distance
		if !was_zero && dial <= 0 || dial >= 100 {
			zeros++
		}
		dial = (dial + 100) % 100
	}
	fmt.Println(zeros)
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
