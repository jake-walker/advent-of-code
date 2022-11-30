package main

import "testing"

func TestPart1Example(t *testing.T) {
	example := []int{199, 200, 208, 210, 200, 207, 240, 269, 260, 263}
	expected := 7
	actual := GetIncreasedMeasurements(example)

	if actual != expected {
		t.Errorf("Day 1 Part 1 Example = %d; want %d", actual, expected)
	}
}

func TestPart2Example(t *testing.T) {
	example := []int{199, 200, 208, 210, 200, 207, 240, 269, 260, 263}
	expected := 5
	actual := GetIncreasedMeasurementsSliding(example)

	if actual != expected {
		t.Errorf("Day 1 Part 2 Example = %d; want %d", actual, expected)
	}
}
