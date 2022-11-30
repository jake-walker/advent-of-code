package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strconv"
)

type Point struct {
	X int
	Y int
}

func LoadInput(content string) [][]int {
	lines := helpers.SplitLines(helpers.GetLines(content))
	out := make([][]int, len(lines))

	for i, line := range lines {
		out[i] = make([]int, len(line))
		for j, char := range line {
			n, err := strconv.Atoi(char)
			if err != nil {
				log.Fatalf("failed to convert %v to int: %v", char, err)
			}
			out[i][j] = n
		}
	}

	return out
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

func GetAdjacent(heightmap [][]int, point Point) []Point {
	adjacent := []Point{}
	if point.Y-1 >= 0 {
		adjacent = append(adjacent, Point{point.X, point.Y - 1})
	}
	if point.Y+1 < len(heightmap) {
		adjacent = append(adjacent, Point{point.X, point.Y + 1})
	}
	if point.X-1 >= 0 {
		adjacent = append(adjacent, Point{point.X - 1, point.Y})
	}
	if point.X+1 < len(heightmap[0]) {
		adjacent = append(adjacent, Point{point.X + 1, point.Y})
	}
	return adjacent
}

func GetAdjacentValue(heightmap [][]int, point Point) []int {
	points := GetAdjacent(heightmap, point)
	values := []int{}
	for _, p := range points {
		values = append(values, heightmap[p.Y][p.X])
	}
	return values
}

func GetLowPoints(heightmap [][]int) []Point {
	out := []Point{}

	for i := 0; i < len(heightmap); i++ {
		for j := 0; j < len(heightmap[i]); j++ {
			adjacent := GetAdjacentValue(heightmap, Point{j, i})

			min, _ := MinMax(adjacent)

			if heightmap[i][j] < min {
				out = append(out, Point{j, i})
			}
		}
	}

	return out
}

func GetTotalRiskLevel(heightmap [][]int, points []Point) int {
	total := 0
	for _, point := range points {
		total += heightmap[point.Y][point.X] + 1
	}
	return total
}

func GetBasin(heightmap [][]int, point Point, checked map[Point]bool) map[Point]bool {
	// get the adjacent values
	adjacent := GetAdjacent(heightmap, point)

	for _, p := range adjacent {
		// skip this point if it has already been checked
		if checked[p] == true {
			continue
		}

		if heightmap[p.Y][p.X] == 9 {
			continue
		}

		checked[p] = true
		checked = GetBasin(heightmap, p, checked)
	}

	return checked
}

func main() {
	input := helpers.GetInput("day9/input.txt")
	heightmap := LoadInput(input)
	lowPoints := GetLowPoints(heightmap)

	part1 := GetTotalRiskLevel(heightmap, lowPoints)
	fmt.Printf("part 1: %v\n", part1)

	first := 0
	second := 0
	third := 0
	for _, point := range lowPoints {
		basin := GetBasin(heightmap, point, map[Point]bool{})
		length := len(basin)
		if length > first {
			first = length
		} else if length > second {
			second = length
		} else if length > third {
			third = length
		}
	}
	fmt.Printf("part 2: %v\n", first*second*third)
}
