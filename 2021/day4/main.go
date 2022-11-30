package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strings"
)

// BoardSpace is a struct for storing one 'box' on a bingo board, the number and whether that number is marked
type BoardSpace struct {
	number int
	marked bool
}

type Board struct {
	spaces [][]BoardSpace
}

type Game struct {
	numbers []int
	boards  []Board
}

// HasWin checks whether a board has won
func (b *Board) HasWin() bool {
	// check for rows
	for i := 0; i < len(b.spaces); i++ {
		for j := 0; j < len(b.spaces[i]); j++ {
			if b.spaces[i][j].marked != true {
				break
			}

			if j >= (len(b.spaces[i]) - 1) {
				return true
			}

			continue
		}
	}

	// check for columns
	for i := 0; i < len(b.spaces[0]); i++ {
		for j := 0; j < len(b.spaces); j++ {
			if b.spaces[j][i].marked != true {
				break
			}

			if j >= (len(b.spaces) - 1) {
				return true
			}

			continue
		}
	}

	return false
}

// MarkNumber marks the numbers on the board that match the given number
func (b *Board) MarkNumber(num int) {
	for i := 0; i < len(b.spaces); i++ {
		for j := 0; j < len(b.spaces[i]); j++ {
			if b.spaces[i][j].number == num {
				b.spaces[i][j].marked = true
			}
		}
	}
}

// SumUnmarked totals all the spaces on the board which haven't been marked, this is for calculating the final answer
func (b *Board) SumUnmarked() int {
	sum := 0

	for i := 0; i < len(b.spaces); i++ {
		for j := 0; j < len(b.spaces[i]); j++ {
			if !b.spaces[i][j].marked {
				sum += b.spaces[i][j].number
			}
		}
	}

	return sum
}

// LoadInput takes an input string and parses it into the above structs
func LoadInput(input string) Game {
	game := Game{}
	lines := strings.Split(strings.ReplaceAll(input, "\r\n", "\n"), "\n")

	// load numbers as int slice
	game.numbers = helpers.StringSliceToIntSlice(strings.Split(lines[0], ","))

	// load boards
	for i := 2; i < len(lines); i += 6 {
		board := Board{}

		numbers := [][]int{
			// strings.Fields split by whitespace
			helpers.StringSliceToIntSlice(strings.Fields(lines[i])),
			helpers.StringSliceToIntSlice(strings.Fields(lines[i+1])),
			helpers.StringSliceToIntSlice(strings.Fields(lines[i+2])),
			helpers.StringSliceToIntSlice(strings.Fields(lines[i+3])),
			helpers.StringSliceToIntSlice(strings.Fields(lines[i+4])),
		}

		board.spaces = make([][]BoardSpace, len(numbers))
		for i := 0; i < len(numbers); i++ {
			board.spaces[i] = make([]BoardSpace, len(numbers[i]))
			for j := 0; j < len(numbers[i]); j++ {
				board.spaces[i][j] = BoardSpace{
					number: numbers[i][j],
					marked: false,
				}
			}
		}

		game.boards = append(game.boards, board)
	}

	return game
}

// Play recursively tests numbers against all the boards, and returns the winning board and the last number called
func Play(game Game) (Board, int) {
	for _, number := range game.numbers {
		// loop through each board in the game
		for i := 0; i < len(game.boards); i++ {
			// mark the number on the board
			game.boards[i].MarkNumber(number)

			// check for a win
			if game.boards[i].HasWin() {
				return game.boards[i], number
			}
		}
	}

	return Board{}, -1
}

// PlayLast works like Play, but returns the last board to win
func PlayLast(game Game) (Board, int) {
	for _, number := range game.numbers {
		// loop through each board in the game
		for i := len(game.boards) - 1; i >= 0; i-- {
			// mark the number on the board
			game.boards[i].MarkNumber(number)

			// check for a win, then remove it, we don't care about this board now
			if game.boards[i].HasWin() {
				if len(game.boards) == 1 {
					return game.boards[0], number
				}
				log.Printf("eliminating board %v\n", i)
				game.boards = append(game.boards[:i], game.boards[i+1:]...)
			}
		}
	}

	return Board{}, -1
}

func main() {
	input := helpers.GetInput("day4/input.txt")
	game := LoadInput(input)

	p1WinBoard, p1LastNum := Play(game)
	p1 := p1WinBoard.SumUnmarked() * p1LastNum
	fmt.Printf("part 1: %v\n", p1)

	p2WinBoard, p2LastNum := PlayLast(game)
	p2 := p2WinBoard.SumUnmarked() * p2LastNum
	fmt.Printf("part 2: %v\n", p2)
}
