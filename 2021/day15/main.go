package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"github.com/yourbasic/graph"
)

func PointToVertex(point helpers.Point, xMax int) int {
	return point.Y*xMax + point.X
}

func VertexToPoint(v int, xMax int) helpers.Point {
	return helpers.Point{
		Y: v / xMax,
		X: v % xMax,
	}
}

func LoadInput(input [][]int) (*graph.Mutable, int) {
	total := len(input) * len(input[0])
	xMax := len(input[0])
	g := graph.New(total)

	for i := 0; i < total; i++ {
		p := VertexToPoint(i, xMax)

		//fmt.Printf("%v -> %v,%v\n", i, p.X, p.Y)

		neighbours := []helpers.Point{
			{X: p.X, Y: p.Y - 1},
			{X: p.X - 1, Y: p.Y}, {X: p.X + 1, Y: p.Y},
			{X: p.X, Y: p.Y + 1},
		}

		for j := len(neighbours) - 1; j >= 0; j-- {
			if neighbours[j].X < 0 || neighbours[j].X >= len(input[0]) || neighbours[j].Y < 0 || neighbours[j].Y >= len(input[0]) {
				continue
			}

			cost := int64(input[neighbours[j].Y][neighbours[j].X])
			vertex := PointToVertex(neighbours[j], xMax)

			//fmt.Printf(" - %v,%v (%v), cost: %v\n", neighbours[j].X, neighbours[j].Y, PointToVertex(neighbours[j], xMax), cost)

			g.AddCost(i, vertex, cost)
		}
	}

	return g, total - 1
}

func ShortestPathCost(g *graph.Mutable, source int, destination int) int64 {
	path, _ := graph.ShortestPath(g, source, destination)

	var totalCost int64

	for i := 0; i < len(path)-1; i++ {
		cost := g.Cost(path[i], path[i+1])
		//log.Printf("%v to %v - cost: %v", path[i], path[i+1], cost)
		totalCost += cost
	}

	return totalCost
}

func ExpandMap(original [][]int, repeat int) [][]int {
	out := make([][]int, len(original)*repeat)

	for y := 0; y < len(original); y++ {
		out[y] = original[y]

		for i := 0; i < repeat-1; i++ {
			out[y] = append(out[y], original[y]...)

			for j := 0; j < len(original[y]); j++ {
				index := (len(original[y]) * (i + 1)) + j
				out[y][index] = (out[y][index]+i)%9 + 1
			}
		}
	}

	for y := len(original); y < len(original)*repeat; y++ {
		out[y] = make([]int, len(original[0])*repeat)
		copy(out[y], out[y-len(original)])

		for i := 0; i < len(out[y]); i++ {
			out[y][i] = (out[y][i])%9 + 1
		}
	}

	return out
}

func main() {
	input := helpers.SplitLinesInt(helpers.GetInputLines("day15/input.txt"))
	g, destination := LoadInput(input)
	fmt.Printf("part 1: %v\n", ShortestPathCost(g, 0, destination))

	input2 := ExpandMap(input, 5)
	g2, dest2 := LoadInput(input2)
	fmt.Printf("part 2: %v\n", ShortestPathCost(g2, 0, dest2))
}
