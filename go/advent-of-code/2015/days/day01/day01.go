// Day 1: Not Quite Lisp
package days

import (
	"aoc2015/utils"
	"fmt"
)

const input_prod = "inputs/day01/prod"

func Day01() {
    directions := utils.ReadFile(input_prod)
	answer1, answer2 := follow_directions(directions)
	fmt.Printf("Day 1, Part 1: %d\n", answer1)
	fmt.Printf("Day 1, Part 2: %d\n", answer2)
}

func follow_directions(directions string) (int32, uint16) {
	// Part 1
	var floor int32 = 0

	// Part 2
	var basement_found bool = false
	var basement_entry_position uint16 = 0

	for i, step := range directions {
		if step == '(' {
			floor += 1
		} else if step == ')' {
			floor -= 1
		} else {
			continue
		}

		if !basement_found {
			if floor == -1 {
				basement_entry_position = uint16(i) + 1
				basement_found = true
			}
		}
	}

	return floor, basement_entry_position
}
