package main

import (
	"reflect"
	"testing"
)

const example string = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n" +
	"\n" +
	"22 13 17 11  0\n" +
	" 8  2 23  4 24\n" +
	"21  9 14 16  7\n" +
	" 6 10  3 18  5\n" +
	" 1 12 20 15 19\n" +
	"\n" +
	" 3 15  0  2 22\n" +
	" 9 18 13 17  5\n" +
	"19  8  7 25 23\n" +
	"20 11 10 24  4\n" +
	"14 21 16 12  6\n" +
	"\n" +
	"14 21 17 24  4\n" +
	"10 16 15  9 19\n" +
	"18  8 23 26 20\n" +
	"22 11 13  6  5\n" +
	" 2  0 12  3  7"

func TestLoadInput(t *testing.T) {
	expected := Game{
		numbers: []int{7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1},
		boards: []Board{
			{
				spaces: [][]BoardSpace{
					{{22, false}, {13, false}, {17, false}, {11, false}, {0, false}},
					{{8, false}, {2, false}, {23, false}, {4, false}, {24, false}},
					{{21, false}, {9, false}, {14, false}, {16, false}, {7, false}},
					{{6, false}, {10, false}, {3, false}, {18, false}, {5, false}},
					{{1, false}, {12, false}, {20, false}, {15, false}, {19, false}},
				},
			},
			{
				spaces: [][]BoardSpace{
					{{3, false}, {15, false}, {0, false}, {2, false}, {22, false}},
					{{9, false}, {18, false}, {13, false}, {17, false}, {5, false}},
					{{19, false}, {8, false}, {7, false}, {25, false}, {23, false}},
					{{20, false}, {11, false}, {10, false}, {24, false}, {4, false}},
					{{14, false}, {21, false}, {16, false}, {12, false}, {6, false}},
				},
			},
			{
				spaces: [][]BoardSpace{
					{{14, false}, {21, false}, {17, false}, {24, false}, {4, false}},
					{{10, false}, {16, false}, {15, false}, {9, false}, {19, false}},
					{{18, false}, {8, false}, {23, false}, {26, false}, {20, false}},
					{{22, false}, {11, false}, {13, false}, {6, false}, {5, false}},
					{{2, false}, {0, false}, {12, false}, {3, false}, {7, false}},
				},
			},
		},
	}
	actual := LoadInput(example)

	if !reflect.DeepEqual(expected, actual) {
		t.Errorf("Day 4 LoadInput = %v; want %v", actual, expected)
	}
}

func TestBoard_HasWin(t *testing.T) {
	tests := []struct {
		name   string
		spaces [][]BoardSpace
		want   bool
	}{
		{
			name: "top row",
			spaces: [][]BoardSpace{
				{{0, true}, {0, true}, {0, true}},
				{{0, false}, {0, false}, {0, false}},
				{{0, false}, {0, false}, {0, false}},
			},
			want: true,
		},
		{
			name: "bottom row",
			spaces: [][]BoardSpace{
				{{0, false}, {0, false}, {0, false}},
				{{0, false}, {0, false}, {0, false}},
				{{0, true}, {0, true}, {0, true}},
			},
			want: true,
		},
		{
			name: "left col",
			spaces: [][]BoardSpace{
				{{0, true}, {0, false}, {0, false}},
				{{0, true}, {0, false}, {0, false}},
				{{0, true}, {0, false}, {0, false}},
			},
			want: true,
		},
		{
			name: "right col",
			spaces: [][]BoardSpace{
				{{0, false}, {0, false}, {0, true}},
				{{0, false}, {0, false}, {0, true}},
				{{0, false}, {0, false}, {0, true}},
			},
			want: true,
		},
		{
			name: "diagonal",
			spaces: [][]BoardSpace{
				{{0, true}, {0, false}, {0, false}},
				{{0, false}, {0, true}, {0, false}},
				{{0, false}, {0, false}, {0, true}},
			},
			want: false,
		},
		{
			name: "empty",
			spaces: [][]BoardSpace{
				{{0, false}, {0, false}, {0, false}},
				{{0, false}, {0, false}, {0, false}},
				{{0, false}, {0, false}, {0, false}},
			},
			want: false,
		},
		{
			name: "full",
			spaces: [][]BoardSpace{
				{{0, true}, {0, true}, {0, true}},
				{{0, true}, {0, true}, {0, true}},
				{{0, true}, {0, true}, {0, true}},
			},
			want: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			b := &Board{
				spaces: tt.spaces,
			}
			if got := b.HasWin(); got != tt.want {
				t.Errorf("HasWin() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestBoard_SumUnmarked(t *testing.T) {
	board := Board{
		spaces: [][]BoardSpace{
			{{14, true}, {21, true}, {17, true}, {24, true}, {4, true}},
			{{10, false}, {16, false}, {15, false}, {9, true}, {19, false}},
			{{18, false}, {8, false}, {23, true}, {26, false}, {20, false}},
			{{22, false}, {11, true}, {13, false}, {6, false}, {5, true}},
			{{2, true}, {0, true}, {12, false}, {3, false}, {7, true}},
		},
	}
	actual := board.SumUnmarked()
	expected := 188

	if actual != expected {
		t.Errorf("SumUnmarked() = %v, want %v", actual, expected)
	}
}

func TestPlay(t *testing.T) {
	game := LoadInput(example)
	actualWinningBoard, actualLastNumber := Play(game)
	expectedWinningBoard, expectedLastNumber := Board{
		spaces: [][]BoardSpace{
			{{14, true}, {21, true}, {17, true}, {24, true}, {4, true}},
			{{10, false}, {16, false}, {15, false}, {9, true}, {19, false}},
			{{18, false}, {8, false}, {23, true}, {26, false}, {20, false}},
			{{22, false}, {11, true}, {13, false}, {6, false}, {5, true}},
			{{2, true}, {0, true}, {12, false}, {3, false}, {7, true}},
		},
	}, 24

	if !reflect.DeepEqual(actualWinningBoard, expectedWinningBoard) {
		t.Errorf("Day 4 Play() = board %v, want board %v", actualWinningBoard, expectedWinningBoard)
	}

	if actualLastNumber != expectedLastNumber {
		t.Errorf("Day 4 Play() = last number %v, want last number %v", actualLastNumber, expectedLastNumber)
	}

	actualAnswer := actualWinningBoard.SumUnmarked() * actualLastNumber
	expectedAnswer := 4512

	if actualAnswer != expectedAnswer {
		t.Errorf("Day 4 Unmarked * Last Number = %v, want %v", actualAnswer, expectedAnswer)
	}
}

func TestPlayLast(t *testing.T) {
	game := LoadInput(example)
	actualLosingBoard, actualLastNumber := PlayLast(game)
	actualLosingSum := actualLosingBoard.SumUnmarked()
	expectedLosingSum, expectedLastNumber := 148, 13

	if actualLastNumber != expectedLastNumber {
		t.Errorf("Day 4 PlayLast() = last number %v, want last number %v", actualLastNumber, expectedLastNumber)
	}

	if actualLosingSum != expectedLosingSum {
		t.Errorf("Day 4 PlayLast() = losing sum %v, want losing sum %v", actualLosingSum, expectedLosingSum)
	}

	actualAnswer := actualLosingSum * actualLastNumber
	expectedAnswer := 1924

	if actualAnswer != expectedAnswer {
		t.Errorf("Day 4 Losing Sum * Last Number = %v, want %v", actualAnswer, expectedAnswer)
	}
}
