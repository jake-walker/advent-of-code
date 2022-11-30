package main

import (
	"fmt"
	"github.com/golang-collections/collections/set"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strconv"
	"strings"
)

type Fold struct {
	Axis       string
	Coordinate int
}

type Paper struct {
	Points *set.Set
	Folds  []Fold
}

func LoadPaper(input []string) Paper {
	points := set.New()
	folds := []Fold{}

	for i := 0; i < len(input); i++ {
		if input[i] == "" {
			continue
		}

		if strings.HasPrefix(input[i], "fold along") {
			foldAlong := input[i][11:12]
			coord, err := strconv.Atoi(input[i][13:])

			if err != nil {
				log.Fatalf("failed to parse fold: %v", err)
			}

			folds = append(folds, Fold{
				Axis:       foldAlong,
				Coordinate: coord,
			})
			continue
		}

		splitCoord := strings.Split(input[i], ",")
		xCoord, xErr := strconv.Atoi(splitCoord[0])
		yCoord, yErr := strconv.Atoi(splitCoord[1])

		if xErr != nil || yErr != nil {
			log.Fatalf("failed to parse x or y: %v, %v", xErr, yErr)
		}

		points.Insert(helpers.Point{X: xCoord, Y: yCoord})
	}

	return Paper{
		Points: points,
		Folds:  folds,
	}
}

func Reflect(source int, reflect int) int {
	return reflect - (source - reflect)
}

func FoldPaper(paper Paper, max int) Paper {
	for i, fold := range paper.Folds {
		if i == max {
			return paper
		}

		remove := []helpers.Point{}
		insert := []helpers.Point{}

		paper.Points.Do(func(i interface{}) {
			point, _ := i.(helpers.Point)
			newPoint := point

			if fold.Axis == "y" && point.Y > fold.Coordinate {
				newPoint.Y = Reflect(point.Y, fold.Coordinate)
			} else if fold.Axis == "x" && point.X > fold.Coordinate {
				newPoint.X = Reflect(point.X, fold.Coordinate)
			}

			if point != newPoint {
				remove = append(remove, point)
				insert = append(insert, newPoint)
			}
		})

		for _, point := range remove {
			paper.Points.Remove(point)
		}

		for _, point := range insert {
			paper.Points.Insert(point)
		}
	}

	return paper
}

func DrawPoints(maxX int, maxY int, points *set.Set) {
	for y := 0; y < maxY; y++ {
		for x := 0; x < maxX; x++ {
			if points.Has(helpers.Point{X: x, Y: y}) {
				fmt.Print("#")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Println()
	}
}

func main() {
	input := LoadPaper(helpers.GetInputLines("day13/input.txt"))

	part1 := FoldPaper(input, 1).Points.Len()
	fmt.Printf("part 1: %v\n", part1)

	part2 := FoldPaper(input, -1).Points
	fmt.Print("part 2:\n")
	DrawPoints(39, 6, part2)
}
