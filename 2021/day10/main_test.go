package main

import (
	"github.com/google/go-cmp/cmp"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"strings"
	"testing"
)

var input = "[({(<(())[]>[[{[]{<()<>>\n" +
	"[(()[<>])]({[<{<<[]>>(\n" +
	"{([(<{}[<>[]}>{[]{[(<()>\n" +
	"(((({<>}<{<{<>}{[]{[]{}\n" +
	"[[<[([]))<([[{}[[()]]]\n" +
	"[{[{({}]{}}([{[{{{}}([]\n" +
	"{<[[]]>}<{[{[{[]{()[[[]\n" +
	"[<(<(<(<{}))><([]([]()\n" +
	"<{([([[(<>()){}]>(<<{{\n" +
	"<{([{{}}[<[[[<>{}]]]>[]]"

func TestCheckLine(t *testing.T) {
	tests := []struct {
		name string
		line []string
		want int
	}{
		{
			name: "invalid1",
			line: strings.Split("{([(<{}[<>[]}>{[]{[(<()>", ""),
			want: 12,
		},
		{
			name: "invalid2",
			line: strings.Split("[[<[([]))<([[{}[[()]]]", ""),
			want: 8,
		},
		{
			name: "invalid3",
			line: strings.Split("[{[{({}]{}}([{[{{{}}([]", ""),
			want: 7,
		},
		{
			name: "valid1",
			line: strings.Split("(((((((((())))))))))", ""),
			want: -1,
		},
		{
			name: "valid2",
			line: strings.Split("[<>({}){}[([])<>]]", ""),
			want: -1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got, _ := CheckLine(tt.line); got != tt.want {
				t.Errorf("CheckLine() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestScoreLines(t *testing.T) {
	inputLines := helpers.SplitLines(helpers.GetLines(input))

	actual := ScoreLines(inputLines)
	expected := 26397

	if actual != expected {
		t.Errorf("ScoreLines() = %v, want %v", actual, expected)
	}
}

func TestCompleteLines(t *testing.T) {
	inputLines := helpers.SplitLines(helpers.GetLines(input))

	actual := CompleteLines(inputLines)
	expected := []int{288957, 5566, 1480781, 995444, 294}

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("CompleteLines() =\n%v", diff)
	}
}
