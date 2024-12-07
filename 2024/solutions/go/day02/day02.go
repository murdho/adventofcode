package day02

import (
	"os"
	"strconv"
	"strings"
)

const (
	inputFile = "../../inputs/day02.txt"
	// inputFile = "../../inputs/day02_example.txt"
)

func PartOne() int {
	bb, err := os.ReadFile(inputFile)
	if err != nil {
		panic(err)
	}

	var safe int

	reports := strings.Split(string(bb), "\n")
	for _, report := range reports {
		if levels := parseLevels(report); safeLevels(levels) {
			safe++
		}
	}

	return safe
}

func parseLevels(report string) []int {
	fields := strings.Fields(report)
	if len(fields) == 0 {
		return nil
	}

	levels := make([]int, len(fields))
	for i, field := range fields {
		l, err := strconv.Atoi(field)
		if err != nil {
			panic(err)
		}

		levels[i] = l
	}

	return levels
}

func safeLevels(levels []int) bool {
	if len(levels) == 0 {
		return false
	}

	var increasing bool
	if levels[0]-levels[1] < 0 {
		increasing = true
	}

	for i := 1; i < len(levels); i++ {
		diff := levels[i-1] - levels[i]

		if increasing && diff > 0 || !increasing && diff < 0 {
			return false
		}

		if abs(diff) < 1 || abs(diff) > 3 {
			return false
		}
	}

	return true
}

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}
