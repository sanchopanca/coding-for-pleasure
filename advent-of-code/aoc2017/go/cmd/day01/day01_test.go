package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	if part1("1122") != 3 {
		t.Error(`part1("1122") != 3`)
	}
	if part1("1111") != 4 {
		t.Error(`part1("1111") != 4`)
	}
	if part1("1234") != 0 {
		t.Error(`part1("1234") != 0`)
	}
	if part1("91212129") != 9 {
		t.Error(`part1("91212129") != 9`)
	}
}

func TestPart2(t *testing.T) {
	if part2("1212") != 6 {
		t.Error(`part2("1212") != 6`)
	}
	if part2("1221") != 0 {
		t.Error(`part2("1221") != 0`)
	}
	if part2("123425") != 4 {
		t.Error(`part2("123425") != 4`)
	}
	if part2("123123") != 12 {
		t.Error(`part2("123123") != 12`)
	}
	if part2("12131415") != 4 {
		t.Error(`part2("12131415") != 4`)
	}
}
