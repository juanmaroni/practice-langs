// Day 2: I Was Told There Would Be No Math
package day02

import (
	"aoc2015/utils"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

const inputProd = "inputs/day02/prod"

func Day02() {
	presents := utils.ReadFile(inputProd)
	requiredPaper, requiredRibbon := calcRequiredMaterial(presents)
	fmt.Printf("Day 2, Part 1: %d\n", requiredPaper)
	fmt.Printf("Day 2, Part 2: %d\n", requiredRibbon)
}

func calcRequiredMaterial(presents []string) (uint64, uint64) {
	var totalWrappingPaper uint64 = 0
	var totalRibbon uint64 = 0

	for _, present := range presents {
		nums := strings.Split(present, "x")

		// There are only three values
		l := parseUintOrPanic(nums[0])
		w := parseUintOrPanic(nums[1])
		h := parseUintOrPanic(nums[2])

		// Order the values
		measures := []uint64{l, w, h}
		sort.Slice(measures, func(i, j int) bool { return measures[i] < measures[j] })

		// Doesn't matter which is which
		v1 := measures[0]
		v2 := measures[1]
		v3 := measures[2]

		// Part 1
		area := 2*v1*v2 + 2*v1*v3 + 2*v2*v3 + v1*v2
		totalWrappingPaper += area

		// Part 2
		totalRibbon += v1*2 + v2*2 + v1*v2*v3
	}

	return totalWrappingPaper, totalRibbon
}

func parseUintOrPanic(num string) uint64 {
	val, err := strconv.ParseUint(num, 10, 32)

	if err != nil {
		panic(fmt.Sprintf("Error: length \"%s\" is not a number", num))
	}

	return val
}
