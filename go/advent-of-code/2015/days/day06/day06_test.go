package day06

import (
	"aoc2015/utils"
	"testing"
)

const inputTest = "../../inputs/day06/test"

func TestPart1(t *testing.T) {
	instructions := utils.ReadFile(inputTest)
	litLights, _ := toggleLights(instructions)

	if litLights != 1000000 {
		t.Errorf("Got: %d; Expected: 1000000\n", litLights)
	}
}

func TestPart2(t *testing.T) {
	instructions := utils.ReadFile(inputTest)
	_, brightness := toggleLights(instructions)

	if brightness != 2000000 {
		t.Errorf("Got: %d; Expected: 2000000\n", brightness)
	}
}
