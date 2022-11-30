package main

import (
	"github.com/google/go-cmp/cmp"
	"testing"
)

var input = "Player 1 starting position: 4\nPlayer 2 starting position: 8"

func TestNewGame(t *testing.T) {
	actual := NewGame(input)
	expected := []int{4, 8}

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("NewGame() = \n%v", diff)
	}
}

func TestPlayDeterministicDie(t *testing.T) {
	actual := PlayDeterministicDie([]int{4, 8})
	expected := 739785

	if actual != expected {
		t.Errorf("PlayDeterministicDie() = %v, want %v", actual, expected)
	}
}

func TestPlayQuantumDie(t *testing.T) {
	actualWins := []int{0, 0}
	expectedWins := []int{444356092776315, 341960390180808}

	PlayQuantumDie([]int{4, 8}, []int{0, 0}, actualWins, 0, 1)

	if diff := cmp.Diff(actualWins, expectedWins); diff != "" {
		t.Errorf("PlayQuantumDie() =\n%v", diff)
	}
}
