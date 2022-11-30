package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strconv"
	"strings"
)

func CalculateDepth(instructions []string) int {
	depth := 0
	horizontalPosition := 0

	for _, line := range instructions {
		instruction := strings.Split(line, " ")
		value, err := strconv.Atoi(instruction[1])
		if err != nil {
			log.Printf("failed to parse value %v: %v", instructions[1], err)
			continue
		}

		switch instruction[0] {
		case "forward":
			horizontalPosition += value
			break
		case "down":
			depth += value
			break
		case "up":
			depth -= value
			break
		default:
			log.Printf("invalid instruction %v", instructions[0])
			continue
		}
	}

	return depth * horizontalPosition
}

func CalculateDepthWithAim(instructions []string) int {
	depth := 0
	horizontalPosition := 0
	aim := 0

	for i, line := range instructions {
		instruction := strings.Split(line, " ")
		value, err := strconv.Atoi(instruction[1])
		if err != nil {
			log.Printf("failed to parse value %v: %v", instructions[1], err)
			continue
		}

		switch instruction[0] {
		case "forward":
			horizontalPosition += value
			depth += aim * value
			break
		case "down":
			aim += value
			break
		case "up":
			aim -= value
			break
		default:
			log.Printf("invalid instruction %v", instructions[0])
			continue
		}

		fmt.Printf("%v: %v -> hor=%v, dep=%v, aim=%v\n", i, line, horizontalPosition, depth, aim)
	}

	return horizontalPosition * depth
}

func main() {
	input := helpers.GetInputLines("day2/input.txt")

	fmt.Printf("Part 1: %v\n", CalculateDepth(input))
	fmt.Printf("Part 2: %v\n", CalculateDepthWithAim(input))
}
