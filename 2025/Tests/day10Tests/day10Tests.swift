import Foundation
import Testing

@testable import day10

@Test func testParse() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day10Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(
        parsed == [
            Machine(
                indicators: 0b0110,
                buttons: [
                    0b1000,
                    0b1010,
                    0b0100,
                    0b1100,
                    0b0101,
                    0b0011,
                ], joltage: [3, 5, 4, 7]
            ),
            Machine(
                indicators: 0b01000,
                buttons: [
                    0b11101,
                    0b01100,
                    0b10001,
                    0b00111,
                    0b11110,
                ], joltage: [7, 5, 12, 7, 2]
            ),
            Machine(
                indicators: 0b101110,
                buttons: [
                    0b011111,
                    0b011001,
                    0b110111,
                    0b000110,
                ], joltage: [10, 11, 11, 5, 10, 5]
            ),
        ]
    )
}

@Test(
    "Test minimum presses",
    arguments: [
        (
            Machine(
                indicators: 0b0110,
                buttons: [
                    0b1000,
                    0b1010,
                    0b0100,
                    0b1100,
                    0b0101,
                    0b0011,
                ], joltage: [3, 5, 4, 7]
            ),
            2
        ),
        (
            Machine(
                indicators: 0b01000,
                buttons: [
                    0b11101,
                    0b01100,
                    0b10001,
                    0b00111,
                    0b11110,
                ], joltage: [7, 5, 12, 7, 2]
            ),
            3
        ),
        (
            Machine(
                indicators: 0b101110,
                buttons: [
                    0b011111,
                    0b011001,
                    0b110111,
                    0b000110,
                ], joltage: [10, 11, 11, 5, 10, 5]
            ),
            2
        ),
    ])
func testMinPressesExample(machine: Machine, expectedCount: Int) async throws {
    let (presses, _) = try #require(minPresses(for: machine))

    #expect(presses == expectedCount)
}
