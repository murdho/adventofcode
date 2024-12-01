package day01

import (
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

const (
	inputFile        = "../inputs/day01.txt"
	inputExampleFile = "../inputs/day01_example.txt"
)

func PartOne() int {
	left, right := readLists(inputFile)

	sort.Ints(left)
	sort.Ints(right)

	distances := make([]int, len(left))
	for i := range left {
		distances[i] = int(math.Abs(float64(right[i]) - float64(left[i])))
	}

	var sum int
	for _, n := range distances {
		sum += n
	}

	return sum
}

func PartTwo() int {
	left, right := readLists(inputFile)

	tallies := make(map[int]int)
	for _, n := range right {
		tallies[n] += 1
	}

	var similarityScore int
	for _, n := range left {
		if tally, ok := tallies[n]; ok {
			similarityScore += n * tally
		}
	}

	return similarityScore
}

func readLists(fname string) ([]int, []int) {
	bb, err := os.ReadFile(fname)
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(bb), "\n")

	left := make([]int, len(lines))
	right := make([]int, len(lines))

	for i, l := range lines {
		parts := strings.Split(l, "   ")
		if len(parts) != 2 {
			continue // skip the last empty line
		}

		num1, err := strconv.Atoi(parts[0])
		if err != nil {
			panic(err)
		}

		num2, err := strconv.Atoi(parts[1])
		if err != nil {
			panic(err)
		}

		left[i] = num1
		right[i] = num2
	}

	return left, right
}
