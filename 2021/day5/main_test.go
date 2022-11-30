package main

import (
	"github.com/google/go-cmp/cmp"
	"github.com/google/go-cmp/cmp/cmpopts"
	"reflect"
	"testing"
)

const input string = "0,9 -> 5,9\n" +
	"8,0 -> 0,8\n" +
	"9,4 -> 3,4\n" +
	"2,2 -> 2,1\n" +
	"7,0 -> 7,4\n" +
	"6,4 -> 2,0\n" +
	"0,9 -> 2,9\n" +
	"3,4 -> 1,4\n" +
	"0,0 -> 8,8\n" +
	"5,5 -> 8,2"

func SliceEqual(a, b []Coordinate) bool {
	sort := func(a, b Coordinate) bool {
		if a.X == b.X {
			return a.Y < b.Y
		}

		return a.X < b.X
	}
	return cmp.Equal(a, b, cmpopts.SortSlices(sort))
}

func TestParseInput(t *testing.T) {
	actual := ParseInput(input, InterpolateStraight)
	expected := map[Coordinate]int{
		{7, 0}: 1,
		{7, 1}: 1,
		{7, 2}: 1,
		{7, 3}: 1,
		{2, 1}: 1,
		{2, 2}: 1,
		{1, 4}: 1,
		{2, 4}: 1,
		{3, 4}: 2,
		{4, 4}: 1,
		{5, 4}: 1,
		{6, 4}: 1,
		{7, 4}: 2,
		{8, 4}: 1,
		{9, 4}: 1,
		{0, 9}: 2,
		{1, 9}: 2,
		{2, 9}: 2,
		{3, 9}: 1,
		{4, 9}: 1,
		{5, 9}: 1,
	}

	if !reflect.DeepEqual(actual, expected) {
		t.Errorf("ParseInput() = %v, want %v", actual, expected)
	}
}

func TestGetOverlaps_Straight(t *testing.T) {
	actual := GetOverlaps(ParseInput(input, InterpolateStraight))
	expected := 5

	if actual != expected {
		t.Errorf("GetOverlaps() = %v, want %v", actual, expected)
	}
}

func TestGetOverlaps_Diagonal(t *testing.T) {
	m := ParseInput(input, InterpolateDiagonal)
	actual := GetOverlaps(m)
	expected := 12

	if actual != expected {
		t.Errorf("GetOverlaps() = %v, want %v", actual, expected)
		PrintMap(m, 10, 10)
	}
}

func TestInterpolateStraight(t *testing.T) {
	type args struct {
		x1 int
		y1 int
		x2 int
		y2 int
	}
	tests := []struct {
		name string
		args args
		want []Coordinate
	}{
		{
			name: "horizontal",
			args: args{
				x1: 0,
				y1: 0,
				x2: 3,
				y2: 0,
			},
			want: []Coordinate{{0, 0}, {1, 0}, {2, 0}, {3, 0}},
		},
		{
			name: "vertical",
			args: args{
				x1: 0,
				y1: 0,
				x2: 0,
				y2: 3,
			},
			want: []Coordinate{{0, 0}, {0, 1}, {0, 2}, {0, 3}},
		},
		{
			name: "diagonal",
			args: args{
				x1: 0,
				y1: 0,
				x2: 3,
				y2: 3,
			},
			want: []Coordinate{},
		},
		{
			name: "diagonal non45",
			args: args{
				x1: 1,
				y1: 0,
				x2: 3,
				y2: 3,
			},
			want: []Coordinate{},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := []Coordinate{}
			InterpolateStraight(tt.args.x1, tt.args.y1, tt.args.x2, tt.args.y2, func(x, y int) {
				got = append(got, Coordinate{x, y})
			})
			if !SliceEqual(got, tt.want) {
				t.Errorf("InterpolateStraight() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestInterpolateDiagonal(t *testing.T) {
	type args struct {
		x1 int
		y1 int
		x2 int
		y2 int
	}
	tests := []struct {
		name string
		args args
		want []Coordinate
	}{
		{
			name: "horizontal",
			args: args{
				x1: 0,
				y1: 0,
				x2: 3,
				y2: 0,
			},
			want: []Coordinate{{0, 0}, {1, 0}, {2, 0}, {3, 0}},
		},
		{
			name: "vertical",
			args: args{
				x1: 0,
				y1: 0,
				x2: 0,
				y2: 3,
			},
			want: []Coordinate{{0, 0}, {0, 1}, {0, 2}, {0, 3}},
		},
		{
			name: "diagonal",
			args: args{
				x1: 0,
				y1: 0,
				x2: 3,
				y2: 3,
			},
			want: []Coordinate{{0, 0}, {1, 1}, {2, 2}, {3, 3}},
		},
		{
			name: "diagonal non45",
			args: args{
				x1: 1,
				y1: 0,
				x2: 3,
				y2: 3,
			},
			want: []Coordinate{},
		},
		{
			name: "diagonal neg45",
			args: args{
				x1: 8,
				y1: 2,
				x2: 5,
				y2: 5,
			},
			want: []Coordinate{{5, 5}, {6, 4}, {7, 3}, {8, 2}},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := []Coordinate{}
			InterpolateDiagonal(tt.args.x1, tt.args.y1, tt.args.x2, tt.args.y2, func(x, y int) {
				got = append(got, Coordinate{x, y})
			})
			if !SliceEqual(got, tt.want) {
				t.Errorf("InterpolateDiagonal() = %v, want %v", got, tt.want)
			}
		})
	}
}
