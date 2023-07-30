// Day 1: Not Quite Lisp
package day01

import (
	"aoc2015/utils"
	"fmt"
)

const inputProd = "inputs/day01/prod"

func Day01() {
    directions := utils.ReadFile(inputProd)
	
	// There is only one line
	answer1, answer2 := followDirections(directions[0])
	fmt.Printf("Day 1, Part 1: %d\n", answer1)
	fmt.Printf("Day 1, Part 2: %d\n", answer2)
}

func followDirections(directions string) (int32, uint16) {
	// Part 1
	var floor int32 = 0

	// Part 2
	var basementFound bool = false
	var basementEntryPosition uint16 = 0

	for i, step := range directions {
		if step == '(' {
			floor += 1
		} else if step == ')' {
			floor -= 1
		} else {
			continue
		}

		if !basementFound {
			if floor == -1 {
				basementEntryPosition = uint16(i) + 1
				basementFound = true
			}
		}
	}

	return floor, basementEntryPosition
}
