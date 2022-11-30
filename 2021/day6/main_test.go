package main

import (
	"testing"
)

func TestCalculateFishTotal(t *testing.T) {
	fish := []int{3, 4, 3, 1, 2}
	actual := CalculateFishTotal(fish, 18)
	expected := 26

	if actual != expected {
		t.Errorf("CalculateFishTotal() = %v, want %v", actual, expected)
	}
}
