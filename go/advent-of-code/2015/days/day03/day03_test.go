package day03

import (
	"aoc2015/utils"
	"testing"
)

const inputTest = "../../inputs/day03/test"

func TestPart1(t *testing.T) {
	directions := utils.ReadFile(inputTest)[0]
	visitedHouses, _ := visitHouses(directions)

	if visitedHouses != 2 {
		t.Errorf("Got: %d; Expected: 2\n", visitedHouses)
	}
}

func TestPart2(t *testing.T) {
	directions := utils.ReadFile(inputTest)[0]
	_, visitedHousesWithRobo := visitHouses(directions)

	if visitedHousesWithRobo != 11 {
		t.Errorf("Got: %d; Expected: 11\n", visitedHousesWithRobo)
	}
}
