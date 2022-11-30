package main

import "testing"

func TestPart1Example(t *testing.T) {
	example := []string{
		"forward 5",
		"down 5",
		"forward 8",
		"up 3",
		"down 8",
		"forward 2",
	}
	expected := 150
	actual := CalculateDepth(example)

	if actual != expected {
		t.Errorf("Day 2 Part 1 Example = %d; want %d", actual, expected)
	}
}

func TestPart2Example(t *testing.T) {
	example := []string{
		"forward 5",
		"down 5",
		"forward 8",
		"up 3",
		"down 8",
		"forward 2",
	}
	expected := 900
	actual := CalculateDepthWithAim(example)

	if actual != expected {
		t.Errorf("Day 2 Part 2 Example = %d; want %d", actual, expected)
	}
}
