// Day 4: The Ideal Stocking Stuffer
package day04

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"strconv"
)

const inputProd = "yzbqklnj"

func Day04() {
	part1, part2 := findLowestPositive(inputProd)
	fmt.Printf("Day 4, Part 1: %d\n", part1)
	fmt.Printf("Day 4, Part 2: %d\n", part2)
}

func findLowestPositive(secretKey string) (uint64, uint64) {
	// String to bytes
	data := []byte(secretKey)
	var count uint64 = 1

	var fiveZeroes uint64 = 0
	var sixZeroes uint64 = 0

	for fiveZeroes == 0 || sixZeroes == 0 {
		// Add current count as bytes
		data_cp := append(data, []byte (strconv.FormatUint(count, 10))...)
		// Get MD5 hash
		hash := md5.Sum([]byte (data_cp))
		// Get hash as hexadecimal
		hex := hex.EncodeToString(hash[:])

		if fiveZeroes == 0 && hex[:5] == "00000" {
			fiveZeroes = count
		}

		if sixZeroes == 0 && hex[:6] == "000000" {
			sixZeroes = count
		}

		count += 1
	}

	return fiveZeroes, sixZeroes
}
