package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strconv"
	"strings"
)

type Coordinate struct {
	X int
	Y int
}

func PrintMap(m map[Coordinate]int, xMax int, yMax int) {
	for x := 0; x < xMax; x++ {
		for y := 0; y < yMax; y++ {
			i, f := m[Coordinate{x, y}]
			if !f {
				fmt.Print(".")
			} else {
				fmt.Print(i)
			}
		}
		fmt.Println()
	}
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func InterpolateStraight(x1 int, y1 int, x2 int, y2 int, setter func(x, y int)) {
	// ignore coordinates that are not a straight line
	if !(x1 == x2 || y1 == y2) {
		return
	}

	for x := x1; x <= x2; x++ {
		for y := y1; y <= y2; y++ {
			setter(x, y)
		}
	}
}

func InterpolateDiagonal(x1 int, y1 int, x2 int, y2 int, setter func(x, y int)) {
	// if the coordinates are in a straight line, use the straight line one
	if x1 == x2 || y1 == y2 {
		log.Println("-straight")
		InterpolateStraight(x1, y1, x2, y2, setter)
		return
	}

	// ignore any coordinates not at 45 degrees
	if x1-x2 == y1-y2 {
		log.Println("-diagonal 45")
		for x, y := x1, y1; x <= x2; x, y = x+1, y+1 {
			setter(x, y)
		}
	} else if Abs(x1-x2) == Abs(y1-y2) {
		log.Println("-diagonal -45")
		for x, y := x1, y1; x >= x2; x, y = x-1, y+1 {
			setter(x, y)
		}
		for x, y := x1, y1; y >= y2; x, y = x+1, y-1 {
			setter(x, y)
		}
	} else {
		log.Println("-ignore")
	}
}

func ParseInput(input string, interpolator func(x1 int, y1 int, x2 int, y2 int, setter func(x, y int))) map[Coordinate]int {
	lines := helpers.GetLines(input)
	out := map[Coordinate]int{}
	for _, line := range lines {
		fields := strings.Fields(line)
		if len(fields) < 3 {
			continue
		}

		x1, _ := strconv.Atoi(strings.Split(fields[0], ",")[0])
		y1, _ := strconv.Atoi(strings.Split(fields[0], ",")[1])
		x2, _ := strconv.Atoi(strings.Split(fields[2], ",")[0])
		y2, _ := strconv.Atoi(strings.Split(fields[2], ",")[1])

		if x1 > x2 || y1 > y2 {
			x1, y1, x2, y2 = x2, y2, x1, y1
		}

		log.Printf("%v,%v -> %v,%v\n", x1, y1, x2, y2)

		interpolator(x1, y1, x2, y2, func(x, y int) {
			//log.Printf(" - %v,%v\n", x, y)

			current, set := out[Coordinate{x, y}]
			if !set {
				current = 0
			}

			out[Coordinate{x, y}] = current + 1
		})
	}
	return out
}

func GetOverlaps(m map[Coordinate]int) int {
	out := 0

	for _, count := range m {
		if count > 1 {
			out += 1
		}
	}

	return out
}

func main() {
	input := helpers.GetInput("day5/input.txt")

	m1 := ParseInput(input, InterpolateStraight)
	p1 := GetOverlaps(m1)
	fmt.Printf("part 1: %v\n", p1)

	m2 := ParseInput(input, InterpolateDiagonal)
	p2 := GetOverlaps(m2)
	fmt.Printf("part 2: %v\n", p2)
}
