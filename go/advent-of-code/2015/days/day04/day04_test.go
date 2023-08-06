package day04

import "testing"

const inputTest = "abcdef"

func TestPart1(t *testing.T) {
	part1, _ := findLowestPositive(inputTest)

	if part1 != 609043 {
		t.Errorf("Got: %d; Expected: 609043\n", part1)
	}
}

// No example for Part 2
