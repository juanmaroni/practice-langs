package day05

import (
	"aoc2015/utils"
	"testing"
)

const inputTest = "../../inputs/day05/test"
const inputTest2 = "../../inputs/day05/test2"

func TestPart1(t *testing.T) {
	strings := utils.ReadFile(inputTest)
	niceStrings, _ := countNiceStrings(strings)

	if niceStrings != 2 {
		t.Errorf("Got: %d; Expected: 2\n", niceStrings)
	}
}

func TestPart2(t *testing.T) {
	strings := utils.ReadFile(inputTest2)
	_, niceStringsImproved := countNiceStrings(strings)

	if niceStringsImproved != 2 {
		t.Errorf("Got: %d; Expected: 2\n", niceStringsImproved)
	}
}
