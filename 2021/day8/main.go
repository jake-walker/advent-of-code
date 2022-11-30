package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"sort"
	"strconv"
	"strings"
)

type Entry struct {
	Inputs  []string
	Outputs []string
}

func ParseNotes(notes string) []Entry {
	entries := []Entry{}

	for _, line := range helpers.GetLines(notes) {
		if line == "" {
			continue
		}

		fields := strings.Split(line, " | ")
		inputs := strings.Fields(fields[0])
		outputs := strings.Fields(fields[1])

		entries = append(entries, Entry{
			Inputs:  inputs,
			Outputs: outputs,
		})
	}

	return entries
}

func GetUniqueDigits(entries []Entry) int {
	count := 0
	for _, entry := range entries {
		for _, output := range entry.Outputs {
			length := len(output)
			if length == 2 || length == 4 || length == 7 || length == 3 {
				count += 1
			}
		}
	}
	return count
}

func ToLengthMap(slice []string) map[int][]string {
	out := make(map[int][]string)
	for _, item := range slice {
		sorted := strings.Split(item, "")
		sort.Strings(sorted)

		out[len(item)] = append(out[len(item)], strings.Join(sorted, ""))
	}
	return out
}

func ToCharCountMap(slice []string) map[string]int {
	out := make(map[string]int)
	for _, s := range slice {
		chars := strings.Split(s, "")
		for _, char := range chars {
			out[char] += 1
		}
	}
	return out
}

func SearchByCharCounts(items []string, counts map[string]int, below int) []string {
	target := ""
	out := []string{}

	for k, count := range counts {
		if count == below {
			target = k
			break
		}
	}

	for _, item := range items {
		if strings.Contains(item, target) {
			out = append(out, item)
		}
	}

	return out
}

func GetNot(slice []string, not []string) []string {
	out := []string{}

	for _, item := range slice {
		found := false

		for _, item1 := range not {
			if item == item1 {
				found = true
				break
			}
		}

		if !found {
			out = append(out, item)
		}
	}
	return out
}

func SubtractString(a string, b string) string {
	out := ""

	for _, char := range strings.Split(a, "") {
		if !strings.Contains(b, char) {
			out += char
		}
	}

	return out
}

func DecodeDigits(entry Entry) map[int]string {
	digits := make(map[int]string)
	lengths := ToLengthMap(entry.Inputs)

	// digits 1, 4, 7 and 8 have a unique number of segments
	digits[1] = lengths[2][0]
	digits[7] = lengths[3][0]
	digits[4] = lengths[4][0]
	digits[8] = lengths[7][0]

	// 5 SEGMENT NUMBERS
	// digit 2 and 5 both have one unique segment, adding in 4 will ensure the 2 is chosen
	counts := ToCharCountMap(append(lengths[5], digits[4]))
	digits[2] = SearchByCharCounts(lengths[5], counts, 1)[0]

	// adding in 2 with the 5 segment numbers will eliminate the other segment, leaving 5
	counts = ToCharCountMap(append(lengths[5], digits[2]))
	digits[5] = SearchByCharCounts(lengths[5], counts, 1)[0]

	// the only other number with 5 segments is 3
	digits[3] = GetNot(lengths[5], []string{digits[2], digits[5]})[0]

	// 6 SEGMENT NUMBERS
	topRight := SubtractString(digits[1], digits[5])

	// adding a 3, gives means the least occurring segment can be a six or zero
	counts = ToCharCountMap(append(lengths[6], digits[3]))
	sixZero := SearchByCharCounts(lengths[6], counts, 2)
	if strings.Contains(sixZero[0], topRight) {
		digits[0], digits[6] = sixZero[0], sixZero[1]
	} else {
		digits[0], digits[6] = sixZero[1], sixZero[0]
	}

	digits[9] = GetNot(lengths[6], []string{digits[0], digits[6]})[0]

	return digits
}

func DecodeOutput(entry Entry, decoded map[int]string) int {
	out := ""

	for _, output := range entry.Outputs {
		sorted := strings.Split(output, "")
		sort.Strings(sorted)
		output = strings.Join(sorted, "")

		for number, digits := range decoded {
			if digits == output {
				out += fmt.Sprint(number)
				break
			}
		}
	}

	n, err := strconv.Atoi(out)
	if err != nil {
		log.Fatalf("failed to convert string to int: %v", err)
	}

	return n
}

func main() {
	input := helpers.GetInput("day8/input.txt")
	entries := ParseNotes(input)

	part1 := GetUniqueDigits(entries)
	fmt.Printf("part 1: %v\n", part1)

	part2 := 0
	for _, entry := range entries {
		decoded := DecodeDigits(entry)
		part2 += DecodeOutput(entry, decoded)
	}
	fmt.Printf("part 2: %v\n", part2)
}
