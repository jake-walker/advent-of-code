package main

import (
	"github.com/google/go-cmp/cmp"
	"testing"
)

var input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n" +
	"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n" +
	"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n" +
	"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n" +
	"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n" +
	"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n" +
	"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n" +
	"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n" +
	"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n" +
	"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"

func TestParseNotes(t *testing.T) {
	actual := ParseNotes("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe")
	expected := []Entry{
		{
			Inputs: []string{
				"be",
				"cfbegad",
				"cbdgef",
				"fgaecd",
				"cgeb",
				"fdcge",
				"agebfd",
				"fecdb",
				"fabcd",
				"edb",
			},
			Outputs: []string{
				"fdgacbe",
				"cefdb",
				"cefbgd",
				"gcbe",
			},
		},
	}

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("ParseNotes() = (-got, +want)\n%v", diff)
	}
}

func TestGetUniqueDigits(t *testing.T) {
	entries := ParseNotes(input)
	actual := GetUniqueDigits(entries)
	expected := 26

	if actual != expected {
		t.Errorf("GetUniqueDigits() = %v, want %v", actual, expected)
	}
}

func TestDecodeDigits(t *testing.T) {
	entries := ParseNotes("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")
	actual := DecodeDigits(entries[0])
	expected := map[int]string{
		0: "abcdeg",
		1: "ab",
		2: "acdfg",
		3: "abcdf",
		4: "abef",
		5: "bcdef",
		6: "bcdefg",
		7: "abd",
		8: "abcdefg",
		9: "abcdef",
	}

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("DecodeDigits() = (-got, +want)\n%v", diff)
	}
}

func TestDecodeOutput(t *testing.T) {
	entries := ParseNotes("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")
	decoded := DecodeDigits(entries[0])
	actual := DecodeOutput(entries[0], decoded)
	expected := 5353

	if actual != expected {
		t.Errorf("DecodeOutput() = %v, want %v", actual, expected)
	}
}
