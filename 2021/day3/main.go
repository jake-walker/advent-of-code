package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strconv"
	"strings"
)

// RotateSlice rotates a 2D array so that the columns are rows and the rows are columns
func RotateSlice(slice [][]string) [][]string {
	out := make([][]string, len(slice[0]))

	for i := 0; i < len(slice[0]); i++ {
		out[i] = make([]string, len(slice))

		for j := 0; j < len(slice); j++ {
			out[i][j] = slice[j][i]
		}
	}
	return out
}

func CalculateCommonDigit(report [][]string, inverse bool) string {
	output := ""

	// rotating the slice means it is easy to use strings.Count to get the occurrences of a digit
	for _, column := range RotateSlice(report) {
		columnStr := strings.Join(column, "")
		ones := strings.Count(columnStr, "1")
		zeroes := strings.Count(columnStr, "0")

		// if this is reversed, swap the ones and zeroes around
		if inverse {
			ones, zeroes = zeroes, ones
		}

		if ones > zeroes {
			output += "1"
		} else {
			output += "0"
		}
	}

	return output
}

// Part 1
func CalculatePowerConsumption(report [][]string) int64 {
	gammaBin := CalculateCommonDigit(report, false)
	epsilonBin := CalculateCommonDigit(report, true)

	// convert the numbers from binary to decimals
	gamma, err := strconv.ParseInt(gammaBin, 2, 64)
	if err != nil {
		log.Fatalf("failed to convert gamma (%v) from binary to int: %v", gammaBin, err)
	}
	epsilon, err := strconv.ParseInt(epsilonBin, 2, 64)
	if err != nil {
		log.Fatalf("failed to convert epsilon (%v) from binary to int: %v", epsilonBin, err)
	}

	return gamma * epsilon
}

// EliminateNumbers removes the numbers which do not have the most common digit in the given position
func EliminateNumbers(numbers [][]string, inverse bool, position int) [][]string {
	// calculate the number of ones and zeroes in the column
	rotated := RotateSlice(numbers)
	col := strings.Join(rotated[position], "")
	ones := strings.Count(col, "1")
	zeroes := strings.Count(col, "0")

	// the digit that is being eliminated
	eliminate := "0"
	if zeroes > ones {
		eliminate = "1"
	}

	if inverse && eliminate == "0" {
		eliminate = "1"
	} else if inverse && eliminate == "1" {
		eliminate = "0"
	}

	// create a new slice, only adding numbers to it that aren't eliminated
	output := [][]string{}
	for _, number := range numbers {
		if number[position] != eliminate {
			output = append(output, number)
		}
	}

	// if there is more than one result left, move onto the next digit
	if len(output) > 1 {
		return EliminateNumbers(output, inverse, position+1)
	}

	return output
}

// Part 2
func CalculateLifeSupportRating(report [][]string) int64 {
	oxygenRatingBin := strings.Join(EliminateNumbers(report, false, 0)[0], "")
	co2RatingBin := strings.Join(EliminateNumbers(report, true, 0)[0], "")

	oxygenRating, err := strconv.ParseInt(oxygenRatingBin, 2, 64)
	if err != nil {
		log.Fatalf("failed to convert oxygen rating (%v) from binary to int: %v", oxygenRating, err)
	}
	co2Rating, err := strconv.ParseInt(co2RatingBin, 2, 64)
	if err != nil {
		log.Fatalf("failed to convert co2 rating (%v) from binary to int: %v", co2Rating, err)
	}

	return oxygenRating * co2Rating
}

func main() {
	report := helpers.SplitLines(helpers.GetInputLines("day3/input.txt"))

	fmt.Printf("part 1: %v\n", CalculatePowerConsumption(report))
	fmt.Printf("part 2: %v\n", CalculateLifeSupportRating(report))
}
