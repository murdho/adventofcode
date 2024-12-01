package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/murdho/adventofcode/2024/go/day01"
)

func main() {
	d := flag.Int("d", 0, "day")
	p := flag.Int("p", 0, "part")

	// Parse flags
	flag.Parse()

	// Validate input
	if *d == 0 || *p == 0 {
		fmt.Println("usage: go run main.go -d DAY -p PART")
		os.Exit(1)
	}

	day := *d
	part := *p

	if day == 1 && part == 1 {
		fmt.Println(day01.PartOne())
	} else if day == 1 && part == 2 {
		fmt.Println(day01.PartTwo())
	} else {
		fmt.Printf("error: unsupported day and part combination: %d %d\n", day, part)
		os.Exit(1)
	}
}
