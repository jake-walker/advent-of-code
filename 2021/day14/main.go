package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strings"
)

func LoadInput(input []string) (map[string]int, map[string]string) {
	pairs := make(map[string]int)
	pairInsertions := make(map[string]string)

	first := strings.Split(input[0], "")
	for i := 0; i < len(first)-1; i++ {
		pair := first[i : i+2]
		pairs[strings.Join(pair, "")] += 1
	}

	for i := 1; i < len(input); i++ {
		if input[i] == "" {
			continue
		}

		fields := strings.Fields(input[i])
		pairInsertions[fields[0]] = fields[2]
	}

	return pairs, pairInsertions
}

func InsertPairs(template map[string]int, pairs map[string]string) map[string]int {
	newTemplate := make(map[string]int)

	for pair, count := range template {
		split := strings.Split(pair, "")

		toInsert, found := pairs[pair]
		if !found {
			log.Fatalf("could not find insertion for pair %v", pair)
		}

		newPair1 := split[0] + toInsert
		newPair2 := toInsert + split[1]

		newTemplate[newPair1] += count
		newTemplate[newPair2] += count
	}

	return newTemplate
}

func InsertLoop(template map[string]int, pairs map[string]string, n int) int {
	for i := 0; i < n; i++ {
		template = InsertPairs(template, pairs)
	}

	counts := make(map[string]int)
	for pair, count := range template {
		split := strings.Split(pair, "")
		counts[split[0]] += count
		counts[split[1]] += count
	}

	values := make([]int, 0, len(counts))
	for _, value := range counts {
		half := value / 2
		// add one onto the half if it was an odd number
		if value%2 != 0 {
			half += 1
		}

		values = append(values, half)
	}
	min, max := helpers.MinMax(values)

	return max - min
}

func main() {
	input := helpers.GetInputLines("day14/input.txt")
	template, pairs := LoadInput(input)

	part1 := InsertLoop(template, pairs, 10)
	fmt.Printf("part 1: %v\n", part1)

	part2 := InsertLoop(template, pairs, 40)
	fmt.Printf("part 2: %v\n", part2)
	// taking away 1 from this gives the right answer, something is wrong in the code!
}
