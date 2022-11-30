package main

import (
	"github.com/golang-collections/collections/set"
	"github.com/google/go-cmp/cmp"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"strings"
	"testing"
)

var input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n" +
	"\n" +
	"#..#.\n" +
	"#....\n" +
	"##..#\n" +
	"..#..\n" +
	"..###"

func SetComparer() cmp.Option {
	return cmp.Comparer(func(a, b *set.Set) bool {
		return a.Difference(b).Len() == 0 && b.Difference(a).Len() == 0
	})
}

func TestParseInput(t *testing.T) {
	actual := ParseInput(input)
	expected := Input{
		Algorithm: "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
		Image:     set.New(),
		XRange: Range{
			From: 0,
			To:   5,
		},
		YRange: Range{
			From: 0,
			To:   5,
		},
	}

	points := []helpers.Point{
		{0, 0},
		{3, 0},
		{0, 1},
		{0, 2},
		{1, 2},
		{4, 2},
		{2, 3},
		{2, 4},
		{3, 4},
		{4, 4},
	}

	for _, point := range points {
		expected.Image.Insert(point)
	}

	if diff := cmp.Diff(actual, expected, SetComparer()); diff != "" {
		t.Errorf("ParseInput() (-want, +got) =\n%v", diff)
	}
}

func TestEnhance_Overlap(t *testing.T) {
	testAlgo := "#" + strings.Repeat(".", 510) + "."

	actual := Enhance(Input{
		Algorithm: testAlgo,
		Image:     set.New(),
		XRange:    Range{0, 3},
		YRange:    Range{0, 3},
		Default:   false,
	})

	expPoints := set.New()
	for x := -1; x <= 4; x++ {
		for y := -1; y <= 4; y++ {
			expPoints.Insert(helpers.Point{x, y})
		}
	}

	expected := Input{
		Algorithm: testAlgo,
		Image:     expPoints,
		XRange:    Range{-1, 4},
		YRange:    Range{-1, 4},
		Default:   true,
	}

	if diff := cmp.Diff(actual, expected, SetComparer()); diff != "" {
		t.Errorf("Enhance() (-want, +got) =\n%v", diff)
	}
}

func TestEnhanceCount2(t *testing.T) {
	actual := EnhanceCount(ParseInput(input), 2)
	expected := 35

	if actual != expected {
		t.Errorf("EnhanceCount() = %v, want %v", actual, expected)
	}
}

func TestEnhanceCount50(t *testing.T) {
	actual := EnhanceCount(ParseInput(input), 50)
	expected := 3351

	if actual != expected {
		t.Errorf("EnhanceCount() = %v, want %v", actual, expected)
	}
}
