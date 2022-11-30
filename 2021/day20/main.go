package main

import (
	"fmt"
	"github.com/golang-collections/collections/set"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strconv"
	"strings"
)

type Range struct {
	From int
	To   int
}

type Input struct {
	Algorithm string
	Image     *set.Set
	XRange    Range
	YRange    Range
	Default   bool
}

func (i *Input) DefaultChar() string {
	if i.Default {
		return "#"
	} else {
		return "."
	}
}

func (i *Input) PrintImage() {
	if (i.YRange.To - i.YRange.From) > 20 {
		return
	}

	fmt.Printf("\n%v\n%v%v%v\n", strings.Repeat(i.DefaultChar(), (i.YRange.To-i.YRange.From)+5), i.DefaultChar(), strings.Repeat(" ", (i.YRange.To-i.YRange.From)+3), i.DefaultChar())
	for y := i.YRange.From; y <= i.YRange.To; y++ {
		fmt.Printf("%v ", i.DefaultChar())
		for x := i.XRange.From; x <= i.XRange.To; x++ {
			if i.Image.Has(helpers.Point{X: x, Y: y}) {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Printf(" %v\n", i.DefaultChar())
	}
	fmt.Printf("%v%v%v\n%v\n", i.DefaultChar(), strings.Repeat(" ", (i.YRange.To-i.YRange.From)+3), i.DefaultChar(), strings.Repeat(i.DefaultChar(), (i.YRange.To-i.YRange.From)+5))
}

func ParseInput(input string) Input {
	lines := helpers.GetLines(input)
	image := set.New()

	for y := 0; y < len(lines)-2; y++ {
		line := lines[y+2]
		for x := 0; x < len(line); x++ {
			if line[x] == '#' {
				image.Insert(helpers.Point{X: x, Y: y})
			}
		}
	}

	return Input{
		Algorithm: lines[0],
		Image:     image,
		YRange: Range{
			From: 0,
			To:   len(lines) - 2,
		},
		XRange: Range{
			From: 0,
			To:   len(lines[2]),
		},
		Default: false,
	}
}

func Enhance(input Input) Input {
	newImage := set.New()

	for y := input.YRange.From - 1; y <= input.YRange.To+1; y++ {
		for x := input.XRange.From - 1; x <= input.XRange.To+1; x++ {
			neighbours := []helpers.Point{
				{x - 1, y - 1}, {x, y - 1}, {x + 1, y - 1},
				{x - 1, y}, {x, y}, {x + 1, y},
				{x - 1, y + 1}, {x, y + 1}, {x + 1, y + 1},
			}
			inputBinary := ""

			for _, point := range neighbours {
				if point.Y < input.YRange.From || point.Y > input.YRange.To || point.X < input.XRange.From || point.X > input.XRange.To {
					if input.Default {
						inputBinary += "1"
					} else {
						inputBinary += "0"
					}
					continue
				}

				if input.Image.Has(point) {
					inputBinary += "1"
				} else {
					inputBinary += "0"
				}
			}

			index, err := strconv.ParseUint(inputBinary, 2, 64)
			if err != nil {
				log.Fatalf("failed to convert binary to int: %v", err)
			}

			if input.Algorithm[index] == '#' {
				newImage.Insert(helpers.Point{X: x, Y: y})
			}
		}
	}

	def := input.Default
	if input.Default {
		def = input.Algorithm[511] == '#'
	} else {
		def = input.Algorithm[0] == '#'
	}

	return Input{
		Algorithm: input.Algorithm,
		Image:     newImage,
		XRange:    Range{From: input.XRange.From - 1, To: input.XRange.To + 1},
		YRange:    Range{From: input.YRange.From - 1, To: input.YRange.To + 1},
		Default:   def,
	}
}

func EnhanceCount(input Input, count int) int {
	input.PrintImage()
	for i := 0; i < count; i++ {
		input = Enhance(input)
		input.PrintImage()
	}
	return input.Image.Len()
}

func main() {
	input := helpers.GetInput("day20/input.txt")
	fmt.Printf("part 1: %v\n", EnhanceCount(ParseInput(input), 2))
	fmt.Printf("part 2: %v\n", EnhanceCount(ParseInput(input), 50))
}
