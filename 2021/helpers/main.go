package helpers

import (
	"log"
	"os"
	"reflect"
	"strconv"
	"strings"
)

type Point struct {
	X int
	Y int
}

func GetInput(name string) string {
	content, err := os.ReadFile(name)
	if err != nil {
		log.Fatalf("failed to read input file: %v", content)
	}
	return strings.TrimSpace(string(content))
}

func GetLines(content string) []string {
	return strings.Split(strings.ReplaceAll(content, "\r\n", "\n"), "\n")
}

func GetInputLines(name string) []string {
	content := GetInput(name)
	lines := GetLines(content)
	output := []string{}
	for _, line := range lines {
		if strings.TrimSpace(line) != "" {
			output = append(output, line)
		}
	}
	return output
}

func SplitLines(content []string) [][]string {
	output := [][]string{}
	for i := 0; i < len(content); i++ {
		output = append(output, strings.Split(content[i], ""))
	}
	return output
}

func SplitLinesInt(content []string) [][]int {
	output := [][]int{}
	for i := 0; i < len(content); i++ {
		output = append(output, StringSliceToIntSlice(strings.Split(content[i], "")))
	}
	return output
}

func GetInputIntList(name string) []int {
	content := GetInputLines(name)
	output := make([]int, len(content))
	for i, str := range content {
		if str == "" {
			continue
		}
		j, err := strconv.Atoi(str)
		if err != nil {
			log.Fatalf("failed to convert input file line to int: %v", err)
		}
		output[i] = j
	}
	return output
}

func StringSliceToIntSlice(in []string) []int {
	out := make([]int, len(in))
	for i, s := range in {
		j, err := strconv.Atoi(s)
		if err != nil {
			log.Fatalf("failed to convert slice element %v to int: %v", out[i], err)
		}
		out[i] = j
	}
	return out
}

func Contains(search interface{}, slice interface{}) bool {
	sliceVal := reflect.ValueOf(slice)
	for i := 0; i < sliceVal.Len(); i++ {
		if sliceVal.Index(i) == search {
			return true
		}
	}

	return false
}

func MinMax(slice []int) (min, max int) {
	min, max = slice[0], slice[0]
	for _, i := range slice {
		if max < i {
			max = i
		}
		if min > i {
			min = i
		}
	}
	return
}
