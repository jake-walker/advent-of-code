package main

import (
	"github.com/google/go-cmp/cmp"
	"testing"
)

var input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678"

func TestLoadInput(t *testing.T) {
	actual := LoadInput(input)
	expected := [][]int{
		{2, 1, 9, 9, 9, 4, 3, 2, 1, 0},
		{3, 9, 8, 7, 8, 9, 4, 9, 2, 1},
		{9, 8, 5, 6, 7, 8, 9, 8, 9, 2},
		{8, 7, 6, 7, 8, 9, 6, 7, 8, 9},
		{9, 8, 9, 9, 9, 6, 5, 6, 7, 8},
	}

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("LoadInput() =\n%v", diff)
	}
}

func TestGetLowPoints(t *testing.T) {
	heightmap := [][]int{
		{2, 1, 9, 9, 9, 4, 3, 2, 1, 0},
		{3, 9, 8, 7, 8, 9, 4, 9, 2, 1},
		{9, 8, 5, 6, 7, 8, 9, 8, 9, 2},
		{8, 7, 6, 7, 8, 9, 6, 7, 8, 9},
		{9, 8, 9, 9, 9, 6, 5, 6, 7, 8},
	}
	actual := GetLowPoints(heightmap)
	expected := []Point{
		{1, 0},
		{9, 0},
		{2, 2},
		{6, 4},
	}

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("GetLowPoints() =\n%v", diff)
	}
}

func TestGetBasin(t *testing.T) {
	heightmap := [][]int{
		{2, 1, 9, 9, 9, 4, 3, 2, 1, 0},
		{3, 9, 8, 7, 8, 9, 4, 9, 2, 1},
		{9, 8, 5, 6, 7, 8, 9, 8, 9, 2},
		{8, 7, 6, 7, 8, 9, 6, 7, 8, 9},
		{9, 8, 9, 9, 9, 6, 5, 6, 7, 8},
	}
	actual := GetBasin(heightmap, Point{1, 0}, map[Point]bool{})
	expected := map[Point]bool{
		{0, 0}: true,
		{0, 1}: true,
		{1, 0}: true,
	}

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("GetBasin() =\n%v", diff)
	}
}
