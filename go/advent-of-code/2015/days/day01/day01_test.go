package day01

import (
	"aoc2015/utils"
	"testing"
)

const input_test = "../inputs/day01/test"

func TestPart1(t *testing.T) {
	directions := utils.ReadFile(input_test)
	floor, _ := follow_directions(directions)

	if floor != 3 {
		t.Errorf("Got: %d; Expected: 3\n", floor)
	}
}

func TestPart2(t *testing.T) {
	directions := utils.ReadFile(input_test)
	_, basement_entry_position := follow_directions(directions)

	if basement_entry_position != 1 {
		t.Errorf("Got: %d; Expected: 1\n", basement_entry_position)
	}
}
