// Day 5: Doesn't He Have Intern-Elves For This?
package day05

import (
	"aoc2015/utils"
	"fmt"

	regexp "github.com/dlclark/regexp2" // Thanks!
)

const inputProd = "inputs/day05/prod"

func Day05() {
	strings := utils.ReadFile(inputProd)

	niceStrings, niceStringsImproved := countNiceStrings(strings)
	fmt.Printf("Day 5, Part 1: %d\n", niceStrings)
	fmt.Printf("Day 5, Part 2: %d\n", niceStringsImproved)
}

func countNiceStrings(strings []string) (uint16, uint16) {
	var niceStrings uint16 = 0
	var niceStringsImproved uint16 = 0
	
	re := regexp.MustCompile(`^(?=(?:[^aeiou]*[aeiou]){3})(?=.*([a-zA-Z])\1)(?!.*(?:ab|cd|pq|xy))[a-zA-Z]+$`, 0)
	reImproved := regexp.MustCompile(`^(?=.*([a-zA-Z]{2}).*\1)(?=.*([a-zA-Z])[a-zA-Z]\2).*$`, 0)

	for _, str := range strings {
		match, _ := re.MatchString(str)

		if match {
			niceStrings += 1
		}

		match, _ = reImproved.MatchString(str)

		if match {
			niceStringsImproved += 1
		}
	}

	return niceStrings, niceStringsImproved
}
