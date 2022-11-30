package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"strings"
)

func CalculateFishTotal(fish []int, days int) int {
	// for the second part, storing fish in a long list is super inefficient, so now just the counts are stored.
	// each 'day' we can shift the array down by one like so (this is backwards to make it look nicer):
	// Day | 8  7  6  5  4  3  2  1  0
	// --- | -------------------------
	//   0 |                1          -> 3
	//   1 |                   1       -> 2
	//   2 |                      1    -> 1
	//   3 |                         1 -> 0
	//   4 | 1     1                   -> 8,6
	//   5 |    1     1                -> 7,5

	counts := make([]int, 9)

	for _, i := range fish {
		counts[i] += 1
	}

	for i := 1; i <= days; i++ {
		// get the number of fish on zero days
		zeros := counts[0]

		// delete the first element in the array to shift down the rest
		counts = counts[1:]

		// now set 8 and 6 for the new fish
		// the counts array is one smaller now, so the 8 value needs to be appended
		counts = append(counts, zeros)
		counts[6] += zeros
	}

	sum := 0
	for i := 0; i < len(counts); i++ {
		sum += counts[i]
	}

	return sum
}

func main() {
	input := helpers.StringSliceToIntSlice(strings.Split(helpers.GetInput("day6/input.txt"), ","))

	part1 := CalculateFishTotal(input, 80)
	fmt.Printf("part 1: %v\n", part1)

	part2 := CalculateFishTotal(input, 256)
	fmt.Printf("part 2: %v\n", part2)
}
