package day02

import (
	"aoc2015/utils"
	"testing"
)

const inputTest = "../../inputs/day02/test"

func TestPart1(t *testing.T) {
	var ans uint64 = 58 + 43
	presents := utils.ReadFile(inputTest)
	requiredPaper, _ := calcRequiredMaterial(presents)

	if requiredPaper != ans {
		t.Errorf("Got: %d; Expected: %d\n", requiredPaper, ans)
	}
}

func TestPart2(t *testing.T) {
	var ans uint64 = 34 + 14
	presents := utils.ReadFile(inputTest)
	_, requiredRibbon := calcRequiredMaterial(presents)

	if requiredRibbon != ans {
		t.Errorf("Got: %d; Expected: %d\n", requiredRibbon, ans)
	}
}
