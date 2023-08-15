// Day 6: Probably a Fire Hazard
package day06

import (
	"aoc2015/utils"
	"fmt"
	"regexp"
	"strconv"
)

const inputProd = "inputs/day06/prod"

// Enum for actions
type action int8

const (
	Off action = -1
	On action = 1
	Toggle action = 2
)

type point struct {
    x, y uint16
}

type instruction struct {
	action action
	start, finish point
}

func Day06() {
	instructions := utils.ReadFile(inputProd)
	
	litLights, brightness := toggleLights(instructions)
	fmt.Printf("Day 6, Part 1: %d\n", litLights)
	fmt.Printf("Day 6, Part 2: %d\n", brightness)
}

func toggleLights(instructions []string) (uint32, uint32) {
    var lightGrid [1000][1000]bool
	var brightGrid[1000][1000]uint16

	var litLights uint32 = 0
	var brightness uint32 = 0

	re := regexp.MustCompile(`^\b(turn on|turn off|toggle)\b (\d+),(\d+) \bthrough\b (\d+),(\d+)$`)

    for _, inst := range instructions {
		instruction := parseInstruction(*re, inst)
		action := instruction.action
		startX := instruction.start.x
		startY := instruction.start.y
		finishX := instruction.finish.x
		finishY := instruction.finish.y

		// Loop inside each condition, so the action is checked only once per instruction
		if action == On {
			for i := startX; i <= finishX; i++ {
				for j := startY; j <= finishY; j++ {
					currentPoint := lightGrid[i][j]
					lightGrid[i][j] = true
					
					// Has the light changed?
					if lightGrid[i][j] != currentPoint {
						litLights += 1
					}

					brightGrid[i][j] += 1					
					brightness += 1
				}
			}			
		} else if action == Off {
			for i := startX; i <= finishX; i++ {
				for j := startY; j <= finishY; j++ {
					currentPoint := lightGrid[i][j]
					lightGrid[i][j] = false

					// Has the light changed? And we don't want to count below 0 (uint)
					if lightGrid[i][j] != currentPoint && litLights > 0 {
						litLights -= 1
					}

					if brightGrid[i][j] > 0 {
						brightGrid[i][j] -= 1
						brightness -= 1
					}
				}
			}
		} else {
			for i := startX; i <= finishX; i++ {
				for j := startY; j <= finishY; j++ {
					currentPoint := lightGrid[i][j]
					lightGrid[i][j] = !currentPoint

					if !currentPoint {
						litLights += 1
					} else {
						litLights -= 1
					}

					brightGrid[i][j] += 2
					brightness += 2
				}
			}
		}
    }

	return litLights, brightness
}

func parseInstruction(re regexp.Regexp, instr string) instruction {
	groups := re.FindStringSubmatch(instr)
	
	// Action
	var action action
	actionStr := groups[1]

	if actionStr == "turn on" {
		action = On
	} else if actionStr == "turn off" {
		action = Off
	} else if actionStr == "toggle" {
		action = Toggle
	} else {
		panic(fmt.Sprintf("Unknown action \"%s\"!", actionStr))
	}

	// Start point
	startX, _ := strconv.ParseUint(groups[2], 10, 32)
	startY, _ := strconv.ParseUint(groups[3], 10, 32)

	// Finish point
	finishX, _ := strconv.ParseUint(groups[4], 10, 32)
	finishY, _ := strconv.ParseUint(groups[5], 10, 32)

    return instruction{
		action,
		point{ uint16(startX), uint16(startY) },
		point{ uint16(finishX), uint16(finishY) },
	}
}
