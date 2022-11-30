package main

import (
	"github.com/google/go-cmp/cmp"
	"testing"
)

func TestStep(t *testing.T) {
	actual := LoadInput([]string{
		"11111",
		"19991",
		"19191",
		"19991",
		"11111",
	})
	actualFlashes := Step(actual)
	expected := LoadInput([]string{
		"34543",
		"40004",
		"50005",
		"40004",
		"34543",
	})
	expectedFlashes := 9

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("Step() = (-want, +got)\n%v", diff)
	}

	if actualFlashes != expectedFlashes {
		t.Errorf("Step() flashes = %v, want %v", actualFlashes, expectedFlashes)
	}
}

func TestStep2(t *testing.T) {
	actual := LoadInput([]string{
		"34543",
		"40004",
		"50005",
		"40004",
		"34543",
	})
	actualFlashes := Step(actual)
	expected := LoadInput([]string{
		"45654",
		"51115",
		"61116",
		"51115",
		"45654",
	})
	expectedFlashes := 0

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("Step() #2 = (-want, +got)\n%v", diff)
	}

	if actualFlashes != expectedFlashes {
		t.Errorf("Step() flashes = %v, want %v", actualFlashes, expectedFlashes)
	}
}

func TestStepLoop(t *testing.T) {
	input := LoadInput([]string{
		"5483143223",
		"2745854711",
		"5264556173",
		"6141336146",
		"6357385478",
		"4167524645",
		"2176841721",
		"6882881134",
		"4846848554",
		"5283751526",
	})
	actualFlashed := StepLoop(input, 10)
	expectedFlashed := 204

	if actualFlashed != expectedFlashed {
		t.Errorf("StepLoop() flashed = %v, want %v", actualFlashed, expectedFlashed)
	}
}
