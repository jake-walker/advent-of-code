package main

import (
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"testing"
)

var input = "on x=10..12,y=10..12,z=10..12\n" +
	"on x=11..13,y=11..13,z=11..13\n" +
	"off x=9..11,y=9..11,z=9..11\n" +
	"on x=10..10,y=10..10,z=10..10"

var inputLarge = "on x=-20..26,y=-36..17,z=-47..7\n" +
	"on x=-20..33,y=-21..23,z=-26..28\n" +
	"on x=-22..28,y=-29..23,z=-38..16\n" +
	"on x=-46..7,y=-6..46,z=-50..-1\n" +
	"on x=-49..1,y=-3..46,z=-24..28\n" +
	"on x=2..47,y=-22..22,z=-23..27\n" +
	"on x=-27..23,y=-28..26,z=-21..29\n" +
	"on x=-39..5,y=-6..47,z=-3..44\n" +
	"on x=-30..21,y=-8..43,z=-13..34\n" +
	"on x=-22..26,y=-27..20,z=-29..19\n" +
	"off x=-48..-32,y=26..41,z=-47..-37\n" +
	"on x=-12..35,y=6..50,z=-50..-2\n" +
	"off x=-48..-32,y=-32..-16,z=-15..-5\n" +
	"on x=-18..26,y=-33..15,z=-7..46\n" +
	"off x=-40..-22,y=-38..-28,z=23..41\n" +
	"on x=-16..35,y=-41..10,z=-47..6\n" +
	"off x=-32..-23,y=11..30,z=-14..3\n" +
	"on x=-49..-5,y=-3..45,z=-29..18\n" +
	"off x=18..30,y=-20..-8,z=-3..13\n" +
	"on x=-41..9,y=-7..43,z=-33..15\n" +
	"on x=-54112..-39298,y=-85059..-49293,z=-27449..7877\n" +
	"on x=967..23432,y=45373..81175,z=27513..53682"

func TestLoadInput(t *testing.T) {
	actual := LoadInput(helpers.GetLines(input)).Len()
	expected := 39

	if actual != expected {
		t.Errorf("LoadInput() = %v, want %v", actual, expected)
	}
}

func TestLoadInput2(t *testing.T) {
	actual := LoadInput(helpers.GetLines(inputLarge)).Len()
	expected := 590784

	if actual != expected {
		t.Errorf("LoadInput() = %v, want %v", actual, expected)
	}
}

func TestSetBounds(t *testing.T) {
	type args struct {
		from int
		to   int
		min  int
		max  int
	}
	tests := []struct {
		name  string
		args  args
		want  int
		want1 int
	}{
		{
			"in bounds",
			args{
				from: -50,
				to:   50,
				min:  -50,
				max:  50,
			},
			-50,
			50,
		},
		{
			"from over",
			args{
				from: -51,
				to:   50,
				min:  -50,
				max:  50,
			},
			-50,
			50,
		},
		{
			"to over",
			args{
				from: -50,
				to:   51,
				min:  -50,
				max:  50,
			},
			-50,
			50,
		},
		{
			"both over",
			args{
				from: -100,
				to:   100,
				min:  -25,
				max:  25,
			},
			-25,
			25,
		},
		{
			"in bounds 1",
			args{
				from: -10,
				to:   10,
				min:  -100,
				max:  100,
			},
			-10,
			10,
		},
		{
			"fully out",
			args{
				from: -100,
				to:   -50,
				min:  -10,
				max:  10,
			},
			1,
			0,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, got1 := SetBounds(tt.args.from, tt.args.to, tt.args.min, tt.args.max)
			if got != tt.want {
				t.Errorf("SetBounds() got = %v, want %v", got, tt.want)
			}
			if got1 != tt.want1 {
				t.Errorf("SetBounds() got1 = %v, want %v", got1, tt.want1)
			}
		})
	}
}
