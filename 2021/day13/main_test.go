package main

import (
	"github.com/golang-collections/collections/set"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"testing"
)

var input = "6,10\n" +
	"0,14\n" +
	"9,10\n" +
	"0,3\n" +
	"10,4\n" +
	"4,11\n" +
	"6,0\n" +
	"6,12\n" +
	"4,1\n" +
	"0,13\n" +
	"10,12\n" +
	"3,4\n" +
	"3,0\n" +
	"8,4\n" +
	"1,10\n" +
	"2,14\n" +
	"8,10\n" +
	"9,0\n" +
	"\n" +
	"fold along y=7\n" +
	"fold along x=5"

func TestLoadPaper(t *testing.T) {
	LoadPaper(helpers.GetLines(input))
}

func TestFoldPaper(t *testing.T) {
	input := LoadPaper(helpers.GetLines(input))
	actual := FoldPaper(input, -1).Points
	expPoints := []helpers.Point{
		{0, 0},
		{1, 0},
		{2, 0},
		{3, 0},
		{4, 0},
		{0, 1},
		{4, 1},
		{0, 2},
		{4, 2},
		{0, 3},
		{4, 3},
		{0, 4},
		{1, 4},
		{2, 4},
		{3, 4},
		{4, 4},
	}
	expected := set.New()
	for _, p := range expPoints {
		expected.Insert(p)
	}

	if diff := actual.Difference(expected); diff.Len() > 0 {
		t.Errorf("FoldPaper() = %v, want %v\ndiff = %v", actual, expected, diff)
	}

	if actual.Len() != expected.Len() {
		t.Errorf("FoldPaper().Len() = %v, want %v", actual.Len(), expected.Len())
	}
}
