// Day 3: Perfectly Spherical Houses in a Vacuum
package day03

import (
	"aoc2015/utils"
	"fmt"
)

const inputProd = "inputs/day03/prod"

type point struct {
	x, y int8
}

func (p *point) existsInList(listPoints []point) bool {
	for _, point := range listPoints {
		if point.x == p.x && point.y == p.y {
			return true
		}
	}

	return false
}

func (p *point) nextPosition(dir rune) {
	if dir == '>' {
		p.x += 1
	} else if dir == '<' {
		p.x -= 1
	} else if dir == '^' {
		p.y += 1
	} else if dir == 'v' {
		p.y -= 1
	} else {
		panic("Where the fuck are you going?")
	}
}

func Day03() {
	// Only one line
	directions := utils.ReadFile(inputProd)[0]

	visitHouses, visitHousesWithRobosanta := visitHouses(directions)
	fmt.Printf("Day 3, Part 1: %d\n", visitHouses)
	fmt.Printf("Day 3, Part 2: %d\n", visitHousesWithRobosanta)
}

func visitHouses(directions string) (uint16, uint16) {
	// Part 1
	currentPosition := point{0, 0}
	visitedHouses := []point{currentPosition}

	// Part 2
	santaCurrentPosition := point{0, 0}
	robosantaCurrentPosition := point{0, 0}
	visitedHousesWithRobo := []point{santaCurrentPosition}

	for i, dir := range directions {
		// Part 1
		visitedHouses = moveAndCheckVisited(&currentPosition, dir, visitedHouses)

		// Part 2
		if i%2 == 0 {
			visitedHousesWithRobo = moveAndCheckVisited(&santaCurrentPosition, dir, visitedHousesWithRobo)
		} else {
			visitedHousesWithRobo = moveAndCheckVisited(&robosantaCurrentPosition, dir, visitedHousesWithRobo)
		}
	}

	return uint16(len(visitedHouses)), uint16(len(visitedHousesWithRobo))
}

func moveAndCheckVisited(pos *point, dir rune, listPoints []point) []point {
	pos.nextPosition(dir)

	if !pos.existsInList(listPoints) {
		listPoints = append(listPoints, *pos)
	}

	return listPoints
}
