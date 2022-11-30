package main

import (
	"fmt"
	"github.com/golang-collections/collections/set"
	"github.com/jake-walker/advent-of-code-2021/helpers"
)

type EnergyMap map[helpers.Point]int

func LoadInput(input []string) EnergyMap {
	out := make(EnergyMap)
	s := helpers.SplitLinesInt(input)
	for i := 0; i < len(s); i++ {
		for j := 0; j < len(s[0]); j++ {
			out[helpers.Point{X: j, Y: i}] = s[i][j]
		}
	}
	return out
}

func Flash(levels EnergyMap, flashed *set.Set, point helpers.Point) {
	adjacent := []helpers.Point{
		{point.X - 1, point.Y - 1}, {point.X, point.Y - 1}, {point.X + 1, point.Y - 1},
		{point.X - 1, point.Y}, {point.X + 1, point.Y},
		{point.X - 1, point.Y + 1}, {point.X, point.Y + 1}, {point.X + 1, point.Y + 1},
	}

	for _, p := range adjacent {
		_, found := levels[p]
		if !found {
			continue
		}

		if levels[p] != 0 {
			levels[p] = (levels[p] + 1) % 10
			if levels[p] == 0 && !flashed.Has(p) {
				flashed.Insert(p)
				Flash(levels, flashed, p)
			}
		}
	}
}

func Step(levels EnergyMap) int {
	flashed := set.New()

	for point, level := range levels {
		levels[point] = (level + 1) % 10
	}

	for point, level := range levels {
		if level == 0 {
			flashed.Insert(point)
			Flash(levels, flashed, point)
			fmt.Println(flashed)
		}
	}

	return flashed.Len()
}

func StepLoop(levels EnergyMap, steps int) int {
	flashes := 0
	for i := 0; i < steps; i++ {
		stepFlashes := Step(levels)
		fmt.Printf("step %v: flashes %v = %v\n", i+1, stepFlashes, flashes+stepFlashes)
		flashes += stepFlashes
	}
	return flashes
}

func main() {
	input := LoadInput(helpers.GetInputLines("day11/input.txt"))
	part1 := StepLoop(input, 100)
	fmt.Printf("part 1: %v\n", part1)
}
