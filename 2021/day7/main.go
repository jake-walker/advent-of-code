package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strings"
)

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func MinMax(slice []int) (min, max int) {
	min, max = slice[0], slice[0]
	for _, i := range slice {
		if max < i {
			max = i
		}
		if min > i {
			min = i
		}
	}
	return
}

func LoadPositions(positions []int) map[int]int {
	out := make(map[int]int)
	for _, pos := range positions {
		out[pos] += 1
	}
	return out
}

func CalculateFuel(positions map[int]int) (int, int) {
	bestFuel := -1
	bestPosition := -1

	for p1, _ := range positions {
		fuel := 0

		for p2, mul := range positions {
			fuel += Abs(p1-p2) * mul
		}

		log.Printf("fuel for position %v is %v", p1, fuel)

		if fuel < bestFuel || bestFuel == -1 {
			bestFuel = fuel
			bestPosition = p1
		}
	}

	return bestFuel, bestPosition
}

func CalculateFuel2(positions []int) (int, int) {
	bestFuel := -1
	bestPosition := -1

	positionMap := LoadPositions(positions)
	min, max := MinMax(positions)

	for p1 := min; p1 < max; p1++ {
		fuel := 0

		for p2, mul := range positionMap {
			diff := Abs(p1 - p2)
			cost := (diff * (diff + 1)) / 2
			fuel += cost * mul
		}

		log.Printf("fuel for position %v is %v", p1, fuel)

		if fuel < bestFuel || bestFuel == -1 {
			bestFuel = fuel
			bestPosition = p1
		}
	}

	return bestFuel, bestPosition
}

func main() {
	input := helpers.StringSliceToIntSlice(strings.Split(helpers.GetInput("day7/input.txt"), ","))
	positions := LoadPositions(input)

	part1Fuel, _ := CalculateFuel(positions)
	fmt.Printf("part 1: %v\n", part1Fuel)
	part2Fuel, _ := CalculateFuel2(input)
	fmt.Printf("part 2: %v\n", part2Fuel)
}
