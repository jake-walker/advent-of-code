package main

import (
	"fmt"
	"github.com/jake-walker/advent-of-code-2021/helpers"
	"log"
	"strconv"
	"strings"
)

type DecodedPacket struct {
	Version    uint64
	TypeId     uint64
	Value      uint64
	SubPackets []DecodedPacket
}

func HexToBinarySlice(hex string) []int {
	out := []int{}
	hexSlice := strings.Split(hex, "")

	// there is a built-in function for converting hex to an int, then int to binary, but the numbers we are working
	// with are larger than 64-bit so this is just easier than chunking up the hex and parsing in 64-bit chunks

	for i := 0; i < len(hexSlice); i++ {
		var s []int

		switch strings.ToLower(hexSlice[i]) {
		case "0":
			s = []int{0, 0, 0, 0}
		case "1":
			s = []int{0, 0, 0, 1}
		case "2":
			s = []int{0, 0, 1, 0}
		case "3":
			s = []int{0, 0, 1, 1}
		case "4":
			s = []int{0, 1, 0, 0}
		case "5":
			s = []int{0, 1, 0, 1}
		case "6":
			s = []int{0, 1, 1, 0}
		case "7":
			s = []int{0, 1, 1, 1}
		case "8":
			s = []int{1, 0, 0, 0}
		case "9":
			s = []int{1, 0, 0, 1}
		case "a":
			s = []int{1, 0, 1, 0}
		case "b":
			s = []int{1, 0, 1, 1}
		case "c":
			s = []int{1, 1, 0, 0}
		case "d":
			s = []int{1, 1, 0, 1}
		case "e":
			s = []int{1, 1, 1, 0}
		case "f":
			s = []int{1, 1, 1, 1}
		}

		out = append(out, s...)
	}

	return out
}

func IntSliceToString(slice []int) string {
	str := []string{}

	for _, el := range slice {
		str = append(str, fmt.Sprint(el))
	}

	return strings.Join(str, "")
}

func BinarySliceToInt(binary []int) uint64 {
	i, err := strconv.ParseUint(IntSliceToString(binary), 2, 64)
	if err != nil {
		log.Fatalf("failed to convert binary string: %v", err)
	}

	return i
}

func DecodeLiteralValue(binary []int) (uint64, []int) {
	log.Printf("decoding literal value %v", IntSliceToString(binary))

	final := ""
	remainder := len(binary) - 1

	for i := 0; i < len(binary); i = i + 5 {
		final += IntSliceToString(binary[i+1 : i+5])

		// if this group of binary starts with 0, exit the loop
		if binary[i] == 0 {
			remainder = i + 5
			break
		}
	}

	n, err := strconv.ParseUint(final, 2, 64)
	if err != nil {
		log.Fatalf("failed to convert binary: %v", err)
	}

	log.Printf("decoded as %v, %v bits remainder", n, len(binary)-remainder)

	return n, binary[remainder:]
}

func DecodePacket(bin []int, pad bool) (DecodedPacket, []int) {
	if pad {
		// add padding zeros to bring length to multiple of 4
		for i := 0; i < len(bin)%4; i++ {
			bin = append([]int{0}, bin...)
		}
	}

	version := BinarySliceToInt(bin[0:3])
	typeId := BinarySliceToInt(bin[3:6])

	log.Printf("processing packet: %v (v=%v, t=%v)", IntSliceToString(bin), version, typeId)

	if typeId == 4 {
		log.Println("this packet is a literal value")

		// literal value
		value, remainder := DecodeLiteralValue(bin[6:])
		return DecodedPacket{
			Version: version,
			TypeId:  typeId,
			Value:   value,
		}, remainder
	}

	// -- decode operator packets --

	log.Println("this packet is an operator packet")
	lengthTypeId := bin[6]
	if lengthTypeId == 0 {
		// next 15 represent total length in bits
		length := BinarySliceToInt(bin[7:22])
		subpackets := bin[22 : 22+length]
		remainder := bin[22+length:]
		log.Printf("subpackets of %v length -> %v", length, IntSliceToString(subpackets))

		subPackets := []DecodedPacket{}

		for len(subpackets) > 0 {
			var p DecodedPacket
			p, subpackets = DecodePacket(subpackets, false)
			subPackets = append(subPackets, p)
		}

		return DecodedPacket{
			Version:    version,
			TypeId:     typeId,
			SubPackets: subPackets,
		}, remainder
	} else if lengthTypeId == 1 {
		// next 11 are number of sub-packets
		num := BinarySliceToInt(bin[7:18])
		log.Printf("there are %v subpackets", num)

		remainder := bin[18:]
		subPackets := []DecodedPacket{}
		for i := uint64(0); i < num; i++ {
			var p DecodedPacket
			p, remainder = DecodePacket(remainder, false)
			subPackets = append(subPackets, p)
		}

		return DecodedPacket{
			Version:    version,
			TypeId:     typeId,
			SubPackets: subPackets,
		}, remainder
	}

	log.Fatalln("failed to decode")
	return DecodedPacket{}, []int{}
}

func SumVersionNumbers(packet DecodedPacket) uint64 {
	sum := packet.Version
	for _, sub := range packet.SubPackets {
		sum += SumVersionNumbers(sub)
	}
	return sum
}

func CalculateValue(packet DecodedPacket) DecodedPacket {
	if packet.TypeId == 4 {
		return packet
	}

	values := []uint64{}
	for _, sub := range packet.SubPackets {
		values = append(values, CalculateValue(sub).Value)
	}

	switch packet.TypeId {
	case 0:
		// sum
		var sum uint64 = 0
		for _, value := range values {
			sum += value
		}
		packet.Value = sum
	case 1:
		// product
		var product uint64 = 1
		for _, value := range values {
			product *= value
		}
		packet.Value = product
	case 2:
		// minimum
		var min = values[0]
		for _, value := range values {
			if value < min {
				min = value
			}
		}
		packet.Value = min
	case 3:
		// maximum
		var max = values[0]
		for _, value := range values {
			if value > max {
				max = value
			}
		}
		packet.Value = max
	case 5:
		// greater than
		if values[0] > values[1] {
			packet.Value = 1
		} else {
			packet.Value = 0
		}
	case 6:
		// less than
		if values[0] < values[1] {
			packet.Value = 1
		} else {
			packet.Value = 0
		}
	case 7:
		// equal to
		if values[0] == values[1] {
			packet.Value = 1
		} else {
			packet.Value = 0
		}
	default:
		log.Fatalf("unknown packet type %v", packet.TypeId)
	}

	return packet
}

func main() {
	input := helpers.GetInput("day16/input.txt")
	binary := HexToBinarySlice(input)

	packet, _ := DecodePacket(binary, true)
	part1 := SumVersionNumbers(packet)
	fmt.Printf("part 1: %v\n", part1)

	part2 := CalculateValue(packet).Value
	fmt.Printf("part 2: %v\n", part2)
}
