package main

import (
	"fmt"
	"github.com/golang-collections/collections/stack"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"sort"
)

func CheckLine(chars []string) (int, *stack.Stack) {
	s := stack.New()

	for i, char := range chars {
		// check if this character is an opening one
		switch char {
		case "(":
			s.Push(")")
			continue
		case "[":
			s.Push("]")
			continue
		case "{":
			s.Push("}")
			continue
		case "<":
			s.Push(">")
			continue
		}

		// otherwise, this character is a closing one, and we should check whether the top of the stack contains the next
		// character
		expected, ok := (s.Pop()).(string)
		if !ok {
			log.Fatalf("got data of type %T but wanted string", s.Peek())
		}

		if expected != char {
			//log.Printf("expected character '%v' but got '%v' @ pos %v", expected, char, i)
			return i, s
		}
	}

	return -1, s
}

func ScoreLines(lines [][]string) int {
	total := 0
	for _, line := range lines {
		pos, _ := CheckLine(line)

		if pos == -1 {
			continue
		}

		switch line[pos] {
		case ")":
			total += 3
		case "]":
			total += 57
		case "}":
			total += 1197
		case ">":
			total += 25137
		}
	}
	return total
}

func CompleteLines(lines [][]string) []int {
	scores := []int{}

	for _, chars := range lines {
		score := 0
		pos, s := CheckLine(chars)

		if pos != -1 {
			continue
		}

		// add the rest of the stack items to the end
		for {
			item := s.Pop()

			if item == nil {
				break
			}

			char, ok := item.(string)
			if !ok {
				log.Fatalf("got data of type %T but wanted string", s.Peek())
			}

			score *= 5
			switch char {
			case ")":
				score += 1
			case "]":
				score += 2
			case "}":
				score += 3
			case ">":
				score += 4
			}
		}

		scores = append(scores, score)
	}

	return scores
}

func Median(numbers []int) int {
	sort.Ints(numbers)
	middle := len(numbers) / 2
	return numbers[middle]
}

func main() {
	input := helpers.SplitLines(helpers.GetInputLines("day10/input.txt"))

	part1 := ScoreLines(input)
	fmt.Printf("part 1: %v\n", part1)

	part2 := Median(CompleteLines(input))
	fmt.Printf("part 2: %v\n", part2)
}
