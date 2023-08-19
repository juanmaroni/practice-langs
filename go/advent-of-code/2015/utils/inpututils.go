package utils

import (
	"bufio"
	"os"
)

// Read file and return its lines
func ReadFile(filepath string) []string {
	file, err := os.Open(filepath)

	if err != nil {
		panic(err)
	}

	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	// Panic if error reading
	if scanner.Err() != nil {
		panic(err)
	}

	return lines
}
