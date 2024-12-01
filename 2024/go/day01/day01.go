package day01

import (
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func PartOne() int {
	bb, err := os.ReadFile("../inputs/day01_part1.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(bb), "\n")

	list1 := make([]int, len(lines))
	list2 := make([]int, len(lines))

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

		list1[i] = num1
		list2[i] = num2
	}

	sort.Ints(list1)
	sort.Ints(list2)

	distances := make([]int, len(list1))
	for i := range list1 {
		distances[i] = int(math.Abs(float64(list2[i]) - float64(list1[i])))
	}

	var sum int
	for _, d := range distances {
		sum += d
	}

	return sum
}
