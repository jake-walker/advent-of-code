package main

import (
	"github.com/google/go-cmp/cmp"
	"testing"
)

func TestHexToBinarySlice(t *testing.T) {
	actual := HexToBinarySlice("A0016C880162017C3686B18A3D47800")
	expected := []int{
		1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0,
		0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1,
		1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	}

	if diff := cmp.Diff(actual, expected); diff != "" {
		t.Errorf("HexToBinarySlice =\n%v", diff)
	}
}

func TestDecodePacket(t *testing.T) {
	tests := []struct {
		name string
		hex  string
		want DecodedPacket
	}{
		{
			"Literal Value",
			"D2FE28",
			DecodedPacket{
				Version: 6,
				TypeId:  4,
				Value:   2021,
			},
		},
		{
			"Operator 0",
			"38006F45291200",
			DecodedPacket{
				Version: 1,
				TypeId:  6,
				SubPackets: []DecodedPacket{
					{
						Version: 6,
						TypeId:  4,
						Value:   10,
					},
					{
						Version: 2,
						TypeId:  4,
						Value:   20,
					},
				},
			},
		},
		{
			"Operator 1",
			"EE00D40C823060",
			DecodedPacket{
				Version: 7,
				TypeId:  3,
				SubPackets: []DecodedPacket{
					{
						Version: 2,
						TypeId:  4,
						Value:   1,
					},
					{
						Version: 4,
						TypeId:  4,
						Value:   2,
					},
					{
						Version: 1,
						TypeId:  4,
						Value:   3,
					},
				},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, _ := DecodePacket(HexToBinarySlice(tt.hex), true)
			if diff := cmp.Diff(got, tt.want); diff != "" {
				t.Errorf("DecodePacket() =\n%v", diff)
			}
		})
	}
}

func TestSumVersionNumbers(t *testing.T) {
	actual := SumVersionNumbers(DecodedPacket{
		Version: 4,
		TypeId:  0,
		SubPackets: []DecodedPacket{
			{
				Version: 1,
				TypeId:  0,
				SubPackets: []DecodedPacket{
					{
						Version: 5,
						TypeId:  0,
						SubPackets: []DecodedPacket{
							{
								Version: 6,
								TypeId:  4,
								Value:   12,
							},
						},
					},
				},
			},
		},
	})
	var expected uint64 = 16

	if actual != expected {
		t.Errorf("SumVersionNumbers() = %v, want %v", actual, expected)
	}
}
