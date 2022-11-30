package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
)

func GetIncreasedMeasurements(measurements []int) int {
	increased := 0
	for i := 1; i < len(measurements); i++ {
		if measurements[i] > measurements[i-1] {
			increased += 1
		}
	}
	return increased
}

func GetIncreasedMeasurementsSliding(measurements []int) int {
	increased := 0

	last := -1
	for i := 2; i < len(measurements); i++ {
		total := measurements[i-2] + measurements[i-1] + measurements[i]
		if last != -1 && total > last {
			increased += 1
		}
		last = total
	}

	return increased
}

func main() {
	input := helpers.GetInputIntList("day1/input.txt")

	fmt.Printf("Part 1: %v\n", GetIncreasedMeasurements(input))
	fmt.Printf("Part 2: %v\n", GetIncreasedMeasurementsSliding(input))
}
