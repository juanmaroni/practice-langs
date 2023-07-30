package day01

import (
	"aoc2015/utils"
	"testing"
)

const inputTest = "../../inputs/day01/test"

func TestPart1(t *testing.T) {
	directions := utils.ReadFile(inputTest)
	floor, _ := followDirections(directions[0])

	if floor != 3 {
		t.Errorf("Got: %d; Expected: 3\n", floor)
	}
}

func TestPart2(t *testing.T) {
	directions := utils.ReadFile(inputTest)
	_, basementEntryPosition := followDirections(directions[0])

	if basementEntryPosition != 1 {
		t.Errorf("Got: %d; Expected: 1\n", basementEntryPosition)
	}
}
