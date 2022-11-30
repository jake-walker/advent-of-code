package main

import (
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"reflect"
	"testing"
)

func TestCommonDigitRegular(t *testing.T) {
	example := helpers.SplitLines([]string{
		"00100",
		"11110",
		"10110",
		"10111",
		"10101",
		"01111",
		"00111",
		"11100",
		"10000",
		"11001",
		"00010",
		"01010",
	})
	expected := "10110"
	actual := CalculateCommonDigit(example, false)

	if actual != expected {
		t.Errorf("Day 3 CalculateCommonDigit = %v; want %v", actual, expected)
	}
}

func TestCommonDigitInverse(t *testing.T) {
	example := helpers.SplitLines([]string{
		"00100",
		"11110",
		"10110",
		"10111",
		"10101",
		"01111",
		"00111",
		"11100",
		"10000",
		"11001",
		"00010",
		"01010",
	})
	expected := "01001"
	actual := CalculateCommonDigit(example, true)

	if actual != expected {
		t.Errorf("Day 3 CalculateCommonDigitInverse = %v; want %v", actual, expected)
	}
}

func TestPart1Example(t *testing.T) {
	example := helpers.SplitLines([]string{
		"00100",
		"11110",
		"10110",
		"10111",
		"10101",
		"01111",
		"00111",
		"11100",
		"10000",
		"11001",
		"00010",
		"01010",
	})
	var expected int64 = 198
	actual := CalculatePowerConsumption(example)

	if actual != expected {
		t.Errorf("Day 3 Part 1 Example = %v; want %v", actual, expected)
	}
}

func TestEliminateNumbers(t *testing.T) {
	example := helpers.SplitLines([]string{
		"00100",
		"11110",
		"10110",
		"10111",
		"10101",
		"01111",
		"00111",
		"11100",
		"10000",
		"11001",
		"00010",
		"01010",
	})
	expected := []string{"1", "0", "1", "1", "1"}
	actual := EliminateNumbers(example, false, 0)

	if !reflect.DeepEqual(actual[0], expected) {
		t.Errorf("Day 3 Part 1 Example = %v; want %v", actual[0], expected)
	}
}

func TestEliminateNumbersInverse(t *testing.T) {
	example := helpers.SplitLines([]string{
		"00100",
		"11110",
		"10110",
		"10111",
		"10101",
		"01111",
		"00111",
		"11100",
		"10000",
		"11001",
		"00010",
		"01010",
	})
	expected := []string{"0", "1", "0", "1", "0"}
	actual := EliminateNumbers(example, true, 0)

	if !reflect.DeepEqual(actual[0], expected) {
		t.Errorf("Day 3 Part 1 Example = %v; want %v", actual[0], expected)
	}
}

func TestPart2Example(t *testing.T) {
	example := helpers.SplitLines([]string{
		"00100",
		"11110",
		"10110",
		"10111",
		"10101",
		"01111",
		"00111",
		"11100",
		"10000",
		"11001",
		"00010",
		"01010",
	})
	var expected int64 = 230
	actual := CalculateLifeSupportRating(example)

	if actual != expected {
		t.Errorf("Day 3 Part 2 Example = %v; want %v", actual, expected)
	}
}
