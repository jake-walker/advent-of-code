package main

import (
	"github.com/google/go-cmp/cmp"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"testing"
)

func TestLoadInput(t *testing.T) {
	input := helpers.GetLines("NNCB\n\n" +
		"CH -> B\n" +
		"HH -> N\n" +
		"CB -> H\n" +
		"NH -> C\n" +
		"HB -> C\n" +
		"HC -> B\n" +
		"HN -> C\n" +
		"NN -> C\n" +
		"BH -> H\n" +
		"NC -> B\n" +
		"NB -> B\n" +
		"BN -> B\n" +
		"BB -> N\n" +
		"BC -> B\n" +
		"CC -> N\n" +
		"CN -> C")
	actualTemplate, actualPairs := LoadInput(input)
	expectedTemplate := map[string]int{
		"NN": 1,
		"NC": 1,
		"CB": 1,
	}
	expectedPairs := map[string]string{
		"CH": "B",
		"HH": "N",
		"CB": "H",
		"NH": "C",
		"HB": "C",
		"HC": "B",
		"HN": "C",
		"NN": "C",
		"BH": "H",
		"NC": "B",
		"NB": "B",
		"BN": "B",
		"BB": "N",
		"BC": "B",
		"CC": "N",
		"CN": "C",
	}

	if diff := cmp.Diff(actualTemplate, expectedTemplate); diff != "" {
		t.Errorf("LoadInput() template =\n%v", diff)
	}

	if diff := cmp.Diff(actualPairs, expectedPairs); diff != "" {
		t.Errorf("LoadInput() pairs =\n%v", diff)
	}
}

func TestInsertPairs(t *testing.T) {
	pairs := map[string]string{
		"CH": "B",
		"HH": "N",
		"CB": "H",
		"NH": "C",
		"HB": "C",
		"HC": "B",
		"HN": "C",
		"NN": "C",
		"BH": "H",
		"NC": "B",
		"NB": "B",
		"BN": "B",
		"BB": "N",
		"BC": "B",
		"CC": "N",
		"CN": "C",
	}
	template := map[string]int{
		"NN": 1,
		"NC": 1,
		"CB": 1,
	}
	actual := InsertPairs(template, pairs)
	expected := map[string]int{
		"NC": 1,
		"CN": 1,
		"NB": 1,
		"BC": 1,
		"CH": 1,
		"HB": 1,
	}

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("InsertPairs() =\n%v", diff)
	}
}

func TestInsertLoop10(t *testing.T) {
	pairs := map[string]string{
		"CH": "B",
		"HH": "N",
		"CB": "H",
		"NH": "C",
		"HB": "C",
		"HC": "B",
		"HN": "C",
		"NN": "C",
		"BH": "H",
		"NC": "B",
		"NB": "B",
		"BN": "B",
		"BB": "N",
		"BC": "B",
		"CC": "N",
		"CN": "C",
	}
	template := map[string]int{
		"NN": 1,
		"NC": 1,
		"CB": 1,
	}
	actual := InsertLoop(template, pairs, 10)
	expected := 1588

	if actual != expected {
		t.Errorf("InsertLoop() = %v, want %v", actual, expected)
	}
}

func TestInsertLoop40(t *testing.T) {
	pairs := map[string]string{
		"CH": "B",
		"HH": "N",
		"CB": "H",
		"NH": "C",
		"HB": "C",
		"HC": "B",
		"HN": "C",
		"NN": "C",
		"BH": "H",
		"NC": "B",
		"NB": "B",
		"BN": "B",
		"BB": "N",
		"BC": "B",
		"CC": "N",
		"CN": "C",
	}
	template := map[string]int{
		"NN": 1,
		"NC": 1,
		"CB": 1,
	}
	actual := InsertLoop(template, pairs, 40)
	expected := 2188189693529

	if actual != expected {
		t.Errorf("InsertLoop() = %v, want %v", actual, expected)
	}
}
