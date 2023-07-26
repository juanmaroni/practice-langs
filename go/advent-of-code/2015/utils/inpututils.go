package utils

import (
	"os"
)

func check(e error) {
    if e != nil {
        panic(e)
    }
}

// Read file and return its content as string
func ReadFile(filename string) string {
	content, err := os.ReadFile(filename)
    check(err)
    
	return string(content)
}
