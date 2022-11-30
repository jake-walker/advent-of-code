package main

import (
	"github.com/google/go-cmp/cmp"
	"testing"
)

func TestLoadPositions(t *testing.T) {
	actual := LoadPositions([]int{16, 1, 2, 0, 4, 2, 7, 1, 2, 14})
	expected := map[int]int{
		16: 1,
		1:  2,
		2:  3,
		0:  1,
		4:  1,
		7:  1,
		14: 1,
	}

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("LoadPositions() -> %v", diff)
	}
}

func TestCalculateFuel(t *testing.T) {
	positions := LoadPositions([]int{16, 1, 2, 0, 4, 2, 7, 1, 2, 14})
	actualFuel, actualPosition := CalculateFuel(positions)
	expectedFuel, expectedPosition := 37, 2

	if actualFuel != expectedFuel {
		t.Errorf("CalculateFuel() fuel = %v, want %v", actualFuel, expectedFuel)
	}

	if actualPosition != expectedPosition {
		t.Errorf("CalculateFuel() position = %v, want %v", actualPosition, expectedPosition)
	}
}

func TestCalculateFuel2(t *testing.T) {
	actualFuel, actualPosition := CalculateFuel2([]int{16, 1, 2, 0, 4, 2, 7, 1, 2, 14})
	expectedFuel, expectedPosition := 168, 5

	if actualFuel != expectedFuel {
		t.Errorf("CalculateFuel2() fuel = %v, want %v", actualFuel, expectedFuel)
	}

	if actualPosition != expectedPosition {
		t.Errorf("CalculateFuel2() position = %v, want %v", actualPosition, expectedPosition)
	}
}
