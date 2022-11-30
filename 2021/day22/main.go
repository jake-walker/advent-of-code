package main

import (
	"github.com/golang-collections/collections/set"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strconv"
	"strings"
)

type Point3 struct {
	X int
	Y int
	Z int
}

func check(err error) {
	if err != nil {
		log.Fatalln(err)
	}
}

func SetBounds(from int, to int, min int, max int) (int, int) {
	if (from < min && to < min) || (from > max && to > max) {
		return 1, 0
	}

	newFrom := from
	newTo := to

	if from < min {
		newFrom = min
	}

	if to > max {
		newTo = max
	}

	return newFrom, newTo
}

func LoadInput(input []string) *set.Set {
	cubes := set.New()
	for _, line := range input {
		fields := strings.Fields(line)
		operation := fields[0]
		ranges := strings.Split(fields[1], ",")

		xRange := strings.Split(ranges[0][2:], "..")
		xFrom, err := strconv.Atoi(xRange[0])
		check(err)
		xTo, err := strconv.Atoi(xRange[1])
		check(err)

		yRange := strings.Split(ranges[1][2:], "..")
		yFrom, err := strconv.Atoi(yRange[0])
		check(err)
		yTo, err := strconv.Atoi(yRange[1])
		check(err)

		zRange := strings.Split(ranges[2][2:], "..")
		zFrom, err := strconv.Atoi(zRange[0])
		check(err)
		zTo, err := strconv.Atoi(zRange[1])
		check(err)

		xFrom, xTo = SetBounds(xFrom, xTo, -50, 50)
		yFrom, yTo = SetBounds(yFrom, yTo, -50, 50)
		zFrom, zTo = SetBounds(zFrom, zTo, -50, 50)

		for x := xFrom; x <= xTo; x++ {
			for y := yFrom; y <= yTo; y++ {
				for z := zFrom; z <= zTo; z++ {
					if operation == "on" {
						cubes.Insert(Point3{x, y, z})
					} else {
						cubes.Remove(Point3{x, y, z})
					}
				}
			}
		}
	}
	return cubes
}

func main() {
	input := helpers.GetInputLines("day22/input.txt")

	part1 := LoadInput(input).Len()
	log.Printf("part 1: %v\n", part1)
}
